//! Document data structures and operations
//!
//! This module defines the document structure for the PSOC image editor,
//! including document metadata, layer management, and document operations.

use crate::color::ColorSpace as DocumentColorSpace;
use crate::command::CommandHistory;
use crate::geometry::{Point, Rect, Size};
use crate::icc::IccProfile;
use crate::layer::Layer;
use crate::pixel::{PixelData, RgbaPixel};
use crate::selection::Selection;
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
    pub color_space: DocumentColorSpace,
    /// ICC color profile (optional)
    pub icc_profile: Option<IccProfile>,
    /// Background color
    pub background_color: RgbaPixel,
    /// Document layers (ordered from bottom to top)
    pub layers: Vec<Layer>,
    /// Active layer index
    pub active_layer_index: Option<usize>,
    /// Document canvas bounds
    pub canvas_bounds: Rect,
    /// Current selection
    pub selection: Selection,
    /// Whether document has unsaved changes
    pub is_dirty: bool,
    /// File path (if document was loaded from or saved to a file)
    pub file_path: Option<std::path::PathBuf>,
    /// Command history for undo/redo operations
    #[serde(skip)]
    pub command_history: CommandHistory,
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
            color_space: DocumentColorSpace::default(),
            icc_profile: None,
            background_color: RgbaPixel::white(),
            layers: Vec::new(),
            active_layer_index: None,
            canvas_bounds,
            selection: Selection::default(),
            is_dirty: false,
            file_path: None,
            command_history: CommandHistory::new(),
        }
    }

    /// Create document from image
    pub fn from_image(title: String, image: &DynamicImage) -> Result<Self> {
        Self::from_image_with_profile(title, image, None)
    }

    /// Create document from image with ICC profile
    pub fn from_image_with_profile(
        title: String,
        image: &DynamicImage,
        icc_profile: Option<IccProfile>,
    ) -> Result<Self> {
        let (width, height) = image.dimensions();
        let mut document = Self::new(title, width, height);

        // Set ICC profile if provided
        document.icc_profile = icc_profile;

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
        self.flatten_with_render_engine(&crate::rendering::RenderEngine::new())
    }

    /// Flatten document using a specific render engine
    pub fn flatten_with_render_engine(
        &self,
        render_engine: &crate::rendering::RenderEngine,
    ) -> Result<DynamicImage> {
        let pixel_data = render_engine.render_document(self)?;
        pixel_data.to_image()
    }

    /// Render a specific region of the document
    pub fn render_region(&self, x: u32, y: u32, width: u32, height: u32) -> Result<PixelData> {
        let render_engine = crate::rendering::RenderEngine::new();
        render_engine.render_region(self, x, y, width, height)
    }

    /// Composite a layer in a specific region
    #[allow(dead_code, clippy::too_many_arguments)]
    fn composite_layer_region(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        layer_data: &PixelData,
        region_x: u32,
        region_y: u32,
        region_width: u32,
        region_height: u32,
    ) -> Result<()> {
        let (layer_width, layer_height) = layer_data.dimensions();
        let offset_x = layer.offset.x as i32;
        let offset_y = layer.offset.y as i32;

        for y in 0..region_height {
            for x in 0..region_width {
                let doc_x = region_x + x;
                let doc_y = region_y + y;

                let layer_x = doc_x as i32 - offset_x;
                let layer_y = doc_y as i32 - offset_y;

                if layer_x < 0
                    || layer_y < 0
                    || layer_x >= layer_width as i32
                    || layer_y >= layer_height as i32
                {
                    continue;
                }

                if let Some(layer_pixel) = layer_data.get_pixel(layer_x as u32, layer_y as u32) {
                    if let Some(base_pixel) = result.get_pixel(x, y) {
                        let blended = layer.blend_mode.blend(
                            base_pixel,
                            layer_pixel,
                            layer.effective_opacity(),
                        );
                        result.set_pixel(x, y, blended)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Composite a layer onto the result image
    #[allow(dead_code)]
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

    /// Set the current selection
    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
        self.mark_dirty();
    }

    /// Execute a command and add it to the history
    pub fn execute_command(&mut self, command: Box<dyn crate::Command>) -> Result<()> {
        // Execute the command first
        command.execute(self)?;

        // Add to history (we'll implement a simpler version for now)
        // TODO: Implement proper command history with undo/redo
        Ok(())
    }

    /// Undo the last command
    pub fn undo(&mut self) -> Result<bool> {
        // TODO: Implement proper undo functionality
        Ok(false)
    }

    /// Redo the last undone command
    pub fn redo(&mut self) -> Result<bool> {
        // TODO: Implement proper redo functionality
        Ok(false)
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.command_history.can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.command_history.can_redo()
    }

    /// Get the description of the next command that would be undone
    pub fn undo_description(&self) -> Option<&str> {
        self.command_history.undo_description()
    }

    /// Get the description of the next command that would be redone
    pub fn redo_description(&self) -> Option<&str> {
        self.command_history.redo_description()
    }

    /// Clear command history
    pub fn clear_history(&mut self) {
        self.command_history.clear();
    }

    /// Get command history statistics
    pub fn history_stats(&self) -> (usize, usize) {
        (
            self.command_history.undo_count(),
            self.command_history.redo_count(),
        )
    }

    /// Get the current selection
    pub fn get_selection(&self) -> &Selection {
        &self.selection
    }

    /// Navigate to a specific position in command history
    pub fn navigate_to_history_position(&mut self, position: usize) -> Result<bool> {
        if let Some(direction) = self.command_history.should_navigate_to_position(position) {
            match direction {
                crate::NavigationDirection::Backward(steps) => {
                    for _ in 0..steps {
                        if !self.undo()? {
                            break;
                        }
                    }
                }
                crate::NavigationDirection::Forward(steps) => {
                    for _ in 0..steps {
                        if !self.redo()? {
                            break;
                        }
                    }
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clear the current selection (select all)
    pub fn clear_selection(&mut self) {
        self.selection = Selection::None;
        self.mark_dirty();
    }

    /// Check if there is an active selection
    pub fn has_selection(&self) -> bool {
        !self.selection.is_select_all()
    }

    /// Check if a point is within the current selection
    pub fn is_point_selected(&self, point: Point) -> bool {
        self.selection.contains_point(point)
    }

    /// Get the bounds of the current selection
    pub fn selection_bounds(&self) -> Option<Rect> {
        self.selection.bounds()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layer::BlendMode;

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

    #[test]
    fn test_document_layer_vector_integration() {
        let mut doc = Document::new("Test".to_string(), 100, 100);

        // Test empty document
        assert!(doc.layers.is_empty());
        assert_eq!(doc.layers.len(), 0);
        assert!(doc.is_empty());

        // Add layers to Vec<Layer>
        let layer1 = Layer::new_pixel("Background".to_string(), 100, 100);
        let layer2 = Layer::new_pixel("Foreground".to_string(), 100, 100);

        doc.add_layer(layer1);
        doc.add_layer(layer2);

        // Test Vec<Layer> properties
        assert_eq!(doc.layers.len(), 2);
        assert!(!doc.is_empty());
        assert_eq!(doc.layer_count(), 2);

        // Test layer access through Vec
        assert_eq!(doc.layers[0].name, "Background");
        assert_eq!(doc.layers[1].name, "Foreground");

        // Test layer ordering (bottom to top)
        assert_eq!(doc.get_layer(0).unwrap().name, "Background");
        assert_eq!(doc.get_layer(1).unwrap().name, "Foreground");
    }

    #[test]
    fn test_layer_properties_in_document() {
        let mut doc = Document::new("Test".to_string(), 100, 100);

        // Create layer with specific properties
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        layer.visible = true;
        layer.opacity = 0.8;
        layer.blend_mode = BlendMode::Normal;

        doc.add_layer(layer);
        doc.set_active_layer(0).unwrap();

        let active_layer = doc.active_layer().unwrap();

        // Verify P2.1 requirements are met
        assert!(active_layer.pixel_data.is_some()); // Has pixel data
        assert!(active_layer.visible); // Has visibility
        assert_eq!(active_layer.opacity, 0.8); // Has opacity
        assert_eq!(active_layer.blend_mode, BlendMode::Normal); // Has blend mode (Normal)
    }

    #[test]
    fn test_layer_blend_mode_normal_only() {
        let mut doc = Document::new("Test".to_string(), 100, 100);

        // Create layers with Normal blend mode (P2.1 requirement: initially only Normal)
        let layer1 = Layer::new_pixel("Layer 1".to_string(), 100, 100);
        let layer2 = Layer::new_pixel("Layer 2".to_string(), 100, 100);

        // Verify default blend mode is Normal
        assert_eq!(layer1.blend_mode, BlendMode::Normal);
        assert_eq!(layer2.blend_mode, BlendMode::Normal);

        doc.add_layer(layer1);
        doc.add_layer(layer2);

        // Verify all layers in document use Normal blend mode
        for layer in &doc.layers {
            assert_eq!(layer.blend_mode, BlendMode::Normal);
        }
    }

    #[test]
    fn test_document_selection_management() {
        let mut doc = Document::new("Test".to_string(), 100, 100);

        // Initially no selection (select all)
        assert!(doc.get_selection().is_select_all());
        assert!(!doc.has_selection());

        // Set a rectangular selection
        let selection = Selection::rectangle(10.0, 20.0, 50.0, 30.0);
        doc.set_selection(selection.clone());

        assert!(!doc.get_selection().is_select_all());
        assert!(doc.has_selection());
        assert!(doc.is_dirty);

        // Check selection bounds
        let bounds = doc.selection_bounds().unwrap();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 50.0);
        assert_eq!(bounds.height, 30.0);

        // Test point selection
        assert!(doc.is_point_selected(Point::new(30.0, 35.0))); // Inside
        assert!(!doc.is_point_selected(Point::new(5.0, 35.0))); // Outside

        // Clear selection
        doc.clear_selection();
        assert!(doc.get_selection().is_select_all());
        assert!(!doc.has_selection());
    }

    #[test]
    fn test_document_icc_profile() {
        let mut doc = Document::new("Test".to_string(), 100, 100);

        // Initially no ICC profile
        assert!(doc.icc_profile.is_none());

        // Test that we can set an ICC profile (even if None for now)
        doc.icc_profile = None;
        assert!(doc.icc_profile.is_none());
    }

    #[test]
    fn test_document_from_image_with_profile() {
        use image::{ImageBuffer, Rgb};

        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        // Test without profile
        let doc1 =
            Document::from_image_with_profile("Test1".to_string(), &dynamic_img, None).unwrap();
        assert!(doc1.icc_profile.is_none());
        assert_eq!(doc1.layers.len(), 1);
        assert_eq!(doc1.dimensions(), (50, 50));

        // Test with profile (None for now, but structure is there)
        let doc2 =
            Document::from_image_with_profile("Test2".to_string(), &dynamic_img, None).unwrap();
        assert!(doc2.icc_profile.is_none());
        assert_eq!(doc2.layers.len(), 1);
    }

    #[test]
    fn test_document_from_image_compatibility() {
        use image::{ImageBuffer, Rgb};

        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(30, 30);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        // Test that old method still works
        let doc1 = Document::from_image("Test1".to_string(), &dynamic_img).unwrap();

        // Test that new method with None profile produces same result
        let doc2 =
            Document::from_image_with_profile("Test2".to_string(), &dynamic_img, None).unwrap();

        assert_eq!(doc1.dimensions(), doc2.dimensions());
        assert_eq!(doc1.layers.len(), doc2.layers.len());
        assert!(doc1.icc_profile.is_none());
        assert!(doc2.icc_profile.is_none());
    }
}
