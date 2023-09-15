use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for SassConditionalClause<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        self.condition
            .doc(ctx)
            .append(Doc::space())
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
        .append(Doc::text(" in "))
        .append(self.expr.doc(ctx))
    }
}

impl<'s> DocGen<'s> for SassIfAtRule<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = vec![Doc::text("@if "), self.if_clause.doc(ctx)];
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
        Doc::list(
            self.elements
                .iter()
                .map(|element| match element {
                    SassInterpolatedIdentElement::Static(s) => s.doc(ctx),
                    SassInterpolatedIdentElement::Expression(expr) => {
                        Doc::text("#{").append(expr.doc(ctx)).append(Doc::text("}"))
                    }
                })
                .collect(),
        )
    }
}

impl<'s> DocGen<'s> for SassInterpolatedUrl<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            self.elements
                .iter()
                .map(|element| match element {
                    SassInterpolatedUrlElement::Static(s) => s.doc(ctx),
                    SassInterpolatedUrlElement::Expression(expr) => {
                        Doc::text("#{").append(expr.doc(ctx)).append(Doc::text("}"))
                    }
                })
                .collect(),
        )
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
        Doc::text("(")
            .append(
                Doc::line_or_nil()
                    .append(self.expr.doc(ctx))
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
