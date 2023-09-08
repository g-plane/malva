use anyhow::{Error, Result};
use malva::config::FormatOptions;
use std::path::Path;
use tokio::{fs, io};

pub async fn resolve_config(base_dir: &Path) -> Result<FormatOptions> {
    match fs::read_to_string(base_dir.join("malva.toml")).await {
        Ok(content) => return toml::from_str(&content).map_err(Error::from),
        Err(error) => {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
    }

    match fs::read_to_string(base_dir.join("malva.json")).await {
        Ok(content) => return serde_json::from_str(&content).map_err(Error::from),
        Err(error) => {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
    }

    Ok(FormatOptions::default())
}
