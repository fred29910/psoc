//! PSOC - A Photoshop-like image editor built with Rust
//!
//! This crate provides the core functionality for a modern image editing application
//! with features similar to Adobe Photoshop, built using Rust for performance and safety.

pub mod app;
pub mod core;
pub mod file_io;
pub mod image_processing;
pub mod rendering;
pub mod tools;
pub mod ui;
pub mod utils;

#[cfg(feature = "plugins")]
pub mod plugins;

// Re-export commonly used types
pub use app::Application;
pub use core::{Document, Layer};
pub use utils::error::PsocError;

/// Result type used throughout the application
pub type Result<T> = std::result::Result<T, PsocError>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert_eq!(NAME, "psoc");
        assert!(!DESCRIPTION.is_empty());
    }
}
