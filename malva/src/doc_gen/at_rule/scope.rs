use super::{super::DocGen, helpers};
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for ScopeEnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("to ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.to_span.end, self.lparen_span.start),
            ))
            .append(helpers::format_parenthesized(
                self.selector.doc(ctx),
                self.selector.span.end,
                self.span.end,
                ctx,
            ))
    }
}

impl<'s> DocGen<'s> for ScopePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ScopePrelude::StartOnly(start_only) => start_only.doc(ctx),
            ScopePrelude::EndOnly(end_only) => end_only.doc(ctx),
            ScopePrelude::Both(both) => both.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for ScopeStart<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        helpers::format_parenthesized(
            self.selector.doc(ctx),
            self.selector.span.end,
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for ScopeStartWithEnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.start
            .doc(ctx)
            .append(Doc::line_or_space().nest(ctx.indent_width))
            .append(self.end.doc(ctx))
            .group()
    }
}
