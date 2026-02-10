//! Ripple - High-Performance Semantic Caching Proxy
//!
//! Ripple intercepts API requests and returns cached responses for
//! semantically similar queries, reducing API costs by 60-80%.

mod auth;
mod cache;
mod config;
mod embeddings;
mod metrics;
mod proxy;
mod utils;
mod vector;

use crate::config::Config;
use crate::metrics::prometheus as prom;
use crate::utils::health;
use crate::utils::logging;

use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info, warn};

/// Shared application state available to all handlers.
pub struct AppState {
    pub config: Config,
    pub start_time: std::time::Instant,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Check for --validate-config flag
    let args: Vec<String> = std::env::args().collect();
    let validate_only = args.iter().any(|a| a == "--validate-config");

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => {
            if validate_only {
                println!("Configuration valid");
                std::process::exit(0);
            }
            cfg
        }
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            if validate_only {
                std::process::exit(1);
            }
            return Err(anyhow::anyhow!(e));
        }
    };

    // Initialize logging
    logging::init(&config).map_err(|e| anyhow::anyhow!("Logging init failed: {}", e))?;
    info!("Ripple v{} starting up", env!("CARGO_PKG_VERSION"));
    info!("Configuration loaded successfully");

    // Initialize metrics
    prom::register_metrics();
    info!("Metrics system initialized");

    // Create shared application state
    let state = Arc::new(AppState {
        config: config.clone(),
        start_time: std::time::Instant::now(),
    });

    // Build the main application router
    let app_state = Arc::clone(&state);
    let app = Router::new()
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
        .route("/health/status", get(health::detailed_status))
        .with_state(app_state);

    // Build the metrics server router
    let metrics_app = Router::new().route("/metrics", get(prom::metrics_handler));

    // Spawn metrics server
    let metrics_port = config.metrics_port;
    let metrics_handle = tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", metrics_port);
        info!("Metrics server listening on {}", addr);
        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, metrics_app).await.unwrap();
    });

    // Start main server
    let addr = format!("{}:{}", config.host, config.port);
    info!("Ripple proxy listening on {}", addr);

    let listener = TcpListener::bind(&addr).await?;

    // Graceful shutdown
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }

    info!("Ripple shutting down gracefully");
    metrics_handle.abort();
    Ok(())
}

/// Waits for a shutdown signal (Ctrl+C or SIGTERM).
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { warn!("Received Ctrl+C, shutting down"); },
        _ = terminate => { warn!("Received SIGTERM, shutting down"); },
    }
}
