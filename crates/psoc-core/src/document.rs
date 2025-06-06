//! Document data structures and operations
//!
//! This module defines the document structure for the PSOC image editor,
//! including document metadata, layer management, and document operations.

use crate::color::ColorSpace;
use crate::geometry::{Rect, Size};
use crate::layer::Layer;
use crate::pixel::{PixelData, RgbaPixel};
use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Document color mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorMode {
    /// RGB color mode
    Rgb,
    /// RGBA color mode (with alpha channel)
    Rgba,
    /// Grayscale color mode
    Grayscale,
    /// CMYK color mode (for print)
    Cmyk,
}

impl Default for ColorMode {
    fn default() -> Self {
        Self::Rgba
    }
}

impl ColorMode {
    /// Get number of channels for this color mode
    pub fn channels(&self) -> u8 {
        match self {
            ColorMode::Rgb => 3,
            ColorMode::Rgba => 4,
            ColorMode::Grayscale => 1,
            ColorMode::Cmyk => 4,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            ColorMode::Rgb => "RGB",
            ColorMode::Rgba => "RGBA",
            ColorMode::Grayscale => "Grayscale",
            ColorMode::Cmyk => "CMYK",
        }
    }
}

/// Document resolution information
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Resolution {
    /// Horizontal resolution in pixels per inch
    pub x_ppi: f32,
    /// Vertical resolution in pixels per inch
    pub y_ppi: f32,
}

impl Resolution {
    /// Create new resolution
    pub fn new(x_ppi: f32, y_ppi: f32) -> Self {
        Self { x_ppi, y_ppi }
    }

    /// Create square resolution (same for both axes)
    pub fn square(ppi: f32) -> Self {
        Self::new(ppi, ppi)
    }

    /// Standard screen resolution (72 PPI)
    pub fn screen() -> Self {
        Self::square(72.0)
    }

    /// Standard print resolution (300 PPI)
    pub fn print() -> Self {
        Self::square(300.0)
    }

    /// High quality print resolution (600 PPI)
    pub fn high_quality_print() -> Self {
        Self::square(600.0)
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self::screen()
    }
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document title
    pub title: String,
    /// Author name
    pub author: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Keywords/tags
    pub keywords: Vec<String>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Application version that created this document
    pub created_with_version: String,
    /// Custom metadata fields
    pub custom_fields: HashMap<String, String>,
}

impl DocumentMetadata {
    /// Create new metadata
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            title,
            author: None,
            description: None,
            keywords: Vec::new(),
            created_at: now,
            modified_at: now,
            created_with_version: env!("CARGO_PKG_VERSION").to_string(),
            custom_fields: HashMap::new(),
        }
    }

    /// Update modification timestamp
    pub fn touch(&mut self) {
        self.modified_at = chrono::Utc::now();
    }
}

/// Document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document identifier
    pub id: Uuid,
    /// Document metadata
    pub metadata: DocumentMetadata,
    /// Document dimensions in pixels
    pub size: Size,
    /// Document resolution
    pub resolution: Resolution,
    /// Color mode
    pub color_mode: ColorMode,
    /// Color space
    pub color_space: ColorSpace,
    /// Background color
    pub background_color: RgbaPixel,
    /// Document layers (ordered from bottom to top)
    pub layers: Vec<Layer>,
    /// Active layer index
    pub active_layer_index: Option<usize>,
    /// Document canvas bounds
    pub canvas_bounds: Rect,
    /// Whether document has unsaved changes
    pub is_dirty: bool,
    /// File path (if document was loaded from or saved to a file)
    pub file_path: Option<std::path::PathBuf>,
}

impl Document {
    /// Create a new empty document
    pub fn new(title: String, width: u32, height: u32) -> Self {
        let id = Uuid::new_v4();
        let metadata = DocumentMetadata::new(title);
        let size = Size::new(width as f32, height as f32);
        let canvas_bounds = Rect::new(0.0, 0.0, width as f32, height as f32);

        Self {
            id,
            metadata,
            size,
            resolution: Resolution::default(),
            color_mode: ColorMode::default(),
            color_space: ColorSpace::default(),
            background_color: RgbaPixel::white(),
            layers: Vec::new(),
            active_layer_index: None,
            canvas_bounds,
            is_dirty: false,
            file_path: None,
        }
    }

    /// Create document from image
    pub fn from_image(title: String, image: &DynamicImage) -> Result<Self> {
        let (width, height) = image.dimensions();
        let mut document = Self::new(title, width, height);

        // Create a layer from the image
        let pixel_data =
            PixelData::from_image(image).context("Failed to create pixel data from image")?;

        let mut layer = Layer::new_pixel("Background".to_string(), width, height);
        layer.pixel_data = Some(pixel_data);

        document.add_layer(layer);
        document.set_active_layer(0)?;

        Ok(document)
    }

    /// Get document dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.size.width as u32, self.size.height as u32)
    }

    /// Add a new layer
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        self.mark_dirty();
    }

    /// Insert layer at specific index
    pub fn insert_layer(&mut self, index: usize, layer: Layer) -> Result<()> {
        if index > self.layers.len() {
            return Err(anyhow::anyhow!("Layer index out of bounds"));
        }

        self.layers.insert(index, layer);

        // Adjust active layer index if necessary
        if let Some(active_index) = self.active_layer_index {
            if index <= active_index {
                self.active_layer_index = Some(active_index + 1);
            }
        }

        self.mark_dirty();
        Ok(())
    }

    /// Remove layer at index
    pub fn remove_layer(&mut self, index: usize) -> Result<Layer> {
        if index >= self.layers.len() {
            return Err(anyhow::anyhow!("Layer index out of bounds"));
        }

        let removed_layer = self.layers.remove(index);

        // Adjust active layer index
        match self.active_layer_index {
            Some(active_index) if active_index == index => {
                // Removed the active layer
                if self.layers.is_empty() {
                    self.active_layer_index = None;
                } else if index >= self.layers.len() {
                    self.active_layer_index = Some(self.layers.len() - 1);
                }
                // else keep the same index (which now points to the next layer)
            }
            Some(active_index) if active_index > index => {
                // Active layer was above the removed layer
                self.active_layer_index = Some(active_index - 1);
            }
            _ => {
                // Active layer was below the removed layer or no active layer
                // No change needed
            }
        }

        self.mark_dirty();
        Ok(removed_layer)
    }

    /// Move layer from one index to another
    pub fn move_layer(&mut self, from_index: usize, to_index: usize) -> Result<()> {
        if from_index >= self.layers.len() || to_index >= self.layers.len() {
            return Err(anyhow::anyhow!("Layer index out of bounds"));
        }

        if from_index == to_index {
            return Ok(());
        }

        let layer = self.layers.remove(from_index);
        self.layers.insert(to_index, layer);

        // Update active layer index
        if let Some(active_index) = self.active_layer_index {
            self.active_layer_index = Some(match active_index {
                i if i == from_index => to_index,
                i if from_index < to_index && i > from_index && i <= to_index => i - 1,
                i if from_index > to_index && i >= to_index && i < from_index => i + 1,
                i => i,
            });
        }

        self.mark_dirty();
        Ok(())
    }

    /// Get layer by index
    pub fn get_layer(&self, index: usize) -> Option<&Layer> {
        self.layers.get(index)
    }

    /// Get mutable layer by index
    pub fn get_layer_mut(&mut self, index: usize) -> Option<&mut Layer> {
        if index < self.layers.len() {
            self.mark_dirty();
        }
        self.layers.get_mut(index)
    }

    /// Get active layer
    pub fn active_layer(&self) -> Option<&Layer> {
        self.active_layer_index
            .and_then(|index| self.get_layer(index))
    }

    /// Get mutable active layer
    pub fn active_layer_mut(&mut self) -> Option<&mut Layer> {
        if let Some(index) = self.active_layer_index {
            self.get_layer_mut(index)
        } else {
            None
        }
    }

    /// Set active layer by index
    pub fn set_active_layer(&mut self, index: usize) -> Result<()> {
        if index >= self.layers.len() {
            return Err(anyhow::anyhow!("Layer index out of bounds"));
        }

        self.active_layer_index = Some(index);
        Ok(())
    }

    /// Get number of layers
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Check if document is empty (no layers)
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    /// Mark document as dirty (has unsaved changes)
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
        self.metadata.touch();
    }

    /// Mark document as clean (no unsaved changes)
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    /// Flatten document to a single image
    pub fn flatten(&self) -> Result<DynamicImage> {
        if self.layers.is_empty() {
            // Create empty image with background color
            let mut pixel_data =
                PixelData::new_rgba(self.size.width as u32, self.size.height as u32);
            pixel_data.fill(self.background_color);
            return pixel_data.to_image();
        }

        // Start with background
        let mut result = PixelData::new_rgba(self.size.width as u32, self.size.height as u32);
        result.fill(self.background_color);

        // Composite layers from bottom to top
        for layer in &self.layers {
            if !layer.is_effectively_visible() {
                continue;
            }

            if let Some(layer_data) = &layer.pixel_data {
                self.composite_layer_onto(&mut result, layer, layer_data)?;
            }
        }

        result.to_image()
    }

    /// Composite a layer onto the result image
    fn composite_layer_onto(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        layer_data: &PixelData,
    ) -> Result<()> {
        let (layer_width, layer_height) = layer_data.dimensions();
        let (doc_width, doc_height) = self.dimensions();

        let offset_x = layer.offset.x as i32;
        let offset_y = layer.offset.y as i32;

        for y in 0..layer_height {
            for x in 0..layer_width {
                let doc_x = x as i32 + offset_x;
                let doc_y = y as i32 + offset_y;

                // Check bounds
                if doc_x < 0 || doc_y < 0 || doc_x >= doc_width as i32 || doc_y >= doc_height as i32
                {
                    continue;
                }

                if let Some(layer_pixel) = layer_data.get_pixel(x, y) {
                    if let Some(base_pixel) = result.get_pixel(doc_x as u32, doc_y as u32) {
                        let blended = layer.blend_mode.blend(
                            base_pixel,
                            layer_pixel,
                            layer.effective_opacity(),
                        );
                        result.set_pixel(doc_x as u32, doc_y as u32, blended)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Resize document
    pub fn resize(&mut self, new_width: u32, new_height: u32) -> Result<()> {
        self.size = Size::new(new_width as f32, new_height as f32);
        self.canvas_bounds = Rect::new(0.0, 0.0, new_width as f32, new_height as f32);

        // TODO: Implement layer resizing/cropping logic

        self.mark_dirty();
        Ok(())
    }

    /// Duplicate layer
    pub fn duplicate_layer(&mut self, index: usize) -> Result<()> {
        let layer = self
            .get_layer(index)
            .ok_or_else(|| anyhow::anyhow!("Layer index out of bounds"))?
            .duplicate();

        self.insert_layer(index + 1, layer)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new("Test Document".to_string(), 800, 600);

        assert_eq!(doc.metadata.title, "Test Document");
        assert_eq!(doc.dimensions(), (800, 600));
        assert!(doc.is_empty());
        assert!(!doc.is_dirty);
    }

    #[test]
    fn test_layer_management() {
        let mut doc = Document::new("Test".to_string(), 100, 100);
        let layer1 = Layer::new_pixel("Layer 1".to_string(), 100, 100);
        let layer2 = Layer::new_pixel("Layer 2".to_string(), 100, 100);

        doc.add_layer(layer1);
        doc.add_layer(layer2);

        assert_eq!(doc.layer_count(), 2);
        assert!(!doc.is_empty());
        assert!(doc.is_dirty);

        doc.set_active_layer(1).unwrap();
        assert_eq!(doc.active_layer().unwrap().name, "Layer 2");
    }

    #[test]
    fn test_layer_removal() {
        let mut doc = Document::new("Test".to_string(), 100, 100);
        let layer1 = Layer::new_pixel("Layer 1".to_string(), 100, 100);
        let layer2 = Layer::new_pixel("Layer 2".to_string(), 100, 100);

        doc.add_layer(layer1);
        doc.add_layer(layer2);
        doc.set_active_layer(1).unwrap();

        let removed = doc.remove_layer(0).unwrap();
        assert_eq!(removed.name, "Layer 1");
        assert_eq!(doc.layer_count(), 1);
        assert_eq!(doc.active_layer_index, Some(0));
    }
}
