use super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, token::TokenWithSpan, Spanned, Syntax};
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
        let mut pos = self.colon_span.end;

        let mut iter = self.value.iter().peekable();
        match &self.name {
            InterpolableIdent::Literal(Ident { name, .. })
                if name.starts_with("--") || name.eq_ignore_ascii_case("filter") =>
            {
                use raffia::token::Token;
                while let Some(value) = iter.next() {
                    let span = value.span();
                    values.extend(ctx.end_padded_comments(pos, span.start));

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

                    pos = span.end;
                }
            }
            _ => {
                while let Some(value) = iter.next() {
                    let span = value.span();
                    values.extend(ctx.end_padded_comments(pos, span.start));

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

                    pos = span.end;
                }
            }
        }

        if let Some(important) = &self.important {
            values.push(Doc::soft_line());
            values.extend(ctx.end_padded_comments(pos, important.span.start));
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
        // we don't use `SelectorList::doc` here
        // because it's a special case for qualified rule
        super::format_selectors_before_block(
            &self.selector.selectors,
            &self.selector.comma_spans,
            self.selector.span.start,
            ctx,
        )
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

        let (mut stmts, mut pos) = self.statements.iter().fold(
            (
                Vec::with_capacity(self.statements.len() * 2),
                self.span.start,
            ),
            |(mut stmts, mut pos), stmt| {
                let span = stmt.span();

                ctx.get_comments_between(pos, span.start)
                    .for_each(|comment| {
                        match ctx.line_bounds.line_distance(pos, comment.span.start) {
                            0 => stmts.push(Doc::space()),
                            1 => stmts.push(Doc::hard_line()),
                            _ => {
                                stmts.push(Doc::empty_line());
                                stmts.push(Doc::hard_line());
                            }
                        }
                        stmts.push(comment.doc(ctx));
                        pos = comment.span.end;
                    });

                if ctx.line_bounds.line_distance(pos, span.start) <= 1 {
                    stmts.push(Doc::hard_line());
                } else {
                    stmts.push(Doc::empty_line());
                    stmts.push(Doc::hard_line());
                }
                stmts.push(stmt.doc(ctx));
                (stmts, span.end)
            },
        );

        ctx.get_comments_between(pos, self.span.end)
            .for_each(|comment| {
                match ctx.line_bounds.line_distance(pos, comment.span.start) {
                    0 => stmts.push(Doc::space()),
                    1 => stmts.push(Doc::hard_line()),
                    _ => {
                        stmts.push(Doc::empty_line());
                        stmts.push(Doc::hard_line());
                    }
                }
                stmts.push(comment.doc(ctx));
                pos = comment.span.end;
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
            Statement::SassVariableDeclaration(sass_variable_declaration) => {
                sass_variable_declaration.doc(ctx)
            }
            Statement::UnknownSassAtRule(unknown_sass_at_rule) => unknown_sass_at_rule.doc(ctx),
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
                Statement::SassVariableDeclaration(..) => stmt.append(Doc::text(";")),
                Statement::UnknownSassAtRule(unknown_sass_at_rule)
                    if unknown_sass_at_rule.block.is_none() =>
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
        let (mut stmts, mut pos) = self.statements.iter().fold(
            (
                Vec::with_capacity(self.statements.len() * 2),
                self.span.start,
            ),
            |(mut stmts, mut pos), stmt| {
                let span = stmt.span();

                ctx.get_comments_between(pos, span.start)
                    .for_each(|comment| {
                        if pos > 0 {
                            match ctx.line_bounds.line_distance(pos, comment.span.start) {
                                0 => stmts.push(Doc::space()),
                                1 => stmts.push(Doc::hard_line()),
                                _ => {
                                    stmts.push(Doc::empty_line());
                                    stmts.push(Doc::hard_line());
                                }
                            }
                        }
                        stmts.push(comment.doc(ctx));
                        pos = comment.span.end;
                    });

                if pos > 0 {
                    if ctx.line_bounds.line_distance(pos, span.start) <= 1 {
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

        ctx.get_comments_between(pos, self.span.end)
            .for_each(|comment| {
                if pos > 0 {
                    match ctx.line_bounds.line_distance(pos, comment.span.start) {
                        0 => stmts.push(Doc::space()),
                        1 => stmts.push(Doc::hard_line()),
                        _ => {
                            stmts.push(Doc::empty_line());
                            stmts.push(Doc::hard_line());
                        }
                    }
                }
                stmts.push(comment.doc(ctx));
                pos = comment.span.end;
            });

        stmts.push(Doc::empty_line());

        Doc::list(stmts)
    }
}
