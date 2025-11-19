use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use url::url::{detect_url, extract_url, has_url};

fn bench_detect_url(c: &mut Criterion) {
    let mut group = c.benchmark_group("detect_url");

    let test_cases = vec![
        ("short", "https://example.com"),
        (
            "medium",
            "Visit https://example.com/path/to/resource for more info",
        ),
        ("long", "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Check out https://example.com/very/long/path/to/some/resource?param1=value1&param2=value2 for details."),
        (
            "no_url",
            "This text contains no URLs at all, just plain text content",
        ),
    ];

    for (name, input) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, &s| {
            b.iter(|| detect_url(black_box(s)));
        });
    }

    group.finish();
}

fn bench_extract_url(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract_url");

    let test_cases = vec![
        ("http", "http://example.com"),
        ("https", "https://example.com"),
        ("with_path", "https://example.com/path/to/page"),
        ("with_query", "https://example.com?foo=bar&baz=qux"),
    ];

    for (name, input) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, &s| {
            b.iter(|| extract_url(black_box(s)));
        });
    }

    group.finish();
}

fn bench_has_url(c: &mut Criterion) {
    c.bench_function("has_url_true", |b| {
        b.iter(|| has_url(black_box("Check https://example.com here")));
    });

    c.bench_function("has_url_false", |b| {
        b.iter(|| has_url(black_box("No URL in this text")));
    });
}

criterion_group!(benches, bench_detect_url, bench_extract_url, bench_has_url);
criterion_main!(benches);
