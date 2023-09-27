use super::{helpers, DocGen};
use crate::ctx::Ctx;
use raffia::{ast::*, token::TokenWithSpan, Span, Spanned, Syntax};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for Declaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(helpers::ident_to_lowercase(&self.name, ctx));

        if let Some(less_property_merge) = &self.less_property_merge {
            docs.push(less_property_merge.doc(ctx));
            docs.extend(
                ctx.start_spaced_comments(less_property_merge.span.end, self.colon_span.start),
            );
        } else {
            docs.extend(ctx.start_spaced_comments(self.name.span().end, self.colon_span.start));
        }

        docs.push(Doc::text(":"));
        let can_break_before_value = self.value.iter().any(|value| {
            matches!(
                value,
                ComponentValue::Delimiter(Delimiter {
                    kind: DelimiterKind::Comma,
                    ..
                })
            )
        });
        if can_break_before_value {
            docs.push(Doc::line_or_space().nest(ctx.indent_width));
        } else {
            docs.push(Doc::space());
        }

        docs.reserve(self.value.len() * 2);
        let mut pos = self.colon_span.end;

        let mut iter = self.value.iter().peekable();
        match &self.name {
            InterpolableIdent::Literal(Ident { name, .. })
                if name.starts_with("--") || name.eq_ignore_ascii_case("filter") =>
            {
                use raffia::token::Token;
                while let Some(value) = iter.next() {
                    let span = value.span();
                    docs.push(
                        Doc::list(ctx.end_spaced_comments(pos, span.start).collect())
                            .nest(ctx.indent_width),
                    );

                    docs.push(value.doc(ctx));
                    if let ComponentValue::TokenWithSpan(TokenWithSpan {
                        token: Token::Comma(..) | Token::Semicolon(..),
                        ..
                    }) = value
                    {
                        docs.push(Doc::soft_line().nest(ctx.indent_width));
                    } else if matches!(iter.peek(), Some(next) if value.span().end < next.span().start)
                    {
                        docs.push(Doc::soft_line().nest(ctx.indent_width));
                    }

                    pos = span.end;
                }
            }
            _ => {
                while let Some(value) = iter.next() {
                    let span = value.span();
                    docs.push(
                        Doc::list(ctx.end_spaced_comments(pos, span.start).collect())
                            .nest(ctx.indent_width),
                    );

                    docs.push(value.doc(ctx));
                    match value {
                        ComponentValue::Delimiter(Delimiter {
                            kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                            ..
                        }) => docs.push(Doc::line_or_space().nest(ctx.indent_width)),
                        ComponentValue::Delimiter(Delimiter {
                            kind: DelimiterKind::Solidus,
                            span,
                        }) => {
                            if pos < span.start {
                                docs.push(Doc::soft_line().nest(ctx.indent_width));
                            }
                        }
                        _ => match iter.peek() {
                            Some(ComponentValue::Delimiter(Delimiter {
                                kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                                ..
                            }))
                            | None => {}
                            Some(ComponentValue::Delimiter(Delimiter {
                                kind: DelimiterKind::Solidus,
                                span: next_span,
                            })) => {
                                if span.end < next_span.start {
                                    docs.push(Doc::soft_line().nest(ctx.indent_width));
                                }
                            }
                            _ => docs.push(Doc::soft_line().nest(ctx.indent_width)),
                        },
                    }

                    pos = span.end;
                }
            }
        }

        if let Some(important) = &self.important {
            docs.push(Doc::soft_line().nest(ctx.indent_width));
            docs.push(
                Doc::list(ctx.end_spaced_comments(pos, important.span.start).collect())
                    .nest(ctx.indent_width),
            );
            docs.push(important.doc(ctx));
        }

        Doc::list(docs).group()
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
        helpers::format_selectors_before_block(
            &self.selector.selectors,
            &self.selector.comma_spans,
            self.selector.span.start,
            ctx,
        )
        .append(helpers::format_space_before_block(ctx))
        .concat(ctx.end_spaced_comments(self.selector.span.end, self.block.span.start))
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

        let mut stmt_docs = vec![];
        if !self.statements.is_empty()
            || ctx
                .get_comments_between(
                    self.span.start,
                    self.statements
                        .first()
                        .map(|stmt| stmt.span().start)
                        .unwrap_or(self.span.end),
                )
                .count()
                > 0
        {
            stmt_docs.push(Doc::hard_line());
        }

        format_statements(&mut stmt_docs, &self.statements, &self.span, ctx);

        let has_stmts = !stmt_docs.is_empty();
        docs.push(Doc::list(stmt_docs).nest(ctx.indent_width));
        if has_stmts {
            docs.push(Doc::hard_line());
        }

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
        let mut stmt_docs = vec![];
        format_statements(&mut stmt_docs, &self.statements, &self.span, ctx);
        if ctx.syntax != Syntax::Sass {
            stmt_docs.push(Doc::empty_line());
        }
        Doc::list(stmt_docs)
    }
}

fn format_statements<'s>(
    docs: &mut Vec<Doc<'s>>,
    statements: &[Statement<'s>],
    outer_span: &Span,
    ctx: &Ctx<'_, 's>,
) {
    docs.reserve(statements.len() * 2);
    let (stmts, mut pos) =
        statements
            .iter()
            .fold((docs, outer_span.start), |(stmts, mut pos), stmt| {
                let span = stmt.span();

                ctx.get_comments_between(pos, span.start)
                    .for_each(|comment| {
                        if pos > outer_span.start {
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

                if pos > outer_span.start {
                    if ctx.line_bounds.line_distance(pos, span.start) <= 1 {
                        stmts.push(Doc::hard_line());
                    } else {
                        stmts.push(Doc::empty_line());
                        stmts.push(Doc::hard_line());
                    }
                }
                stmts.push(stmt.doc(ctx));
                (stmts, span.end)
            });

    ctx.get_comments_between(pos, outer_span.end)
        .for_each(|comment| {
            if pos > outer_span.start {
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
}
