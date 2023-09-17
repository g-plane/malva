use crate::ctx::Ctx;
use itertools::{EitherOrBoth, Itertools};
use raffia::{ast::*, Span, Spanned};
use std::{iter, mem};
use tiny_pretty::Doc;

mod at_rule;
mod comment;
mod less;
mod sass;
mod selector;
mod stmt;
mod value;

pub(super) trait DocGen<'s> {
    fn doc(&self, ctx: &Ctx<'_, 's>) -> Doc<'s>;
}

fn format_selectors_before_block<'s, N>(
    selectors: &[N],
    comma_spans: &[Span],
    start: usize,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s>
where
    N: DocGen<'s> + Spanned,
{
    use crate::config::BlockSelectorLineBreak;

    Doc::list(
        itertools::intersperse(
            selectors
                .iter()
                .zip_longest(comma_spans.iter())
                .scan(start, |pos, item| match item {
                    EitherOrBoth::Both(selector, comma_span) => {
                        let selector_span = selector.span();
                        let mut docs = ctx
                            .end_padded_comments(
                                mem::replace(pos, comma_span.end),
                                selector_span.start,
                            )
                            .collect::<Vec<_>>();
                        docs.push(selector.doc(ctx));
                        docs.extend(ctx.start_padded_comments(selector_span.end, comma_span.start));
                        Some(docs.into_iter())
                    }
                    EitherOrBoth::Left(selector) => {
                        let mut docs = ctx
                            .end_padded_comments(*pos, selector.span().start)
                            .collect::<Vec<_>>();
                        docs.push(selector.doc(ctx));
                        Some(docs.into_iter())
                    }
                    EitherOrBoth::Right(..) => unreachable!(),
                }),
            vec![
                Doc::text(","),
                match ctx.options.block_selector_linebreak {
                    BlockSelectorLineBreak::Always => Doc::hard_line(),
                    BlockSelectorLineBreak::Consistent => Doc::line_or_space(),
                    BlockSelectorLineBreak::Wrap => Doc::soft_line(),
                },
            ]
            .into_iter(),
        )
        .flatten()
        .collect(),
    )
}

/// Only for SCSS/Sass/Less.
fn format_values_list<'s>(
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
                                .end_padded_comments(
                                    mem::replace(pos, comma_span.end),
                                    value_span.start,
                                )
                                .collect::<Vec<_>>();
                            docs.push(value.doc(ctx));
                            docs.extend(
                                ctx.start_padded_comments(value_span.end, comma_span.start),
                            );
                            Some(docs.into_iter())
                        }
                        EitherOrBoth::Left(value) => {
                            let mut docs = ctx
                                .end_padded_comments(*pos, value.span().start)
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
            Doc::text(",")
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
                    ctx.end_padded_comments(mem::replace(pos, value_span.end), value_span.start)
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
            docs.extend(ctx.start_padded_comments(last.span().end, list_span.end));
        }

        Doc::list(docs).group().nest(ctx.indent_width)
    }
}
