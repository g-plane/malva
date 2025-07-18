use crate::config::resolve_config;
use anyhow::{Error, Result};
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration},
    plugins::{
        CheckConfigUpdatesMessage, ConfigChange, FormatResult, PluginInfo,
        PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest,
        SyncPluginHandler,
    },
};
use malva::{config::FormatOptions, detect_syntax, format_text};

mod config;

pub struct MalvaPluginHandler;

impl SyncPluginHandler<FormatOptions> for MalvaPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "malva".to_string(),
            help_url: "https://github.com/g-plane/malva".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/g-plane/malva/v{version}/schema.json",
            ),
            update_url: Some("https://plugins.dprint.dev/g-plane/malva/latest.json".into()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../../LICENSE").into()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<FormatOptions> {
        resolve_config(config, global_config)
    }

    fn check_config_updates(&self, _: CheckConfigUpdatesMessage) -> Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<FormatOptions>,
        _: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        let Some(syntax) = detect_syntax(request.file_path) else {
            return Err(anyhow::anyhow!(
                "unknown file extension of file: {}",
                request.file_path.display()
            ));
        };
        format_text(
            std::str::from_utf8(&request.file_bytes)?,
            syntax,
            request.config,
        )
        .map(|s| Some(s.into_bytes()))
        .map_err(Error::from)
    }
}

#[cfg(target_arch = "wasm32")]
dprint_core::generate_plugin_code!(MalvaPluginHandler, MalvaPluginHandler, FormatOptions);
