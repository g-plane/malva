use super::{DocGen, comment::format_comment, helpers};
use crate::{ctx::Ctx, state::State};
use raffia::{Span, Spanned, Syntax, ast::*, token::TokenWithSpan};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for Declaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        docs.push(if state.keep_decl_name_case {
            self.name.doc(ctx, state)
        } else {
            helpers::ident_to_lowercase(&self.name, ctx, state)
        });

        if let Some(less_property_merge) = &self.less_property_merge {
            docs.push(less_property_merge.doc(ctx, state));
            docs.extend(ctx.start_spaced_comments(
                ctx.get_comments_between(less_property_merge.span.end, self.colon_span.start),
            ));
        } else {
            docs.extend(ctx.start_spaced_comments(
                ctx.get_comments_between(self.name.span().end, self.colon_span.start),
            ));
        }

        docs.push(Doc::text(":"));
        let has_comma = self.value.iter().any(|value| {
            matches!(
                value,
                ComponentValue::Delimiter(Delimiter {
                    kind: DelimiterKind::Comma,
                    ..
                })
            )
        });
        let space_after_colon = if has_comma {
            Doc::line_or_space().nest(ctx.indent_width)
        } else {
            Doc::space()
        };

        docs.reserve(self.value.len() * 2);
        let mut pos = self.colon_span.end;

        match &self.name {
            InterpolableIdent::Literal(Ident { name, .. })
                if name.starts_with("--") || name.eq_ignore_ascii_case("filter") =>
            {
                use raffia::token::Token;
                docs.push(space_after_colon);

                let mut iter = self.value.iter().peekable();
                while let Some(value) = iter.next() {
                    let span = value.span();
                    docs.push(
                        Doc::list(
                            ctx.end_spaced_comments(ctx.get_comments_between(pos, span.start))
                                .collect(),
                        )
                        .nest(ctx.indent_width),
                    );

                    docs.push(value.doc(ctx, state));
                    if matches!(
                        value,
                        ComponentValue::Delimiter(Delimiter {
                            kind: DelimiterKind::Comma,
                            ..
                        })
                    ) {
                        docs.push(Doc::line_or_space().nest(ctx.indent_width));
                    } else if matches!(
                        value,
                        ComponentValue::TokenWithSpan(TokenWithSpan {
                            token: Token::Comma(..) | Token::Semicolon(..),
                            ..
                        })
                    ) || matches!(iter.peek(), Some(next) if value.span().end < next.span().start)
                    {
                        docs.push(Doc::soft_line().nest(ctx.indent_width));
                    }

                    pos = span.end;
                }
            }
            InterpolableIdent::Literal(Ident { name, .. })
                if name.eq_ignore_ascii_case("grid")
                    || name
                        .get(0..13)
                        .is_some_and(|s| s.eq_ignore_ascii_case("grid-template")) =>
            {
                pos = self
                    .value
                    .iter()
                    .enumerate()
                    .fold(pos, |pos, (index, value)| {
                        let span = value.span();
                        let comments = ctx
                            .end_spaced_comments(ctx.get_comments_between(pos, span.start))
                            .collect::<Vec<_>>();

                        if !comments.is_empty() {
                            docs.push(Doc::space());
                        } else if index == 0 {
                            docs.push(Doc::line_or_space().nest(ctx.indent_width));
                        } else if ctx.line_bounds.line_distance(pos, span.start) == 0 {
                            docs.push(Doc::space());
                        } else {
                            docs.push(Doc::hard_line().nest(ctx.indent_width));
                        }
                        docs.push(Doc::list(comments).nest(ctx.indent_width));
                        docs.push(value.doc(ctx, state).nest(ctx.indent_width));

                        span.end
                    });
            }
            _ => {
                let mut iter = self.value.iter().enumerate().peekable();

                if !matches!(iter.peek(), Some((_, ComponentValue::Function(..)))) {
                    docs.push(space_after_colon);
                }

                let space_after_comma = match &self.name {
                    InterpolableIdent::Literal(Ident { name, .. })
                        if name.eq_ignore_ascii_case("font-family") =>
                    {
                        Doc::soft_line()
                    }
                    _ => Doc::line_or_space(),
                };
                while let Some((index, value)) = iter.next() {
                    let span = value.span();

                    if let ComponentValue::Function(function) = value {
                        let mut has_last_line_comment = false;
                        docs.push(
                            Doc::list(
                                ctx.start_spaced_comments_without_last_hard_line(
                                    ctx.get_comments_between(pos, span.start),
                                    &mut has_last_line_comment,
                                )
                                .collect(),
                            )
                            .nest(ctx.indent_width),
                        );
                        if has_last_line_comment || has_comma && index == 0 {
                            docs.push(
                                Doc::hard_line()
                                    .append(function.doc(ctx, state))
                                    .nest(ctx.indent_width),
                            );
                        } else if index == 0 {
                            docs.push(Doc::space());
                            docs.push(function.doc(ctx, state));
                        } else if matches!(
                            self.value.get(index - 1),
                            Some(ComponentValue::Delimiter(Delimiter {
                                kind: DelimiterKind::Solidus | DelimiterKind::Comma,
                                ..
                            }))
                        ) {
                            // spaces around solidus have been
                            // considered when formatting solidus
                            docs.push(function.doc(ctx, state));
                        } else {
                            docs.push(
                                Doc::line_or_space()
                                    .append(function.doc(ctx, state))
                                    .group()
                                    .nest(ctx.indent_width),
                            );
                        }
                    } else {
                        docs.push(
                            Doc::list(
                                ctx.end_spaced_comments(ctx.get_comments_between(pos, span.start))
                                    .collect(),
                            )
                            .nest(ctx.indent_width),
                        );
                        docs.push(value.doc(ctx, state));
                    }
                    match value {
                        ComponentValue::Delimiter(Delimiter {
                            kind: DelimiterKind::Comma,
                            ..
                        }) => docs.push(space_after_comma.clone().nest(ctx.indent_width)),
                        ComponentValue::Delimiter(Delimiter {
                            kind: DelimiterKind::Semicolon,
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
                            Some((
                                _,
                                ComponentValue::Delimiter(Delimiter {
                                    kind: DelimiterKind::Comma | DelimiterKind::Semicolon,
                                    ..
                                })
                                | ComponentValue::Function(..),
                            ))
                            | None => {}
                            Some((
                                _,
                                ComponentValue::Delimiter(Delimiter {
                                    kind: DelimiterKind::Solidus,
                                    span: next_span,
                                }),
                            )) => {
                                if span.end < next_span.start {
                                    docs.push(Doc::soft_line().nest(ctx.indent_width));
                                }
                            }
                            Some((_, next)) => {
                                if span.end < next.span().start {
                                    docs.push(Doc::soft_line().nest(ctx.indent_width));
                                }
                            }
                        },
                    }

                    pos = span.end;
                }
            }
        }

        if let Some(important) = &self.important {
            docs.push(Doc::soft_line().nest(ctx.indent_width));
            docs.push(
                Doc::list(
                    ctx.end_spaced_comments(ctx.get_comments_between(pos, important.span.start))
                        .collect(),
                )
                .nest(ctx.indent_width),
            );
            docs.push(important.doc(ctx, state));
        }

        Doc::list(docs).group()
    }
}

impl<'s> DocGen<'s> for ImportantAnnotation<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text("!important")
    }
}

impl<'s> DocGen<'s> for QualifiedRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut keep_decl_name_case = false;
        if let [ComplexSelector { children, .. }] = &self.selector.selectors[..]
            && let [ComplexSelectorChild::CompoundSelector(CompoundSelector { children, .. })] =
                &children[..]
            && let [
                SimpleSelector::PseudoClass(PseudoClassSelector {
                    name: InterpolableIdent::Literal(Ident { raw: "export", .. }),
                    arg: None,
                    ..
                }),
            ] = &children[..]
        {
            // CSS modules
            keep_decl_name_case = true;
        }
        let state = State {
            keep_decl_name_case,
            ..state.clone()
        };
        // we don't use `SelectorList::doc` here
        // because it's a special case for qualified rule
        helpers::format_selectors_before_block(
            &self.selector.selectors,
            &self.selector.comma_spans,
            self.selector.span.start,
            ctx,
            &state,
        )
        .append(helpers::format_space_before_block(
            self.selector.span.end,
            self.block.span.start,
            ctx,
        ))
        .append(self.block.doc(ctx, &state))
    }
}

impl<'s> DocGen<'s> for SimpleBlock<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
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
            state,
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
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            Statement::QualifiedRule(qualified_rule) => qualified_rule.doc(ctx, state),
            Statement::AtRule(at_rule) => at_rule.doc(ctx, state),
            Statement::Declaration(declaration) => declaration.doc(ctx, state),
            Statement::KeyframeBlock(keyframe_block) => keyframe_block.doc(ctx, state),
            Statement::LessConditionalQualifiedRule(less_conditional_qualified_rule) => {
                less_conditional_qualified_rule.doc(ctx, state)
            }
            Statement::LessExtendRule(less_extend_rule) => less_extend_rule.doc(ctx, state),
            Statement::LessFunctionCall(less_function_call) => less_function_call.doc(ctx, state),
            Statement::LessMixinCall(less_mixin_call) => less_mixin_call.doc(ctx, state),
            Statement::LessMixinDefinition(less_mixin_definition) => {
                less_mixin_definition.doc(ctx, state)
            }
            Statement::LessVariableCall(less_variable_call) => less_variable_call.doc(ctx, state),
            Statement::LessVariableDeclaration(less_variable_declaration) => {
                less_variable_declaration.doc(ctx, state)
            }
            Statement::SassIfAtRule(sass_if_at_rule) => sass_if_at_rule.doc(ctx, state),
            Statement::SassVariableDeclaration(sass_variable_declaration) => {
                sass_variable_declaration.doc(ctx, state)
            }
            Statement::UnknownSassAtRule(unknown_sass_at_rule) => {
                unknown_sass_at_rule.doc(ctx, state)
            }
        }
    }
}

impl<'s> DocGen<'s> for Stylesheet<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut stmt_docs = vec![];
        if ctx.syntax == Syntax::Css
            && ctx.options.single_line_top_level_declarations
            && self
                .statements
                .iter()
                .all(|stmt| matches!(stmt, Statement::Declaration(..)))
        {
            // Declarations can't be at the top level in CSS,
            // but parser allows them and treat them as recoverable errors.
            // This situation can happen when formatting declarations
            // inside `style` attribute in HTML.
            // All comments are ignored.
            stmt_docs = itertools::intersperse(
                self.statements.iter().map(|stmt| stmt.doc(ctx, state)),
                Doc::text("; "),
            )
            .collect();
        } else {
            format_statements(
                &mut stmt_docs,
                &self.statements,
                &self.span,
                Doc::hard_line(),
                ctx,
                state,
            );
        }

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
    state: &State,
) {
    use crate::config::DeclarationOrderGroupBy;

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
                    SingleStmtFormatter {
                        stmt,
                        next_stmt: next_stmt.copied(),
                        pos: &mut pos,
                        outer_span,
                        ignore_leading_whitespace: true,
                        line_break_doc: line_break_doc.clone(),
                    }
                    .format(ctx, state),
                ));
                let is_grouped_by_empty_line = matches!(
                    ctx.options.declaration_order_group_by,
                    DeclarationOrderGroupBy::NonDeclarationAndEmptyLine
                ) && match next_stmt {
                    Some(Statement::Declaration(declaration)) => {
                        let next_start = declaration.span.start;
                        ctx.line_bounds.line_distance(pos, next_start) > 1
                            && ctx.get_comments_between(pos, next_start).count() == 0
                    }
                    _ => false,
                };
                // the end boundary of sortable declarations group
                if !matches!(
                    next_stmt,
                    Some(Statement::Declaration(Declaration {
                        name: InterpolableIdent::Literal(..),
                        ..
                    }))
                ) || is_grouped_by_empty_line
                {
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
                    if is_first_stmt_or_decls_group {
                        is_first_stmt_or_decls_group = false;
                    } else {
                        docs.push(Doc::hard_line());
                    }
                    docs.extend(
                        itertools::intersperse(
                            sortable_decls.drain(..).map(|(_, docs)| docs),
                            vec![Doc::hard_line()],
                        )
                        .flatten(),
                    );
                    if is_grouped_by_empty_line {
                        docs.push(Doc::empty_line());
                    }
                }
            } else {
                docs.append(
                    &mut SingleStmtFormatter {
                        stmt,
                        next_stmt: next_stmt.copied(),
                        pos: &mut pos,
                        outer_span,
                        ignore_leading_whitespace: false,
                        line_break_doc: line_break_doc.clone(),
                    }
                    .format(ctx, state),
                );
                is_first_stmt_or_decls_group = false;
            }
        }
    } else {
        while let Some(stmt) = stmts.next() {
            docs.append(
                &mut SingleStmtFormatter {
                    stmt,
                    next_stmt: stmts.peek().copied(),
                    pos: &mut pos,
                    outer_span,
                    ignore_leading_whitespace: false,
                    line_break_doc: line_break_doc.clone(),
                }
                .format(ctx, state),
            );
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
            docs.push(format_comment(comment, ctx));
            pos = comment.span.end;
        });
}

struct SingleStmtFormatter<'a, 's> {
    stmt: &'a Statement<'s>,
    next_stmt: Option<&'a Statement<'s>>,
    pos: &'a mut usize,
    outer_span: &'a Span,
    ignore_leading_whitespace: bool,
    line_break_doc: Doc<'s>,
}
impl<'s> SingleStmtFormatter<'_, 's> {
    fn format(self, ctx: &Ctx<'_, 's>, state: &State) -> Vec<Doc<'s>> {
        use crate::state::SelectorOverride;

        let mut docs = Vec::with_capacity(3);

        let span = self.stmt.span();
        let is_qualified_rule = matches!(self.stmt, Statement::QualifiedRule(..));
        let mut selector_override = SelectorOverride::Unset;

        let comments = ctx.get_comments_between(*self.pos, span.start);
        let has_comments =
            comments
                .clone()
                .fold(!self.ignore_leading_whitespace, |has_comments, comment| {
                    if has_comments && *self.pos > self.outer_span.start {
                        match ctx.line_bounds.line_distance(*self.pos, comment.span.start) {
                            0 => docs.push(Doc::space()),
                            1 => docs.push(self.line_break_doc.clone()),
                            _ => {
                                docs.push(Doc::empty_line());
                                docs.push(Doc::hard_line());
                            }
                        }
                    }
                    docs.push(format_comment(comment, ctx));
                    if is_qualified_rule {
                        selector_override = comment
                            .content
                            .trim()
                            .strip_prefix(&ctx.options.selector_override_comment_directive)
                            .map(|s| match s.strip_prefix(':').unwrap_or(s).trim() {
                                "ignore" => SelectorOverride::Ignore,
                                "always" => SelectorOverride::Always,
                                "consistent" => SelectorOverride::Consistent,
                                "wrap" => SelectorOverride::Wrap,
                                _ => SelectorOverride::Unset,
                            })
                            .unwrap_or(SelectorOverride::Unset);
                    }
                    *self.pos = comment.span.end;
                    true
                });

        if has_comments && *self.pos > self.outer_span.start {
            if ctx.line_bounds.line_distance(*self.pos, span.start) <= 1 {
                docs.push(self.line_break_doc);
            } else {
                docs.push(Doc::empty_line());
                docs.push(Doc::hard_line());
            }
        }
        let state = if is_qualified_rule {
            &State {
                selector_override,
                ..state.clone()
            }
        } else {
            state
        };
        if comments
            .last()
            .and_then(|comment| {
                comment
                    .content
                    .trim_start()
                    .strip_prefix(&ctx.options.ignore_comment_directive)
            })
            .is_some_and(|rest| {
                rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace())
            })
        {
            if let Some(source) = ctx.source {
                docs.extend(itertools::intersperse(
                    source[span.start..span.end].lines().map(Doc::text),
                    Doc::empty_line(),
                ));
            } else {
                docs.push(self.stmt.doc(ctx, state));
            }
        } else {
            docs.push(self.stmt.doc(ctx, state));
        }
        *self.pos = span.end;

        if ctx.syntax != Syntax::Sass {
            match self.stmt {
                Statement::AtRule(at_rule) if at_rule.block.is_none() => docs.push(Doc::text(";")),
                Statement::Declaration(decl)
                    if !matches!(
                        decl.value.last(),
                        Some(ComponentValue::SassNestingDeclaration(..))
                    ) =>
                {
                    docs.push(Doc::text(";"));
                }
                Statement::LessExtendRule(..)
                | Statement::LessFunctionCall(..)
                | Statement::LessMixinCall(..)
                | Statement::LessVariableCall(..)
                | Statement::LessVariableDeclaration(..)
                | Statement::SassVariableDeclaration(..) => docs.push(Doc::text(";")),
                Statement::UnknownSassAtRule(unknown_sass_at_rule)
                    if unknown_sass_at_rule.block.is_none() =>
                {
                    docs.push(Doc::text(";"));
                }
                _ => {}
            }
        }

        ctx.get_comments_between(
            *self.pos,
            self.next_stmt
                .map(|next| next.span().start)
                .unwrap_or_else(|| self.outer_span.end),
        )
        .for_each(|comment| {
            if *self.pos > self.outer_span.start
                && ctx.line_bounds.line_distance(*self.pos, comment.span.start) == 0
            {
                docs.push(Doc::space());
                docs.push(format_comment(comment, ctx));
                *self.pos = comment.span.end;
            }
        });

        docs
    }
}
