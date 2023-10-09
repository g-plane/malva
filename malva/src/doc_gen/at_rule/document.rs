use super::{super::DocGen, helpers};
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for DocumentPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(&self.matchers, &self.comma_spans, self.span.start, ctx)
            .group()
            .nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for DocumentPreludeMatcher<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            DocumentPreludeMatcher::Function(function) => function.doc(ctx),
            DocumentPreludeMatcher::Url(url) => url.doc(ctx),
        }
    }
}
