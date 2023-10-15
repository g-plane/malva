//! Types about configuration.

#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase", default))]
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
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase", default))]
/// Configuration related to layout, such as indentation or print width.
pub struct LayoutOptions {
    /// See [`printWidth`](https://github.com/g-plane/malva/blob/main/docs/config.md#printwidth) on GitHub
    pub print_width: usize,
    /// See [`useTabs`](https://github.com/g-plane/malva/blob/main/docs/config.md#usetabs) on GitHub
    pub use_tabs: bool,
    /// See [`indentWidth`](https://github.com/g-plane/malva/blob/main/docs/config.md#indentwidth) on GitHub
    pub indent_width: usize,
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
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase"))]
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
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase", default))]
/// Configuration related to syntax.
pub struct LanguageOptions {
    /// See [`hexCase`](https://github.com/g-plane/malva/blob/main/docs/config.md#hexcase) on GitHub
    pub hex_case: HexCase,
    /// See [`quotes`](https://github.com/g-plane/malva/blob/main/docs/config.md#quotes) on GitHub
    pub quotes: Quotes,
    /// See [`operatorLinebreak`](https://github.com/g-plane/malva/blob/main/docs/config.md#operatorlinebreak) on GitHub
    pub operator_linebreak: OperatorLineBreak,
    /// See [`blockSelectorLinebreak`](https://github.com/g-plane/malva/blob/main/docs/config.md#blockselectorlinebreak) on GitHub
    pub block_selector_linebreak: BlockSelectorLineBreak,
    /// See [`omitNumberLeadingZero`](https://github.com/g-plane/malva/blob/main/docs/config.md#omitnumberleadingzero) on GitHub
    pub omit_number_leading_zero: bool,
    /// See [`trailingComma`](https://github.com/g-plane/malva/blob/main/docs/config.md#trailingcomma) on GitHub
    pub trailing_comma: bool,
    /// See [`padComments`](https://github.com/g-plane/malva/blob/main/docs/config.md#padcomments) on GitHub
    pub pad_comments: bool,
    /// See [`linebreakInPseudoParens`](https://github.com/g-plane/malva/blob/main/docs/config.md#linebreakinpseudoparens) on GitHub
    pub linebreak_in_pseudo_parens: bool,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase"))]
pub enum HexCase {
    /// Keep the color hex case as-is.
    Ignore,
    #[default]
    Lower,
    Upper,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase"))]
pub enum Quotes {
    #[default]
    /// Make all strings to double quoted.
    AlwaysDouble,
    /// Make all strings to single quoted.
    AlwaysSingle,
    /// Make string to double quoted unless it contains single quotes inside.
    PreferDouble,
    /// Make string to single quoted unless it contains double quotes inside.
    PreferSingle,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase"))]
pub enum OperatorLineBreak {
    Before,
    #[default]
    After,
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase"))]
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
