//! Structured logging system for Ripple.
//!
//! Supports console and file output, configurable log levels, and
//! both human-readable and JSON formats.

use crate::config::Config;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

/// Initialize the logging subsystem based on the application configuration.
///
/// Returns a guard that keeps the log writer alive. The caller must hold
/// onto this guard for the lifetime of the application.
pub fn init(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_new(&config.log_level)?;

    if let Some(ref log_path) = config.log_file_path {
        // Determine directory and file name
        let path = std::path::Path::new(log_path);
        let dir = path.parent().unwrap_or(std::path::Path::new("."));
        let filename = path
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("ripple.log"));

        // Create directory if it doesn't exist
        std::fs::create_dir_all(dir)?;

        let file_appender = tracing_appender::rolling::never(dir, filename);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Leak the guard so the writer stays alive for the process lifetime.
        // This is acceptable for a long-running server process.
        std::mem::forget(_guard);

        if config.log_format == "json" {
            let subscriber = Registry::default()
                .with(filter)
                .with(fmt::layer().json().with_writer(non_blocking))
                .with(fmt::layer().with_target(true));
            tracing::subscriber::set_global_default(subscriber)?;
        } else {
            let subscriber = Registry::default()
                .with(filter)
                .with(fmt::layer().with_ansi(false).with_writer(non_blocking))
                .with(fmt::layer().with_target(true));
            tracing::subscriber::set_global_default(subscriber)?;
        }
    } else {
        // Console only
        if config.log_format == "json" {
            let subscriber = Registry::default()
                .with(filter)
                .with(fmt::layer().json().with_target(true));
            tracing::subscriber::set_global_default(subscriber)?;
        } else {
            let subscriber = Registry::default()
                .with(filter)
                .with(fmt::layer().with_target(true));
            tracing::subscriber::set_global_default(subscriber)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_filter_creation_info() {
        let filter = EnvFilter::try_new("info");
        assert!(filter.is_ok());
    }

    #[test]
    fn test_env_filter_creation_debug() {
        let filter = EnvFilter::try_new("debug");
        assert!(filter.is_ok());
    }

    #[test]
    fn test_env_filter_creation_error() {
        let filter = EnvFilter::try_new("error");
        assert!(filter.is_ok());
    }

    #[test]
    fn test_env_filter_creation_warn() {
        let filter = EnvFilter::try_new("warn");
        assert!(filter.is_ok());
    }

    #[test]
    fn test_env_filter_creation_trace() {
        let filter = EnvFilter::try_new("trace");
        assert!(filter.is_ok());
    }
}
