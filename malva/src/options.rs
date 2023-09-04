pub struct Options {
    pub general: GeneralOptions,
    pub language: LanguageOptions,
}

pub struct GeneralOptions {
    pub print_width: usize,
    pub use_tabs: bool,
    pub indent_width: usize,
    pub line_break: LineBreak,
}

pub enum LineBreak {
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

pub struct LanguageOptions {
    pub hex_case: HexCase,
}

pub enum HexCase {
    Ignore,
    Lower,
    Upper,
}
