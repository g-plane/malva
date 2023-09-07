use crate::ctx::Ctx;
use tiny_pretty::Doc;

mod less;
mod sass;
mod selector;
mod stmt;
mod value;

pub(super) trait DocGen {
    fn doc(&self, ctx: &Ctx) -> Doc;
}
