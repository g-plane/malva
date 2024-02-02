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

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to syntax.
pub struct LanguageOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "hexCase"))]
    /// See [`hexCase`](https://github.com/g-plane/malva/blob/main/docs/config.md#hexcase) on GitHub
    pub hex_case: HexCase,

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
