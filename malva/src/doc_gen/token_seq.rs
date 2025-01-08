use super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for TokenSeq<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut pos = self.span.start;
        let mut docs = Vec::with_capacity(self.tokens.len());
        let mut iter = self.tokens.iter().peekable();
        while let Some(token) = iter.next() {
            let span = token.span();
            docs.extend(ctx.start_spaced_comments(ctx.get_comments_between(pos, span.start)));

            docs.push(token.doc(ctx, state));
            if matches!(iter.peek(), Some(next) if token.span().end < next.span().start) {
                docs.push(Doc::soft_line());
            }

            pos = span.end;
        }
        Doc::list(docs)
    }
}
