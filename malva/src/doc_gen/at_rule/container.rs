use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for ContainerCondition<'s> {
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

impl<'s> DocGen<'s> for ContainerConditionAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("and"), Doc::line_or_space()],
        };
        docs.push(self.query_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ContainerConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            ContainerConditionKind::QueryInParens(query_in_parens) => query_in_parens.doc(ctx),
            ContainerConditionKind::And(and) => and.doc(ctx),
            ContainerConditionKind::Not(not) => not.doc(ctx),
            ContainerConditionKind::Or(or) => or.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for ContainerConditionNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.push(self.query_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ContainerConditionOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("or"), Doc::line_or_space()],
        };
        docs.push(self.query_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ContainerPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        if let Some(name) = &self.name {
            docs.push(name.doc(ctx));
            docs.push(Doc::space());
        }
        docs.push(self.condition.doc(ctx));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for QueryInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match &self.kind {
            QueryInParensKind::ContainerCondition(condition) => Doc::text("(")
                .append(condition.doc(ctx))
                .append(Doc::text(")")),
            QueryInParensKind::SizeFeature(size_feature) => Doc::text("(")
                .append(size_feature.doc(ctx))
                .append(Doc::text(")")),
            QueryInParensKind::StyleQuery(style_query) => Doc::text("style(")
                .append(style_query.doc(ctx))
                .append(Doc::text(")")),
        }
    }
}

impl<'s> DocGen<'s> for StyleCondition<'s> {
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

impl<'s> DocGen<'s> for StyleConditionAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("and"), Doc::line_or_space()],
        };
        docs.push(self.style_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for StyleConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            StyleConditionKind::StyleInParens(style_in_parens) => style_in_parens.doc(ctx),
            StyleConditionKind::And(and) => and.doc(ctx),
            StyleConditionKind::Not(not) => not.doc(ctx),
            StyleConditionKind::Or(or) => or.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for StyleConditionNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.push(self.style_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for StyleConditionOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("or"), Doc::line_or_space()],
        };
        docs.push(self.style_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for StyleInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        Doc::text("(")
            .append(self.kind.doc(ctx))
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for StyleInParensKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            StyleInParensKind::Condition(condition) => condition.doc(ctx),
            StyleInParensKind::Feature(feature) => feature.doc(ctx),
        }
    }
}

impl<'s> DocGen<'s> for StyleQuery<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s> {
        match self {
            StyleQuery::Condition(condition) => condition.doc(ctx),
            StyleQuery::Feature(feature) => feature.doc(ctx),
        }
    }
}
