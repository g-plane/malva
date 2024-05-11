use malva::{config::FormatOptions, detect_syntax, format_text};
use std::{env, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = env::args().nth(1).unwrap();
    let syntax = detect_syntax(&file_path).unwrap();
    let input = fs::read_to_string(&file_path)?;
    let options = match fs::read_to_string("malva.toml") {
        Ok(s) => toml::from_str(&s)?,
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                FormatOptions::default()
            } else {
                return Err(Box::new(error));
            }
        }
    };

    let formatted = format_text(&input, syntax, &options)?;
    print!("{formatted}");
    Ok(())
}
