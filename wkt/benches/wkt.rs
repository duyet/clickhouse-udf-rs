use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use parse_wkt::parse_wkt::parse_wkt;

fn parse_wkt_benchmark(c: &mut Criterion) {
    let inputs = ["LINESTRING(0 0,    1 1,    2 2)"];

    let mut group = c.benchmark_group("parse_wkt");
    for input in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), &input, |b, &i| {
            b.iter(|| parse_wkt(i));
        });
    }
    group.finish();
}

criterion_group!(benches, parse_wkt_benchmark,);
criterion_main!(benches);
