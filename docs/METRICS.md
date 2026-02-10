# Metrics Reference

Ripple exposes Prometheus-compatible metrics at `http://localhost:9090/metrics`.

## Available Metrics

### Request Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `ripple_requests_total` | Counter | endpoint, cache_status | Total requests processed |
| `ripple_active_requests` | Gauge | — | Currently active requests |
| `ripple_request_duration_seconds` | Histogram | endpoint, cache_status | Request latency |

### Cache Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `ripple_cache_hits_total` | Counter | tier | Cache hits (l1, l2) |
| `ripple_cache_misses_total` | Counter | endpoint | Cache misses |
| `ripple_cache_size` | Gauge | — | Current cached entry count |
| `ripple_cache_evictions_total` | Counter | reason | Eviction events |

### Embedding Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `ripple_embedding_duration_seconds` | Histogram | batch_size | Embedding generation time |

### Upstream API Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `ripple_upstream_duration_seconds` | Histogram | provider | Upstream call latency |
| `ripple_upstream_errors_total` | Counter | provider, error_type | Upstream errors |

### Business Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `ripple_cost_saved_usd` | Gauge | provider | Estimated cost savings |

### Error Metrics

| Metric | Type | Labels | Description |
|--------|------|--------|-------------|
| `ripple_errors_total` | Counter | error_type | Total errors by type |

## Example Prometheus Queries

```promql
# Cache hit rate (last 5 minutes)
sum(rate(ripple_cache_hits_total[5m])) / sum(rate(ripple_requests_total[5m]))

# P95 request latency
histogram_quantile(0.95, rate(ripple_request_duration_seconds_bucket[5m]))

# Total cost savings
sum(ripple_cost_saved_usd)

# Error rate
sum(rate(ripple_errors_total[5m]))
```

## Scrape Configuration

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'ripple'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 15s
```

## Accessing Metrics

```bash
curl http://localhost:9090/metrics
```

Output is in Prometheus text exposition format (plain text with `# TYPE` and `# HELP` annotations).
