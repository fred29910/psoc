//! PSOC Core - Core data structures and algorithms
//!
//! This crate provides the fundamental data structures and algorithms for the PSOC image editor.
//! It includes document management, layer handling, pixel data representation, color management,
//! geometric calculations, and command system for undo/redo functionality.

pub mod adjustment;
pub mod adjustments;
pub mod color;
pub mod command;
pub mod document;
pub mod geometry;
pub mod gradient;
pub mod icc;
pub mod layer;
pub mod math;
pub mod pixel;
pub mod rendering;
pub mod selection;

// Re-export commonly used types
pub use adjustment::*;
pub use adjustments::*;
pub use color::{ColorAdjustment, ColorConverter, HslColor, HsvColor};
pub use command::*;
pub use document::*;
pub use geometry::*;
pub use gradient::*;
pub use icc::{CmsConfig, ColorManager, IccProfile, RenderingIntent};
pub use layer::*;
pub use math::*;
pub use pixel::*;
pub use rendering::*;
pub use selection::*;

// Re-export color space from color module to avoid conflicts
pub use color::ColorSpace as DocumentColorSpace;
