use std::{cmp::Ordering, iter, ops::ControlFlow};

/// `LineBounds` is used to compute line distance.
/// This is for internal use only.
pub struct LineBounds(Vec<usize>);

impl LineBounds {
    /// Generate line bounds from input.
    pub fn new(input: &str) -> Self {
        Self(
            iter::once(0)
                .chain(memchr::memchr_iter(b'\n', input.as_bytes()))
                .collect(),
        )
    }

    pub(crate) fn line_distance(&self, start: usize, end: usize) -> usize {
        debug_assert!(
            end >= start,
            "end {end} must be greater than or equal start {start}"
        );

        let start = self
            .0
            .iter()
            .try_fold(0, |i, offset| match start.cmp(offset) {
                Ordering::Less => ControlFlow::Break(i),
                Ordering::Equal => ControlFlow::Continue(i),
                Ordering::Greater => ControlFlow::Continue(i + 1),
            });
        let end = self
            .0
            .iter()
            .try_fold(0, |i, offset| match end.cmp(offset) {
                Ordering::Less => ControlFlow::Break(i),
                Ordering::Equal => ControlFlow::Continue(i),
                Ordering::Greater => ControlFlow::Continue(i + 1),
            });

        match (start, end) {
            (ControlFlow::Break(start), ControlFlow::Break(end)) => end - start,
            (ControlFlow::Break(start), ControlFlow::Continue(end)) => end - start,
            (ControlFlow::Continue(start), ControlFlow::Break(end)) => end - start,
            (ControlFlow::Continue(start), ControlFlow::Continue(end)) => end - start,
        }
    }
}
