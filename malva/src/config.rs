//! Types about configuration.

#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// The whole configuration of Malva.
///
/// For detail, please refer to [Configuration](https://github.com/g-plane/malva/blob/main/docs/config.md) on GitHub.
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
    /// See [`printWidth`](https://github.com/g-plane/malva/blob/main/docs/config.md#printwidth) on GitHub
    pub print_width: usize,

    #[cfg_attr(feature = "config_serde", serde(alias = "useTabs"))]
    /// See [`useTabs`](https://github.com/g-plane/malva/blob/main/docs/config.md#usetabs) on GitHub
    pub use_tabs: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "indentWidth"))]
    /// See [`indentWidth`](https://github.com/g-plane/malva/blob/main/docs/config.md#indentwidth) on GitHub
    pub indent_width: usize,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "lineBreak", alias = "linebreak")
    )]
    /// See [`lineBreak`](https://github.com/g-plane/malva/blob/main/docs/config.md#linebreak) on GitHub
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
    /// See [`hexCase`](https://github.com/g-plane/malva/blob/main/docs/config.md#hexcase) on GitHub
    pub hex_case: HexCase,

    #[cfg_attr(feature = "config_serde", serde(alias = "hexColorLength"))]
    /// See [`hexColorLength`](https://github.com/g-plane/malva/blob/main/docs/config.md#hexcolorlength) on GitHub
    pub hex_color_length: Option<HexColorLength>,

    /// See [`quotes`](https://github.com/g-plane/malva/blob/main/docs/config.md#quotes) on GitHub
    pub quotes: Quotes,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "operatorLinebreak", alias = "operatorLineBreak")
    )]
    /// See [`operatorLinebreak`](https://github.com/g-plane/malva/blob/main/docs/config.md#operatorlinebreak) on GitHub
    pub operator_linebreak: OperatorLineBreak,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "blockSelectorLinebreak", alias = "blockSelectorLineBreak")
    )]
    /// See [`blockSelectorLinebreak`](https://github.com/g-plane/malva/blob/main/docs/config.md#blockselectorlinebreak) on GitHub
    pub block_selector_linebreak: BlockSelectorLineBreak,

    #[cfg_attr(feature = "config_serde", serde(alias = "omitNumberLeadingZero"))]
    /// See [`omitNumberLeadingZero`](https://github.com/g-plane/malva/blob/main/docs/config.md#omitnumberleadingzero) on GitHub
    pub omit_number_leading_zero: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "trailingComma"))]
    /// See [`trailingComma`](https://github.com/g-plane/malva/blob/main/docs/config.md#trailingcomma) on GitHub
    pub trailing_comma: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "padComments"))]
    /// See [`padComments`](https://github.com/g-plane/malva/blob/main/docs/config.md#padcomments) on GitHub
    pub pad_comments: bool,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "linebreakInPseudoParens", alias = "lineBreakInPseudoParens")
    )]
    /// See [`linebreakInPseudoParens`](https://github.com/g-plane/malva/blob/main/docs/config.md#linebreakinpseudoparens) on GitHub
    pub linebreak_in_pseudo_parens: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "declarationOrder"))]
    /// See [`declarationOrder`](https://github.com/g-plane/malva/blob/main/docs/config.md#declarationorder) on GitHub
    pub declaration_order: Option<DeclarationOrder>,

    #[cfg_attr(feature = "config_serde", serde(alias = "singleLineBlockThreshold"))]
    /// See [`singleLineBlockThreshold`](https://github.com/g-plane/malva/blob/main/docs/config.md#singlelineblockthreshold) on GitHub
    pub single_line_block_threshold: Option<usize>,

    #[cfg_attr(feature = "config_serde", serde(alias = "keyframeSelectorNotation"))]
    /// See [`keyframeSelectorNotation`](https://github.com/g-plane/malva/blob/main/docs/config.md#keyframeselectornotation) on GitHub
    pub keyframe_selector_notation: Option<KeyframeSelectorNotation>,

    #[cfg_attr(feature = "config_serde", serde(alias = "attrValueQuotes"))]
    /// See [`attrValueQuotes`](https://github.com/g-plane/malva/blob/main/docs/config.md#attrvaluequotes) on GitHub
    pub attr_value_quotes: AttrValueQuotes,

    #[cfg_attr(feature = "config_serde", serde(alias = "ignoreCommentDirective"))]
    /// See [`ignoreCommentDirective`](https://github.com/g-plane/malva/blob/main/docs/config.md#ignorecommentdirective) on GitHub
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
            pad_comments: false,
            linebreak_in_pseudo_parens: false,
            declaration_order: None,
            single_line_block_threshold: None,
            keyframe_selector_notation: None,
            attr_value_quotes: AttrValueQuotes::default(),
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
