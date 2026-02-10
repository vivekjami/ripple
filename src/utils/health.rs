//! Health check endpoints for Ripple.
//!
//! Provides liveness, readiness, and detailed status endpoints for
//! monitoring and orchestration systems.

use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use serde_json::{json, Value};
use std::sync::Arc;

/// Liveness probe — returns 200 if the application is running.
///
/// This does not check external dependencies; it simply confirms that
/// the HTTP server is alive and processing requests.
pub async fn liveness() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "alive"})))
}

/// Readiness probe — returns 200 if the application can serve traffic.
///
/// Checks connectivity to Qdrant. Returns 503 if any critical
/// dependency is unavailable.
pub async fn readiness(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let qdrant_ok = check_qdrant(&state.config.qdrant_url).await;

    if qdrant_ok {
        (
            StatusCode::OK,
            Json(json!({
                "status": "ready",
                "checks": {
                    "qdrant": "healthy"
                }
            })),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "status": "not_ready",
                "checks": {
                    "qdrant": "unhealthy"
                }
            })),
        )
    }
}

/// Detailed health status — returns JSON with component-level information.
pub async fn detailed_status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let qdrant_ok = check_qdrant(&state.config.qdrant_url).await;
    let uptime_secs = state.start_time.elapsed().as_secs();

    let overall = if qdrant_ok { "healthy" } else { "degraded" };

    let status: Value = json!({
        "status": overall,
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": uptime_secs,
        "components": {
            "qdrant": {
                "status": if qdrant_ok { "healthy" } else { "unhealthy" },
                "url": state.config.qdrant_url
            },
            "embedding_model": {
                "status": "not_loaded",
                "path": state.config.embedding_model_path
            }
        },
        "configuration": {
            "cache_ttl_hours": state.config.cache_ttl_hours,
            "similarity_threshold": state.config.similarity_threshold,
            "max_cache_size": state.config.max_cache_size,
        }
    });

    // Always return 200 for informational endpoint
    let code = StatusCode::OK;

    (code, Json(status))
}

/// Check Qdrant connectivity by hitting its health endpoint.
async fn check_qdrant(qdrant_url: &str) -> bool {
    let url = format!("{}/healthz", qdrant_url);
    match reqwest::Client::new()
        .get(&url)
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_liveness_returns_ok() {
        // Verify liveness handler runs without panic
        let _response = liveness().await;
    }

    #[tokio::test]
    async fn test_check_qdrant_unreachable() {
        // Qdrant at a non-existent port should return false
        let result = check_qdrant("http://127.0.0.1:19999").await;
        assert!(!result);
    }
}
