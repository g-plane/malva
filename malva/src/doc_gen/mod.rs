use crate::ctx::Ctx;
use tiny_pretty::Doc;

mod at_rule;
mod less;
mod sass;
mod selector;
mod stmt;
mod value;

pub(super) trait DocGen<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s>;
}
