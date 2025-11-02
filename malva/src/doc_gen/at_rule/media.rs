use super::{super::DocGen, helpers};
use crate::{ctx::Ctx, state::State};
use raffia::{Spanned, ast::*};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for MediaAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("and"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.media_in_parens.span.start),
        ));
        docs.push(self.media_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for MediaCondition<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::list(
            self.conditions
                .iter()
                .fold(
                    (Vec::with_capacity(self.conditions.len()), self.span.start),
                    |(mut docs, pos), condition| {
                        let span = condition.span();
                        docs.extend(
                            ctx.start_spaced_comments(ctx.get_comments_between(pos, span.start)),
                        );
                        docs.push(condition.doc(ctx, state));
                        (docs, span.end)
                    },
                )
                .0,
        )
    }
}

impl<'s> DocGen<'s> for MediaConditionAfterMediaType<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("and"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.and.span.end, self.condition.span.start),
        ));
        docs.push(self.condition.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for MediaConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            MediaConditionKind::MediaInParens(media_in_parens) => media_in_parens.doc(ctx, state),
            MediaConditionKind::And(and) => and.doc(ctx, state),
            MediaConditionKind::Or(or) => or.doc(ctx, state),
            MediaConditionKind::Not(not) => not.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for MediaFeature<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            MediaFeature::Plain(plain) => plain.doc(ctx, state),
            MediaFeature::Boolean(boolean) => boolean.doc(ctx, state),
            MediaFeature::Range(range) => range.doc(ctx, state),
            MediaFeature::RangeInterval(range_interval) => range_interval.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for MediaFeatureBoolean<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.name.doc(ctx, state)
    }
}

impl<'s> DocGen<'s> for MediaFeatureComparison {
    fn doc(&self, _: &Ctx<'_, 's>, _: &State) -> Doc<'s> {
        Doc::text(match self.kind {
            MediaFeatureComparisonKind::Equal => "=",
            MediaFeatureComparisonKind::GreaterThan => ">",
            MediaFeatureComparisonKind::GreaterThanOrEqual => ">=",
            MediaFeatureComparisonKind::LessThan => "<",
            MediaFeatureComparisonKind::LessThanOrEqual => "<=",
        })
    }
}

impl<'s> DocGen<'s> for MediaFeatureName<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            MediaFeatureName::Ident(InterpolableIdent::Literal(ident))
                if !ident.name.starts_with("--") =>
            {
                Doc::text(ident.raw.to_ascii_lowercase())
            }
            MediaFeatureName::Ident(ident) => ident.doc(ctx, state),
            MediaFeatureName::SassVariable(sass_variable) => sass_variable.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for MediaFeaturePlain<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.name
            .doc(ctx, state)
            .concat(ctx.start_spaced_comments(
                ctx.get_comments_between(self.name.span().end, self.colon_span.start),
            ))
            .append(Doc::text(": "))
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.colon_span.start, self.value.span().start),
            ))
            .append(self.value.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for MediaFeatureRange<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        self.left
            .doc(ctx, state)
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.left.span().end, self.comparison.span.start),
            ))
            .append(self.comparison.doc(ctx, state))
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.comparison.span.end, self.right.span().start),
            ))
            .append(self.right.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for MediaFeatureRangeInterval<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let name_span = self.name.span();
        self.left
            .doc(ctx, state)
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.left.span().end, self.left_comparison.span.start),
            ))
            .append(self.left_comparison.doc(ctx, state))
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.left_comparison.span.end, name_span.start),
            ))
            .append(self.name.doc(ctx, state))
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(name_span.end, self.right_comparison.span.start),
            ))
            .append(self.right_comparison.doc(ctx, state))
            .append(Doc::space())
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.right_comparison.span.end, self.right.span().start),
            ))
            .append(self.right.doc(ctx, state))
    }
}

impl<'s> DocGen<'s> for MediaInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let kind = match &self.kind {
            MediaInParensKind::MediaCondition(media_condition) => media_condition.doc(ctx, state),
            MediaInParensKind::MediaFeature(media_feature) => media_feature.doc(ctx, state),
        };
        let kind_span = self.kind.span();

        Doc::text("(")
            .concat(
                ctx.end_spaced_comments(ctx.get_comments_between(self.span.start, kind_span.start)),
            )
            .append(kind)
            .concat(
                ctx.start_spaced_comments(ctx.get_comments_between(kind_span.end, self.span.end)),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for MediaNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_nil(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.media_in_parens.span.start),
        ));
        docs.push(self.media_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for MediaOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("or"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.media_in_parens.span.start),
        ));
        docs.push(self.media_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for MediaQuery<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            MediaQuery::ConditionOnly(media_condition) => media_condition.doc(ctx, state),
            MediaQuery::WithType(media_query_with_type) => media_query_with_type.doc(ctx, state),
            MediaQuery::Function(function) => function.doc(ctx, state),
            MediaQuery::LessVariable(less_variable) => less_variable.doc(ctx, state),
            MediaQuery::LessNamespaceValue(less_namespace_value) => {
                less_namespace_value.doc(ctx, state)
            }
        }
    }
}

impl<'s> DocGen<'s> for MediaQueryWithType<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let media_type_span = self.media_type.span();

        let mut docs = Vec::with_capacity(5);
        if let Some(modifier) = &self.modifier {
            docs.push(modifier.doc(ctx, state));
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(
                ctx.get_comments_between(modifier.span.end, media_type_span.start),
            ));
        }
        docs.push(self.media_type.doc(ctx, state));
        if let Some(condition) = &self.condition {
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(
                ctx.get_comments_between(media_type_span.end, condition.span.start),
            ));
            docs.push(condition.doc(ctx, state));
        }
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for MediaQueryList<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        helpers::SeparatedListFormatter::new(",", Doc::line_or_space())
            .format(
                &self.queries,
                &self.comma_spans,
                self.span.start,
                ctx,
                state,
            )
            .group()
            .nest(ctx.indent_width)
    }
}
