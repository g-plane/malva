use super::{super::DocGen, helpers};
use crate::{ctx::Ctx, state::State};
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'a, 's: 'a> DocGen<'a, 's> for DocumentPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'a, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(
                &self.matchers,
                &self.comma_spans,
                self.span.start,
                ctx,
                state,
            )
            .group()
            .nest(ctx.indent_width)
    }
}

impl<'a, 's: 'a> DocGen<'a, 's> for DocumentPreludeMatcher<'s> {
    fn doc(&self, ctx: &Ctx<'a, 's>, state: &State) -> Doc<'s> {
        match self {
            DocumentPreludeMatcher::Function(function) => function.doc(ctx, state),
            DocumentPreludeMatcher::Url(url) => url.doc(ctx, state),
        }
    }
}
