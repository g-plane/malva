use super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::{Spanned, ast::*};
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
            if let Some(next) = iter.peek()
                && token.span.end < next.span.start
            {
                if ctx
                    .line_bounds
                    .line_distance(token.span.end, next.span.start)
                    == 0
                {
                    docs.push(Doc::space());
                } else {
                    docs.push(Doc::hard_line());
                }
            }

            pos = span.end;
        }
        Doc::list(docs)
    }
}
