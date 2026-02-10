//! Prometheus-compatible metrics collection for Ripple.
//!
//! Defines counters, gauges, and histograms for request handling,
//! cache performance, embedding generation, and system health.

use axum::response::IntoResponse;
use lazy_static::lazy_static;
use prometheus::{
    Encoder, GaugeVec, HistogramOpts, HistogramVec, IntCounterVec, IntGauge, Opts, Registry,
    TextEncoder,
};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // -----------------------------------------------------------------------
    // Request metrics
    // -----------------------------------------------------------------------

    /// Total number of requests (labeled by endpoint, tenant, cache_status).
    pub static ref REQUESTS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("ripple_requests_total", "Total number of requests"),
        &["endpoint", "cache_status"]
    )
    .unwrap();

    /// Currently active requests.
    pub static ref ACTIVE_REQUESTS: IntGauge = IntGauge::new(
        "ripple_active_requests",
        "Number of currently active requests"
    )
    .unwrap();

    /// Request duration in seconds.
    pub static ref REQUEST_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("ripple_request_duration_seconds", "Request duration in seconds")
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]),
        &["endpoint", "cache_status"]
    )
    .unwrap();

    // -----------------------------------------------------------------------
    // Cache metrics
    // -----------------------------------------------------------------------

    /// Total cache hits.
    pub static ref CACHE_HITS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("ripple_cache_hits_total", "Total cache hits"),
        &["tier"]
    )
    .unwrap();

    /// Total cache misses.
    pub static ref CACHE_MISSES_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("ripple_cache_misses_total", "Total cache misses"),
        &["endpoint"]
    )
    .unwrap();

    /// Current number of cached entries.
    pub static ref CACHE_SIZE: IntGauge = IntGauge::new(
        "ripple_cache_size",
        "Current number of cached entries"
    )
    .unwrap();

    /// Total cache evictions.
    pub static ref CACHE_EVICTIONS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("ripple_cache_evictions_total", "Total cache evictions"),
        &["reason"]
    )
    .unwrap();

    // -----------------------------------------------------------------------
    // Embedding metrics
    // -----------------------------------------------------------------------

    /// Embedding generation duration in seconds.
    pub static ref EMBEDDING_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("ripple_embedding_duration_seconds", "Embedding generation duration")
            .buckets(vec![0.001, 0.005, 0.01, 0.02, 0.05, 0.1, 0.25]),
        &["batch_size"]
    )
    .unwrap();

    // -----------------------------------------------------------------------
    // Upstream API metrics
    // -----------------------------------------------------------------------

    /// Upstream API call duration in seconds.
    pub static ref UPSTREAM_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("ripple_upstream_duration_seconds", "Upstream API call duration")
            .buckets(vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
        &["provider"]
    )
    .unwrap();

    /// Total upstream API errors.
    pub static ref UPSTREAM_ERRORS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("ripple_upstream_errors_total", "Total upstream API errors"),
        &["provider", "error_type"]
    )
    .unwrap();

    // -----------------------------------------------------------------------
    // Cost savings metrics
    // -----------------------------------------------------------------------

    /// Estimated cost saved in USD.
    pub static ref COST_SAVED_USD: GaugeVec = GaugeVec::new(
        Opts::new("ripple_cost_saved_usd", "Estimated cost saved in USD"),
        &["provider"]
    )
    .unwrap();

    // -----------------------------------------------------------------------
    // Error metrics
    // -----------------------------------------------------------------------

    /// Total errors by type.
    pub static ref ERRORS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("ripple_errors_total", "Total errors by type"),
        &["error_type"]
    )
    .unwrap();
}

/// Register all metrics with the global Prometheus registry.
///
/// Should be called once at application startup.
pub fn register_metrics() {
    REGISTRY.register(Box::new(REQUESTS_TOTAL.clone())).unwrap();
    REGISTRY
        .register(Box::new(ACTIVE_REQUESTS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(REQUEST_DURATION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CACHE_HITS_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CACHE_MISSES_TOTAL.clone()))
        .unwrap();
    REGISTRY.register(Box::new(CACHE_SIZE.clone())).unwrap();
    REGISTRY
        .register(Box::new(CACHE_EVICTIONS_TOTAL.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(EMBEDDING_DURATION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(UPSTREAM_DURATION.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(UPSTREAM_ERRORS_TOTAL.clone()))
        .unwrap();
    REGISTRY.register(Box::new(COST_SAVED_USD.clone())).unwrap();
    REGISTRY.register(Box::new(ERRORS_TOTAL.clone())).unwrap();
}

/// HTTP handler that returns all metrics in Prometheus text exposition format.
pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    let output = String::from_utf8(buffer).unwrap();
    (
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4",
        )],
        output,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_registry() {
        // Ignore errors if already registered (tests run in same process)
        let _ = REGISTRY.register(Box::new(REQUESTS_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(ACTIVE_REQUESTS.clone()));
        let _ = REGISTRY.register(Box::new(REQUEST_DURATION.clone()));
        let _ = REGISTRY.register(Box::new(CACHE_HITS_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(CACHE_MISSES_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(CACHE_SIZE.clone()));
        let _ = REGISTRY.register(Box::new(CACHE_EVICTIONS_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(EMBEDDING_DURATION.clone()));
        let _ = REGISTRY.register(Box::new(UPSTREAM_DURATION.clone()));
        let _ = REGISTRY.register(Box::new(UPSTREAM_ERRORS_TOTAL.clone()));
        let _ = REGISTRY.register(Box::new(COST_SAVED_USD.clone()));
        let _ = REGISTRY.register(Box::new(ERRORS_TOTAL.clone()));
    }

    #[test]
    fn test_counter_increment() {
        setup_registry();
        REQUESTS_TOTAL.with_label_values(&["openai", "hit"]).inc();
        let val = REQUESTS_TOTAL.with_label_values(&["openai", "hit"]).get();
        assert!(val >= 1);
    }

    #[test]
    fn test_gauge_update() {
        setup_registry();
        ACTIVE_REQUESTS.set(5);
        assert_eq!(ACTIVE_REQUESTS.get(), 5);
        ACTIVE_REQUESTS.inc();
        assert_eq!(ACTIVE_REQUESTS.get(), 6);
        ACTIVE_REQUESTS.dec();
        assert_eq!(ACTIVE_REQUESTS.get(), 5);
    }

    #[test]
    fn test_histogram_observe() {
        setup_registry();
        REQUEST_DURATION
            .with_label_values(&["openai", "miss"])
            .observe(0.045);
        // No panic means histogram observation works
    }

    #[test]
    fn test_cache_hits_counter() {
        setup_registry();
        CACHE_HITS_TOTAL.with_label_values(&["l1"]).inc();
        CACHE_HITS_TOTAL.with_label_values(&["l2"]).inc();
        assert!(CACHE_HITS_TOTAL.with_label_values(&["l1"]).get() >= 1);
        assert!(CACHE_HITS_TOTAL.with_label_values(&["l2"]).get() >= 1);
    }

    #[test]
    fn test_cache_size_gauge() {
        setup_registry();
        CACHE_SIZE.set(1000);
        assert_eq!(CACHE_SIZE.get(), 1000);
    }

    #[test]
    fn test_cost_saved_gauge() {
        setup_registry();
        COST_SAVED_USD.with_label_values(&["openai"]).set(123.45);
        let val = COST_SAVED_USD.with_label_values(&["openai"]).get();
        assert!((val - 123.45).abs() < f64::EPSILON);
    }

    #[test]
    fn test_errors_counter() {
        setup_registry();
        ERRORS_TOTAL.with_label_values(&["network"]).inc();
        assert!(ERRORS_TOTAL.with_label_values(&["network"]).get() >= 1);
    }

    #[test]
    fn test_metrics_encoding() {
        setup_registry();
        REQUESTS_TOTAL.with_label_values(&["test", "miss"]).inc();
        let encoder = TextEncoder::new();
        let metric_families = REGISTRY.gather();
        let mut buffer = Vec::new();
        let result = encoder.encode(&metric_families, &mut buffer);
        assert!(result.is_ok());
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("ripple_requests_total"));
    }

    #[test]
    fn test_embedding_duration_histogram() {
        setup_registry();
        EMBEDDING_DURATION.with_label_values(&["1"]).observe(0.012);
        // No panic
    }

    #[test]
    fn test_upstream_duration_histogram() {
        setup_registry();
        UPSTREAM_DURATION
            .with_label_values(&["openai"])
            .observe(0.85);
        // No panic
    }

    #[test]
    fn test_metrics_text_contains_type() {
        setup_registry();
        REQUESTS_TOTAL.with_label_values(&["enc_test", "hit"]).inc();
        let encoder = TextEncoder::new();
        let families = REGISTRY.gather();
        let mut buf = Vec::new();
        encoder.encode(&families, &mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("# TYPE"));
    }
}
