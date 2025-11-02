use crate::{LineBounds, config::LanguageOptions, doc_gen::format_comment};
use raffia::{
    Syntax,
    token::{Comment, CommentKind},
};
use std::{array, iter::Peekable, mem};
use tiny_pretty::Doc;

pub(crate) struct Ctx<'a, 's: 'a> {
    pub source: Option<&'s str>,
    pub syntax: Syntax,
    pub options: &'a LanguageOptions,
    pub comments: &'a [Comment<'s>],
    pub indent_width: usize,
    pub line_bounds: LineBounds,
}

impl<'a, 's> Ctx<'a, 's> {
    pub(crate) fn get_comments_between(
        &self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = &Comment<'s>> + Clone {
        debug_assert!(start <= end);

        self.comments
            .iter()
            .filter(move |comment| comment.span.start >= start && comment.span.end <= end)
    }

    pub(crate) fn start_spaced_comments(
        &'a self,
        comments: impl Iterator<Item = &'a Comment<'s>> + 'a,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        comments
            .scan(CommentKind::Block, |prev_kind, comment| {
                Some(
                    [
                        match mem::replace(prev_kind, comment.kind.clone()) {
                            CommentKind::Block => Doc::soft_line(),
                            CommentKind::Line => Doc::nil(),
                        },
                        format_comment(comment, self),
                        match comment.kind {
                            CommentKind::Block => Doc::nil(),
                            CommentKind::Line => Doc::hard_line(),
                        },
                    ]
                    .into_iter(),
                )
            })
            .flatten()
    }

    pub(crate) fn start_spaced_comments_without_last_hard_line(
        &'a self,
        comments: impl Iterator<Item = &'a Comment<'s>> + 'a,
        has_last_line_comment: &'a mut bool,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        StartSpacedCommentsWithoutLastHardLine {
            ctx: self,
            iter: comments.peekable(),
            prev_kind: CommentKind::Block,
            has_last_line_comment,
        }
        .flatten()
    }

    pub(crate) fn end_spaced_comments(
        &'a self,
        comments: impl Iterator<Item = &'a Comment<'s>> + 'a,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        comments.flat_map(|comment| {
            [
                format_comment(comment, self),
                match comment.kind {
                    CommentKind::Block => Doc::soft_line(),
                    CommentKind::Line => Doc::hard_line(),
                },
            ]
            .into_iter()
        })
    }

    pub(crate) fn end_spaced_comments_without_last_space(
        &'a self,
        comments: impl Iterator<Item = &'a Comment<'s>> + 'a,
        comment_end: &'a mut Option<usize>,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        EndSpacedCommentsWithoutLastSpace {
            ctx: self,
            iter: comments.peekable(),
            comment_end,
        }
        .flatten()
    }

    pub(crate) fn unspaced_comments(
        &'a self,
        comments: impl Iterator<Item = &'a Comment<'s>> + 'a,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        comments.filter_map(|comment| match comment.kind {
            CommentKind::Block => Some(format_comment(comment, self)),
            CommentKind::Line => None,
        })
    }
}

struct StartSpacedCommentsWithoutLastHardLine<'a, 's, I>
where
    's: 'a,
    I: Iterator<Item = &'a Comment<'s>>,
{
    ctx: &'a Ctx<'a, 's>,
    iter: Peekable<I>,
    prev_kind: CommentKind,
    has_last_line_comment: &'a mut bool,
}

impl<'a, 's, I> Iterator for StartSpacedCommentsWithoutLastHardLine<'a, 's, I>
where
    's: 'a,
    I: Iterator<Item = &'a Comment<'s>>,
{
    type Item = array::IntoIter<Doc<'s>, 3>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|comment| {
            let has_next = self.iter.peek().is_some();
            if !has_next {
                *self.has_last_line_comment = comment.kind == CommentKind::Line;
            }

            [
                match mem::replace(&mut self.prev_kind, comment.kind.clone()) {
                    CommentKind::Block => Doc::soft_line(),
                    CommentKind::Line => Doc::nil(),
                },
                format_comment(comment, self.ctx),
                match comment.kind {
                    CommentKind::Line if has_next => Doc::hard_line(),
                    _ => Doc::nil(),
                },
            ]
            .into_iter()
        })
    }
}

struct EndSpacedCommentsWithoutLastSpace<'a, 's, I>
where
    's: 'a,
    I: Iterator<Item = &'a Comment<'s>>,
{
    ctx: &'a Ctx<'a, 's>,
    iter: Peekable<I>,
    comment_end: &'a mut Option<usize>,
}

impl<'a, 's, I> Iterator for EndSpacedCommentsWithoutLastSpace<'a, 's, I>
where
    's: 'a,
    I: Iterator<Item = &'a Comment<'s>>,
{
    type Item = array::IntoIter<Doc<'s>, 2>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|comment| {
            let is_last = self.iter.peek().is_none();
            if is_last && comment.kind == CommentKind::Block {
                *self.comment_end = Some(comment.span.end);
            }

            [
                format_comment(comment, self.ctx),
                match comment.kind {
                    CommentKind::Block => {
                        if is_last {
                            Doc::nil()
                        } else {
                            Doc::soft_line()
                        }
                    }
                    CommentKind::Line => Doc::hard_line(),
                },
            ]
            .into_iter()
        })
    }
}
