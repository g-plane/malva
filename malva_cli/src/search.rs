use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use std::{
    borrow::Cow,
    path::{self, Path, PathBuf},
};
use walkdir::WalkDir;

pub fn search_files(
    base_dir: &PathBuf,
    files: &[impl AsRef<str>],
    ignore: &[String],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut ignore_pats = vec![String::from("**/node_modules/**")];
    ignore_pats.extend_from_slice(ignore);
    let ignore = build_glob_set(&ignore_pats)?;

    let inputs = build_glob_set(files)?;
    WalkDir::new(base_dir)
        .into_iter()
        .map(|entry| entry.map(|entry| entry.into_path()))
        .filter(|path| {
            path.as_ref()
                .map(|path| {
                    // SAFETY: all the paths are walked from `base_dir`,
                    // so it should be safe to strip that prefix.
                    let path = path.strip_prefix(base_dir).unwrap();
                    inputs.is_match(path) && !ignore.is_match(path)
                })
                .unwrap_or_default()
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(anyhow::Error::from)
}

fn build_glob_set(globs: &[impl AsRef<str>]) -> Result<GlobSet, globset::Error> {
    globs
        .iter()
        .try_fold(GlobSetBuilder::new(), |mut globset_builder, glob_raw| {
            let glob_raw = glob_raw
                .as_ref()
                .trim_end_matches([path::MAIN_SEPARATOR, '/']); // remove trailing path separator
            let glob = if Path::new(glob_raw).is_dir() {
                // `globset` can't recognize if it's a directory,
                // so we do a trick here.
                Cow::from(format!("{glob_raw}/**/*.{{css,scss,sass,less}}"))
            } else {
                Cow::from(glob_raw)
            };
            // Remove the beginning "current directory" mark if existed,
            // because when iterating input files, their paths won't start with that mark,
            // otherwise the matching will fail.
            let mut glob_builder =
                GlobBuilder::new(glob.trim_start_matches("./").trim_start_matches(".\\"));
            glob_builder.literal_separator(true);
            globset_builder.add(glob_builder.build()?);
            Ok(globset_builder)
        })?
        .build()
}
