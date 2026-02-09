# Architecture

## System Overview

Ripple is a semantic caching proxy that sits between clients and external APIs.
It intercepts requests, computes embeddings, and uses vector similarity to
determine cache hits.

---

## Component Diagram

[Client]
   |
   v
[API Gateway]
   |
   v
[Embedding Engine] ---> [Vector Store (Qdrant)]
   |
   v
[Cache Manager]
   |
   v
[External API]

---

## Data Flow

1. Client sends request
2. Request is embedded
3. Vector similarity search is performed
4. Cache is checked
5. Response is returned or forwarded
6. Result is stored for future reuse

---

## Technology Stack

- Rust
- Tokio
- Serde
- Qdrant
- ONNX Runtime
- Docker (optional)
