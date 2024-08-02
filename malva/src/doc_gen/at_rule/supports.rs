use super::super::DocGen;
use crate::{ctx::Ctx, state::State};
use raffia::{ast::*, Spanned};
use tiny_pretty::Doc;

impl<'s> DocGen<'s> for SupportsAnd<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("and"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("and"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.condition.span.start),
        ));
        docs.push(self.condition.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for SupportsCondition<'s> {
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

impl<'s> DocGen<'s> for SupportsConditionKind<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match self {
            SupportsConditionKind::SupportsInParens(supports_in_parens) => {
                supports_in_parens.doc(ctx, state)
            }
            SupportsConditionKind::And(and) => and.doc(ctx, state),
            SupportsConditionKind::Or(or) => or.doc(ctx, state),
            SupportsConditionKind::Not(not) => not.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SupportsDecl<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        Doc::text("(")
            .concat(ctx.end_spaced_comments(
                ctx.get_comments_between(self.span.start, self.decl.span.start),
            ))
            .append(self.decl.doc(ctx, state))
            .concat(
                ctx.start_spaced_comments(
                    ctx.get_comments_between(self.decl.span.end, self.span.end),
                ),
            )
            .append(Doc::text(")"))
    }
}

impl<'s> DocGen<'s> for SupportsInParens<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        match &self.kind {
            SupportsInParensKind::Feature(feature) => feature.doc(ctx, state),
            SupportsInParensKind::SupportsCondition(condition) => Doc::text("(")
                .concat(ctx.end_spaced_comments(
                    ctx.get_comments_between(self.span.start, condition.span.start),
                ))
                .append(condition.doc(ctx, state))
                .concat(ctx.start_spaced_comments(
                    ctx.get_comments_between(condition.span.end, self.span.end),
                ))
                .append(Doc::text(")")),
            SupportsInParensKind::Selector(selector) => Doc::text("selector(")
                .append(
                    Doc::line_or_nil()
                        .concat(ctx.end_spaced_comments(
                            ctx.get_comments_between(self.span.start, selector.span.start),
                        ))
                        .append(selector.doc(ctx, state))
                        .concat(ctx.start_spaced_comments(
                            ctx.get_comments_between(selector.span.end, self.span.end),
                        ))
                        .nest(ctx.indent_width)
                        .append(Doc::line_or_nil())
                        .group(),
                )
                .append(Doc::text(")")),
            SupportsInParensKind::Function(function) => function.doc(ctx, state),
        }
    }
}

impl<'s> DocGen<'s> for SupportsNot<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_nil(), Doc::text("not"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::text("not"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.condition.span.start),
        ));
        docs.push(self.condition.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

impl<'s> DocGen<'s> for SupportsOr<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>, state: &State) -> Doc<'s> {
        use crate::config::OperatorLineBreak;

        let mut docs = match ctx.options.operator_linebreak {
            OperatorLineBreak::Before => vec![Doc::line_or_space(), Doc::text("or"), Doc::space()],
            OperatorLineBreak::After => vec![Doc::space(), Doc::text("or"), Doc::line_or_space()],
        };
        docs.extend(ctx.end_spaced_comments(
            ctx.get_comments_between(self.keyword.span.end, self.condition.span.start),
        ));
        docs.push(self.condition.doc(ctx, state));

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}
