use crate::{config::LanguageOptions, doc_gen::DocGen, LineBounds};
use raffia::{
    token::{Comment, CommentKind},
    Syntax,
};
use std::mem;
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
}
