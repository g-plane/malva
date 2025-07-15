use super::super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for ContainerCondition<'s> {
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

impl<'s> DocGen<'s> for ContainerConditionAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("and"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.query_in_parens.span.start),
        ));
        docs.push(self.query_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ContainerConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            ContainerConditionKind::QueryInParens(query_in_parens) => {
                query_in_parens.doc(ctx, state)
            }
            ContainerConditionKind::And(and) => and.doc(ctx, state),
            ContainerConditionKind::Not(not) => not.doc(ctx, state),
            ContainerConditionKind::Or(or) => or.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for ContainerConditionNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_nil(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.query_in_parens.span.start),
        ));
        docs.push(self.query_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ContainerConditionOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("or"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.query_in_parens.span.start),
        ));
        docs.push(self.query_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for ContainerPrelude<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let mut docs = Vec::with_capacity(3);
        if let Some(name) = &self.name {
            docs.push(name.doc(ctx, state));
            docs.push(Doc::space());
            docs.extend(ctx.end_spaced_comments(
                ctx.get_comments_between(name.span().start, self.condition.span.start),
            ));
        }
        docs.push(self.condition.doc(ctx, state));
        Doc::list(docs)
    }
}

impl<'s> DocGen<'s> for QueryInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match &self.kind {
            QueryInParensKind::ContainerCondition(condition) => Doc::text("(")
                .concat(ctx.end_spaced_comments(
                    ctx.get_comments_between(self.span.start, condition.span.start),
                ))
                .append(condition.doc(ctx, state))
                .concat(ctx.start_spaced_comments(
                    ctx.get_comments_between(condition.span.end, self.span.end),
                ))
                .append(Doc::text(")")),
            QueryInParensKind::SizeFeature(size_feature) => {
                let span = size_feature.span();
                Doc::text("(")
                    .concat(
                        ctx.end_spaced_comments(
                            ctx.get_comments_between(self.span.start, span.start),
                        ),
                    )
                    .append(size_feature.doc(ctx, state))
                    .concat(
                        ctx.start_spaced_comments(
                            ctx.get_comments_between(span.end, self.span.end),
                        ),
                    )
                    .append(Doc::text(")"))
            }
            QueryInParensKind::StyleQuery(style_query) => {
                let span = style_query.span();
                Doc::text("style(")
                    .concat(
                        ctx.end_spaced_comments(
                            ctx.get_comments_between(self.span.start, span.start),
                        ),
                    )
                    .append(style_query.doc(ctx, state))
                    .concat(
                        ctx.start_spaced_comments(
                            ctx.get_comments_between(span.end, self.span.end),
                        ),
                    )
                    .append(Doc::text(")"))
            }
            QueryInParensKind::ScrollState(scroll_state) => {
                let span = scroll_state.span();
                Doc::text("scroll-state(")
                    .concat(
                        ctx.end_spaced_comments(
                            ctx.get_comments_between(self.span.start, span.start),
                        ),
                    )
                    .append(scroll_state.doc(ctx, state))
                    .concat(
                        ctx.start_spaced_comments(
                            ctx.get_comments_between(span.end, self.span.end),
                        ),
                    )
                    .append(Doc::text(")"))
            }
        }
    }
}

impl<'s> DocGen<'s> for StyleCondition<'s> {
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

impl<'s> DocGen<'s> for StyleConditionAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("and"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.style_in_parens.span.start),
        ));
        docs.push(self.style_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for StyleConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            StyleConditionKind::StyleInParens(style_in_parens) => style_in_parens.doc(ctx, state),
            StyleConditionKind::And(and) => and.doc(ctx, state),
            StyleConditionKind::Not(not) => not.doc(ctx, state),
            StyleConditionKind::Or(or) => or.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for StyleConditionNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_nil(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.style_in_parens.span.start),
        ));
        docs.push(self.style_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for StyleConditionOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("or"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.style_in_parens.span.start),
        ));
        docs.push(self.style_in_parens.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for StyleInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        let kind_span = self.kind.span();
        Doc::text("(")
            .concat(
                ctx.end_spaced_comments(ctx.get_comments_between(self.span.start, kind_span.start)),
            )
            .append(self.kind.doc(ctx, state))
            .concat(
                ctx.start_spaced_comments(ctx.get_comments_between(kind_span.end, self.span.end)),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for StyleInParensKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            StyleInParensKind::Condition(condition) => condition.doc(ctx, state),
            StyleInParensKind::Feature(feature) => feature.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for StyleQuery<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            StyleQuery::Condition(condition) => condition.doc(ctx, state),
            StyleQuery::Feature(feature) => feature.doc(ctx, state),
        }
    }
}
