use crate::ctx::Ctx;
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

pub(super) trait DocGen<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s>;
}
