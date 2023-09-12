pub struct LineBounds(Vec<usize>);

impl LineBounds {
    pub fn new(input: &str) -> Self {
        Self(memchr::memchr_iter(b'\n', input.as_bytes()).collect())
    }

    pub(crate) fn line_distance(&self, a: usize, b: usize) -> usize {
        let a = self.0.iter().position(|offset| a < *offset);
        let b = self.0.iter().position(|offset| b < *offset);
        a.zip(b)
            .map(|(a, b)| if a > b { a - b } else { b - a })
            .unwrap_or_default()
    }
}
