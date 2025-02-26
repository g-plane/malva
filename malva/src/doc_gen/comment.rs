use crate::ctx::Ctx;
use raffia::{
    token::{Comment, CommentKind},
    Syntax,
};
use tiny_pretty::Doc;

pub(crate) fn format_comment<'s>(comment: &Comment<'s>, ctx: &Ctx<'_, 's>) -> Doc<'s> {
    match comment.kind {
        CommentKind::Block => {
            let mut docs = vec![Doc::text("/*")];
            if ctx.options.format_comments
                && !comment
                    .content
                    .as_bytes()
                    .first()
                    .map(|b| b.is_ascii_whitespace())
                    .unwrap_or(true)
            {
                docs.push(Doc::space());
            }

            // we don't use `str::lines()` since it uses `split_inclusive`
            let mut lines = comment
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
            } else if ctx.options.align_comments {
                docs.append(&mut reflow(comment, ctx));
            } else {
                docs.extend(itertools::intersperse(
                    lines.map(Doc::text),
                    Doc::empty_line(),
                ));
            }

            if ctx.options.format_comments
                && !comment
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
            let content = comment.content.trim_end();
            if ctx.options.format_comments {
                let (is_doc_comment, content) = match (ctx.syntax, content.strip_prefix('/')) {
                    (Syntax::Scss | Syntax::Sass, Some(content)) => (true, content),
                    _ => (false, content),
                };
                let prefix = if is_doc_comment { "///" } else { "//" };
                if content
                    .as_bytes()
                    .first()
                    .map_or(true, |b| b.is_ascii_whitespace())
                {
                    Doc::text(format!("{prefix}{content}"))
                } else {
                    Doc::text(format!("{prefix} {content}",))
                }
            } else {
                Doc::text(format!("//{content}"))
            }
        }
    }
}

pub(super) fn reflow<'s>(comment: &Comment<'s>, ctx: &Ctx<'_, 's>) -> Vec<Doc<'s>> {
    let col = comment
        .content
        .lines()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.as_bytes()
                .iter()
                .take_while(|byte| byte.is_ascii_whitespace())
                .count()
        })
        .min()
        .unwrap_or_default()
        .min(
            ctx.line_bounds
                .get_line_col(comment.span.start)
                .1
                .saturating_sub(1),
        );
    let mut docs = Vec::with_capacity(2);
    let mut lines = comment.content.split('\n').enumerate().peekable();
    while let Some((i, line)) = lines.next() {
        let s = line.strip_suffix('\r').unwrap_or(line);
        let s = if s.starts_with([' ', '\t']) && i > 0 {
            s.get(col..).unwrap_or(s)
        } else {
            s
        };
        if i > 0 {
            if s.trim().is_empty() && lines.peek().is_some() {
                docs.push(Doc::empty_line());
            } else {
                docs.push(Doc::hard_line());
            }
        }
        docs.push(Doc::text(s));
    }
    docs
}
