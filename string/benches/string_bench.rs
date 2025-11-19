use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn string_format_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_format");

    // Simulate the string_format function logic
    fn string_format(s: &str) -> Option<String> {
        let args = s.split('\t').collect::<Vec<&str>>();
        let (s, args) = args.split_at(1);
        let mut result = s.get(0).map(|s| s.to_string()).unwrap_or_default();
        for arg in args.iter() {
            result = result.replacen("{}", arg, 1);
        }
        Some(result)
    }

    let test_cases = vec![
        ("no_args", "Hello, World!"),
        ("one_arg", "Hello, {}!\tWorld"),
        ("two_args", "Hello, {} {}!\tWorld\tRust"),
        ("many_args", "{} {} {} {} {}\ta\tb\tc\td\te"),
    ];

    for (name, input) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &input, |b, &s| {
            b.iter(|| string_format(black_box(s)));
        });
    }

    group.finish();
}

criterion_group!(benches, string_format_benchmark);
criterion_main!(benches);
