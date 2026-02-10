//! Benchmarks for configuration loading.

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_env_parse(c: &mut Criterion) {
    c.bench_function("parse_env_var_string", |b| {
        std::env::set_var("BENCH_TEST_VAR", "hello");
        b.iter(|| {
            let _val: String = std::env::var("BENCH_TEST_VAR").unwrap();
        });
        std::env::remove_var("BENCH_TEST_VAR");
    });

    c.bench_function("parse_env_var_u16", |b| {
        std::env::set_var("BENCH_TEST_PORT", "8080");
        b.iter(|| {
            let val: u16 = std::env::var("BENCH_TEST_PORT").unwrap().parse().unwrap();
            criterion::black_box(val);
        });
        std::env::remove_var("BENCH_TEST_PORT");
    });
}

criterion_group!(benches, bench_env_parse);
criterion_main!(benches);
