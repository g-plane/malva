use crate::config::resolve_config;
use anyhow::{Error, Result};
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{FileMatchingInfo, PluginInfo, SyncPluginHandler, SyncPluginInfo},
};
use malva::{config::FormatOptions, detect_syntax, format_text};
use std::path::Path;

mod config;

#[cfg(target_arch = "wasm32")]
type Configuration = FormatOptions;

pub struct MalvaPluginHandler;

impl SyncPluginHandler<FormatOptions> for MalvaPluginHandler {
    fn plugin_info(&mut self) -> SyncPluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        SyncPluginInfo {
            info: PluginInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: version.clone(),
                config_key: "malva".to_string(),
                help_url: "https://github.com/g-plane/malva".to_string(),
                config_schema_url: format!(
                    "https://plugins.dprint.dev/g-plane/malva/v{}/schema.json",
                    version
                ),
                update_url: Some("https://plugins.dprint.dev/g-plane/malva/latest.json".into()),
            },
            file_matching: FileMatchingInfo {
                file_extensions: vec!["css".into(), "scss".into(), "sass".into(), "less".into()],
                file_names: vec![],
            },
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../../LICENSE").into()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<FormatOptions> {
        resolve_config(config, global_config)
    }

    fn format(
        &mut self,
        file_path: &Path,
        file_text: Vec<u8>,
        config: &FormatOptions,
        _: impl FnMut(&Path, Vec<u8>, &ConfigKeyMap) -> Result<Option<Vec<u8>>>,
    ) -> Result<Option<Vec<u8>>> {
        let Some(syntax) = detect_syntax(file_path) else {
            return Err(anyhow::anyhow!(
                "unknown file extension of file: {}",
                file_path.display()
            ));
        };
        format_text(std::str::from_utf8(&file_text)?, syntax, config)
            .map(|s| Some(s.into_bytes()))
            .map_err(Error::from)
    }
}

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(MalvaPluginHandler, MalvaPluginHandler);
