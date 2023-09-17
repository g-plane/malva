use super::DocGen;
use crate::ctx::Ctx;
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for SassBinaryExpression<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.left
            .doc(ctx)
            .append(Doc::space())
            .concat(ctx.end_padded_comments(self.left.span().end, self.op.span.start))
            .append(self.op.doc(ctx))
            .append(Doc::space())
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
        Doc::list(
            itertools::intersperse(
                self.bindings.iter().map(|binding| binding.doc(ctx)),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )
        .append(Doc::space())
        .concat(ctx.end_padded_comments(self.bindings.last().unwrap().span.end, self.in_span.start))
        .append(Doc::text("in "))
        .concat(ctx.end_padded_comments(self.in_span.end, self.expr.span().start))
        .append(self.expr.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassIfAtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![Doc::text("@if ")];
        docs.extend(ctx.end_padded_comments(self.span.start, self.if_clause.span.start));
        docs.push(self.if_clause.doc(ctx));

        self.else_if_clauses.iter().for_each(|clause| {
            docs.push(Doc::text(" @else if "));
            docs.push(clause.doc(ctx));
        });

        if let Some(else_clause) = &self.else_clause {
            docs.push(Doc::text(" @else "));
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
                    .append(Doc::list(
                        itertools::intersperse(
                            self.items.iter().map(|item| item.doc(ctx)),
                            Doc::text(",").append(Doc::line_or_space()),
                        )
                        .collect(),
                    ))
                    .append(if ctx.options.trailing_comma {
                        Doc::flat_or_break(Doc::nil(), Doc::text(","))
                    } else {
                        Doc::nil()
                    })
                    .nest(ctx.indent_width)
                    .append(Doc::line_or_nil())
                    .group(),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for SassMapItem<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.key
            .doc(ctx)
            .append(Doc::text(": "))
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

        if let Some(namespace) = &self.namespace {
            docs.push(namespace.doc(ctx));
            docs.push(Doc::text("."));
        }
        docs.push(self.name.doc(ctx));

        docs.push(Doc::text(": "));

        docs.push(self.value.doc(ctx));

        if self.overridable {
            docs.push(Doc::text("!default"));
        }
        if self.force_global {
            docs.push(Doc::text("!global"));
        }

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
