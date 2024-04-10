use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn topk_benchmark(c: &mut Criterion) {
    let inputs = [1, 1, 1];

    let mut group = c.benchmark_group("topk");
    for input in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), &input, |b, &i| {
            b.iter(|| i + i);
        });
    }
    group.finish();
}

criterion_group!(benches, topk_benchmark);
criterion_main!(benches);
