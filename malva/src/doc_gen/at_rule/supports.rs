use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for SupportsAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("and"), Doc::line_or_space()],
        };
        docs.push(self.condition.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for SupportsCondition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::list(
            itertools::intersperse(
                self.conditions.iter().map(|condition| condition.doc(ctx)),
                Doc::space(),
            )
            .collect(),
        )
    }
}

impl<'s> DocGen<'s> for SupportsConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            SupportsConditionKind::SupportsInParens(supports_in_parens) => {
                supports_in_parens.doc(ctx)
            }
            SupportsConditionKind::And(and) => and.doc(ctx),
            SupportsConditionKind::Or(or) => or.doc(ctx),
            SupportsConditionKind::Not(not) => not.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for SupportsDecl<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("(")
            .append(self.decl.doc(ctx))
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for SupportsInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match &self.kind {
            SupportsInParensKind::Feature(feature) => feature.doc(ctx),
            SupportsInParensKind::SupportsCondition(condition) => Doc::text("(")
                .append(condition.doc(ctx))
                .append(Doc::text(")")),
            SupportsInParensKind::Selector(selector) => Doc::text("selector(")
                .append(selector.doc(ctx))
                .append(Doc::text(")")),
            SupportsInParensKind::Function(function) => function.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for SupportsNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.push(self.condition.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for SupportsOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("or"), Doc::line_or_space()],
        };
        docs.push(self.condition.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}
