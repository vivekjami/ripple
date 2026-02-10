# Health Check Endpoints

Ripple provides three health check endpoints for monitoring and orchestration.

## Endpoints

### GET `/health/live` — Liveness Probe

Returns 200 if the application is running. Does **not** check external dependencies.

**Response (200):**
```json
{"status": "alive"}
```

Use this for Kubernetes liveness probes or basic uptime monitoring.

---

### GET `/health/ready` — Readiness Probe

Returns 200 if the application is ready to serve traffic. Checks Qdrant connectivity.

**Response (200 — Ready):**
```json
{
  "status": "ready",
  "checks": {
    "qdrant": "healthy"
  }
}
```

**Response (503 — Not Ready):**
```json
{
  "status": "not_ready",
  "checks": {
    "qdrant": "unhealthy"
  }
}
```

Use this for Kubernetes readiness probes or load balancer health checks.

---

### GET `/health/status` — Detailed Status

Returns comprehensive health information including component status, uptime, and configuration.

**Response (200):**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "components": {
    "qdrant": {
      "status": "healthy",
      "url": "http://localhost:6333"
    },
    "embedding_model": {
      "status": "not_loaded",
      "path": "./models/nomic-embed-text.onnx"
    }
  },
  "configuration": {
    "cache_ttl_hours": 168,
    "similarity_threshold": 0.85,
    "max_cache_size": 10000000
  }
}
```

## Status Values

| Status | Meaning |
|--------|---------|
| `healthy` | All components operational |
| `degraded` | Some components unavailable but service partially functional |
| `unhealthy` | Critical failure, service cannot operate |
| `alive` | Application process is running |
| `ready` | Application is ready to accept traffic |
| `not_ready` | Application is not yet ready (dependencies unavailable) |

## HTTP Status Codes

| Endpoint | Code | Meaning |
|----------|------|---------|
| `/health/live` | 200 | Application alive |
| `/health/ready` | 200 | Ready to serve |
| `/health/ready` | 503 | Not ready |
| `/health/status` | 200 | Always returns status info |

## Kubernetes Configuration

```yaml
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5
```
