//! Utility modules

pub mod error;
pub mod logging;

pub use error::{PsocError, Result, ContextResult};
pub use logging::{
    init_default_logging, init_env_logging, init_logging, log_error_with_context,
    log_performance, LogConfig, LogFormat, LogLevel,
};
