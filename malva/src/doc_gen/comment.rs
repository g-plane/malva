use super::DocGen;
use crate::ctx::Ctx;
use raffia::token::{Comment, CommentKind};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for Comment<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            CommentKind::Block => {
                let mut docs = vec![Doc::text("/*")];
                if ctx.options.pad_comments
                    && !self
                        .content
                        .as_bytes()
                        .first()
                        .map(|b| b.is_ascii_whitespace())
                        .unwrap_or(true)
                {
                    docs.push(Doc::space());
                }

                docs.extend(itertools::intersperse(
                    // we don't use `str::lines()` since it uses `split_inclusive`
                    self.content
                        .split('\n')
                        .map(|s| Doc::text(s.strip_suffix('\r').unwrap_or(s))),
                    Doc::empty_line(),
                ));

                if ctx.options.pad_comments
                    && !self
                        .content
                        .as_bytes()
                        .last()
                        .map(|b| b.is_ascii_whitespace())
                        .unwrap_or(true)
                {
                    docs.push(Doc::space());
                }
                docs.push(Doc::text("*/"));

                Doc::list(docs)
            }
            CommentKind::Line => {
                let content = self.content.trim_end();
                if ctx.options.pad_comments
                    && !content
                        .as_bytes()
                        .first()
                        .map(|b| b.is_ascii_whitespace())
                        .unwrap_or(true)
                {
                    Doc::text(format!("// {content}"))
                } else {
                    Doc::text(format!("//{content}"))
                }
            }
        }
    }
}
