pub use tiny_pretty::LineBreak;

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub general: GeneralOptions,
    pub language: LanguageOptions,
}

#[derive(Clone, Debug)]
pub struct GeneralOptions {
    pub print_width: usize,
    pub use_tabs: bool,
    pub indent_width: usize,
    pub line_break: LineBreak,
}

impl Default for GeneralOptions {
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
pub struct LanguageOptions {
    pub hex_case: HexCase,
    pub quotes: Quotes,
}

#[derive(Clone, Debug, Default)]
pub enum HexCase {
    Ignore,
    #[default]
    Lower,
    Upper,
}

#[derive(Clone, Debug, Default)]
pub enum Quotes {
    #[default]
    AlwaysDouble,
    AlwaysSingle,
    PreferDouble,
    PreferSingle,
}
