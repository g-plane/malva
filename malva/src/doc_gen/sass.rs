use super::{
    helpers,
    str::{
        format_str, InterpolatedFirstStrRawFormatter, InterpolatedLastStrRawFormatter,
        InterpolatedMidStrRawFormatter,
    },
    DocGen,
};
use crate::{ctx::Ctx, state::State};
use raffia::{ast::*, Spanned};
use std::{iter, mem};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for SassArbitraryArgument<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.value.doc(ctx, state).append(Doc::text("..."))
    }
}

impl<'s> DocGen<'s> for SassArbitraryParameter<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.name.doc(ctx, state).append(Doc::text("..."))
    }
}

impl<'s> DocGen<'s> for SassAtRoot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match &self.kind {
            SassAtRootKind::Selector(selector) => selector.doc(ctx, state),
            SassAtRootKind::Query(query) => query.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SassAtRootQuery<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(7);
        docs.push(Doc::text("("));
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.span.start, self.modifier.span.start),
        ));

        docs.push(self.modifier.doc(ctx, state));
        docs.extend(ctx.start_spaced_comments(
            ctx.get_comments_between(self.modifier.span.end, self.colon_span.start),
        ));
        docs.push(Doc::text(": "));

        docs.extend(
            itertools::intersperse(
                self.rules.iter().scan(self.colon_span.start, |pos, rule| {
                    let rule_span = rule.span();
                    Some(
                        ctx.end_spaced_comments(ctx.get_comments_between(
                            mem::replace(pos, rule_span.end),
                            rule_span.start,
                        ))
                        .chain(iter::once(rule.doc(ctx, state)))
                        .collect::<Vec<_>>()
                        .into_iter(),
                    )
                }),
                vec![Doc::soft_line()].into_iter(),
            )
            .flatten(),
        );

        if let Some(last) = self.rules.last() {
            docs.extend(
                ctx.start_spaced_comments(ctx.get_comments_between(last.span().end, self.span.end)),
            );
        }

        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassAtRootQueryModifier {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        match self.kind {
            SassAtRootQueryModifierKind::With => Doc::text("with"),
            SassAtRootQueryModifierKind::Without => Doc::text("without"),
        }
    }
}

impl<'s> DocGen<'s> for SassAtRootQueryRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            SassAtRootQueryRule::Ident(ident) => ident.doc(ctx, state),
            SassAtRootQueryRule::Str(str) => str.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SassBinaryExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.left
            .doc(ctx, state)
            .append(helpers::format_operator_prefix_space(ctx))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.left.span().end, self.op.span.start),
            ))
            .append(self.op.doc(ctx, state))
            .append(helpers::format_operator_suffix_space(ctx))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.op.span.end, self.right.span().start),
            ))
            .append(self.right.doc(ctx, state))
            .group()
    }
}

impl<'s> DocGen<'s> for SassBinaryOperator {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(match self.kind {
            SassBinaryOperatorKind::Multiply => "*",
            SassBinaryOperatorKind::Division => "/",
            SassBinaryOperatorKind::Modulo => "%",
            SassBinaryOperatorKind::Plus => "+",
            SassBinaryOperatorKind::Minus => "-",
            SassBinaryOperatorKind::GreaterThan => ">",
            SassBinaryOperatorKind::GreaterThanOrEqual => ">=",
            SassBinaryOperatorKind::LessThan => "<",
            SassBinaryOperatorKind::LessThanOrEqual => "<=",
            SassBinaryOperatorKind::EqualsEquals => "==",
            SassBinaryOperatorKind::ExclamationEquals => "!=",
            SassBinaryOperatorKind::And => "and",
            SassBinaryOperatorKind::Or => "or",
        })
    }
}

impl<'s> DocGen<'s> for SassConditionalClause<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.condition
            .doc(ctx, state)
            .append(helpers::format_space_before_block(
                self.condition.span().end,
                self.block.span.start,
                ctx,
            ))
            .append(self.block.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassContent<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                ",",
                helpers::get_smart_linebreak(
                    self.span.start,
                    &self.args,
                    ctx.options.sass_content_at_rule_prefer_single_line,
                    ctx,
                ),
            )
            .with_trailing()
            .format(&self.args, &self.comma_spans, self.span.start, ctx, state),
            self.args
                .len()
                .checked_sub(1)
                .and_then(|i| self.comma_spans.get(i))
                .or_else(|| self.args.last().map(|param| param.span()))
                .map(|span| span.end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for SassEach<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(
                &self.bindings,
                &self.comma_spans,
                self.span.start,
                ctx,
                state,
            )
            .group()
            .nest(ctx.indent_width)
            .append(helpers::format_operator_prefix_space(ctx))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(
                    self.bindings.last().unwrap().span.end,
                    self.in_span.start,
                ),
            ))
            .append(Doc::text("in"))
            .append(helpers::format_operator_suffix_space(ctx))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.in_span.end, self.expr.span().start),
            ))
            .append(self.expr.doc(ctx, state).nest(ctx.indent_width))
            .group()
    }
}

impl<'s> DocGen<'s> for SassExtend<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let selectors =
            helpers::SeparatedListFormatter::new(",", Doc::line_or_space().nest(ctx.indent_width))
                .format(
                    &self.selectors.selectors,
                    &self.selectors.comma_spans,
                    self.selectors.span.start,
                    ctx,
                    state,
                )
                .group();
        if let Some(optional) = &self.optional {
            selectors
                .append(Doc::space())
                .concat(ctx.end_spaced_comments(
                    ctx.get_comments_between(self.selectors.span().end, optional.span.start),
                ))
                .append(optional.doc(ctx, state))
        } else {
            selectors
        }
    }
}

impl<'s> DocGen<'s> for SassFlag<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(format!("!{}", self.keyword.raw))
    }
}

impl<'s> DocGen<'s> for SassFor<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let start_value_span = self.start.span();
        self.binding
            .doc(ctx, state)
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::soft_line().nest(ctx.indent_width),
                OperatorLineBreak::After => Doc::space(),
            })
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.binding.span.end, self.from_span.start),
            ))
            .append(Doc::text("from"))
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::space(),
                OperatorLineBreak::After => Doc::soft_line().nest(ctx.indent_width),
            })
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.from_span.end, start_value_span.start),
            ))
            .append(self.start.doc(ctx, state))
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::soft_line().nest(ctx.indent_width),
                OperatorLineBreak::After => Doc::space(),
            })
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(start_value_span.end, self.boundary.span.start),
            ))
            .append(self.boundary.doc(ctx, state))
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::space(),
                OperatorLineBreak::After => Doc::soft_line().nest(ctx.indent_width),
            })
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.boundary.span.end, self.end.span().start),
            ))
            .append(self.end.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassForBoundary {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        match self.kind {
            SassForBoundaryKind::Exclusive => Doc::text("to"),
            SassForBoundaryKind::Inclusive => Doc::text("through"),
        }
    }
}

impl<'s> DocGen<'s> for SassForward<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = vec![self.path.doc(ctx, state)];
        let mut pos = self.path.span().end;

        if let Some(prefix) = &self.prefix {
            docs.reserve(2);
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(
                ctx.get_comments_between(
                    mem::replace(&mut pos, prefix.span.end),
                    prefix.span.start,
                ),
            ));
            docs.push(prefix.doc(ctx, state));
        }

        if let Some(visibility) = &self.visibility {
            docs.reserve(2);
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(
                mem::replace(&mut pos, visibility.span.end),
                visibility.span.start,
            )));
            docs.push(visibility.doc(ctx, state));
        }

        if let Some(config) = &self.config {
            docs.reserve(2);
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(
                ctx.get_comments_between(
                    mem::replace(&mut pos, config.span.end),
                    config.span.start,
                ),
            ));
            docs.push(config.doc(ctx, state));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassForwardMember<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            SassForwardMember::Ident(ident) => ident.doc(ctx, state),
            SassForwardMember::Variable(variable) => variable.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SassForwardPrefix<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("as ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.as_span.end, self.name.span.start),
            ))
            .append(self.name.doc(ctx, state))
            .append(Doc::text("*"))
    }
}

impl<'s> DocGen<'s> for SassForwardVisibility<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.modifier.doc(ctx, state).append(Doc::space()).append(
            helpers::SeparatedListFormatter::new(",", Doc::soft_line()).format(
                &self.members,
                &self.comma_spans,
                self.modifier.span.end,
                ctx,
                state,
            ),
        )
    }
}

impl<'s> DocGen<'s> for SassForwardVisibilityModifier {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        match self.kind {
            SassForwardVisibilityModifierKind::Hide => Doc::text("hide"),
            SassForwardVisibilityModifierKind::Show => Doc::text("show"),
        }
    }
}

impl<'s> DocGen<'s> for SassFunction<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.name
            .doc(ctx, state)
            .append(self.parameters.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassIfAtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = vec![Doc::text("@if ")];
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.span.start, self.if_clause.span.start),
        ));
        docs.push(self.if_clause.doc(ctx, state));
        let mut pos = self.if_clause.span.end;

        docs.extend(
            self.else_if_clauses
                .iter()
                .zip(self.else_spans.iter())
                .scan(&mut pos, |pos, (clause, elseif_span)| {
                    Some(
                        iter::once(Doc::space())
                            .chain(ctx.end_spaced_comments(ctx.get_comments_between(
                                mem::replace(*pos, elseif_span.end),
                                elseif_span.start,
                            )))
                            .chain(iter::once(Doc::text("@else if ")))
                            .chain(ctx.end_spaced_comments(ctx.get_comments_between(
                                mem::replace(*pos, clause.span.end),
                                clause.span.start,
                            )))
                            .chain(iter::once(clause.doc(ctx, state))),
                    )
                })
                .flatten(),
        );

        if let Some((else_clause, else_span)) =
            self.else_clause.as_ref().zip(self.else_spans.last())
        {
            docs.reserve(3);
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(pos, else_span.start)));
            docs.push(Doc::text("@else"));
            docs.push(helpers::format_space_before_block(
                else_span.end,
                else_clause.span.start,
                ctx,
            ));
            docs.push(else_clause.doc(ctx, state));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassImportPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space().nest(ctx.indent_width))
            .format(&self.paths, &self.comma_spans, self.span.start, ctx, state)
            .group()
    }
}

impl<'s> DocGen<'s> for SassInclude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = vec![self.name.doc(ctx, state)];
        let mut pos = self.name.span().end;

        if let Some(arguments) = &self.arguments {
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(
                mem::replace(&mut pos, arguments.span.end),
                arguments.span.start,
            )));
            docs.push(arguments.doc(ctx, state));
        }

        if let Some(content_block_params) = &self.content_block_params {
            docs.reserve(2);
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(
                mem::replace(&mut pos, content_block_params.span.end),
                content_block_params.span.start,
            )));
            docs.push(content_block_params.doc(ctx, state));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassIncludeArgs<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                ",",
                helpers::get_smart_linebreak(
                    self.span.start,
                    &self.args,
                    ctx.options.sass_include_at_rule_prefer_single_line,
                    ctx,
                ),
            )
            .with_trailing()
            .format(&self.args, &self.comma_spans, self.span.start, ctx, state),
            self.args
                .len()
                .checked_sub(1)
                .and_then(|i| self.comma_spans.get(i))
                .or_else(|| self.args.last().map(|param| param.span()))
                .map(|span| span.end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for SassIncludeContentBlockParams<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("using ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.using_span.end, self.params.span.start),
            ))
            .append(self.params.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassInterpolatedIdent<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(self.elements.len());
        let mut iter = self.elements.iter().peekable();
        let mut pos = self.span.start;
        while let Some(element) = iter.next() {
            match element {
                SassInterpolatedIdentElement::Static(s) => {
                    pos = s.span.end;
                    docs.push(s.doc(ctx, state));
                }
                SassInterpolatedIdentElement::Expression(expr) => {
                    let expr_span = expr.span();
                    docs.push(Doc::text("#{"));
                    docs.extend(
                        ctx.end_spaced_comments(ctx.get_comments_between(pos, expr_span.start)),
                    );
                    docs.push(expr.doc(ctx, state));
                    docs.extend(
                        ctx.start_spaced_comments(
                            ctx.get_comments_between(
                                expr_span.end,
                                iter.peek()
                                    .map(|element| element.span().start)
                                    .unwrap_or(self.span.end),
                            ),
                        ),
                    );
                    docs.push(Doc::text("}"));
                }
            }
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassInterpolatedStr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        if let [SassInterpolatedStrElement::Static(first), mid @ .., SassInterpolatedStrElement::Static(last)] =
            &self.elements[..]
        {
            let allow_prefer = is_preferred_quote_allowed(self, ctx);

            let mut docs = Vec::with_capacity(self.elements.len());
            docs.push(Doc::text(format_str(
                first.raw,
                InterpolatedFirstStrRawFormatter::new(first.raw),
                allow_prefer,
                ctx.options.quotes,
            )));
            let mut iter = mid.iter().peekable();
            let mut pos = first.span.end;
            while let Some(element) = iter.next() {
                match element {
                    SassInterpolatedStrElement::Static(s) => {
                        pos = s.span.end;
                        docs.push(Doc::text(format_str(
                            s.raw,
                            InterpolatedMidStrRawFormatter::new(s.raw),
                            allow_prefer,
                            ctx.options.quotes,
                        )));
                    }
                    SassInterpolatedStrElement::Expression(expr) => {
                        let expr_span = expr.span();
                        docs.push(Doc::text("#{"));
                        docs.extend(
                            ctx.end_spaced_comments(ctx.get_comments_between(pos, expr_span.start)),
                        );
                        docs.push(expr.doc(ctx, state));
                        docs.extend(
                            ctx.start_spaced_comments(
                                ctx.get_comments_between(
                                    expr_span.end,
                                    iter.peek()
                                        .map(|element| element.span().start)
                                        .unwrap_or(self.span.end),
                                ),
                            ),
                        );
                        docs.push(Doc::text("}"));
                    }
                }
            }
            docs.push(Doc::text(format_str(
                last.raw,
                InterpolatedLastStrRawFormatter::new(last.raw),
                allow_prefer,
                ctx.options.quotes,
            )));
            Doc::list(docs)
        } else {
            unreachable!()
        }
    }
}

impl<'s> DocGen<'s> for SassInterpolatedUrl<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(self.elements.len());
        let mut iter = self.elements.iter().peekable();
        let mut pos = self.span.start;
        while let Some(element) = iter.next() {
            match element {
                SassInterpolatedUrlElement::Static(s) => {
                    pos = s.span.end;
                    docs.push(s.doc(ctx, state));
                }
                SassInterpolatedUrlElement::Expression(expr) => {
                    let expr_span = expr.span();
                    docs.push(Doc::text("#{"));
                    docs.extend(
                        ctx.end_spaced_comments(ctx.get_comments_between(pos, expr_span.start)),
                    );
                    docs.push(expr.doc(ctx, state));
                    docs.extend(
                        ctx.start_spaced_comments(
                            ctx.get_comments_between(
                                expr_span.end,
                                iter.peek()
                                    .map(|element| element.span().start)
                                    .unwrap_or(self.span.end),
                            ),
                        ),
                    );
                    docs.push(Doc::text("}"));
                }
            }
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassKeywordArgument<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.name
            .doc(ctx, state)
            .concat(ctx.start_spaced_comments(
                ctx.get_comments_between(self.name.span.start, self.colon_span.start),
            ))
            .append(Doc::text(": "))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.colon_span.end, self.value.span().start),
            ))
            .append(self.value.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::format_values_list(
            &self.elements,
            self.comma_spans.as_deref(),
            &self.span,
            ctx,
            state,
        )
    }
}

impl<'s> DocGen<'s> for SassMap<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                ",",
                helpers::get_smart_linebreak(
                    self.span.start,
                    &self.items,
                    ctx.options.sass_map_prefer_single_line,
                    ctx,
                ),
            )
            .with_trailing()
            .format(&self.items, &self.comma_spans, self.span.start, ctx, state),
            self.items
                .last()
                .map(|item| item.span.end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for SassMapItem<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.key
            .doc(ctx, state)
            .concat(ctx.start_spaced_comments(
                ctx.get_comments_between(self.key.span().end, self.colon_span.start),
            ))
            .append(Doc::text(": "))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.colon_span.end, self.value.span().start),
            ))
            .append(self.value.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassMixin<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let name = self.name.doc(ctx, state);
        if let Some(parameters) = &self.parameters {
            name.append(parameters.doc(ctx, state))
        } else {
            name
        }
    }
}

impl<'s> DocGen<'s> for SassModuleConfig<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("with ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.with_span.end, self.lparen_span.start),
            ))
            .append(helpers::format_parenthesized(
                helpers::SeparatedListFormatter::new(
                    ",",
                    helpers::get_smart_linebreak(
                        self.span.start,
                        &self.items,
                        ctx.options.sass_module_config_prefer_single_line,
                        ctx,
                    ),
                )
                .with_trailing()
                .format(
                    &self.items,
                    &self.comma_spans,
                    self.lparen_span.end,
                    ctx,
                    state,
                ),
                self.items
                    .last()
                    .map(|item| item.span.end)
                    .unwrap_or(self.lparen_span.end),
                self.span.end,
                ctx,
            ))
    }
}

impl<'s> DocGen<'s> for SassModuleConfigItem<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let value_span = self.value.span();
        self.variable
            .doc(ctx, state)
            .concat(ctx.start_spaced_comments(
                ctx.get_comments_between(self.variable.span.end, self.colon_span.start),
            ))
            .append(Doc::text(": "))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.colon_span.end, value_span.start),
            ))
            .append(self.value.doc(ctx, state))
            .concat(
                self.flags
                    .iter()
                    .scan(value_span.end, |pos, flag| {
                        Some(
                            iter::once(Doc::soft_line())
                                .chain(ctx.end_spaced_comments(ctx.get_comments_between(
                                    mem::replace(pos, flag.span.end),
                                    flag.span.start,
                                )))
                                .chain(iter::once(flag.doc(ctx, state)))
                                .collect::<Vec<_>>()
                                .into_iter(),
                        )
                    })
                    .flatten(),
            )
    }
}

impl<'s> DocGen<'s> for SassModuleMemberName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            SassModuleMemberName::Ident(ident) => ident.doc(ctx, state),
            SassModuleMemberName::Variable(variable) => variable.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SassNestingDeclaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.block.doc(ctx, state)
    }
}

impl<'s> DocGen<'s> for SassParameter<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let name = self.name.doc(ctx, state);
        if let Some(default_value) = &self.default_value {
            name.concat(ctx.start_spaced_comments(
                ctx.get_comments_between(self.name.span.end, default_value.span.start),
            ))
            .append(default_value.doc(ctx, state))
        } else {
            name
        }
    }
}

impl<'s> DocGen<'s> for SassParameterDefaultValue<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text(": ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.colon_span.end, self.value.span().start),
            ))
            .append(self.value.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassParameters<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        enum ParameterOrArbitrary<'a, 's> {
            Parameter(&'a SassParameter<'s>),
            Arbitrary(&'a SassArbitraryParameter<'s>),
        }
        impl Spanned for ParameterOrArbitrary<'_, '_> {
            fn span(&self) -> &raffia::Span {
                match self {
                    ParameterOrArbitrary::Parameter(p) => p.span(),
                    ParameterOrArbitrary::Arbitrary(a) => a.span(),
                }
            }
        }
        impl<'s> DocGen<'s> for ParameterOrArbitrary<'_, 's> {
            fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
                match self {
                    ParameterOrArbitrary::Parameter(p) => p.doc(ctx, state),
                    ParameterOrArbitrary::Arbitrary(a) => a.doc(ctx, state),
                }
            }
        }

        let params = self
            .params
            .iter()
            .map(ParameterOrArbitrary::Parameter)
            .chain(
                self.arbitrary_param
                    .iter()
                    .map(ParameterOrArbitrary::Arbitrary),
            )
            .collect::<Vec<_>>();
        helpers::format_parenthesized(
            helpers::SeparatedListFormatter::new(
                ",",
                helpers::get_smart_linebreak(
                    self.span.start,
                    &params,
                    ctx.options.sass_params_prefer_single_line,
                    ctx,
                ),
            )
            .with_trailing()
            .format(&params, &self.comma_spans, self.span.start, ctx, state),
            self.params
                .len()
                .checked_sub(1)
                .and_then(|i| self.comma_spans.get(i))
                .or_else(|| self.params.last().map(|param| param.span()))
                .map(|span| span.end)
                .unwrap_or(self.span.start),
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for SassParenthesizedExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let expr_span = self.expr.span();
        helpers::format_parenthesized(
            Doc::list(
                ctx.end_spaced_comments(ctx.get_comments_between(self.span.start, expr_span.start))
                    .collect(),
            )
            .append(self.expr.doc(ctx, state)),
            expr_span.end,
            self.span.end,
            ctx,
        )
    }
}

impl<'s> DocGen<'s> for SassPlaceholderSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("%").append(self.name.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassQualifiedName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::list(vec![
            self.module.doc(ctx, state),
            Doc::text("."),
            self.member.doc(ctx, state),
        ])
    }
}

impl<'s> DocGen<'s> for SassUnaryExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.op.doc(ctx, state).append(self.expr.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for SassUnaryOperator {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        match self.kind {
            SassUnaryOperatorKind::Plus => Doc::text("+"),
            SassUnaryOperatorKind::Minus => Doc::text("-"),
            SassUnaryOperatorKind::Not => Doc::text("not "),
        }
    }
}

impl<'s> DocGen<'s> for SassUnnamedNamespace {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text("*")
    }
}

impl<'s> DocGen<'s> for SassUse<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = vec![self.path.doc(ctx, state)];
        let mut pos = self.path.span().end;

        if let Some(namespace) = &self.namespace {
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(
                mem::replace(&mut pos, namespace.span.end),
                namespace.span.start,
            )));
            docs.push(namespace.doc(ctx, state));
        }

        if let Some(config) = &self.config {
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(pos, config.span.start)));
            docs.push(config.doc(ctx, state));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassUseNamespace<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("as ")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.as_span.end, self.kind.span().start),
            ))
            .append(match &self.kind {
                SassUseNamespaceKind::Named(named) => named.doc(ctx, state),
                SassUseNamespaceKind::Unnamed(unnamed) => unnamed.doc(ctx, state),
            })
    }
}

impl<'s> DocGen<'s> for SassVariable<'s> {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(format!("${}", self.name.raw))
    }
}

impl<'s> DocGen<'s> for SassVariableDeclaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        let value_span = self.value.span();

        if let Some(namespace) = &self.namespace {
            docs.push(namespace.doc(ctx, state));
            docs.push(Doc::text("."));
        }
        docs.push(self.name.doc(ctx, state));

        docs.extend(ctx.start_spaced_comments(
            ctx.get_comments_between(self.name.span.end, self.colon_span.start),
        ));
        docs.push(Doc::text(":"));

        let should_group = match &self.value {
            ComponentValue::SassList(SassList {
                elements,
                comma_spans: Some(comma_spans),
                span,
                ..
            }) => {
                docs.push(Doc::line_or_space());
                docs.extend(ctx.end_spaced_comments(
                    ctx.get_comments_between(self.colon_span.end, value_span.start),
                ));
                docs.push(
                    helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
                        .with_trailing()
                        .format(elements, comma_spans, span.start, ctx, state),
                );
                if elements.len() == 1 {
                    docs.push(Doc::text(","));
                }
                true
            }
            ComponentValue::SassList(sass_list) => {
                docs.push(Doc::space());
                docs.extend(ctx.end_spaced_comments(
                    ctx.get_comments_between(self.colon_span.end, value_span.start),
                ));
                docs.push(sass_list.doc(ctx, state).nest(ctx.indent_width));
                false
            }
            _ => {
                docs.push(Doc::space());
                docs.extend(ctx.end_spaced_comments(
                    ctx.get_comments_between(self.colon_span.end, value_span.start),
                ));
                docs.push(self.value.doc(ctx, state));
                false
            }
        };

        docs.extend(
            self.flags
                .iter()
                .scan(value_span.end, |pos, flag| {
                    Some(
                        iter::once(Doc::soft_line().nest(ctx.indent_width))
                            .chain(ctx.end_spaced_comments(ctx.get_comments_between(
                                mem::replace(pos, flag.span.end),
                                flag.span.start,
                            )))
                            .chain(iter::once(flag.doc(ctx, state)))
                            .collect::<Vec<_>>()
                            .into_iter(),
                    )
                })
                .flatten(),
        );

        let doc = Doc::list(docs);
        if should_group {
            doc.group().nest(ctx.indent_width)
        } else {
            doc
        }
    }
}

impl<'s> DocGen<'s> for UnknownSassAtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(6);
        let mut pos = self.name.span().end;

        docs.push(Doc::text("@"));
        docs.push(self.name.doc(ctx, state));

        if let Some(prelude) = &self.prelude {
            docs.push(Doc::space());
            let span = prelude.span();
            docs.extend(ctx.end_spaced_comments(ctx.get_comments_between(pos, span.start)));
            docs.push(prelude.doc(ctx, state));
            pos = span.end;
        }

        if let Some(block) = &self.block {
            docs.push(helpers::format_space_before_block(
                pos,
                block.span.start,
                ctx,
            ));
            docs.push(block.doc(ctx, state));
        }

        Doc::list(docs)
    }
}

fn is_preferred_quote_allowed(interpolated_str: &SassInterpolatedStr, ctx: &Ctx) -> bool {
    use crate::config::Quotes;

    match ctx.options.quotes {
        Quotes::AlwaysDouble | Quotes::AlwaysSingle => false,
        Quotes::PreferDouble => interpolated_str
            .elements
            .iter()
            .any(|element| match element {
                SassInterpolatedStrElement::Static(InterpolableStrStaticPart { value, .. }) => {
                    value.contains('"')
                }
                SassInterpolatedStrElement::Expression(_) => false,
            }),
        Quotes::PreferSingle => interpolated_str
            .elements
            .iter()
            .any(|element| match element {
                SassInterpolatedStrElement::Static(InterpolableStrStaticPart { value, .. }) => {
                    value.contains('\'')
                }
                SassInterpolatedStrElement::Expression(_) => false,
            }),
    }
}
