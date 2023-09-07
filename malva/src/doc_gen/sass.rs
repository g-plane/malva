use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for SassPlaceholderSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("%").append(self.name.doc(ctx))
    }
}
