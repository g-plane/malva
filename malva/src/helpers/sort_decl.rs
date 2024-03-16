use std::cmp::Ordering;

pub fn compare_in_alphabetical(a: &str, b: &str) -> Ordering {
    strip_vendor_prefix(a).cmp(strip_vendor_prefix(b))
}

pub fn strip_vendor_prefix(s: &str) -> &str {
    s.strip_prefix('-')
        .and_then(|s| {
            let trimmed = s.trim_start_matches(|c: char| c.is_ascii_alphanumeric());
            if s == trimmed {
                None
            } else {
                Some(trimmed)
            }
        })
        .and_then(|s| s.strip_prefix('-'))
        .unwrap_or(s)
}
