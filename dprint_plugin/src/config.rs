use dprint_core::configuration::{
    get_nullable_value, get_unknown_property_diagnostics, get_value, ConfigKeyMap,
    ConfigurationDiagnostic, GlobalConfiguration, NewLineKind, ResolveConfigurationResult,
};
use malva::config::{
    BlockSelectorLineBreak, DeclarationOrder, FormatOptions, HexCase, LanguageOptions,
    LayoutOptions, LineBreak, OperatorLineBreak, Quotes,
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
                match global_config.new_line_kind {
                    Some(NewLineKind::LineFeed) => "lf",
                    Some(NewLineKind::CarriageReturnLineFeed) => "crlf",
                    _ => "lf",
                }
                .to_string(),
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
                    Default::default()
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
                    Default::default()
                }
            },
            operator_linebreak: match &*get_value(
                &mut config,
                "operatorLinebreak",
                "after".to_string(),
                &mut diagnostics,
            ) {
                "before" => OperatorLineBreak::Before,
                "after" => OperatorLineBreak::After,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "operatorLinebreak".into(),
                        message: "invalid value for config `operatorLinebreak`".into(),
                    });
                    Default::default()
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
                    Default::default()
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
            linebreak_in_pseudo_parens: get_value(
                &mut config,
                "linebreakInPseudoParens",
                false,
                &mut diagnostics,
            ),
            declaration_order: get_nullable_value::<String>(
                &mut config,
                "declarationOrder",
                &mut diagnostics,
            )
            .as_deref()
            .and_then(|value| match value {
                "alphabetical" => Some(DeclarationOrder::Alphabetical),
                "smacss" => Some(DeclarationOrder::Smacss),
                "concentric" => Some(DeclarationOrder::Concentric),
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "declarationOrder".into(),
                        message: "invalid value for config `declarationOrder`".into(),
                    });
                    None
                }
            }),
            single_line_block_threshold: get_nullable_value(
                &mut config,
                "singleLineBlockThreshold",
                &mut diagnostics,
            ),
        },
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: malva_config,
        diagnostics,
    }
}
