use dprint_core::configuration::{
    get_unknown_property_diagnostics, get_value, ConfigKeyMap, ConfigurationDiagnostic,
    GlobalConfiguration, ResolveConfigurationResult,
};
use malva::config::{
    BlockSelectorLineBreak, FormatOptions, HexCase, LanguageOptions, LayoutOptions, LineBreak,
    OperatorLineBreak, Quotes,
};

pub(crate) fn resolve_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<FormatOptions> {
    let mut diagnostics = Vec::new();
    let malva_config = FormatOptions {
        layout: LayoutOptions {
            print_width: get_value(
                &mut config,
                "printWidth",
                global_config.line_width.unwrap_or(80),
                &mut diagnostics,
            ) as usize,
            use_tabs: get_value(
                &mut config,
                "useTabs",
                global_config.use_tabs.unwrap_or_default(),
                &mut diagnostics,
            ),
            indent_width: get_value(
                &mut config,
                "indentWidth",
                global_config.indent_width.unwrap_or(2),
                &mut diagnostics,
            ) as usize,
            line_break: match &*get_value(
                &mut config,
                "lineBreak",
                "lf".to_string(),
                &mut diagnostics,
            ) {
                "lf" => LineBreak::Lf,
                "crlf" => LineBreak::Crlf,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "lineBreak".into(),
                        message: "invalid value for config `lineBreak`".into(),
                    });
                    LineBreak::Lf
                }
            },
        },
        language: LanguageOptions {
            hex_case: match &*get_value(
                &mut config,
                "hexCase",
                "lower".to_string(),
                &mut diagnostics,
            ) {
                "ignore" => HexCase::Ignore,
                "lower" => HexCase::Lower,
                "upper" => HexCase::Upper,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "hexCase".into(),
                        message: "invalid value for config `hexCase`".into(),
                    });
                    HexCase::Lower
                }
            },
            quotes: match &*get_value(
                &mut config,
                "quotes",
                "alwaysDouble".to_string(),
                &mut diagnostics,
            ) {
                "alwaysDouble" => Quotes::AlwaysDouble,
                "alwaysSingle" => Quotes::AlwaysSingle,
                "preferDouble" => Quotes::PreferDouble,
                "preferSingle" => Quotes::PreferSingle,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "quotes".into(),
                        message: "invalid value for config `quotes`".into(),
                    });
                    Quotes::AlwaysDouble
                }
            },
            operator_linebreak: match &*get_value(
                &mut config,
                "operatorLineBreak",
                "after".to_string(),
                &mut diagnostics,
            ) {
                "before" => OperatorLineBreak::Before,
                "after" => OperatorLineBreak::After,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "operatorLineBreak".into(),
                        message: "invalid value for config `operatorLineBreak`".into(),
                    });
                    OperatorLineBreak::After
                }
            },
            block_selector_linebreak: match &*get_value(
                &mut config,
                "blockSelectorLinebreak",
                "consistent".to_string(),
                &mut diagnostics,
            ) {
                "always" => BlockSelectorLineBreak::Always,
                "consistent" => BlockSelectorLineBreak::Consistent,
                "wrap" => BlockSelectorLineBreak::Wrap,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "blockSelectorLinebreak".into(),
                        message: "invalid value for config `blockSelectorLinebreak`".into(),
                    });
                    BlockSelectorLineBreak::Consistent
                }
            },
            omit_number_leading_zero: get_value(
                &mut config,
                "omitNumberLeadingZero",
                false,
                &mut diagnostics,
            ),
            trailing_comma: get_value(&mut config, "trailingComma", false, &mut diagnostics),
            pad_comments: get_value(&mut config, "padComments", false, &mut diagnostics),
        },
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: malva_config,
        diagnostics,
    }
}
