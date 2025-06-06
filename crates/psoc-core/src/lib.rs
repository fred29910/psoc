//! PSOC Core - Core data structures and algorithms
//!
//! This crate provides the fundamental data structures and algorithms for the PSOC image editor.
//! It includes document management, layer handling, pixel data representation, color management,
//! and geometric calculations.

pub mod color;
pub mod document;
pub mod geometry;
pub mod layer;
pub mod math;
pub mod pixel;
pub mod rendering;
pub mod selection;

// Re-export commonly used types
pub use color::*;
pub use document::*;
pub use geometry::*;
pub use layer::*;
pub use math::*;
pub use pixel::*;
pub use rendering::*;
pub use selection::*;
