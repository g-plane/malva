use crate::ctx::Ctx;
use itertools::{EitherOrBoth, Itertools};
use raffia::{Span, Spanned};
use std::mem;
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
