//! Error handling for PSOC
//!
//! This module provides a comprehensive error handling system for the PSOC application.
//! It uses `thiserror` for custom error types and `anyhow` for error context and propagation.

use thiserror::Error;

/// Main error type for PSOC application
#[derive(Error, Debug)]
pub enum PsocError {
    /// IO-related errors (file operations, network, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Image processing and manipulation errors
    #[error("Image processing error: {message}")]
    ImageProcessing { message: String },

    /// GUI and user interface errors
    #[error("GUI error: {message}")]
    Gui { message: String },

    /// File format parsing and writing errors
    #[error("File format error: {format} - {message}")]
    FileFormat { format: String, message: String },

    /// Configuration and settings errors
    #[error("Configuration error: {key} - {message}")]
    Config { key: String, message: String },

    /// Plugin system errors
    #[error("Plugin error: {plugin} - {message}")]
    Plugin { plugin: String, message: String },

    /// Rendering and graphics errors
    #[error("Rendering error: {message}")]
    Rendering { message: String },

    /// Memory allocation and management errors
    #[error("Memory error: {message}")]
    Memory { message: String },

    /// Validation errors for user input or data
    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    /// Network and communication errors
    #[error("Network error: {message}")]
    Network { message: String },

    /// Permission and access control errors
    #[error("Permission denied: {resource} - {message}")]
    Permission { resource: String, message: String },

    /// Resource not found errors
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// Operation timeout errors
    #[error("Operation timeout: {operation} after {duration_ms}ms")]
    Timeout { operation: String, duration_ms: u64 },

    /// Tool system errors
    #[error("Tool error: {message}")]
    Tool { message: String },

    /// Generic application errors
    #[error("Application error: {message}")]
    Application { message: String },

    /// Unknown or unexpected errors
    #[error("Unknown error: {message}")]
    Unknown { message: String },

    /// Anyhow error wrapper for external library errors
    #[error("External error: {0}")]
    External(#[from] anyhow::Error),
}

/// Result type alias for PSOC operations
pub type Result<T> = std::result::Result<T, PsocError>;

/// Result type alias with anyhow for error context
pub type ContextResult<T> = anyhow::Result<T>;

impl PsocError {
    /// Create a new image processing error
    pub fn image_processing<S: Into<String>>(message: S) -> Self {
        Self::ImageProcessing {
            message: message.into(),
        }
    }

    /// Create a new GUI error
    pub fn gui<S: Into<String>>(message: S) -> Self {
        Self::Gui {
            message: message.into(),
        }
    }

    /// Create a new file format error
    pub fn file_format<S1: Into<String>, S2: Into<String>>(format: S1, message: S2) -> Self {
        Self::FileFormat {
            format: format.into(),
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn config<S1: Into<String>, S2: Into<String>>(key: S1, message: S2) -> Self {
        Self::Config {
            key: key.into(),
            message: message.into(),
        }
    }

    /// Create a new plugin error
    pub fn plugin<S1: Into<String>, S2: Into<String>>(plugin: S1, message: S2) -> Self {
        Self::Plugin {
            plugin: plugin.into(),
            message: message.into(),
        }
    }

    /// Create a new rendering error
    pub fn rendering<S: Into<String>>(message: S) -> Self {
        Self::Rendering {
            message: message.into(),
        }
    }

    /// Create a new memory error
    pub fn memory<S: Into<String>>(message: S) -> Self {
        Self::Memory {
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation<S1: Into<String>, S2: Into<String>>(field: S1, message: S2) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a new network error
    pub fn network<S: Into<String>>(message: S) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    /// Create a new permission error
    pub fn permission<S1: Into<String>, S2: Into<String>>(resource: S1, message: S2) -> Self {
        Self::Permission {
            resource: resource.into(),
            message: message.into(),
        }
    }

    /// Create a new not found error
    pub fn not_found<S: Into<String>>(resource: S) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(operation: S, duration_ms: u64) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration_ms,
        }
    }

    /// Create a new tool error
    pub fn tool<S: Into<String>>(message: S) -> Self {
        Self::Tool {
            message: message.into(),
        }
    }

    /// Create a new application error
    pub fn application<S: Into<String>>(message: S) -> Self {
        Self::Application {
            message: message.into(),
        }
    }

    /// Create a new unknown error
    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Io(_) => false,
            Self::ImageProcessing { .. } => true,
            Self::Gui { .. } => true,
            Self::FileFormat { .. } => true,
            Self::Config { .. } => true,
            Self::Plugin { .. } => true,
            Self::Rendering { .. } => true,
            Self::Tool { .. } => true,
            Self::Memory { .. } => false,
            Self::Validation { .. } => true,
            Self::Network { .. } => true,
            Self::Permission { .. } => false,
            Self::NotFound { .. } => true,
            Self::Timeout { .. } => true,
            Self::Application { .. } => true,
            Self::Unknown { .. } => false,
            Self::External(_) => true,
        }
    }

    /// Get the error category for logging and metrics
    pub fn category(&self) -> &'static str {
        match self {
            Self::Io(_) => "io",
            Self::ImageProcessing { .. } => "image_processing",
            Self::Gui { .. } => "gui",
            Self::FileFormat { .. } => "file_format",
            Self::Config { .. } => "config",
            Self::Plugin { .. } => "plugin",
            Self::Rendering { .. } => "rendering",
            Self::Tool { .. } => "tool",
            Self::Memory { .. } => "memory",
            Self::Validation { .. } => "validation",
            Self::Network { .. } => "network",
            Self::Permission { .. } => "permission",
            Self::NotFound { .. } => "not_found",
            Self::Timeout { .. } => "timeout",
            Self::Application { .. } => "application",
            Self::Unknown { .. } => "unknown",
            Self::External(_) => "external",
        }
    }
}
