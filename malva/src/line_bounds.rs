pub struct LineBounds(Vec<usize>);

impl LineBounds {
    pub fn new(input: &str) -> Self {
        Self(memchr::memchr_iter(b'\n', input.as_bytes()).collect())
    }

    pub(crate) fn line_distance(&self, start: usize, end: usize) -> usize {
        debug_assert!(end > start);

        let start = self.0.iter().position(|offset| start < *offset);
        let end = self.0.iter().position(|offset| end < *offset);
        start
            .zip(end)
            .map(|(start, end)| end - start)
            .unwrap_or_default()
    }
}
