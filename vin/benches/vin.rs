use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use vin::vin::{vin_cleaner, vin_manuf, vin_year};

fn vin_cleaner_benchmark(c: &mut Criterion) {
    let inputs = vec!["G1ND52F14M700000 (bla bla)", " this is long long long long long long input G1ND52F14M700000 and long long long extra things", "G1ND52F14M700000", "invalid", ""];

    let mut group = c.benchmark_group("vin_cleaner");
    for input in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), &input, |b, &i| {
            b.iter(|| vin_cleaner(i));
        });
    }
    group.finish();
}

fn vin_year_benchmark(c: &mut Criterion) {
    let inputs = vec!["G1ND52F14M700000 (bla bla)", " this is long long long long long long input G1ND52F14M700000 and long long long extra things", "G1ND52F14M700000", "invalid", ""];

    let mut group = c.benchmark_group("vin_year");
    for input in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), &input, |b, &i| {
            b.iter(|| vin_year(i));
        });
    }
    group.finish();
}

fn vin_manuf_benchmark(c: &mut Criterion) {
    let inputs = vec!["G1ND52F14M700000 (bla bla)", " this is long long long long long long input G1ND52F14M700000 and long long long extra things", "G1ND52F14M700000", "invalid", ""];

    let mut group = c.benchmark_group("vin_manuf");
    for input in inputs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), &input, |b, &i| {
            b.iter(|| vin_manuf(i));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    vin_cleaner_benchmark,
    vin_year_benchmark,
    vin_manuf_benchmark
);
criterion_main!(benches);
