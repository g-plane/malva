use super::{helpers, DocGen};
use crate::{ctx::Ctx, state};
use raffia::{ast::*, token::TokenWithSpan, Span, Spanned, Syntax};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for Declaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(
            if ctx.state.get().has(state::STATE_IN_LESS_DETACHED_RULESET) {
                self.name.doc(ctx)
            } else {
                helpers::ident_to_lowercase(&self.name, ctx)
            },
        );

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
        .append(helpers::format_space_before_block(
            self.selector.span.end,
            self.block.span.start,
            ctx,
        ))
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

        let line_break_doc = match ctx.options.single_line_block_threshold {
            Some(threshold) if self.statements.len() <= threshold => Doc::line_or_space(),
            _ => Doc::hard_line(),
        };

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
            stmt_docs.push(line_break_doc.clone());
        }

        format_statements(
            &mut stmt_docs,
            &self.statements,
            &self.span,
            line_break_doc.clone(),
            ctx,
        );

        let has_stmts = !stmt_docs.is_empty();
        docs.push(Doc::list(stmt_docs).nest(ctx.indent_width));
        if has_stmts {
            docs.push(line_break_doc);
        }

        if !is_sass {
            docs.push(Doc::text("}"));
        }

        if ctx.options.single_line_block_threshold.is_some() {
            Doc::list(docs).group()
        } else {
            Doc::list(docs)
        }
    }
}

impl<'s> DocGen<'s> for Statement<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let stmt = match self {
            Statement::QualifiedRule(qualified_rule) => qualified_rule.doc(ctx),
            Statement::AtRule(at_rule) => at_rule.doc(ctx),
            Statement::Declaration(declaration) => declaration.doc(ctx),
            Statement::KeyframeBlock(keyframe_block) => keyframe_block.doc(ctx),
            Statement::LessConditionalQualifiedRule(less_conditional_qualified_rule) => {
                less_conditional_qualified_rule.doc(ctx)
            }
            Statement::LessExtendRule(less_extend_rule) => less_extend_rule.doc(ctx),
            Statement::LessFunctionCall(less_function_call) => less_function_call.doc(ctx),
            Statement::LessMixinCall(less_mixin_call) => less_mixin_call.doc(ctx),
            Statement::LessMixinDefinition(less_mixin_definition) => less_mixin_definition.doc(ctx),
            Statement::LessVariableCall(less_variable_call) => less_variable_call.doc(ctx),
            Statement::LessVariableDeclaration(less_variable_declaration) => {
                less_variable_declaration.doc(ctx)
            }
            Statement::SassIfAtRule(sass_if_at_rule) => sass_if_at_rule.doc(ctx),
            Statement::SassVariableDeclaration(sass_variable_declaration) => {
                sass_variable_declaration.doc(ctx)
            }
            Statement::UnknownSassAtRule(unknown_sass_at_rule) => unknown_sass_at_rule.doc(ctx),
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
                Statement::LessExtendRule(..)
                | Statement::LessFunctionCall(..)
                | Statement::LessMixinCall(..)
                | Statement::LessVariableCall(..)
                | Statement::LessVariableDeclaration(..)
                | Statement::SassVariableDeclaration(..) => stmt.append(Doc::text(";")),
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
        format_statements(
            &mut stmt_docs,
            &self.statements,
            &self.span,
            Doc::hard_line(),
            ctx,
        );
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
    line_break_doc: Doc<'s>,
    ctx: &Ctx<'_, 's>,
) {
    docs.reserve(statements.len() * 2);

    let mut pos = outer_span.start;
    let mut stmts = statements.iter().peekable();

    if let Some(declaration_order) = &ctx.options.declaration_order {
        let mut sortable_decls = Vec::with_capacity(3);
        let mut is_first_stmt_or_decls_group = true;

        while let Some(stmt) = stmts.next() {
            let next_stmt = stmts.peek();
            if let Statement::Declaration(Declaration {
                name: InterpolableIdent::Literal(ident),
                ..
            }) = stmt
            {
                sortable_decls.push((
                    &*ident.name,
                    format_single_stmt(
                        stmt,
                        next_stmt.copied(),
                        &mut pos,
                        outer_span,
                        true, /* ignore_leading_whitespace */
                        line_break_doc.clone(),
                        ctx,
                    ),
                ));
                // the end boundary of sortable declarations group
                if !matches!(
                    next_stmt,
                    Some(Statement::Declaration(Declaration {
                        name: InterpolableIdent::Literal(..),
                        ..
                    }))
                ) {
                    use crate::{config::DeclarationOrder, helpers::sort_decl};
                    match declaration_order {
                        DeclarationOrder::Alphabetical => {
                            sortable_decls
                                .sort_by(|(a, _), (b, _)| sort_decl::compare_in_alphabetical(a, b));
                        }
                        DeclarationOrder::Smacss => {
                            sortable_decls
                                .sort_by(|(a, _), (b, _)| sort_decl::compare_in_smacss(a, b));
                        }
                        DeclarationOrder::Concentric => {
                            sortable_decls
                                .sort_by(|(a, _), (b, _)| sort_decl::compare_in_concentric(a, b));
                        }
                    }
                    if !is_first_stmt_or_decls_group {
                        docs.push(Doc::hard_line());
                        is_first_stmt_or_decls_group = false;
                    }
                    docs.extend(
                        itertools::intersperse(
                            sortable_decls.drain(..).map(|(_, docs)| docs),
                            vec![Doc::hard_line()],
                        )
                        .flatten(),
                    );
                }
            } else {
                docs.append(&mut format_single_stmt(
                    stmt,
                    next_stmt.copied(),
                    &mut pos,
                    outer_span,
                    false, /* ignore_leading_whitespace */
                    line_break_doc.clone(),
                    ctx,
                ));
                is_first_stmt_or_decls_group = false;
            }
        }
    } else {
        while let Some(stmt) = stmts.next() {
            docs.append(&mut format_single_stmt(
                stmt,
                stmts.peek().copied(),
                &mut pos,
                outer_span,
                false, /* ignore_leading_whitespace */
                line_break_doc.clone(),
                ctx,
            ));
        }
    }

    ctx.get_comments_between(pos, outer_span.end)
        .for_each(|comment| {
            if pos > outer_span.start {
                match ctx.line_bounds.line_distance(pos, comment.span.start) {
                    0 => docs.push(Doc::space()),
                    1 => docs.push(Doc::hard_line()),
                    _ => {
                        docs.push(Doc::empty_line());
                        docs.push(Doc::hard_line());
                    }
                }
            }
            docs.push(comment.doc(ctx));
            pos = comment.span.end;
        });
}

fn format_single_stmt<'s>(
    stmt: &Statement<'s>,
    next_stmt: Option<&Statement<'s>>,
    pos: &mut usize,
    outer_span: &Span,
    ignore_leading_whitespace: bool,
    line_break_doc: Doc<'s>,
    ctx: &Ctx<'_, 's>,
) -> Vec<Doc<'s>> {
    let mut docs = Vec::with_capacity(3);

    let span = stmt.span();

    let has_comments = ctx.get_comments_between(*pos, span.start).fold(
        !ignore_leading_whitespace,
        |has_comments, comment| {
            if has_comments && *pos > outer_span.start {
                match ctx.line_bounds.line_distance(*pos, comment.span.start) {
                    0 => docs.push(Doc::space()),
                    1 => docs.push(line_break_doc.clone()),
                    _ => {
                        docs.push(Doc::empty_line());
                        docs.push(Doc::hard_line());
                    }
                }
            }
            docs.push(comment.doc(ctx));
            *pos = comment.span.end;
            true
        },
    );

    if has_comments && *pos > outer_span.start {
        if ctx.line_bounds.line_distance(*pos, span.start) <= 1 {
            docs.push(line_break_doc);
        } else {
            docs.push(Doc::empty_line());
            docs.push(Doc::hard_line());
        }
    }
    docs.push(stmt.doc(ctx));
    *pos = span.end;

    ctx.get_comments_between(
        *pos,
        next_stmt
            .map(|next| next.span().start)
            .unwrap_or_else(|| outer_span.end),
    )
    .for_each(|comment| {
        if *pos > outer_span.start && ctx.line_bounds.line_distance(*pos, comment.span.start) == 0 {
            docs.push(Doc::space());
            docs.push(comment.doc(ctx));
            *pos = comment.span.end;
        }
    });

    docs
}
