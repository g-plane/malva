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
        self.get_line_at(end) - self.get_line_at(start)
    }

    pub(crate) fn get_line_col(&self, pos: usize) -> (usize, usize) {
        let line = self.get_line_at(pos);
        (line, pos - self.0[line.saturating_sub(1)])
    }

    fn get_line_at(&self, pos: usize) -> usize {
        let (ControlFlow::Break(line) | ControlFlow::Continue(line)) =
            self.0
                .iter()
                .try_fold(0, |i, offset| match pos.cmp(offset) {
                    Ordering::Less => ControlFlow::Break(i),
                    Ordering::Equal => ControlFlow::Continue(i),
                    Ordering::Greater => ControlFlow::Continue(i + 1),
                });
        line
    }
}
