pub struct LineBounds(Vec<usize>);

impl LineBounds {
    pub fn new(input: &str) -> Self {
        Self(memchr::memchr_iter(b'\n', input.as_bytes()).collect())
    }

    pub(crate) fn is_away_more_than_one_line(&self, a: usize, b: usize) -> bool {
        let a = self.0.iter().position(|offset| a < *offset);
        let b = self.0.iter().position(|offset| b < *offset);
        a.zip(b)
            .map(|(a, b)| if a > b { a - b > 1 } else { b - a > 1 })
            .unwrap_or_default()
    }
}
