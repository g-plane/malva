use clap::Parser;
use std::{env, path::PathBuf, sync::Arc};
use tokio::{
    fs,
    io::{self, AsyncWriteExt},
    task::spawn_blocking,
};

mod config;
mod format;
mod search;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Specify config file path.
    #[arg(short, long)]
    config: Option<String>,

    /// Format then write to original files.
    #[arg(short, long)]
    write: bool,

    #[arg(required(true))]
    files: Vec<String>,

    /// Specify which files should be ignored.
    #[arg(long)]
    ignore: Vec<String>,

    #[arg(long)]
    cwd: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let base_dir = args
        .cwd
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    let config = Arc::new(config::resolve_config(&base_dir).await?);
    let files = search::search_files(&base_dir, &args.files, &args.ignore)?;

    let tasks = files
        .into_iter()
        .map(|file| async {
            let file = Arc::new(file);
            let config = config.clone();

            let content = fs::read_to_string(&*file).await?;
            let formatted = {
                let file = file.clone();
                spawn_blocking(move || format::format_file(content, &file, &config)).await??
            };

            if args.write {
                fs::write(&*file, formatted).await?;
            } else {
                let mut stdout = io::stdout();
                stdout.write_all(formatted.as_bytes()).await?;
            }
            Ok::<_, anyhow::Error>(())
        })
        .map(|result| async {
            if let Err(error) = result.await {
                let mut stderr = io::stderr();
                let _ = stderr.write_all(error.to_string().as_bytes()).await;
            }
        });
    futures::future::join_all(tasks).await;

    Ok(())
}
