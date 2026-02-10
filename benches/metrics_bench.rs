//! Benchmarks for metrics operations.

use criterion::{criterion_group, criterion_main, Criterion};
use prometheus::{Histogram, HistogramOpts, IntCounter, IntGauge, Opts};

fn bench_metrics(c: &mut Criterion) {
    let counter = IntCounter::new("bench_counter", "benchmark counter").unwrap();
    let gauge = IntGauge::new("bench_gauge", "benchmark gauge").unwrap();
    let histogram = Histogram::with_opts(
        HistogramOpts::new("bench_histogram", "benchmark histogram")
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1]),
    )
    .unwrap();

    c.bench_function("counter_increment", |b| {
        b.iter(|| {
            counter.inc();
        });
    });

    c.bench_function("gauge_set", |b| {
        b.iter(|| {
            gauge.set(42);
        });
    });

    c.bench_function("histogram_observe", |b| {
        b.iter(|| {
            histogram.observe(0.015);
        });
    });
}

criterion_group!(benches, bench_metrics);
criterion_main!(benches);
