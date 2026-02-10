//! Benchmarks for logging performance.

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_logging(c: &mut Criterion) {
    // Initialize a no-op subscriber for benchmarking log macro overhead
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::sink)
        .finish();
    let _guard = tracing::subscriber::set_default(subscriber);

    c.bench_function("log_info_message", |b| {
        b.iter(|| {
            tracing::info!("benchmark log message");
        });
    });

    c.bench_function("log_debug_filtered_out", |b| {
        b.iter(|| {
            tracing::debug!("this should be filtered out");
        });
    });

    c.bench_function("log_info_with_fields", |b| {
        b.iter(|| {
            tracing::info!(request_id = "abc-123", latency_ms = 42, "request completed");
        });
    });
}

criterion_group!(benches, bench_logging);
criterion_main!(benches);
