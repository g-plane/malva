use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use malva::{format_text, Syntax};
use std::fs;

fn bench_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("fmt");

    fs::read_dir("bench_samples")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .and_then(|ext| match ext {
                    "css" => Some((entry, Syntax::Css)),
                    "scss" => Some((entry, Syntax::Scss)),
                    "sass" => Some((entry, Syntax::Sass)),
                    "less" => Some((entry, Syntax::Less)),
                    _ => None,
                })
        })
        .filter(|(entry, ..)| {
            entry
                .file_type()
                .map(|file_type| file_type.is_file())
                .unwrap_or_default()
        })
        .for_each(|(entry, syntax)| {
            let path = &entry.path();
            let name = entry.file_name();
            let name = &name.to_string_lossy();
            let code = black_box(fs::read_to_string(path).unwrap());

            group.bench_with_input(BenchmarkId::from_parameter(name), &code, |b, code| {
                b.iter(|| {
                    black_box(format_text(code, syntax, &Default::default()).unwrap());
                });
            });
        });
    group.finish();
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
