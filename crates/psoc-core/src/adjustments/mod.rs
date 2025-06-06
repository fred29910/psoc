//! Built-in image adjustments
//!
//! This module contains implementations of common image adjustments
//! that can be applied to images in PSOC.

pub mod brightness;
pub mod contrast;

// Re-export commonly used adjustments
pub use brightness::*;
pub use contrast::*;
