use super::{super::DocGen, helpers};
use crate::{ctx::Ctx, state::State};
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for ScopeEnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("to ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.to_span.end, self.lparen_span.start),
            ))
            .append(helpers::format_parenthesized(
                self.selector.doc(ctx, state),
                self.selector.span.end,
                self.span.end,
                ctx,
            ))
    }
}

impl<'s> DocGen<'s> for ScopePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            ScopePrelude::StartOnly(start_only) => start_only.doc(ctx, state),
            ScopePrelude::EndOnly(end_only) => end_only.doc(ctx, state),
            ScopePrelude::Both(both) => both.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for ScopeStart<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::format_parenthesized(
            self.selector.doc(ctx, state),
            self.selector.span.end,
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for ScopeStartWithEnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.start
            .doc(ctx, state)
            .append(Doc::line_or_space().nest(ctx.indent_width))
            .append(self.end.doc(ctx, state))
            .group()
    }
}
