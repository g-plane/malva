use crate::config::resolve_config;
use anyhow::{Error, Result};
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult},
    plugins::{FileMatchingInfo, PluginInfo, SyncPluginHandler, SyncPluginInfo},
};
use malva::{config::FormatOptions, format_text, Syntax};
use std::path::Path;

mod config;

#[cfg(target_arch = "wasm32")]
type Configuration = FormatOptions;

pub struct MalvaPluginHandler {}

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
        file_text: &str,
        config: &FormatOptions,
        _: impl FnMut(&Path, String, &ConfigKeyMap) -> Result<Option<String>>,
    ) -> Result<Option<String>> {
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
        format_text(file_text, syntax, config)
            .map(Some)
            .map_err(Error::from)
    }
}

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(MalvaPluginHandler, MalvaPluginHandler {});
