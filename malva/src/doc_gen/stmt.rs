use super::DocGen;
use crate::ctx::Ctx;
use itertools::{EitherOrBoth, Itertools};
use raffia::{ast::*, token::TokenWithSpan, Spanned, Syntax};
use std::mem;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for Declaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(self.name.doc(ctx));
        if let Some(less_property_merge) = &self.less_property_merge {
            docs.push(less_property_merge.doc(ctx));
            docs.extend(
                ctx.start_padded_comments(less_property_merge.span.end, self.colon_span.start),
            );
        } else {
            docs.extend(ctx.start_padded_comments(self.name.span().end, self.colon_span.start));
        }
        docs.push(Doc::text(": "));

        let mut values = Vec::with_capacity(self.value.len() * 2);
        let mut end = self.colon_span.end;

        let mut iter = self.value.iter().peekable();
        match &self.name {
            InterpolableIdent::Literal(Ident { name, .. })
                if name.starts_with("--") || name.eq_ignore_ascii_case("filter") =>
            {
                use raffia::token::Token;
                while let Some(value) = iter.next() {
                    let span = value.span();
                    values.extend(ctx.end_padded_comments(end, span.start));

                    values.push(value.doc(ctx));
                    if let ComponentValue::TokenWithSpan(TokenWithSpan {
                        token: Token::Comma(..) | Token::Semicolon(..),
                        ..
                    }) = value
                    {
                        values.push(Doc::soft_line());
                    } else if matches!(iter.peek(), Some(next) if value.span().end < next.span().start)
                    {
                        values.push(Doc::soft_line());
                    }

                    end = span.end;
                }
            }
            _ => {
                while let Some(value) = iter.next() {
                    let span = value.span();
                    values.extend(ctx.end_padded_comments(end, span.start));

                    values.push(value.doc(ctx));
                    if !matches!(
                        iter.peek(),
                        Some(ComponentValue::Delimiter(Delimiter {
                            kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                            ..
                        })) | None
                    ) {
                        values.push(Doc::soft_line());
                    }

                    end = span.end;
                }
            }
        }

        if let Some(important) = &self.important {
            values.push(Doc::soft_line());
            values.extend(ctx.end_padded_comments(end, important.span.start));
            values.push(important.doc(ctx));
        }

        docs.push(Doc::list(values).group().nest(ctx.indent_width));

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for ImportantAnnotation<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("!important")
    }
}

impl<'s> DocGen<'s> for QualifiedRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::BlockSelectorLineBreak;

        // we don't use `SelectorList::doc` here
        // because it's a special case for qualified rule
        Doc::list(
            itertools::intersperse(
                self.selector
                    .selectors
                    .iter()
                    .zip_longest(self.selector.comma_spans.iter())
                    .scan(self.selector.span.start, |end, item| match item {
                        EitherOrBoth::Both(selector, comma_span) => {
                            let mut docs = ctx
                                .end_padded_comments(
                                    mem::replace(end, comma_span.end),
                                    selector.span.start,
                                )
                                .collect::<Vec<_>>();
                            docs.push(selector.doc(ctx));
                            docs.extend(
                                ctx.start_padded_comments(selector.span.end, comma_span.start),
                            );
                            Some(docs.into_iter())
                        }
                        EitherOrBoth::Left(selector) => {
                            let mut docs = ctx
                                .end_padded_comments(*end, selector.span.start)
                                .collect::<Vec<_>>();
                            docs.push(selector.doc(ctx));
                            Some(docs.into_iter())
                        }
                        EitherOrBoth::Right(..) => unreachable!(),
                    }),
                vec![
                    Doc::text(","),
                    match ctx.options.block_selector_linebreak {
                        BlockSelectorLineBreak::Always => Doc::hard_line(),
                        BlockSelectorLineBreak::Consistent => Doc::line_or_space(),
                        BlockSelectorLineBreak::Wrap => Doc::soft_line(),
                    },
                ]
                .into_iter(),
            )
            .flatten()
            .collect(),
        )
        .group()
        .append(Doc::space())
        .concat(ctx.end_padded_comments(self.selector.span.end, self.block.span.start))
        .append(self.block.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SimpleBlock<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let is_sass = ctx.syntax == Syntax::Sass;
        let mut docs = vec![];

        if !is_sass {
            docs.push(Doc::text("{"));
        }

        let (mut stmts, mut end) = self.statements.iter().fold(
            (
                Vec::with_capacity(self.statements.len() * 2),
                self.span.start,
            ),
            |(mut stmts, mut end), stmt| {
                let span = stmt.span();

                ctx.get_comments_between(end, span.start)
                    .for_each(|comment| {
                        match ctx.line_bounds.line_distance(end, comment.span.start) {
                            0 => stmts.push(Doc::space()),
                            1 => stmts.push(Doc::hard_line()),
                            _ => {
                                stmts.push(Doc::empty_line());
                                stmts.push(Doc::hard_line());
                            }
                        }
                        stmts.push(comment.doc(ctx));
                        end = comment.span.end;
                    });

                if ctx.line_bounds.line_distance(end, span.start) <= 1 {
                    stmts.push(Doc::hard_line());
                } else {
                    stmts.push(Doc::empty_line());
                    stmts.push(Doc::hard_line());
                }
                stmts.push(stmt.doc(ctx));
                (stmts, span.end)
            },
        );

        ctx.get_comments_between(end, self.span.end)
            .for_each(|comment| {
                match ctx.line_bounds.line_distance(end, comment.span.start) {
                    0 => stmts.push(Doc::space()),
                    1 => stmts.push(Doc::hard_line()),
                    _ => {
                        stmts.push(Doc::empty_line());
                        stmts.push(Doc::hard_line());
                    }
                }
                stmts.push(comment.doc(ctx));
                end = comment.span.end;
            });

        docs.push(Doc::list(stmts).nest(ctx.indent_width));
        docs.push(Doc::hard_line());

        if !is_sass {
            docs.push(Doc::text("}"));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for Statement<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let stmt = match self {
            Statement::AtRule(at_rule) => at_rule.doc(ctx),
            Statement::Declaration(declaration) => declaration.doc(ctx),
            Statement::KeyframeBlock(keyframe_block) => keyframe_block.doc(ctx),
            Statement::QualifiedRule(qualified_rule) => qualified_rule.doc(ctx),
            Statement::SassIfAtRule(sass_if_at_rule) => sass_if_at_rule.doc(ctx),
            _ => todo!(),
        };
        if ctx.syntax == Syntax::Sass {
            stmt
        } else {
            match self {
                Statement::AtRule(at_rule) if at_rule.block.is_none() => {
                    stmt.append(Doc::text(";"))
                }
                Statement::Declaration(decl)
                    if !matches!(
                        decl.value.last(),
                        Some(ComponentValue::SassNestingDeclaration(..))
                    ) =>
                {
                    stmt.append(Doc::text(";"))
                }
                _ => stmt,
            }
        }
    }
}

impl<'s> DocGen<'s> for Stylesheet<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let (mut stmts, mut end) = self.statements.iter().fold(
            (
                Vec::with_capacity(self.statements.len() * 2),
                self.span.start,
            ),
            |(mut stmts, mut end), stmt| {
                let span = stmt.span();

                ctx.get_comments_between(end, span.start)
                    .for_each(|comment| {
                        if end > 0 {
                            match ctx.line_bounds.line_distance(end, comment.span.start) {
                                0 => stmts.push(Doc::space()),
                                1 => stmts.push(Doc::hard_line()),
                                _ => {
                                    stmts.push(Doc::empty_line());
                                    stmts.push(Doc::hard_line());
                                }
                            }
                        }
                        stmts.push(comment.doc(ctx));
                        end = comment.span.end;
                    });

                if end > 0 {
                    if ctx.line_bounds.line_distance(end, span.start) <= 1 {
                        stmts.push(Doc::hard_line());
                    } else {
                        stmts.push(Doc::empty_line());
                        stmts.push(Doc::hard_line());
                    }
                }
                stmts.push(stmt.doc(ctx));
                (stmts, span.end)
            },
        );

        ctx.get_comments_between(end, self.span.end)
            .for_each(|comment| {
                if end > 0 {
                    match ctx.line_bounds.line_distance(end, comment.span.start) {
                        0 => stmts.push(Doc::space()),
                        1 => stmts.push(Doc::hard_line()),
                        _ => {
                            stmts.push(Doc::empty_line());
                            stmts.push(Doc::hard_line());
                        }
                    }
                }
                stmts.push(comment.doc(ctx));
                end = comment.span.end;
            });

        stmts.push(Doc::empty_line());

        Doc::list(stmts)
    }
}
