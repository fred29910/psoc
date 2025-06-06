//! Logging configuration and utilities for PSOC
//!
//! This module provides a comprehensive logging system using the `tracing` framework.
//! It supports structured logging, multiple output formats, and configurable log levels.

use std::env;
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Only error messages
    Error,
    /// Error and warning messages
    Warn,
    /// Error, warning, and info messages
    Info,
    /// Error, warning, info, and debug messages
    Debug,
    /// All messages including trace
    Trace,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }
    }
}

impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            LogLevel::Warn => tracing_subscriber::filter::LevelFilter::WARN,
            LogLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
            LogLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            LogLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
        }
    }
}

/// Log output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Human-readable format for development
    Pretty,
    /// Compact format for production
    Compact,
    /// JSON format for structured logging
    Json,
}

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Log level
    pub level: LogLevel,
    /// Output format
    pub format: LogFormat,
    /// Whether to include file and line information
    pub include_location: bool,
    /// Whether to include thread information
    pub include_thread: bool,
    /// Whether to include span information
    pub include_spans: bool,
    /// Custom filter for modules
    pub module_filter: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Pretty,
            include_location: true,
            include_thread: false,
            include_spans: true,
            module_filter: None,
        }
    }
}

impl LogConfig {
    /// Create a new log configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the log level
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the output format
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }

    /// Enable or disable location information
    pub fn with_location(mut self, include_location: bool) -> Self {
        self.include_location = include_location;
        self
    }

    /// Enable or disable thread information
    pub fn with_thread(mut self, include_thread: bool) -> Self {
        self.include_thread = include_thread;
        self
    }

    /// Enable or disable span information
    pub fn with_spans(mut self, include_spans: bool) -> Self {
        self.include_spans = include_spans;
        self
    }

    /// Set a custom module filter
    pub fn with_module_filter<S: Into<String>>(mut self, filter: S) -> Self {
        self.module_filter = Some(filter.into());
        self
    }

    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Set log level from environment
        if let Ok(level_str) = env::var("PSOC_LOG_LEVEL") {
            config.level = match level_str.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" | "warning" => LogLevel::Warn,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                "trace" => LogLevel::Trace,
                _ => LogLevel::Info,
            };
        }

        // Set format from environment
        if let Ok(format_str) = env::var("PSOC_LOG_FORMAT") {
            config.format = match format_str.to_lowercase().as_str() {
                "pretty" => LogFormat::Pretty,
                "compact" => LogFormat::Compact,
                "json" => LogFormat::Json,
                _ => LogFormat::Pretty,
            };
        }

        // Set location from environment
        if let Ok(location_str) = env::var("PSOC_LOG_LOCATION") {
            config.include_location = location_str.to_lowercase() == "true";
        }

        // Set thread from environment
        if let Ok(thread_str) = env::var("PSOC_LOG_THREAD") {
            config.include_thread = thread_str.to_lowercase() == "true";
        }

        // Set spans from environment
        if let Ok(spans_str) = env::var("PSOC_LOG_SPANS") {
            config.include_spans = spans_str.to_lowercase() == "true";
        }

        // Set module filter from environment
        if let Ok(filter) = env::var("PSOC_LOG_FILTER") {
            config.module_filter = Some(filter);
        }

        config
    }
}

/// Initialize the logging system with the given configuration
pub fn init_logging(config: LogConfig) -> crate::Result<()> {
    // Create the base filter
    let level_filter: tracing_subscriber::filter::LevelFilter = config.level.into();
    let mut filter = EnvFilter::from_default_env()
        .add_directive(level_filter.into())
        .add_directive("psoc=trace".parse().unwrap());

    // Add custom module filter if specified
    if let Some(module_filter) = &config.module_filter {
        filter = filter.add_directive(module_filter.parse().map_err(|e| {
            crate::PsocError::config("log_filter", format!("Invalid filter: {}", e))
        })?);
    }

    // Create the subscriber based on format
    match config.format {
        LogFormat::Pretty => {
            let fmt_layer = fmt::layer()
                .pretty()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_thread_ids(config.include_thread)
                .with_thread_names(config.include_thread)
                .with_span_events(if config.include_spans {
                    FmtSpan::ENTER | FmtSpan::EXIT
                } else {
                    FmtSpan::NONE
                });

            tracing_subscriber::registry()
                .with(filter)
                .with(fmt_layer)
                .init();
        }
        LogFormat::Compact => {
            let fmt_layer = fmt::layer()
                .compact()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_thread_ids(config.include_thread)
                .with_thread_names(config.include_thread)
                .with_span_events(if config.include_spans {
                    FmtSpan::ENTER | FmtSpan::EXIT
                } else {
                    FmtSpan::NONE
                });

            tracing_subscriber::registry()
                .with(filter)
                .with(fmt_layer)
                .init();
        }
        LogFormat::Json => {
            let fmt_layer = fmt::layer()
                .json()
                .with_file(config.include_location)
                .with_line_number(config.include_location)
                .with_thread_ids(config.include_thread)
                .with_thread_names(config.include_thread)
                .with_span_events(if config.include_spans {
                    FmtSpan::ENTER | FmtSpan::EXIT
                } else {
                    FmtSpan::NONE
                });

            tracing_subscriber::registry()
                .with(filter)
                .with(fmt_layer)
                .init();
        }
    }

    tracing::info!(
        level = ?config.level,
        format = ?config.format,
        location = config.include_location,
        thread = config.include_thread,
        spans = config.include_spans,
        "Logging system initialized"
    );

    Ok(())
}

/// Initialize logging with default configuration
pub fn init_default_logging() -> crate::Result<()> {
    init_logging(LogConfig::default())
}

/// Initialize logging from environment variables
pub fn init_env_logging() -> crate::Result<()> {
    init_logging(LogConfig::from_env())
}

/// Convenience macros for structured logging
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        tracing::trace!($($arg)*)
    };
}

/// Log an error with context
pub fn log_error_with_context<E: std::fmt::Display>(error: &E, context: &str) {
    tracing::error!(
        error = %error,
        context = context,
        "Error occurred"
    );
}

/// Log a performance measurement
pub fn log_performance<F, R>(operation: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();

    tracing::info!(
        operation = operation,
        duration_ms = duration.as_millis(),
        "Performance measurement"
    );

    result
}
