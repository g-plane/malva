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

                // we don't use `str::lines()` since it uses `split_inclusive`
                let mut lines = self
                    .content
                    .split('\n')
                    .map(|s| s.strip_suffix('\r').unwrap_or(s));

                let is_jsdoc_like = lines.clone().skip(1).all(|line| {
                    let trimmed = line.trim_start();
                    trimmed.is_empty() || trimmed.starts_with('*')
                });

                if is_jsdoc_like {
                    if let Some(first) = lines.next() {
                        docs.push(Doc::text(first));
                    };
                    docs.extend(
                        lines.map(|line| Doc::hard_line().append(Doc::text(line.trim_start()))),
                    );
                } else {
                    docs.extend(itertools::intersperse(
                        lines.map(Doc::text),
                        Doc::empty_line(),
                    ));
                }

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

                if is_jsdoc_like {
                    Doc::list(docs).nest(1)
                } else {
                    Doc::list(docs)
                }
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
