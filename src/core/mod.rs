//! Core data structures and logic
//!
//! This module provides high-level interfaces to the core data structures
//! and re-exports commonly used types from the psoc-core crate.

// Re-export core types for convenience
pub use psoc_core::{
    Document, DocumentMetadata, Layer, LayerType, BlendMode,
    PixelData, RgbaPixel, ColorSpace, ColorMode, Resolution,
    Point, Size, Rect, Transform, HslColor, HsvColor, ColorAdjustment,
};

/// Create a new document with default settings
pub fn create_document(title: String, width: u32, height: u32) -> Document {
    Document::new(title, width, height)
}

/// Create a document from an image
pub fn create_document_from_image(title: String, image: &image::DynamicImage) -> anyhow::Result<Document> {
    Document::from_image(title, image)
}

/// Create a new pixel layer
pub fn create_pixel_layer(name: String, width: u32, height: u32) -> Layer {
    Layer::new_pixel(name, width, height)
}

/// Create a new text layer
pub fn create_text_layer(
    name: String,
    content: String,
    font_family: String,
    font_size: f32,
    color: RgbaPixel,
    position: Point,
) -> Layer {
    Layer::new_text(name, content, font_family, font_size, color, position)
}

/// Utility functions for common operations
pub mod utils {
    use super::*;

    /// Create a standard document for web use (72 PPI)
    pub fn create_web_document(title: String, width: u32, height: u32) -> Document {
        let mut doc = Document::new(title, width, height);
        doc.resolution = Resolution::screen();
        doc
    }

    /// Create a standard document for print use (300 PPI)
    pub fn create_print_document(title: String, width: u32, height: u32) -> Document {
        let mut doc = Document::new(title, width, height);
        doc.resolution = Resolution::print();
        doc
    }

    /// Create a background layer filled with white
    pub fn create_white_background(width: u32, height: u32) -> Layer {
        let mut layer = Layer::new_pixel("Background".to_string(), width, height);
        layer.fill(RgbaPixel::white());
        layer
    }

    /// Create a background layer filled with transparent
    pub fn create_transparent_background(width: u32, height: u32) -> Layer {
        let mut layer = Layer::new_pixel("Background".to_string(), width, height);
        layer.fill(RgbaPixel::transparent());
        layer
    }
}
