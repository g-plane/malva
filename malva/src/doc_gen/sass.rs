use super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use std::{iter, mem};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for SassArbitraryArgument<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.value.doc(ctx).append(Doc::text("..."))
    }
}

impl<'s> DocGen<'s> for SassAtRoot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match &self.kind {
            SassAtRootKind::Selector(selector) => selector.doc(ctx),
            SassAtRootKind::Query(query) => query.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for SassAtRootQuery<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(7);
        docs.push(Doc::text("("));
        docs.extend(ctx.end_padded_comments(self.span.start, self.modifier.span.start));

        docs.push(self.modifier.doc(ctx));
        docs.extend(ctx.start_padded_comments(self.modifier.span.end, self.colon_span.start));
        docs.push(Doc::text(": "));

        docs.extend(
            itertools::intersperse(
                self.rules.iter().scan(self.colon_span.start, |pos, rule| {
                    let rule_span = rule.span();
                    Some(
                        ctx.end_padded_comments(mem::replace(pos, rule_span.end), rule_span.start)
                            .chain(iter::once(rule.doc(ctx)))
                            .collect::<Vec<_>>()
                            .into_iter(),
                    )
                }),
                vec![Doc::soft_line()].into_iter(),
            )
            .flatten(),
        );

        if let Some(last) = self.rules.last() {
            docs.extend(ctx.start_padded_comments(last.span().end, self.span.end));
        }

        docs.push(Doc::text(")"));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassAtRootQueryModifier {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            SassAtRootQueryModifierKind::With => Doc::text("with"),
            SassAtRootQueryModifierKind::Without => Doc::text("without"),
        }
    }
}

impl<'s> DocGen<'s> for SassAtRootQueryRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            SassAtRootQueryRule::Ident(ident) => ident.doc(ctx),
            SassAtRootQueryRule::Str(str) => str.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for SassBinaryExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        self.left
            .doc(ctx)
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::soft_line().nest(ctx.indent_width),
                OperatorLineBreak::After => Doc::space(),
            })
            .concat(ctx.end_padded_comments(self.left.span().end, self.op.span.start))
            .append(self.op.doc(ctx))
            .append(match ctx.options.operator_linebreak {
                OperatorLineBreak::Before => Doc::space(),
                OperatorLineBreak::After => Doc::soft_line().nest(ctx.indent_width),
            })
            .concat(ctx.end_padded_comments(self.op.span.end, self.right.span().start))
            .append(self.right.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassBinaryOperator {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
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
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.condition
            .doc(ctx)
            .append(Doc::space())
            .concat(ctx.end_padded_comments(self.condition.span().end, self.block.span.start))
            .append(self.block.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassEach<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        super::format_comma_separated_list(
            &self.bindings,
            &self.comma_spans,
            self.span.start,
            Doc::line_or_space(),
            ctx,
        )
        .group()
        .append(Doc::space())
        .concat(ctx.end_padded_comments(self.bindings.last().unwrap().span.end, self.in_span.start))
        .append(Doc::text("in "))
        .concat(ctx.end_padded_comments(self.in_span.end, self.expr.span().start))
        .append(self.expr.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassFlag<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("!{}", self.keyword.raw))
    }
}

impl<'s> DocGen<'s> for SassIfAtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![Doc::text("@if ")];
        docs.extend(ctx.end_padded_comments(self.span.start, self.if_clause.span.start));
        docs.push(self.if_clause.doc(ctx));
        let mut pos = self.if_clause.span.end;

        docs.extend(
            self.else_if_clauses
                .iter()
                .zip(self.else_spans.iter())
                .scan(&mut pos, |pos, (clause, elseif_span)| {
                    Some(
                        iter::once(Doc::space())
                            .chain(ctx.end_padded_comments(
                                mem::replace(*pos, elseif_span.end),
                                elseif_span.start,
                            ))
                            .chain(iter::once(Doc::text("@else if ")))
                            .chain(ctx.end_padded_comments(
                                mem::replace(*pos, clause.span.end),
                                clause.span.start,
                            ))
                            .chain(iter::once(clause.doc(ctx))),
                    )
                })
                .flatten(),
        );

        if let Some((else_clause, else_span)) =
            self.else_clause.as_ref().zip(self.else_spans.last())
        {
            docs.reserve(3);
            docs.push(Doc::space());
            docs.extend(ctx.end_padded_comments(pos, else_span.start));
            docs.push(Doc::text("@else "));
            docs.extend(ctx.end_padded_comments(else_span.end, else_clause.span.start));
            docs.push(else_clause.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassInterpolatedIdent<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(self.elements.len());
        let mut iter = self.elements.iter().peekable();
        let mut pos = self.span.start;
        while let Some(element) = iter.next() {
            match element {
                SassInterpolatedIdentElement::Static(s) => {
                    pos = s.span.end;
                    docs.push(s.doc(ctx));
                }
                SassInterpolatedIdentElement::Expression(expr) => {
                    let expr_span = expr.span();
                    docs.push(Doc::text("#{"));
                    docs.extend(ctx.end_padded_comments(pos, expr_span.start));
                    docs.push(expr.doc(ctx));
                    docs.extend(
                        ctx.start_padded_comments(
                            expr_span.end,
                            iter.peek()
                                .map(|element| element.span().start)
                                .unwrap_or(self.span.end),
                        ),
                    );
                    docs.push(Doc::text("}"));
                }
            }
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassInterpolatedUrl<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(self.elements.len());
        let mut iter = self.elements.iter().peekable();
        let mut pos = self.span.start;
        while let Some(element) = iter.next() {
            match element {
                SassInterpolatedUrlElement::Static(s) => {
                    pos = s.span.end;
                    docs.push(s.doc(ctx));
                }
                SassInterpolatedUrlElement::Expression(expr) => {
                    let expr_span = expr.span();
                    docs.push(Doc::text("#{"));
                    docs.extend(ctx.end_padded_comments(pos, expr_span.start));
                    docs.push(expr.doc(ctx));
                    docs.extend(
                        ctx.start_padded_comments(
                            expr_span.end,
                            iter.peek()
                                .map(|element| element.span().start)
                                .unwrap_or(self.span.end),
                        ),
                    );
                    docs.push(Doc::text("}"));
                }
            }
        }

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for SassList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        super::format_values_list(&self.elements, self.comma_spans.as_deref(), &self.span, ctx)
    }
}

impl<'s> DocGen<'s> for SassMap<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("(")
            .append(
                Doc::line_or_nil()
                    .append(super::format_comma_separated_list_with_trailing(
                        &self.items,
                        &self.comma_spans,
                        self.span.start,
                        Doc::line_or_space(),
                        ctx,
                    ))
                    .nest(ctx.indent_width)
                    .append(Doc::line_or_nil())
                    .group(),
            )
            .concat(
                ctx.start_padded_comments(
                    self.items
                        .last()
                        .map(|item| item.span.end)
                        .unwrap_or(self.span.start),
                    self.span.end,
                ),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for SassMapItem<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.key
            .doc(ctx)
            .concat(ctx.start_padded_comments(self.key.span().end, self.colon_span.start))
            .append(Doc::text(": "))
            .concat(ctx.end_padded_comments(self.colon_span.end, self.value.span().start))
            .append(self.value.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassModuleMemberName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            SassModuleMemberName::Ident(ident) => ident.doc(ctx),
            SassModuleMemberName::Variable(variable) => variable.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for SassNestingDeclaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.block.doc(ctx)
    }
}

impl<'s> DocGen<'s> for SassParenthesizedExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let expr_span = self.expr.span();
        Doc::text("(")
            .append(
                Doc::line_or_nil()
                    .concat(ctx.end_padded_comments(self.span.start, expr_span.start))
                    .append(self.expr.doc(ctx))
                    .concat(ctx.start_padded_comments(expr_span.end, self.span.end))
                    .nest(ctx.indent_width)
                    .append(Doc::line_or_nil())
                    .group(),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for SassPlaceholderSelector<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("%").append(self.name.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassQualifiedName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(vec![
            self.module.doc(ctx),
            Doc::text("."),
            self.member.doc(ctx),
        ])
    }
}

impl<'s> DocGen<'s> for SassUnaryExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.op.doc(ctx).append(self.expr.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassUnaryOperator {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        match self.kind {
            SassUnaryOperatorKind::Plus => Doc::text("+"),
            SassUnaryOperatorKind::Minus => Doc::text("-"),
            SassUnaryOperatorKind::Not => Doc::text("not "),
        }
    }
}

impl<'s> DocGen<'s> for SassVariable<'s> {
    fn doc(&self, _: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text(format!("${}", self.name.raw))
    }
}

impl<'s> DocGen<'s> for SassVariableDeclaration<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        let value_span = self.value.span();

        if let Some(namespace) = &self.namespace {
            docs.push(namespace.doc(ctx));
            docs.push(Doc::text("."));
        }
        docs.push(self.name.doc(ctx));

        docs.extend(ctx.start_padded_comments(self.name.span.end, self.colon_span.start));
        docs.push(Doc::text(": "));
        docs.extend(ctx.end_padded_comments(self.colon_span.end, value_span.start));

        docs.push(self.value.doc(ctx));

        docs.extend(
            self.flags
                .iter()
                .scan(value_span.end, |pos, flag| {
                    Some(
                        iter::once(Doc::soft_line())
                            .chain(ctx.end_padded_comments(
                                mem::replace(pos, flag.span.end),
                                flag.span.start,
                            ))
                            .chain(iter::once(flag.doc(ctx)))
                            .collect::<Vec<_>>()
                            .into_iter(),
                    )
                })
                .flatten(),
        );

        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for UnknownSassAtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(6);
        let mut pos = self.name.span().end;

        docs.push(Doc::text("@"));
        docs.push(self.name.doc(ctx));

        if let Some(prelude) = &self.prelude {
            docs.push(Doc::space());
            let span = prelude.span();
            docs.extend(ctx.end_padded_comments(pos, span.start));
            docs.push(prelude.doc(ctx));
            pos = span.end;
        }

        if let Some(block) = &self.block {
            docs.push(Doc::space());
            docs.extend(ctx.end_padded_comments(pos, block.span.start));
            docs.push(block.doc(ctx));
        }

        Doc::list(docs)
    }
}
