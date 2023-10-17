use super::super::{helpers, DocGen};
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for LayerName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.idents.iter().map(|ident| ident.doc(ctx)),
                Doc::text("."),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for LayerNames<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(&self.names, &self.comma_spans, self.span.start, ctx)
            .group()
    }
}
