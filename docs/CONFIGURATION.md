# Configuration Reference

Ripple loads all configuration from environment variables. Create a `.env` file from the template:
```bash
cp .env.example .env
```

## Server Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `RIPPLE_HOST` | String | `0.0.0.0` | IP address to bind to |
| `RIPPLE_PORT` | u16 | `8080` | Server port (1024–65535) |
| `RIPPLE_WORKERS` | usize | CPU count | Worker threads (1–128) |

## Qdrant Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `QDRANT_URL` | URL | `http://localhost:6333` | Qdrant HTTP endpoint |
| `QDRANT_COLLECTION_NAME` | String | `cache_vectors` | Collection name (alphanumeric, `_`, `-`) |
| `QDRANT_API_KEY` | String | *(none)* | Optional authentication key |

## Embedding Model

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `EMBEDDING_MODEL_PATH` | Path | `./models/nomic-embed-text.onnx` | ONNX model file |
| `EMBEDDING_DIMENSION` | usize | `768` | Embedding vector dimension (1–4096) |
| `EMBEDDING_BATCH_SIZE` | usize | `10` | Batch size for generation |
| `EMBEDDING_CACHE_SIZE` | usize | `10000` | LRU cache size for embeddings |

## Cache Configuration

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `SIMILARITY_THRESHOLD` | f64 | `0.85` | Minimum similarity for cache hit (0.0–1.0) |
| `CACHE_TTL_HOURS` | u64 | `168` | Time-to-live in hours (>0) |
| `MAX_CACHE_SIZE` | usize | `10000000` | Maximum cached entries (>0) |
| `ENABLE_L1_CACHE` | bool | `true` | Enable in-memory LRU cache |
| `L1_CACHE_SIZE` | usize | `10000` | L1 hot-key cache entries |

## API Provider Keys

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `OPENAI_API_KEY` | String | *(none)* | OpenAI API key |
| `ANTHROPIC_API_KEY` | String | *(none)* | Anthropic API key |
| `EXA_API_KEY` | String | *(none)* | Exa API key |

## Logging

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `LOG_LEVEL` | String | `info` | Log level: trace, debug, info, warn, error |
| `LOG_FORMAT` | String | `text` | Output format: text, json |
| `LOG_FILE_PATH` | Path | *(none)* | Optional file output path |

## Metrics

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `METRICS_ENABLED` | bool | `true` | Enable Prometheus metrics |
| `METRICS_PORT` | u16 | `9090` | Metrics endpoint port (1024–65535) |

## Validation Rules

- **Port numbers** must be in range 1024–65535
- **Similarity threshold** must be between 0.0 and 1.0
- **Cache TTL** must be greater than 0
- **Workers** must be between 1 and 128
- **Qdrant URL** must start with `http://` or `https://`
- **Collection name** must only contain alphanumeric characters, underscores, and hyphens
- **Log level** must be one of: trace, debug, info, warn, error
- **Log format** must be one of: text, json
- **Embedding dimension** must be between 1 and 4096

Validation runs automatically at startup. Use `--validate-config` flag to check without starting the server:
```bash
cargo run -- --validate-config
```
