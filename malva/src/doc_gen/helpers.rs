use super::DocGen;
use crate::{ctx::Ctx, state::State};
use itertools::{EitherOrBoth, Itertools};
use raffia::{ast::*, Span, Spanned, Syntax};
use std::{iter, mem};
use tiny_pretty::Doc;

pub(super) fn format_selectors_before_block<'s, N>(
    selectors: &[N],
    comma_spans: &[Span],
    start: usize,
    ctx: &Ctx<'_, 's>,
    state: &State,
) -> Doc<'s>
where
    N: DocGen<'s> + Spanned,
{
    use crate::{config::BlockSelectorLineBreak, state::SelectorOverride};

    let linebreak = match state.selector_override {
        SelectorOverride::Unset => ctx.options.block_selector_linebreak.clone(),
        SelectorOverride::Ignore => {
            if let Some(source) = ctx.source {
                let first = selectors[0].span();
                let raw = &source[first.start
                    ..selectors
                        .last()
                        .map(|last| last.span())
                        .unwrap_or(first)
                        .end];
                return Doc::list(
                    itertools::intersperse(raw.lines().map(Doc::text), Doc::empty_line()).collect(),
                );
            } else {
                ctx.options.block_selector_linebreak.clone()
            }
        }
        SelectorOverride::Always => BlockSelectorLineBreak::Always,
        SelectorOverride::Consistent => BlockSelectorLineBreak::Consistent,
        SelectorOverride::Wrap => BlockSelectorLineBreak::Wrap,
    };

    SeparatedListFormatter::new(
        ",",
        match linebreak {
            BlockSelectorLineBreak::Always => Doc::hard_line(),
            BlockSelectorLineBreak::Consistent => {
                if ctx
                    .options
                    .selectors_prefer_single_line
                    .unwrap_or(ctx.options.prefer_single_line)
                    || selectors
                        .first()
                        .zip(selectors.get(1))
                        .is_some_and(|(first, second)| {
                            ctx.line_bounds
                                .line_distance(first.span().end, second.span().start)
                                == 0
                        })
                {
                    Doc::line_or_space()
                } else {
                    Doc::hard_line()
                }
            }
            BlockSelectorLineBreak::Wrap => Doc::soft_line(),
        },
    )
    .format(selectors, comma_spans, start, ctx, state)
    .group()
}

pub(super) struct SeparatedListFormatter {
    separator: Doc<'static>,
    space_after_separator: Doc<'static>,
    trailing: bool,
}

impl SeparatedListFormatter {
    pub(super) fn new(separator: &'static str, space_after_separator: Doc<'static>) -> Self {
        Self {
            separator: Doc::text(separator),
            space_after_separator,
            trailing: false,
        }
    }

    /// Remember to call `.group()` if enabling trailing separator,
    /// otherwise it can't decide whether to add or not.
    pub(super) fn with_trailing(mut self) -> Self {
        self.trailing = true;
        self
    }

    pub(super) fn format<'s, N>(
        self,
        list: &[N],
        separator_spans: &[Span],
        start: usize,
        ctx: &Ctx<'_, 's>,
        state: &State,
    ) -> Doc<'s>
    where
        N: DocGen<'s> + Spanned,
    {
        let mut pos = start;
        let mut docs = Vec::<Doc<'s>>::with_capacity(list.len() * 2);
        let mut iter = list.iter().zip_longest(separator_spans.iter()).peekable();
        while let Some(either_or_both) = iter.next() {
            match either_or_both {
                EitherOrBoth::Both(list_item, separator_span) => {
                    let mut comment_end = None;
                    let list_item_span = list_item.span();
                    docs.extend(ctx.end_spaced_comments_without_last_space(
                        ctx.get_comments_between(pos, list_item_span.start),
                        &mut comment_end,
                    ));
                    if let Some(end) = comment_end {
                        if ctx.line_bounds.line_distance(end, list_item_span.start) > 0
                            && ctx.line_bounds.line_distance(pos, end) > 0
                        {
                            docs.push(Doc::hard_line());
                        } else {
                            docs.push(Doc::soft_line());
                        }
                    }
                    docs.push(list_item.doc(ctx, state));
                    docs.extend(ctx.start_spaced_comments(
                        ctx.get_comments_between(list_item_span.end, separator_span.start),
                    ));
                    pos = separator_span.end;
                    if let Some(peeked) = iter.peek() {
                        docs.push(self.separator.clone());
                        let mut has_last_line_comment = false;
                        if let EitherOrBoth::Both(list_item, _) | EitherOrBoth::Left(list_item) =
                            peeked
                        {
                            docs.extend(
                                ctx.start_spaced_comments_without_last_hard_line(
                                    ctx.get_comments_between(
                                        separator_span.end,
                                        list_item.span().start,
                                    )
                                    .take_while(|comment| {
                                        ctx.line_bounds
                                            .line_distance(separator_span.end, comment.span.start)
                                            == 0
                                    })
                                    .inspect(|comment| pos = comment.span.end),
                                    &mut has_last_line_comment,
                                ),
                            );
                        }
                        if has_last_line_comment {
                            docs.push(Doc::hard_line());
                        } else {
                            docs.push(self.space_after_separator.clone());
                        }
                    }
                }
                EitherOrBoth::Left(list_item) => {
                    let mut comment_end = None;
                    let list_item_span = list_item.span();
                    docs.extend(ctx.end_spaced_comments_without_last_space(
                        ctx.get_comments_between(pos, list_item_span.start),
                        &mut comment_end,
                    ));
                    if let Some(end) = comment_end {
                        if ctx.line_bounds.line_distance(end, list_item_span.start) > 0
                            && ctx.line_bounds.line_distance(pos, end) > 0
                        {
                            docs.push(Doc::hard_line());
                        } else {
                            docs.push(Doc::soft_line());
                        }
                    }
                    docs.push(list_item.doc(ctx, state));
                }
                EitherOrBoth::Right(..) => unreachable!(),
            }
        }

        if self.trailing && ctx.options.trailing_comma {
            docs.push(Doc::flat_or_break(Doc::nil(), self.separator));
        }
        Doc::list(docs)
    }
}

/// Only for SCSS/Sass/Less.
pub(super) fn format_values_list<'s>(
    values: &[ComponentValue<'s>],
    comma_spans: Option<&[Span]>,
    list_span: &Span,
    ctx: &Ctx<'_, 's>,
    state: &State,
) -> Doc<'s> {
    if let Some(comma_spans) = comma_spans {
        let doc = SeparatedListFormatter::new(",", Doc::line_or_space())
            .with_trailing()
            .format(values, comma_spans, list_span.start, ctx, state)
            .group();
        if values.len() == 1 {
            if ctx.options.trailing_comma {
                // trailing comma was added by `SeparatedListFormatter` when there're multiple lines
                doc.append(Doc::flat_or_break(Doc::text(","), Doc::nil()))
                    .group()
            } else {
                doc.append(Doc::text(","))
            }
        } else {
            doc
        }
    } else {
        let mut docs =
            itertools::intersperse(
                values.iter().scan(list_span.start, |pos, value| {
                    let value_span = value.span();
                    Some(
                        ctx.end_spaced_comments(ctx.get_comments_between(
                            mem::replace(pos, value_span.end),
                            value_span.start,
                        ))
                        .chain(iter::once(value.doc(ctx, state)))
                        .collect::<Vec<_>>()
                        .into_iter(),
                    )
                }),
                vec![Doc::line_or_space()].into_iter(),
            )
            .flatten()
            .collect::<Vec<_>>();

        if let Some(last) = values.last() {
            docs.extend(
                ctx.start_spaced_comments(ctx.get_comments_between(last.span().end, list_span.end)),
            );
        }

        Doc::list(docs).group()
    }
}

/// Remember to call `.group()` if use this,
/// otherwise it will always add linebreak.
pub(super) fn format_operator_prefix_space<'s>(ctx: &Ctx<'_, 's>) -> Doc<'s> {
    use crate::config::OperatorLineBreak;

    match ctx.options.operator_linebreak {
        OperatorLineBreak::Before => Doc::line_or_space().nest(ctx.indent_width),
        OperatorLineBreak::After => Doc::space(),
    }
}

/// Remember to call `.group()` if use this,
/// otherwise it will always add linebreak.
pub(super) fn format_operator_suffix_space<'s>(ctx: &Ctx<'_, 's>) -> Doc<'s> {
    use crate::config::OperatorLineBreak;

    match ctx.options.operator_linebreak {
        OperatorLineBreak::Before => Doc::space(),
        OperatorLineBreak::After => Doc::line_or_space().nest(ctx.indent_width),
    }
}

pub(super) fn format_parenthesized<'s>(
    body: Doc<'s>,
    trailing_comments_start: usize,
    trailing_comments_end: usize,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s> {
    let mut has_last_line_comment = false;

    Doc::text("(")
        .append(Doc::line_or_nil())
        .append(body)
        .concat(ctx.start_spaced_comments_without_last_hard_line(
            ctx.get_comments_between(trailing_comments_start, trailing_comments_end),
            &mut has_last_line_comment,
        ))
        .nest(ctx.indent_width)
        .append(if has_last_line_comment {
            Doc::hard_line()
        } else {
            Doc::line_or_nil()
        })
        .group()
        .append(Doc::text(")"))
}

pub(super) fn format_space_before_block<'s>(
    previous_end: usize,
    block_start: usize,
    ctx: &Ctx<'_, 's>,
) -> Doc<'s> {
    if ctx.syntax == Syntax::Sass {
        let mut has_last_line_comment = false;
        Doc::list(
            ctx.start_spaced_comments_without_last_hard_line(
                ctx.get_comments_between(previous_end, block_start),
                &mut has_last_line_comment,
            )
            .collect(),
        )
    } else {
        Doc::space()
            .concat(ctx.end_spaced_comments(ctx.get_comments_between(previous_end, block_start)))
    }
}

pub(super) fn ident_to_lowercase<'s>(
    interpolable_ident: &InterpolableIdent<'s>,
    ctx: &Ctx<'_, 's>,
    state: &State,
) -> Doc<'s> {
    match &interpolable_ident {
        InterpolableIdent::Literal(ident)
            if !ident.name.starts_with("--") && !state.in_unknown_at_rule =>
        {
            Doc::text(ident.raw.to_ascii_lowercase())
        }
        name => name.doc(ctx, state),
    }
}

pub(super) fn get_smart_linebreak<N>(
    start: usize,
    elements: &[N],
    prefer_single_line: Option<bool>,
    ctx: &Ctx<'_, '_>,
) -> Doc<'static>
where
    N: Spanned,
{
    let prefer_single_line = prefer_single_line.unwrap_or(ctx.options.prefer_single_line);
    match elements.first() {
        Some(element)
            if !prefer_single_line
                && ctx.line_bounds.line_distance(start, element.span().start) > 0 =>
        {
            Doc::hard_line()
        }
        _ => Doc::line_or_space(),
    }
}
