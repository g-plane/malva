use crate::config::LanguageOptions;
use raffia::{token::Comment, Syntax};

pub(crate) struct Ctx<'a, 's> {
    pub syntax: Syntax,
    pub options: &'a LanguageOptions,
    pub comments: &'a [Comment<'s>],
    pub indent_width: usize,
}
