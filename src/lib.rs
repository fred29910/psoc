//! PSOC - A Photoshop-like image editor built with Rust
//!
//! This crate provides the core functionality for a modern image editing application
//! with features similar to Adobe Photoshop, built using Rust for performance and safety.

pub mod app;
pub mod commands;
pub mod core;
pub mod file_io;
pub mod i18n;
pub mod image_processing;
pub mod rendering;
pub mod shortcuts;
pub mod tools;
#[cfg(feature = "gui")]
pub mod ui;
pub mod utils;

#[cfg(feature = "plugins")]
pub mod plugins;

// Re-export commonly used types
pub use app::{AppConfig, Application};
pub use core::{Document, Layer};
pub use i18n::{init_localization, t, t_with_args, Language, LocalizationManager};
pub use tools::{Tool, ToolManager, ToolType};
#[cfg(feature = "gui")]
pub use ui::{AppState, ImageCanvas, ImageData, Message, PsocApp};
pub use utils::{
    error::{ContextResult, PsocError, Result},
    logging::{init_default_logging, init_env_logging, LogConfig, LogFormat, LogLevel},
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        #[allow(clippy::const_is_empty)]
        {
            assert!(!VERSION.is_empty());
            assert!(!DESCRIPTION.is_empty());
        }
        assert_eq!(NAME, "psoc");
    }
}
