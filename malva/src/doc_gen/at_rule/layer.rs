use super::super::{DocGen, helpers};
use crate::{ctx::Ctx, state::State};
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for LayerName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.idents.iter().map(|ident| ident.doc(ctx, state)),
                Doc::text("."),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for LayerNames<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(&self.names, &self.comma_spans, self.span.start, ctx, state)
            .group()
    }
}
