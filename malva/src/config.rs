//! Types about configuration.

#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

/// The whole configuration of Malva.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase", default))]
pub struct FormatOptions {
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub layout: LayoutOptions,
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub language: LanguageOptions,
}

/// Configuration related to layout, such as indentation or print width.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase", default))]
pub struct LayoutOptions {
    pub print_width: usize,
    pub use_tabs: bool,
    pub indent_width: usize,
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

/// Configuration related to syntax.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "camelCase", default))]
pub struct LanguageOptions {
    pub hex_case: HexCase,
    pub quotes: Quotes,
    pub operator_linebreak: OperatorLineBreak,
    pub block_selector_linebreak: BlockSelectorLineBreak,
    pub omit_number_leading_zero: bool,
    pub trailing_comma: bool,
    pub pad_comments: bool,
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
    /// Make all strings to double quoted.
    #[default]
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
