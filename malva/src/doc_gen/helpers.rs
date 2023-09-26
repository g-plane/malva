use super::DocGen;
use crate::ctx::Ctx;
use itertools::{EitherOrBoth, Itertools};
use raffia::{ast::*, Span, Spanned};
use std::{iter, mem};
use tiny_pretty::Doc;

pub(super) fn format_selectors_before_block<'s, N>(
    selectors: &[N],
    comma_spans: &[Span],
    start: usize,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s>
where
    N: DocGen<'s> + Spanned,
{
    use crate::config::BlockSelectorLineBreak;

    format_comma_separated_list(
        selectors,
        comma_spans,
        start,
        match ctx.options.block_selector_linebreak {
            BlockSelectorLineBreak::Always => Doc::hard_line(),
            BlockSelectorLineBreak::Consistent => Doc::line_or_space(),
            BlockSelectorLineBreak::Wrap => Doc::soft_line(),
        },
        ctx,
    )
    .group()
}

pub(super) fn format_comma_separated_list<'s, N>(
    list: &[N],
    comma_spans: &[Span],
    start: usize,
    space_after_comma: Doc<'s>,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s>
where
    N: DocGen<'s> + Spanned,
{
    Doc::list(
        itertools::intersperse(
            list.iter()
                .zip_longest(comma_spans.iter())
                .scan(start, |pos, either_or_both| match either_or_both {
                    EitherOrBoth::Both(list_item, comma_span) => {
                        let list_item_span = list_item.span();
                        let mut docs = ctx
                            .end_spaced_comments(
                                mem::replace(pos, comma_span.end),
                                list_item_span.start,
                            )
                            .collect::<Vec<_>>();
                        docs.push(list_item.doc(ctx));
                        docs.extend(
                            ctx.start_spaced_comments(list_item_span.end, comma_span.start),
                        );
                        Some(docs.into_iter())
                    }
                    EitherOrBoth::Left(list_item) => {
                        let mut docs = ctx
                            .end_spaced_comments(*pos, list_item.span().start)
                            .collect::<Vec<_>>();
                        docs.push(list_item.doc(ctx));
                        Some(docs.into_iter())
                    }
                    EitherOrBoth::Right(..) => unreachable!(),
                }),
            vec![Doc::text(","), space_after_comma].into_iter(),
        )
        .flatten()
        .collect(),
    )
}

/// Remember to call `.group()` if use this,
/// otherwise it can't decide whether to add trailing comma or not.
pub(super) fn format_comma_separated_list_with_trailing<'s, N>(
    list: &[N],
    comma_spans: &[Span],
    start: usize,
    space_after_comma: Doc<'s>,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s>
where
    N: DocGen<'s> + Spanned,
{
    format_comma_separated_list(list, comma_spans, start, space_after_comma, ctx).append(
        if ctx.options.trailing_comma {
            Doc::flat_or_break(Doc::nil(), Doc::text(","))
        } else {
            Doc::nil()
        },
    )
}

/// Only for SCSS/Sass/Less.
pub(super) fn format_values_list<'s>(
    values: &[ComponentValue<'s>],
    comma_spans: Option<&[Span]>,
    list_span: &Span,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s> {
    if let Some(comma_spans) = comma_spans {
        Doc::list(
            itertools::intersperse(
                values
                    .iter()
                    .zip_longest(comma_spans.iter())
                    .scan(list_span.start, |pos, item| match item {
                        EitherOrBoth::Both(value, comma_span) => {
                            let value_span = value.span();
                            let mut docs = ctx
                                .end_spaced_comments(
                                    mem::replace(pos, comma_span.end),
                                    value_span.start,
                                )
                                .collect::<Vec<_>>();
                            docs.push(value.doc(ctx));
                            docs.extend(
                                ctx.start_spaced_comments(value_span.end, comma_span.start),
                            );
                            Some(docs.into_iter())
                        }
                        EitherOrBoth::Left(value) => {
                            let mut docs = ctx
                                .end_spaced_comments(*pos, value.span().start)
                                .collect::<Vec<_>>();
                            docs.push(value.doc(ctx));
                            Some(docs.into_iter())
                        }
                        EitherOrBoth::Right(..) => unreachable!(),
                    }),
                vec![Doc::text(","), Doc::line_or_space()].into_iter(),
            )
            .flatten()
            .collect(),
        )
        .append(if ctx.options.trailing_comma {
            Doc::flat_or_break(Doc::nil(), Doc::text(","))
        } else {
            Doc::nil()
        })
        .group()
        .nest(ctx.indent_width)
    } else {
        let mut docs = itertools::intersperse(
            values.iter().scan(list_span.start, |pos, value| {
                let value_span = value.span();
                Some(
                    ctx.end_spaced_comments(mem::replace(pos, value_span.end), value_span.start)
                        .chain(iter::once(value.doc(ctx)))
                        .collect::<Vec<_>>()
                        .into_iter(),
                )
            }),
            vec![Doc::line_or_space()].into_iter(),
        )
        .flatten()
        .collect::<Vec<_>>();

        if let Some(last) = values.last() {
            docs.extend(ctx.start_spaced_comments(last.span().end, list_span.end));
        }

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}

pub(super) fn format_operator_prefix_space<'s>(ctx: &Ctx<'_, 's>) -> Doc<'s> {
    use crate::config::OperatorLineBreak;

    match ctx.options.operator_linebreak {
        OperatorLineBreak::Before => Doc::soft_line().nest(ctx.indent_width),
        OperatorLineBreak::After => Doc::space(),
    }
}

pub(super) fn format_operator_suffix_space<'s>(ctx: &Ctx<'_, 's>) -> Doc<'s> {
    use crate::config::OperatorLineBreak;

    match ctx.options.operator_linebreak {
        OperatorLineBreak::Before => Doc::space(),
        OperatorLineBreak::After => Doc::soft_line().nest(ctx.indent_width),
    }
}

pub(super) fn format_parenthesized<'s>(
    body: Doc<'s>,
    trailing_comments_start: usize,
    trailing_comments_end: usize,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s> {
    Doc::text("(")
        .append(
            Doc::line_or_nil()
                .append(body)
                .concat(ctx.start_spaced_comments_without_last_hard_line(
                    trailing_comments_start,
                    trailing_comments_end,
                ))
                .nest(ctx.indent_width)
                .append(Doc::line_or_nil())
                .group(),
        )
        .append(Doc::text(")"))
}

pub(super) fn ident_to_lowercase<'s>(
    interpolable_ident: &InterpolableIdent<'s>,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s> {
    match &interpolable_ident {
        InterpolableIdent::Literal(ident) if !ident.name.starts_with("--") => {
            Doc::text(ident.raw.to_ascii_lowercase())
        }
        name => name.doc(ctx),
    }
}
