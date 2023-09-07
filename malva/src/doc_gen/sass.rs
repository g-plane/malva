use super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

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

impl DocGen for SassVariable<'_> {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(format!("${}", self.name.raw))
    }
}
