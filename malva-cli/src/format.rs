use anyhow::{Error, Result};
use malva::{config::FormatOptions, format_text, Syntax};
use std::path::Path;

pub fn format_file(code: String, file_path: &Path, options: &FormatOptions) -> Result<String> {
    let syntax = match file_path.extension().and_then(|s| s.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("css") => Syntax::Css,
        Some(ext) if ext.eq_ignore_ascii_case("scss") => Syntax::Scss,
        Some(ext) if ext.eq_ignore_ascii_case("sass") => Syntax::Sass,
        Some(ext) if ext.eq_ignore_ascii_case("less") => Syntax::Less,
        _ => {
            return Err(anyhow::anyhow!(
                "unknown file extension of file: {}",
                file_path.display()
            ));
        }
    };

    format_text(&code, syntax, options).map_err(Error::from)
}
