# Ripple

Ripple is a high-performance semantic caching proxy designed to reduce API costs
by intelligently caching and reusing previous responses using vector similarity
and embedding-based retrieval.

It sits between clients and external APIs, analyzing requests and responses to
maximize cache reuse while maintaining accuracy and performance.

---

## Features

- Semantic cache using vector similarity search
- High-performance async server powered by Tokio
- Integration with Qdrant vector database
- Configurable embedding model support
- Production-ready logging and monitoring

---

## Quick Start

Clone the repository and set up environment variables before running the server.

---

## Installation

### Prerequisites

- Rust (stable)
- Qdrant
- ONNX Runtime (optional for embeddings)

### Build

```bash
cargo build --release
