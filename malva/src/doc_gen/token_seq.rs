use super::DocGen;
use crate::ctx::Ctx;
use raffia::{
    ast::*,
    token::{Token, TokenWithSpan},
    Spanned,
};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for TokenSeq<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut pos = self.span.start;
        let mut docs = Vec::with_capacity(self.tokens.len());
        let mut iter = self.tokens.iter().peekable();
        while let Some(token) = iter.next() {
            let span = token.span();
            docs.extend(ctx.start_spaced_comments(pos, span.start));

            docs.push(token.doc(ctx));
            if let TokenWithSpan {
                token: Token::Comma(..) | Token::Semicolon(..),
                ..
            } = token
            {
                docs.push(Doc::soft_line());
            } else if matches!(iter.peek(), Some(next) if token.span().end < next.span().start) {
                docs.push(Doc::soft_line());
            }

            pos = span.end;
        }
        Doc::list(docs)
    }
}
