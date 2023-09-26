use crate::{config::LanguageOptions, doc_gen::DocGen, LineBounds};
use raffia::{
    token::{Comment, CommentKind},
    Syntax,
};
use std::{array, iter::Peekable, mem};
use tiny_pretty::Doc;

pub(crate) struct Ctx<'a, 's: 'a> {
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
    ) -> impl Iterator<Item = &Comment<'s>> {
        self.comments
            .iter()
            .filter(move |comment| comment.span.start >= start && comment.span.end <= end)
    }

    pub(crate) fn start_spaced_comments(
        &'a self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        debug_assert!(start <= end);

        self.get_comments_between(start, end)
            .scan(CommentKind::Block, |prev_kind, comment| {
                Some(
                    [
                        match mem::replace(prev_kind, comment.kind.clone()) {
                            CommentKind::Block => Doc::soft_line(),
                            CommentKind::Line => Doc::nil(),
                        },
                        comment.doc(self),
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
        start: usize,
        end: usize,
        has_last_line_comment: &'a mut bool,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        debug_assert!(start <= end);

        StartSpacedCommentsWithoutLastHardLine {
            ctx: self,
            iter: self.get_comments_between(start, end).peekable(),
            prev_kind: CommentKind::Block,
            has_last_line_comment,
        }
        .flatten()
    }

    pub(crate) fn end_spaced_comments(
        &'a self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        debug_assert!(start <= end);

        self.get_comments_between(start, end).flat_map(|comment| {
            [
                comment.doc(self),
                match comment.kind {
                    CommentKind::Block => Doc::soft_line(),
                    CommentKind::Line => Doc::hard_line(),
                },
            ]
            .into_iter()
        })
    }

    pub(crate) fn unspaced_comments(
        &'a self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = Doc<'s>> + 'a {
        debug_assert!(start <= end);

        self.get_comments_between(start, end)
            .filter_map(|comment| match comment.kind {
                CommentKind::Block => Some(comment.doc(self)),
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
            let peeked = self.iter.peek();
            if peeked.is_none() {
                *self.has_last_line_comment = comment.kind == CommentKind::Line;
            }

            [
                match mem::replace(&mut self.prev_kind, comment.kind.clone()) {
                    CommentKind::Block => Doc::soft_line(),
                    CommentKind::Line => Doc::nil(),
                },
                comment.doc(self.ctx),
                match comment.kind {
                    CommentKind::Line if self.iter.peek().is_some() => Doc::hard_line(),
                    _ => Doc::nil(),
                },
            ]
            .into_iter()
        })
    }
}
