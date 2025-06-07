//! Built-in image adjustments
//!
//! This module contains implementations of common image adjustments
//! that can be applied to images in PSOC.

pub mod brightness;
pub mod color_balance;
pub mod contrast;
pub mod grayscale;
pub mod hsl;

// Re-export commonly used adjustments
pub use brightness::*;
pub use color_balance::*;
pub use contrast::*;
pub use grayscale::*;
pub use hsl::*;
