# Ripple 🌊

### High-Performance Semantic Caching Proxy for API Cost Reduction

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Save 60-80% on LLM and Search API costs** through intelligent semantic caching.

Ripple is a drop-in caching proxy that understands query semantics. Unlike traditional exact-match caching, Ripple recognizes that "How do I reset my password?" and "I forgot my password, what should I do?" are semantically identical — returning cached responses instantly without redundant API calls.

**Built in pure Rust for production workloads.**

---

## Features

- **Semantic caching** — vector similarity search identifies equivalent queries
- **Drop-in proxy** — change one URL, no code changes required
- **High performance** — async Rust server powered by Tokio + Axum
- **Qdrant integration** — HNSW-indexed vector database for fast similarity search
- **Configurable thresholds** — tune precision vs. recall for your use case
- **Prometheus metrics** — full observability with counters, gauges, and histograms
- **Health checks** — liveness, readiness, and detailed status endpoints
- **Multi-provider support** — OpenAI, Anthropic, and Exa APIs

---

## Quick Start

### Prerequisites

- **Rust 1.75+** (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **Docker** (for Qdrant vector database)
- **8 GB RAM** minimum (16 GB recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/vivekjami/ripple.git
cd ripple

# Start Qdrant vector database
docker compose up -d

# Configure environment
cp .env.example .env
# Edit .env with your API keys

# Build and run
cargo build --release
cargo run --release
```

Ripple is now running on `http://localhost:8080` 🎉

---

## Configuration

All settings are loaded from environment variables (`.env` file). See [`.env.example`](.env.example) for the full list.

| Variable | Default | Description |
|----------|---------|-------------|
| `RIPPLE_PORT` | `8080` | Server port |
| `QDRANT_URL` | `http://localhost:6333` | Qdrant connection URL |
| `SIMILARITY_THRESHOLD` | `0.85` | Cache hit similarity threshold (0.0–1.0) |
| `CACHE_TTL_HOURS` | `168` | Cache entry time-to-live (hours) |
| `LOG_LEVEL` | `info` | Logging level (trace/debug/info/warn/error) |
| `METRICS_PORT` | `9090` | Prometheus metrics endpoint port |

See [`docs/CONFIGURATION.md`](docs/CONFIGURATION.md) for full documentation.

---

## Usage

Change your API endpoint from:
```
https://api.openai.com/v1/chat/completions
```

To:
```
http://localhost:8080/openai/v1/chat/completions
```

**That's it!** No code changes required.

---

## Monitoring

- **Health:** `curl http://localhost:8080/health/live`
- **Readiness:** `curl http://localhost:8080/health/ready`
- **Status:** `curl http://localhost:8080/health/status`
- **Metrics:** `curl http://localhost:9090/metrics`

---

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Security audit
cargo audit
```

---

## Architecture

See [`ARCHITECTURE.md`](ARCHITECTURE.md) for detailed system design.

```
Client → Ripple Proxy → Embedding Engine → Qdrant Vector DB
                                         → Upstream API (on cache miss)
```

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

**Built with ❤️ in Rust**
**Saving developers money, one cached query at a time** 🌊
