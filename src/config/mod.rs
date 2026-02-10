//! Configuration management for Ripple.
//!
//! Loads settings from environment variables (via `.env` file) and validates
//! all values at startup. Provides sensible defaults for optional settings.

use crate::utils::errors::ConfigurationError;

/// Central configuration structure holding all Ripple settings.
#[derive(Debug, Clone)]
pub struct Config {
    // Server
    pub host: String,
    pub port: u16,
    pub workers: usize,

    // Qdrant
    pub qdrant_url: String,
    pub qdrant_collection_name: String,
    pub qdrant_api_key: Option<String>,

    // Embedding model
    pub embedding_model_path: String,
    pub embedding_dimension: usize,
    pub embedding_batch_size: usize,
    pub embedding_cache_size: usize,

    // Cache
    pub similarity_threshold: f64,
    pub cache_ttl_hours: u64,
    pub max_cache_size: usize,
    pub enable_l1_cache: bool,
    pub l1_cache_size: usize,

    // API keys
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub exa_api_key: Option<String>,

    // Logging
    pub log_level: String,
    pub log_format: String,
    pub log_file_path: Option<String>,

    // Metrics
    pub metrics_enabled: bool,
    pub metrics_port: u16,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// Reads the `.env` file if present, then loads each variable with
    /// appropriate defaults and validation.
    pub fn load() -> Result<Self, ConfigurationError> {
        // Load .env file (ignore if missing)
        let _ = dotenvy::dotenv();

        let config = Self {
            host: env_or_default("RIPPLE_HOST", "0.0.0.0"),
            port: parse_env("RIPPLE_PORT", 8080)?,
            workers: parse_env("RIPPLE_WORKERS", num_cpus())?,

            qdrant_url: env_or_default("QDRANT_URL", "http://localhost:6333"),
            qdrant_collection_name: env_or_default("QDRANT_COLLECTION_NAME", "cache_vectors"),
            qdrant_api_key: std::env::var("QDRANT_API_KEY").ok(),

            embedding_model_path: env_or_default(
                "EMBEDDING_MODEL_PATH",
                "./models/nomic-embed-text.onnx",
            ),
            embedding_dimension: parse_env("EMBEDDING_DIMENSION", 768)?,
            embedding_batch_size: parse_env("EMBEDDING_BATCH_SIZE", 10)?,
            embedding_cache_size: parse_env("EMBEDDING_CACHE_SIZE", 10000)?,

            similarity_threshold: parse_env("SIMILARITY_THRESHOLD", 0.85)?,
            cache_ttl_hours: parse_env("CACHE_TTL_HOURS", 168)?,
            max_cache_size: parse_env("MAX_CACHE_SIZE", 10_000_000)?,
            enable_l1_cache: parse_env("ENABLE_L1_CACHE", true)?,
            l1_cache_size: parse_env("L1_CACHE_SIZE", 10_000)?,

            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            exa_api_key: std::env::var("EXA_API_KEY").ok(),

            log_level: env_or_default("LOG_LEVEL", "info"),
            log_format: env_or_default("LOG_FORMAT", "text"),
            log_file_path: std::env::var("LOG_FILE_PATH").ok(),

            metrics_enabled: parse_env("METRICS_ENABLED", true)?,
            metrics_port: parse_env("METRICS_PORT", 9090)?,
        };

        config.validate()?;
        Ok(config)
    }

    /// Validate all configuration values.
    fn validate(&self) -> Result<(), ConfigurationError> {
        self.validate_port(self.port, "RIPPLE_PORT")?;
        self.validate_port(self.metrics_port, "METRICS_PORT")?;
        self.validate_similarity_threshold()?;
        self.validate_ttl()?;
        self.validate_workers()?;
        self.validate_cache_size()?;
        self.validate_embedding_dimension()?;
        self.validate_url(&self.qdrant_url, "QDRANT_URL")?;
        self.validate_log_level()?;
        self.validate_log_format()?;
        self.validate_collection_name()?;
        Ok(())
    }

    fn validate_port(&self, port: u16, name: &str) -> Result<(), ConfigurationError> {
        if port < 1024 {
            return Err(ConfigurationError::InvalidRange {
                field: name.to_string(),
                value: port.to_string(),
                min: "1024".to_string(),
                max: "65535".to_string(),
            });
        }
        Ok(())
    }

    fn validate_similarity_threshold(&self) -> Result<(), ConfigurationError> {
        if !(0.0..=1.0).contains(&self.similarity_threshold) {
            return Err(ConfigurationError::InvalidRange {
                field: "SIMILARITY_THRESHOLD".to_string(),
                value: self.similarity_threshold.to_string(),
                min: "0.0".to_string(),
                max: "1.0".to_string(),
            });
        }
        Ok(())
    }

    fn validate_ttl(&self) -> Result<(), ConfigurationError> {
        if self.cache_ttl_hours == 0 {
            return Err(ConfigurationError::InvalidRange {
                field: "CACHE_TTL_HOURS".to_string(),
                value: "0".to_string(),
                min: "1".to_string(),
                max: "unlimited".to_string(),
            });
        }
        Ok(())
    }

    fn validate_workers(&self) -> Result<(), ConfigurationError> {
        if self.workers == 0 || self.workers > 128 {
            return Err(ConfigurationError::InvalidRange {
                field: "RIPPLE_WORKERS".to_string(),
                value: self.workers.to_string(),
                min: "1".to_string(),
                max: "128".to_string(),
            });
        }
        Ok(())
    }

    fn validate_cache_size(&self) -> Result<(), ConfigurationError> {
        if self.max_cache_size == 0 {
            return Err(ConfigurationError::InvalidRange {
                field: "MAX_CACHE_SIZE".to_string(),
                value: "0".to_string(),
                min: "1".to_string(),
                max: "unlimited".to_string(),
            });
        }
        Ok(())
    }

    fn validate_embedding_dimension(&self) -> Result<(), ConfigurationError> {
        if self.embedding_dimension == 0 || self.embedding_dimension > 4096 {
            return Err(ConfigurationError::InvalidRange {
                field: "EMBEDDING_DIMENSION".to_string(),
                value: self.embedding_dimension.to_string(),
                min: "1".to_string(),
                max: "4096".to_string(),
            });
        }
        Ok(())
    }

    fn validate_url(&self, url: &str, name: &str) -> Result<(), ConfigurationError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(ConfigurationError::InvalidFormat {
                field: name.to_string(),
                value: url.to_string(),
                expected: "URL starting with http:// or https://".to_string(),
            });
        }
        Ok(())
    }

    fn validate_log_level(&self) -> Result<(), ConfigurationError> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.to_lowercase().as_str()) {
            return Err(ConfigurationError::InvalidFormat {
                field: "LOG_LEVEL".to_string(),
                value: self.log_level.clone(),
                expected: "One of: trace, debug, info, warn, error".to_string(),
            });
        }
        Ok(())
    }

    fn validate_log_format(&self) -> Result<(), ConfigurationError> {
        let valid_formats = ["text", "json"];
        if !valid_formats.contains(&self.log_format.to_lowercase().as_str()) {
            return Err(ConfigurationError::InvalidFormat {
                field: "LOG_FORMAT".to_string(),
                value: self.log_format.clone(),
                expected: "One of: text, json".to_string(),
            });
        }
        Ok(())
    }

    fn validate_collection_name(&self) -> Result<(), ConfigurationError> {
        if self.qdrant_collection_name.is_empty() {
            return Err(ConfigurationError::MissingField {
                field: "QDRANT_COLLECTION_NAME".to_string(),
            });
        }
        let valid = self
            .qdrant_collection_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-');
        if !valid {
            return Err(ConfigurationError::InvalidFormat {
                field: "QDRANT_COLLECTION_NAME".to_string(),
                value: self.qdrant_collection_name.clone(),
                expected: "Alphanumeric characters, underscores, and hyphens only".to_string(),
            });
        }
        Ok(())
    }
}

/// Read an environment variable or return a default.
fn env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Parse an environment variable to a specific type, with a default.
fn parse_env<T>(key: &str, default: T) -> Result<T, ConfigurationError>
where
    T: std::str::FromStr + std::fmt::Display,
    T::Err: std::fmt::Display,
{
    match std::env::var(key) {
        Ok(val) => val
            .parse::<T>()
            .map_err(|e| ConfigurationError::ParseError {
                field: key.to_string(),
                value: val,
                expected_type: std::any::type_name::<T>().to_string(),
                details: e.to_string(),
            }),
        Err(_) => Ok(default),
    }
}

/// Get the number of CPUs available.
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a valid default config for testing (no env vars needed).
    fn make_default() -> Config {
        Config {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: 4,
            qdrant_url: "http://localhost:6333".to_string(),
            qdrant_collection_name: "cache_vectors".to_string(),
            qdrant_api_key: None,
            embedding_model_path: "./models/nomic-embed-text.onnx".to_string(),
            embedding_dimension: 768,
            embedding_batch_size: 10,
            embedding_cache_size: 10000,
            similarity_threshold: 0.85,
            cache_ttl_hours: 168,
            max_cache_size: 10_000_000,
            enable_l1_cache: true,
            l1_cache_size: 10_000,
            openai_api_key: None,
            anthropic_api_key: None,
            exa_api_key: None,
            log_level: "info".to_string(),
            log_format: "text".to_string(),
            log_file_path: None,
            metrics_enabled: true,
            metrics_port: 9090,
        }
    }

    #[test]
    fn test_valid_config_validates() {
        let config = make_default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_port_below_1024() {
        let mut config = make_default();
        config.port = 80;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("RIPPLE_PORT"));
    }

    #[test]
    fn test_invalid_metrics_port_below_1024() {
        let mut config = make_default();
        config.metrics_port = 443;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("METRICS_PORT"));
    }

    #[test]
    fn test_invalid_similarity_threshold_too_high() {
        let mut config = make_default();
        config.similarity_threshold = 1.5;
        let result = config.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("SIMILARITY_THRESHOLD"));
    }

    #[test]
    fn test_invalid_similarity_threshold_negative() {
        let mut config = make_default();
        config.similarity_threshold = -0.1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_similarity_threshold_boundaries() {
        let mut config = make_default();
        config.similarity_threshold = 0.0;
        assert!(config.validate().is_ok());
        config.similarity_threshold = 1.0;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_ttl_zero() {
        let mut config = make_default();
        config.cache_ttl_hours = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_workers_zero() {
        let mut config = make_default();
        config.workers = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_workers_too_high() {
        let mut config = make_default();
        config.workers = 200;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_workers_boundaries() {
        let mut config = make_default();
        config.workers = 1;
        assert!(config.validate().is_ok());
        config.workers = 128;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_cache_size_zero() {
        let mut config = make_default();
        config.max_cache_size = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_qdrant_url_format() {
        let mut config = make_default();
        config.qdrant_url = "not-a-url".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_qdrant_url_https() {
        let mut config = make_default();
        config.qdrant_url = "https://qdrant.example.com".to_string();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_log_level() {
        let mut config = make_default();
        config.log_level = "verbose".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_log_levels() {
        let mut config = make_default();
        for level in &["trace", "debug", "info", "warn", "error"] {
            config.log_level = level.to_string();
            assert!(config.validate().is_ok(), "level {} should be valid", level);
        }
    }

    #[test]
    fn test_invalid_log_format() {
        let mut config = make_default();
        config.log_format = "xml".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_log_formats() {
        let mut config = make_default();
        for fmt in &["text", "json"] {
            config.log_format = fmt.to_string();
            assert!(config.validate().is_ok());
        }
    }

    #[test]
    fn test_invalid_collection_name_special_chars() {
        let mut config = make_default();
        config.qdrant_collection_name = "bad name!".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_empty_collection_name() {
        let mut config = make_default();
        config.qdrant_collection_name = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_collection_name_with_hyphens() {
        let mut config = make_default();
        config.qdrant_collection_name = "my-collection_v2".to_string();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_embedding_dimension_zero() {
        let mut config = make_default();
        config.embedding_dimension = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_embedding_dimension_too_large() {
        let mut config = make_default();
        config.embedding_dimension = 5000;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_valid_embedding_dimension_boundaries() {
        let mut config = make_default();
        config.embedding_dimension = 1;
        assert!(config.validate().is_ok());
        config.embedding_dimension = 4096;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_default_values_correct() {
        let config = make_default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert_eq!(config.qdrant_url, "http://localhost:6333");
        assert_eq!(config.qdrant_collection_name, "cache_vectors");
        assert!((config.similarity_threshold - 0.85).abs() < f64::EPSILON);
        assert_eq!(config.cache_ttl_hours, 168);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_format, "text");
        assert_eq!(config.metrics_port, 9090);
        assert_eq!(config.embedding_dimension, 768);
        assert_eq!(config.embedding_batch_size, 10);
        assert_eq!(config.embedding_cache_size, 10000);
        assert_eq!(config.max_cache_size, 10_000_000);
        assert!(config.enable_l1_cache);
        assert_eq!(config.l1_cache_size, 10_000);
        assert!(config.metrics_enabled);
    }

    #[test]
    fn test_parse_env_valid_u16() {
        std::env::set_var("__TEST_PARSE_U16", "3000");
        let result: Result<u16, _> = parse_env("__TEST_PARSE_U16", 8080);
        assert_eq!(result.unwrap(), 3000);
        std::env::remove_var("__TEST_PARSE_U16");
    }

    #[test]
    fn test_parse_env_invalid_u16() {
        std::env::set_var("__TEST_PARSE_BAD", "abc");
        let result: Result<u16, _> = parse_env("__TEST_PARSE_BAD", 8080);
        assert!(result.is_err());
        std::env::remove_var("__TEST_PARSE_BAD");
    }

    #[test]
    fn test_parse_env_missing_uses_default() {
        std::env::remove_var("__TEST_PARSE_MISSING");
        let result: Result<u16, _> = parse_env("__TEST_PARSE_MISSING", 9999);
        assert_eq!(result.unwrap(), 9999);
    }

    #[test]
    fn test_env_or_default_missing() {
        std::env::remove_var("__TEST_ENV_DEFAULT");
        let val = env_or_default("__TEST_ENV_DEFAULT", "fallback");
        assert_eq!(val, "fallback");
    }

    #[test]
    fn test_env_or_default_present() {
        std::env::set_var("__TEST_ENV_PRESENT", "custom");
        let val = env_or_default("__TEST_ENV_PRESENT", "fallback");
        assert_eq!(val, "custom");
        std::env::remove_var("__TEST_ENV_PRESENT");
    }
}
