# Ripple Architecture

## System Overview

Ripple is a high-performance semantic caching proxy that sits between client applications and upstream API providers (OpenAI, Anthropic, Exa). It intercepts requests, generates semantic embeddings, and uses vector similarity search to return cached responses for semantically similar queries.

## Component Diagram

```
┌──────────┐     ┌─────────────────────────────────┐
│  Client  │────▶│  Ripple Proxy (Rust/Tokio)      │
│   App    │     │  ┌───────────┐  ┌────────────┐  │
└──────────┘     │  │ Embedding │  │  Semantic   │  │
                 │  │  Engine   │──│   Matcher   │  │
                 │  └───────────┘  └──────┬─────┘  │
                 └─────────────────────────┼───────┘
                                           │
                         ┌─────────────────┼─────────────┐
                         │                 │             │
                    ┌────▼────┐      ┌────▼─────┐  ┌───▼────┐
                    │ Qdrant  │      │ Upstream │  │ Cached │
                    │ Vector  │      │   API    │  │Response│
                    │  DB     │      │ (OpenAI) │  └────────┘
                    └─────────┘      └──────────┘
```

## Data Flow

1. Client sends request to Ripple proxy
2. Request is parsed and query text extracted
3. Query is normalized and embedded (768-dim vector)
4. Vector similarity search in Qdrant (cosine distance)
5. If similarity ≥ threshold → return cached response (cache hit)
6. If below threshold → forward to upstream API (cache miss)
7. Cache the new response asynchronously

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Proxy Server | Tokio + Axum | Async HTTP handling |
| Embedding Model | nomic-embed-text (ONNX) | CPU-optimized embeddings |
| Vector Database | Qdrant | HNSW-indexed similarity search |
| Metrics | Prometheus | Observability |
| Logging | tracing | Structured logging |

## Key Design Decisions

- **Pure Rust**: Performance, safety, and Exa stack alignment
- **CPU embeddings**: No GPU dependency, predictable latency
- **Cosine similarity**: Best for normalized embedding comparison
- **Two-tier cache**: L1 in-memory LRU + L2 vector store
- **Async-first**: Tokio runtime for 10K+ concurrent connections
