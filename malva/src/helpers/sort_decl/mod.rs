pub use smacss::compare_in_smacss;
use std::{cmp::Ordering, ops::ControlFlow};

mod smacss;

pub fn compare_in_alphabetical(a: &str, b: &str) -> Ordering {
    strip_vendor_prefix(a).cmp(strip_vendor_prefix(b))
}

fn strip_vendor_prefix(s: &str) -> &str {
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

fn compare_by_list_index<const N: usize>(a: &str, b: &str, list: [&'static str; N]) -> Ordering {
    let a = strip_vendor_prefix(a);
    let b = strip_vendor_prefix(b);

    let result = list
        .iter()
        .enumerate()
        .try_fold((None, None), |mut result, (index, property)| {
            if result.0.is_none() && property.eq_ignore_ascii_case(a) {
                result.0 = Some(index)
            }
            if result.1.is_none() && property.eq_ignore_ascii_case(b) {
                result.1 = Some(index)
            }
            if result.0.is_some() && result.1.is_some() {
                ControlFlow::Break(result)
            } else {
                ControlFlow::Continue(result)
            }
        });
    if let ControlFlow::Break((Some(a), Some(b))) | ControlFlow::Continue((Some(a), Some(b))) =
        result
    {
        a.cmp(&b)
    } else {
        Ordering::Equal
    }
}
