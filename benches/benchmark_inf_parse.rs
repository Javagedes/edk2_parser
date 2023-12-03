#[macro_use]
extern crate criterion;
use std::path::PathBuf;

use criterion::Criterion;
use edk2_parser::InfParser;

// The function you want to benchmark
fn parse_inf(file_path: PathBuf) {
    let mut parser = InfParser::new(None);
    parser.parse_file(file_path).unwrap();
}

// Criterion benchmark group
fn my_benchmark(c: &mut Criterion) {
    let d1 = PathBuf::from("tests/data/baseLib.inf");
    let d2 = PathBuf::from("tests/data/opensslLib.inf");

    let mut group = c.benchmark_group("parse_inf");
    group.sample_size(500);
    group.bench_function("baseLib", |b| b.iter(|| parse_inf(d1.clone())));
    group.bench_function("opensslLib", |b| b.iter(|| parse_inf(d2.clone())));
}

criterion_group!(benches, my_benchmark /*, another_benchmark*/);
criterion_main!(benches);
