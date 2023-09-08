use super::super::DocGen;
use crate::ctx::Ctx;
use raffia::ast::*;
use tiny_pretty::Doc;

impl DocGen for MediaAnd<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("and"), Doc::line_or_space()],
        };
        docs.push(self.media_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl DocGen for MediaCondition<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(
                self.conditions.iter().map(|condition| condition.doc(ctx)),
                Doc::space(),
            )
            .collect(),
        )
    }
}

impl DocGen for MediaConditionAfterMediaType<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("and"), Doc::line_or_space()],
        };
        docs.push(self.condition.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl DocGen for MediaConditionKind<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            MediaConditionKind::MediaInParens(media_in_parens) => media_in_parens.doc(ctx),
            MediaConditionKind::And(and) => and.doc(ctx),
            MediaConditionKind::Or(or) => or.doc(ctx),
            MediaConditionKind::Not(not) => not.doc(ctx),
        }
    }
}

impl DocGen for MediaFeature<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            MediaFeature::Plain(plain) => plain.doc(ctx),
            MediaFeature::Boolean(boolean) => boolean.doc(ctx),
            MediaFeature::Range(range) => range.doc(ctx),
            MediaFeature::RangeInterval(range_interval) => range_interval.doc(ctx),
        }
    }
}

impl DocGen for MediaFeatureBoolean<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        self.name.doc(ctx)
    }
}

impl DocGen for MediaFeatureComparison {
    fn doc(&self, _: &Ctx) -> Doc {
        Doc::text(match self.kind {
            MediaFeatureComparisonKind::Equal => "=",
            MediaFeatureComparisonKind::GreaterThan => ">",
            MediaFeatureComparisonKind::GreaterThanOrEqual => ">=",
            MediaFeatureComparisonKind::LessThan => "<",
            MediaFeatureComparisonKind::LessThanOrEqual => "<=",
        })
    }
}

impl DocGen for MediaFeatureName<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            MediaFeatureName::Ident(ident) => ident.doc(ctx),
            MediaFeatureName::SassVariable(sass_variable) => sass_variable.doc(ctx),
        }
    }
}

impl DocGen for MediaFeaturePlain<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(vec![
            self.name.doc(ctx),
            Doc::text(": "),
            self.value.doc(ctx),
        ])
    }
}

impl DocGen for MediaFeatureRange<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(vec![
            self.left.doc(ctx),
            Doc::space(),
            self.comparison.doc(ctx),
            Doc::space(),
            self.right.doc(ctx),
        ])
    }
}

impl DocGen for MediaFeatureRangeInterval<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(vec![
            self.left.doc(ctx),
            Doc::space(),
            self.left_comparison.doc(ctx),
            Doc::space(),
            self.name.doc(ctx),
            Doc::space(),
            self.right_comparison.doc(ctx),
            Doc::space(),
            self.right.doc(ctx),
        ])
    }
}

impl DocGen for MediaInParens<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let kind = match &self.kind {
            MediaInParensKind::MediaCondition(media_condition) => media_condition.doc(ctx),
            MediaInParensKind::MediaFeature(media_feature) => media_feature.doc(ctx),
        };
        Doc::list(vec![Doc::text("("), kind, Doc::text(")")])
    }
}

impl DocGen for MediaNot<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.push(self.media_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl DocGen for MediaOr<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("or"), Doc::line_or_space()],
        };
        docs.push(self.media_in_parens.doc(ctx));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl DocGen for MediaQuery<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        match self {
            MediaQuery::ConditionOnly(media_condition) => media_condition.doc(ctx),
            MediaQuery::WithType(media_query_with_type) => media_query_with_type.doc(ctx),
            MediaQuery::LessVariable(less_variable) => less_variable.doc(ctx),
            MediaQuery::LessNamespaceValue(less_namespace_value) => todo!(),
        }
    }
}

impl DocGen for MediaQueryWithType<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        let mut docs = Vec::with_capacity(5);
        if let Some(modifier) = &self.modifier {
            docs.push(modifier.doc(ctx));
            docs.push(Doc::space());
        }
        docs.push(self.media_type.doc(ctx));
        if let Some(condition) = &self.condition {
            docs.push(Doc::space());
            docs.push(condition.doc(ctx));
        }
        Doc::list(docs)
    }
}

impl DocGen for MediaQueryList<'_> {
    fn doc(&self, ctx: &Ctx) -> Doc {
        Doc::list(
            itertools::intersperse(
                self.queries.iter().map(|query| query.doc(ctx)),
                Doc::text(",").append(Doc::line_or_space()),
            )
            .collect(),
        )
        .group()
        .nest(ctx.indent_width)
    }
}
