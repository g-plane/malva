use crate::{config::LanguageOptions, LineBounds};
use raffia::{token::Comment, Syntax};

pub(crate) struct Ctx<'a, 's> {
    pub syntax: Syntax,
    pub options: &'a LanguageOptions,
    pub comments: &'a [Comment<'s>],
    pub indent_width: usize,
    pub line_bounds: LineBounds,
}
