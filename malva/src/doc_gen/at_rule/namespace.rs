use super::super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for NamespacePrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        if let Some(prefix) = &self.prefix {
            prefix
                .doc(ctx, state)
                .append(Doc::line_or_space())
                .concat(ctx.end_spaced_comments(
                    ctx.get_comments_between(prefix.span().end, self.uri.span().start),
                ))
                .append(self.uri.doc(ctx, state))
                .group()
                .nest(ctx.indent_width)
        } else {
            self.uri.doc(ctx, state)
        }
    }
}

impl<'s> DocGen<'s> for NamespacePreludeUri<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            NamespacePreludeUri::Str(str) => str.doc(ctx, state),
            NamespacePreludeUri::Url(url) => url.doc(ctx, state),
        }
    }
}
