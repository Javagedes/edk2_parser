#[macro_use]
extern crate criterion;
use criterion::Criterion;
use edk2_parser::{inf::Inf, ConfigParser};

// The function you want to benchmark
fn parse_inf(data: String) {
    let mut parser = ConfigParser::<Inf>::new();
    parser.parse(data).unwrap();
}

// Criterion benchmark group
fn my_benchmark(c: &mut Criterion) {
    let d1 = include_str!("../tests/data/baseLib.inf").to_string();
    let d2 = include_str!("../tests/data/opensslLib.inf").to_string();

    let mut group = c.benchmark_group("parse_inf");
    group.sample_size(500);
    group.bench_function("baseLib", |b| b.iter(|| parse_inf(d1.clone())));
    group.bench_function("opensslLib", |b| b.iter(|| parse_inf(d2.clone())));
}

criterion_group!(benches, my_benchmark /*, another_benchmark*/);
criterion_main!(benches);
