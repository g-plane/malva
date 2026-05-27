pub(crate) use self::comment::format_comment;
use crate::{ctx::Ctx, state::State};
use tiny_pretty::Doc;

mod at_rule;
mod comment;
mod helpers;
mod less;
mod sass;
mod selector;
mod stmt;
mod str;
mod token_seq;
mod value;

pub(super) trait DocGen<'a, 's: 'a> {
    fn doc(&self, ctx: &Ctx<'a, 's>, state: &State) -> Doc<'s>;
}
