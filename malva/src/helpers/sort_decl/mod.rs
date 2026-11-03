use crate::config::DeclarationOrder;

mod concentric;
mod shorthand;
mod smacss;

pub fn sort<T>(decls: &mut [(&str, T)], order: &DeclarationOrder) {
    match order {
        DeclarationOrder::Alphabetical => {
            sort_by_key(decls, |name| Some(name.to_ascii_lowercase()))
        }
        DeclarationOrder::Smacss => sort_by_key(decls, |name| position(name, &smacss::PROPERTIES)),
        DeclarationOrder::Concentric => {
            sort_by_key(decls, |name| position(name, &concentric::PROPERTIES))
        }
    }
}

// Not `sort_by`: barriers and override pairs don't form a total order, and it panics on one.
fn sort_by_key<'s, T, K: Ord>(decls: &mut [(&'s str, T)], key: impl Fn(&'s str) -> Option<K>) {
    let mut keys: Vec<_> = decls
        .iter()
        .map(|(name, _)| {
            if name.starts_with("--") {
                None
            } else {
                key(strip_vendor_prefix(name))
            }
        })
        .collect();
    for i in 1..decls.len() {
        for j in (1..=i).rev() {
            let (Some(prev), Some(next)) = (&keys[j - 1], &keys[j]) else {
                break;
            };
            if prev <= next || shorthand::overrides_either(decls[j - 1].0, decls[j].0) {
                break;
            }
            decls.swap(j - 1, j);
            keys.swap(j - 1, j);
        }
    }
}

fn position(name: &str, list: &[&str]) -> Option<usize> {
    list.iter()
        .position(|property| property.eq_ignore_ascii_case(name))
}

fn strip_vendor_prefix(s: &str) -> &str {
    s.strip_prefix('-')
        .and_then(|s| {
            let trimmed = s.trim_start_matches(|c: char| c.is_ascii_alphanumeric());
            if s == trimmed { None } else { Some(trimmed) }
        })
        .and_then(|s| s.strip_prefix('-'))
        .unwrap_or(s)
}
