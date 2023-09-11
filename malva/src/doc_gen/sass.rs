use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for SassInterpolatedIdent<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
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

impl DocGen for SassModuleMemberName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            SassModuleMemberName::Ident(ident) => ident.doc(ctx),
            SassModuleMemberName::Variable(variable) => variable.doc(ctx),
        }
    }
}

impl DocGen for SassNestingDeclaration<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.block.doc(ctx)
    }
}

impl DocGen for SassParenthesizedExpression<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
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

impl DocGen for SassPlaceholderSelector<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::text("%").append(self.name.doc(ctx))
    }
}

impl DocGen for SassQualifiedName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(vec![
            self.module.doc(ctx),
            Doc::text("."),
            self.member.doc(ctx),
        ])
    }
}

impl DocGen for SassVariable<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(format!("${}", self.name.raw))
    }
}
