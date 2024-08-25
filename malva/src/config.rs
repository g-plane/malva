//! Types about configuration.

#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// The whole configuration of Malva.
///
/// For detail, please refer to [Configuration](https://github.com/g-plane/malva/blob/main/docs/config.md).
pub struct FormatOptions {
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub layout: LayoutOptions,
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub language: LanguageOptions,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to layout, such as indentation or print width.
pub struct LayoutOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "printWidth"))]
    /// See [`printWidth`](https://malva.netlify.app/config/print-width.html)
    pub print_width: usize,

    #[cfg_attr(feature = "config_serde", serde(alias = "useTabs"))]
    /// See [`useTabs`](https://malva.netlify.app/config/use-tabs.html)
    pub use_tabs: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "indentWidth"))]
    /// See [`indentWidth`](https://malva.netlify.app/config/indent-width.html)
    pub indent_width: usize,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "lineBreak", alias = "linebreak")
    )]
    /// See [`lineBreak`](https://malva.netlify.app/config/line-break.html)
    pub line_break: LineBreak,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            print_width: 80,
            use_tabs: false,
            indent_width: 2,
            line_break: LineBreak::Lf,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum LineBreak {
    #[default]
    Lf,
    Crlf,
}

impl From<LineBreak> for tiny_pretty::LineBreak {
    fn from(value: LineBreak) -> Self {
        match value {
            LineBreak::Lf => tiny_pretty::LineBreak::Lf,
            LineBreak::Crlf => tiny_pretty::LineBreak::Crlf,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to syntax.
pub struct LanguageOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "hexCase"))]
    /// See [`hexCase`](https://malva.netlify.app/config/hex-case.html)
    pub hex_case: HexCase,

    #[cfg_attr(feature = "config_serde", serde(alias = "hexColorLength"))]
    /// See [`hexColorLength`](https://malva.netlify.app/config/hex-color-length.html)
    pub hex_color_length: Option<HexColorLength>,

    /// See [`quotes`](https://malva.netlify.app/config/quotes.html)
    pub quotes: Quotes,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "operatorLinebreak", alias = "operatorLineBreak")
    )]
    /// See [`operatorLinebreak`](https://malva.netlify.app/config/operator-linebreak.html)
    pub operator_linebreak: OperatorLineBreak,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "blockSelectorLinebreak", alias = "blockSelectorLineBreak")
    )]
    /// See [`blockSelectorLinebreak`](https://malva.netlify.app/config/block-selector-linebreak.html)
    pub block_selector_linebreak: BlockSelectorLineBreak,

    #[cfg_attr(feature = "config_serde", serde(alias = "omitNumberLeadingZero"))]
    /// See [`omitNumberLeadingZero`](https://malva.netlify.app/config/omit-number-leading-zero.html)
    pub omit_number_leading_zero: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "trailingComma"))]
    /// See [`trailingComma`](https://malva.netlify.app/config/trailing-comma.html)
    pub trailing_comma: bool,

    #[cfg_attr(
        feature = "config_serde",
        serde(
            alias = "formatComments",
            alias = "pad_comments", // for backward compatibility
            alias = "padComments" // for backward compatibility
        )
    )]
    /// See [`formatComments`](https://malva.netlify.app/config/format-comments.html)
    pub format_comments: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "alignComments"))]
    pub align_comments: bool,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "linebreakInPseudoParens", alias = "lineBreakInPseudoParens")
    )]
    /// See [`linebreakInPseudoParens`](https://malva.netlify.app/config/linebreak-in-pseudo-parens.html)
    pub linebreak_in_pseudo_parens: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "declarationOrder"))]
    /// See [`declarationOrder`](https://malva.netlify.app/config/declaration-order.html)
    pub declaration_order: Option<DeclarationOrder>,

    #[cfg_attr(feature = "config_serde", serde(alias = "singleLineBlockThreshold"))]
    /// See [`singleLineBlockThreshold`](https://malva.netlify.app/config/single-line-block-threshold.html)
    pub single_line_block_threshold: Option<usize>,

    #[cfg_attr(feature = "config_serde", serde(alias = "keyframeSelectorNotation"))]
    /// See [`keyframeSelectorNotation`](https://malva.netlify.app/config/keyframe-selector-notation.html)
    pub keyframe_selector_notation: Option<KeyframeSelectorNotation>,

    #[cfg_attr(feature = "config_serde", serde(alias = "attrValueQuotes"))]
    /// See [`attrValueQuotes`](https://malva.netlify.app/config/attr-value-quotes.html)
    pub attr_value_quotes: AttrValueQuotes,

    #[cfg_attr(feature = "config_serde", serde(alias = "preferSingleLine"))]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub prefer_single_line: bool,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "selectors.prefer_single_line",
            alias = "selectors.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub selectors_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "function_args.prefer_single_line",
            alias = "functionArgs.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub function_args_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "sass_content_at_rule.prefer_single_line",
            alias = "sassContentAtRule.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub sass_content_at_rule_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "sass_include_at_rule.prefer_single_line",
            alias = "sassIncludeAtRule.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub sass_include_at_rule_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "sass_map.prefer_single_line",
            alias = "sassMap.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub sass_map_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "sass_module_config.prefer_single_line",
            alias = "sassModuleConfig.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub sass_module_config_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "sass_params.prefer_single_line",
            alias = "sassParams.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub sass_params_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "less_import_options.prefer_single_line",
            alias = "lessImportOptions.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub less_import_options_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "less_mixin_args.prefer_single_line",
            alias = "lessMixinArgs.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub less_mixin_args_prefer_single_line: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "less_mixin_params.prefer_single_line",
            alias = "lessMixinParams.preferSingleLine"
        )
    )]
    /// See [`preferSingleLine`](https://malva.netlify.app/config/prefer-single-line.html)
    pub less_mixin_params_prefer_single_line: Option<bool>,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "singleLineTopLevelDeclarations")
    )]
    /// See [`singleLineTopLevelDeclarations`](https://malva.netlify.app/config/single-line-top-level-declarations.html)
    pub single_line_top_level_declarations: bool,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "selectorOverrideCommentDirective")
    )]
    /// See [`selectorOverrideCommentDirective`](https://malva.netlify.app/config/selector-override-comment-directive.html)
    pub selector_override_comment_directive: String,

    #[cfg_attr(feature = "config_serde", serde(alias = "ignoreCommentDirective"))]
    /// See [`ignoreCommentDirective`](https://malva.netlify.app/config/ignore-comment-directive.html)
    pub ignore_comment_directive: String,
}

impl Default for LanguageOptions {
    fn default() -> Self {
        LanguageOptions {
            hex_case: HexCase::default(),
            hex_color_length: None,
            quotes: Quotes::default(),
            operator_linebreak: OperatorLineBreak::default(),
            block_selector_linebreak: BlockSelectorLineBreak::default(),
            omit_number_leading_zero: false,
            trailing_comma: false,
            format_comments: false,
            align_comments: true,
            linebreak_in_pseudo_parens: false,
            declaration_order: None,
            single_line_block_threshold: None,
            keyframe_selector_notation: None,
            attr_value_quotes: AttrValueQuotes::default(),
            prefer_single_line: false,
            selectors_prefer_single_line: None,
            function_args_prefer_single_line: None,
            sass_content_at_rule_prefer_single_line: None,
            sass_include_at_rule_prefer_single_line: None,
            sass_map_prefer_single_line: None,
            sass_module_config_prefer_single_line: None,
            sass_params_prefer_single_line: None,
            less_import_options_prefer_single_line: None,
            less_mixin_args_prefer_single_line: None,
            less_mixin_params_prefer_single_line: None,
            single_line_top_level_declarations: false,
            selector_override_comment_directive: "malva-selector-override".into(),
            ignore_comment_directive: "malva-ignore".into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum HexCase {
    /// Keep the color hex case as-is.
    Ignore,
    #[default]
    Lower,
    Upper,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum HexColorLength {
    Short,
    Long,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum Quotes {
    #[default]
    #[cfg_attr(feature = "config_serde", serde(alias = "alwaysDouble"))]
    /// Make all strings to double quoted.
    AlwaysDouble,

    #[cfg_attr(feature = "config_serde", serde(alias = "alwaysSingle"))]
    /// Make all strings to single quoted.
    AlwaysSingle,

    #[cfg_attr(feature = "config_serde", serde(alias = "preferDouble"))]
    /// Make string to double quoted unless it contains single quotes inside.
    PreferDouble,

    #[cfg_attr(feature = "config_serde", serde(alias = "preferSingle"))]
    /// Make string to single quoted unless it contains double quotes inside.
    PreferSingle,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum OperatorLineBreak {
    Before,
    #[default]
    After,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum BlockSelectorLineBreak {
    /// Always insert linebreaks after all commas.
    Always,
    #[default]
    /// Put selectors at one line as possible.
    /// If exceeded print width, insert linebreaks after all commas.
    Consistent,
    /// Put selectors at one line as possible.
    /// If exceeded print width, insert one linebreak where it exceeds.
    Wrap,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
// https://github.com/Siilwyn/css-declaration-sorter
pub enum DeclarationOrder {
    /// Order in a simple alphabetical manner from a - z.
    Alphabetical,
    /// Order from most important, flow affecting properties, to least important properties.
    Smacss,
    /// Order properties applying outside the box model, moving inward to intrinsic changes.
    Concentric,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum KeyframeSelectorNotation {
    Keyword,
    Percentage,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum AttrValueQuotes {
    #[default]
    Always,
    Ignore,
}
