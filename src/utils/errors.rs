//! Error handling framework for Ripple.
//!
//! Defines error types for every subsystem and a unified [`RippleError`] enum
//! that wraps them all for convenient propagation.

use thiserror::Error;

// ---------------------------------------------------------------------------
// 1. ConfigurationError
// ---------------------------------------------------------------------------

/// Errors related to application configuration.
#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("Missing required configuration field: {field}")]
    MissingField { field: String },

    #[error("Invalid value for {field}: '{value}' (expected {expected})")]
    InvalidFormat {
        field: String,
        value: String,
        expected: String,
    },

    #[error("{field} value '{value}' is out of range [{min}, {max}]")]
    InvalidRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    #[error("Failed to parse {field}: '{value}' is not a valid {expected_type} ({details})")]
    ParseError {
        field: String,
        value: String,
        expected_type: String,
        details: String,
    },

    #[error("File not found at path '{path}': {details}")]
    FileNotFound { path: String, details: String },
}

// ---------------------------------------------------------------------------
// 2. NetworkError
// ---------------------------------------------------------------------------

/// Errors originating from network operations.
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection failed to {endpoint}: {details}")]
    ConnectionFailed { endpoint: String, details: String },

    #[error("Request timed out after {timeout_ms}ms to {endpoint}")]
    Timeout { endpoint: String, timeout_ms: u64 },

    #[error("TLS error connecting to {endpoint}: {details}")]
    TlsError { endpoint: String, details: String },

    #[error("DNS resolution failed for {host}: {details}")]
    DnsError { host: String, details: String },
}

// ---------------------------------------------------------------------------
// 3. CacheError
// ---------------------------------------------------------------------------

/// Errors related to the caching subsystem.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache read failed for key '{key}': {details}")]
    ReadFailed { key: String, details: String },

    #[error("Cache write failed for key '{key}': {details}")]
    WriteFailed { key: String, details: String },

    #[error("Cache invalidation failed: {details}")]
    InvalidationFailed { details: String },

    #[error("Cache capacity exceeded (max: {max_size})")]
    CapacityExceeded { max_size: usize },
}

// ---------------------------------------------------------------------------
// 4. EmbeddingError
// ---------------------------------------------------------------------------

/// Errors from the embedding engine.
#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Failed to load embedding model from '{path}': {details}")]
    ModelLoadFailed { path: String, details: String },

    #[error("Embedding inference failed: {details}")]
    InferenceFailed { details: String },

    #[error("Tokenization failed for input: {details}")]
    TokenizationFailed { details: String },

    #[error("Invalid embedding dimensions: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
}

// ---------------------------------------------------------------------------
// 5. VectorStoreError
// ---------------------------------------------------------------------------

/// Errors from the vector database (Qdrant) layer.
#[derive(Debug, Error)]
pub enum VectorStoreError {
    #[error("Vector store connection failed: {details}")]
    ConnectionFailed { details: String },

    #[error("Collection '{collection}' not found")]
    CollectionNotFound { collection: String },

    #[error("Vector insertion failed: {details}")]
    InsertionFailed { details: String },

    #[error("Vector search failed: {details}")]
    SearchFailed { details: String },

    #[error("Vector deletion failed: {details}")]
    DeletionFailed { details: String },
}

// ---------------------------------------------------------------------------
// 6. UpstreamApiError
// ---------------------------------------------------------------------------

/// Errors from upstream API providers (OpenAI, Anthropic, etc.).
#[derive(Debug, Error)]
pub enum UpstreamApiError {
    #[error("Upstream API request failed ({provider}): {details}")]
    RequestFailed { provider: String, details: String },

    #[error("Upstream API rate limited ({provider}): retry after {retry_after_secs}s")]
    RateLimited {
        provider: String,
        retry_after_secs: u64,
    },

    #[error("Invalid response from upstream ({provider}): {details}")]
    InvalidResponse { provider: String, details: String },

    #[error("Upstream API key invalid for {provider}")]
    InvalidApiKey { provider: String },
}

// ---------------------------------------------------------------------------
// 7. AuthenticationError
// ---------------------------------------------------------------------------

/// Errors related to request authentication and authorization.
#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Missing API key in request")]
    MissingApiKey,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("API key expired")]
    ExpiredApiKey,

    #[error("Insufficient permissions for tenant '{tenant_id}'")]
    InsufficientPermissions { tenant_id: String },
}

// ---------------------------------------------------------------------------
// 8. ValidationError
// ---------------------------------------------------------------------------

/// Errors from request validation.
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid request format: {details}")]
    InvalidFormat { details: String },

    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Field '{field}' value too large: max {max}")]
    ValueTooLarge { field: String, max: String },

    #[error("Unsupported model: {model}")]
    UnsupportedModel { model: String },
}

// ---------------------------------------------------------------------------
// Unified RippleError
// ---------------------------------------------------------------------------

/// Top-level error type that wraps all domain-specific errors.
#[derive(Debug, Error)]
pub enum RippleError {
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigurationError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("Embedding error: {0}")]
    Embedding(#[from] EmbeddingError),

    #[error("Vector store error: {0}")]
    VectorStore(#[from] VectorStoreError),

    #[error("Upstream API error: {0}")]
    UpstreamApi(#[from] UpstreamApiError),

    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Internal error: {0}")]
    Internal(String),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_error_missing_field() {
        let err = ConfigurationError::MissingField {
            field: "API_KEY".to_string(),
        };
        assert!(err.to_string().contains("API_KEY"));
    }

    #[test]
    fn test_configuration_error_invalid_range() {
        let err = ConfigurationError::InvalidRange {
            field: "PORT".to_string(),
            value: "80".to_string(),
            min: "1024".to_string(),
            max: "65535".to_string(),
        };
        assert!(err.to_string().contains("80"));
        assert!(err.to_string().contains("1024"));
    }

    #[test]
    fn test_configuration_error_parse() {
        let err = ConfigurationError::ParseError {
            field: "PORT".to_string(),
            value: "abc".to_string(),
            expected_type: "u16".to_string(),
            details: "invalid digit".to_string(),
        };
        assert!(err.to_string().contains("abc"));
    }

    #[test]
    fn test_configuration_error_format() {
        let err = ConfigurationError::InvalidFormat {
            field: "URL".to_string(),
            value: "bad".to_string(),
            expected: "http://".to_string(),
        };
        assert!(err.to_string().contains("bad"));
    }

    #[test]
    fn test_network_error_connection_failed() {
        let err = NetworkError::ConnectionFailed {
            endpoint: "localhost:6333".to_string(),
            details: "connection refused".to_string(),
        };
        assert!(err.to_string().contains("localhost:6333"));
    }

    #[test]
    fn test_network_error_timeout() {
        let err = NetworkError::Timeout {
            endpoint: "api.openai.com".to_string(),
            timeout_ms: 5000,
        };
        assert!(err.to_string().contains("5000ms"));
    }

    #[test]
    fn test_cache_error_read() {
        let err = CacheError::ReadFailed {
            key: "test_key".to_string(),
            details: "corruption".to_string(),
        };
        assert!(err.to_string().contains("test_key"));
    }

    #[test]
    fn test_cache_error_capacity() {
        let err = CacheError::CapacityExceeded { max_size: 1000 };
        assert!(err.to_string().contains("1000"));
    }

    #[test]
    fn test_embedding_error_model_load() {
        let err = EmbeddingError::ModelLoadFailed {
            path: "/models/test.onnx".to_string(),
            details: "file not found".to_string(),
        };
        assert!(err.to_string().contains("/models/test.onnx"));
    }

    #[test]
    fn test_embedding_error_dimension_mismatch() {
        let err = EmbeddingError::DimensionMismatch {
            expected: 768,
            actual: 384,
        };
        assert!(err.to_string().contains("768"));
        assert!(err.to_string().contains("384"));
    }

    #[test]
    fn test_vector_store_error_connection() {
        let err = VectorStoreError::ConnectionFailed {
            details: "refused".to_string(),
        };
        assert!(err.to_string().contains("refused"));
    }

    #[test]
    fn test_upstream_api_error_rate_limited() {
        let err = UpstreamApiError::RateLimited {
            provider: "OpenAI".to_string(),
            retry_after_secs: 60,
        };
        assert!(err.to_string().contains("60s"));
    }

    #[test]
    fn test_auth_error_missing_key() {
        let err = AuthenticationError::MissingApiKey;
        assert!(err.to_string().contains("Missing API key"));
    }

    #[test]
    fn test_validation_error_missing_field() {
        let err = ValidationError::MissingField {
            field: "messages".to_string(),
        };
        assert!(err.to_string().contains("messages"));
    }

    #[test]
    fn test_ripple_error_from_config() {
        let cfg_err = ConfigurationError::MissingField {
            field: "test".to_string(),
        };
        let ripple_err: RippleError = cfg_err.into();
        assert!(ripple_err.to_string().contains("Configuration error"));
    }

    #[test]
    fn test_ripple_error_from_network() {
        let net_err = NetworkError::Timeout {
            endpoint: "host".to_string(),
            timeout_ms: 100,
        };
        let ripple_err: RippleError = net_err.into();
        assert!(ripple_err.to_string().contains("Network error"));
    }

    #[test]
    fn test_ripple_error_from_cache() {
        let cache_err = CacheError::ReadFailed {
            key: "k".to_string(),
            details: "d".to_string(),
        };
        let ripple_err: RippleError = cache_err.into();
        assert!(ripple_err.to_string().contains("Cache error"));
    }

    #[test]
    fn test_ripple_error_from_embedding() {
        let emb_err = EmbeddingError::InferenceFailed {
            details: "oom".to_string(),
        };
        let ripple_err: RippleError = emb_err.into();
        assert!(ripple_err.to_string().contains("Embedding error"));
    }

    #[test]
    fn test_ripple_error_from_vector_store() {
        let vs_err = VectorStoreError::SearchFailed {
            details: "timeout".to_string(),
        };
        let ripple_err: RippleError = vs_err.into();
        assert!(ripple_err.to_string().contains("Vector store error"));
    }

    #[test]
    fn test_ripple_error_from_auth() {
        let auth_err = AuthenticationError::InvalidApiKey;
        let ripple_err: RippleError = auth_err.into();
        assert!(ripple_err.to_string().contains("Authentication error"));
    }

    #[test]
    fn test_ripple_error_from_validation() {
        let val_err = ValidationError::InvalidFormat {
            details: "bad json".to_string(),
        };
        let ripple_err: RippleError = val_err.into();
        assert!(ripple_err.to_string().contains("Validation error"));
    }

    #[test]
    fn test_ripple_error_from_upstream() {
        let up_err = UpstreamApiError::InvalidApiKey {
            provider: "Anthropic".to_string(),
        };
        let ripple_err: RippleError = up_err.into();
        assert!(ripple_err.to_string().contains("Upstream API error"));
    }

    #[test]
    fn test_ripple_error_internal() {
        let err = RippleError::Internal("something broke".to_string());
        assert!(err.to_string().contains("something broke"));
    }

    #[test]
    fn test_error_chain_config_to_ripple() {
        fn load_config() -> Result<(), ConfigurationError> {
            Err(ConfigurationError::MissingField {
                field: "KEY".to_string(),
            })
        }

        fn do_work() -> Result<(), RippleError> {
            load_config()?;
            Ok(())
        }

        let result = do_work();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("KEY"));
    }

    #[test]
    fn test_network_tls_error() {
        let err = NetworkError::TlsError {
            endpoint: "api.example.com".to_string(),
            details: "certificate expired".to_string(),
        };
        assert!(err.to_string().contains("TLS"));
    }

    #[test]
    fn test_network_dns_error() {
        let err = NetworkError::DnsError {
            host: "invalid.host".to_string(),
            details: "NXDOMAIN".to_string(),
        };
        assert!(err.to_string().contains("DNS"));
    }
}
