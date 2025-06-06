//! Image adjustment and filter framework
//!
//! This module provides the core framework for applying adjustments and filters
//! to images in PSOC. It includes:
//! - Core traits for adjustments and filters
//! - Application scope management (layer vs selection)
//! - Integration with the command system for undo/redo
//! - Parameter validation and type safety

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use crate::{Document, PixelData, RgbaPixel, Selection};

/// Core trait for all image adjustments and filters
///
/// This trait defines the interface that all adjustments and filters must implement.
/// Adjustments modify pixel values (brightness, contrast, etc.) while filters
/// apply convolution or other spatial operations (blur, sharpen, etc.).
pub trait Adjustment: Debug + Send + Sync {
    /// Get a unique identifier for this adjustment type
    fn id(&self) -> &'static str;

    /// Get a human-readable name for this adjustment
    fn name(&self) -> &'static str;

    /// Get a description of what this adjustment does
    fn description(&self) -> &'static str;

    /// Apply the adjustment to pixel data
    ///
    /// This method applies the adjustment to the provided pixel data.
    /// The implementation should be pure (no side effects) and thread-safe.
    fn apply(&self, pixel_data: &mut PixelData) -> Result<()>;

    /// Apply the adjustment to a single pixel
    ///
    /// This is used for preview purposes and fine-grained control.
    /// Default implementation applies to a 1x1 pixel data.
    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        let mut data = PixelData::new_rgba(1, 1);
        data.set_pixel(0, 0, pixel)?;
        self.apply(&mut data)?;
        data.get_pixel(0, 0)
            .ok_or_else(|| anyhow::anyhow!("Failed to get pixel"))
    }

    /// Check if this adjustment would modify the given pixel
    ///
    /// This can be used for optimization - if an adjustment wouldn't
    /// change a pixel, we can skip processing it.
    fn would_modify_pixel(&self, pixel: RgbaPixel) -> bool {
        // Default implementation: assume all adjustments modify pixels
        // Specific adjustments can override this for optimization
        let _ = pixel;
        true
    }

    /// Get the parameters of this adjustment as a serializable value
    ///
    /// This is used for saving adjustment settings and undo/redo.
    fn get_parameters(&self) -> serde_json::Value;

    /// Set the parameters of this adjustment from a serializable value
    ///
    /// This is used for loading adjustment settings and undo/redo.
    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()>;

    /// Clone this adjustment
    ///
    /// Since we can't use Clone trait with trait objects, we provide
    /// this method for cloning adjustments.
    fn clone_adjustment(&self) -> Box<dyn Adjustment>;
}

/// Defines the scope where an adjustment should be applied
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdjustmentScope {
    /// Apply to the entire active layer
    EntireLayer,
    /// Apply only to the current selection
    Selection,
    /// Apply to a specific rectangular region
    Region {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
}

impl Default for AdjustmentScope {
    fn default() -> Self {
        Self::EntireLayer
    }
}

impl AdjustmentScope {
    /// Check if this scope includes the given pixel coordinates
    pub fn contains_point(&self, x: u32, y: u32, selection: Option<&Selection>) -> bool {
        match self {
            Self::EntireLayer => true,
            Self::Selection => {
                if let Some(selection) = selection {
                    use crate::geometry::Point;
                    selection.contains_point(Point::new(x as f32, y as f32))
                } else {
                    false
                }
            }
            Self::Region {
                x: rx,
                y: ry,
                width,
                height,
            } => x >= *rx && x < rx + width && y >= *ry && y < ry + height,
        }
    }

    /// Get the bounding rectangle of this scope
    pub fn get_bounds(
        &self,
        layer_width: u32,
        layer_height: u32,
        selection: Option<&Selection>,
    ) -> Option<(u32, u32, u32, u32)> {
        match self {
            Self::EntireLayer => Some((0, 0, layer_width, layer_height)),
            Self::Selection => {
                if let Some(selection) = selection {
                    selection.bounds().map(|rect| {
                        (
                            rect.x.max(0.0) as u32,
                            rect.y.max(0.0) as u32,
                            rect.width.min(layer_width as f32) as u32,
                            rect.height.min(layer_height as f32) as u32,
                        )
                    })
                } else {
                    None
                }
            }
            Self::Region {
                x,
                y,
                width,
                height,
            } => {
                let clipped_width = (*width).min(layer_width.saturating_sub(*x));
                let clipped_height = (*height).min(layer_height.saturating_sub(*y));
                if clipped_width > 0 && clipped_height > 0 {
                    Some((*x, *y, clipped_width, clipped_height))
                } else {
                    None
                }
            }
        }
    }
}

/// Parameters for applying an adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustmentApplication {
    /// Unique identifier for this application
    pub id: Uuid,
    /// The adjustment to apply
    pub adjustment_id: String,
    /// Parameters for the adjustment
    pub parameters: serde_json::Value,
    /// Scope where the adjustment should be applied
    pub scope: AdjustmentScope,
    /// Target layer index
    pub layer_index: usize,
    /// Whether to create a new layer for the result
    pub create_new_layer: bool,
    /// Opacity of the adjustment (0.0 to 1.0)
    pub opacity: f32,
}

impl AdjustmentApplication {
    /// Create a new adjustment application
    pub fn new(
        adjustment_id: String,
        parameters: serde_json::Value,
        scope: AdjustmentScope,
        layer_index: usize,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            adjustment_id,
            parameters,
            scope,
            layer_index,
            create_new_layer: false,
            opacity: 1.0,
        }
    }

    /// Set whether to create a new layer
    pub fn with_new_layer(mut self, create_new_layer: bool) -> Self {
        self.create_new_layer = create_new_layer;
        self
    }

    /// Set the opacity of the adjustment
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

/// Registry for available adjustments
#[derive(Debug, Default)]
pub struct AdjustmentRegistry {
    adjustments: std::collections::HashMap<String, Box<dyn Adjustment>>,
}

impl AdjustmentRegistry {
    /// Create a new adjustment registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an adjustment
    pub fn register(&mut self, adjustment: Box<dyn Adjustment>) {
        let id = adjustment.id().to_string();
        self.adjustments.insert(id, adjustment);
    }

    /// Get an adjustment by ID
    pub fn get(&self, id: &str) -> Option<&dyn Adjustment> {
        self.adjustments.get(id).map(|adj| adj.as_ref())
    }

    /// Create a new instance of an adjustment by ID
    pub fn create(&self, id: &str) -> Option<Box<dyn Adjustment>> {
        self.adjustments.get(id).map(|adj| adj.clone_adjustment())
    }

    /// Get all available adjustment IDs
    pub fn list_ids(&self) -> Vec<String> {
        self.adjustments.keys().cloned().collect()
    }

    /// Get all available adjustments with their metadata
    pub fn list_adjustments(&self) -> Vec<(&str, &str, &str)> {
        self.adjustments
            .values()
            .map(|adj| (adj.id(), adj.name(), adj.description()))
            .collect()
    }
}

/// Apply an adjustment to a document
pub fn apply_adjustment_to_document(
    document: &mut Document,
    application: &AdjustmentApplication,
    registry: &AdjustmentRegistry,
) -> Result<()> {
    // Get the adjustment from the registry
    let mut adjustment = registry
        .create(&application.adjustment_id)
        .ok_or_else(|| anyhow::anyhow!("Unknown adjustment: {}", application.adjustment_id))?;

    // Set the adjustment parameters
    adjustment.set_parameters(application.parameters.clone())?;

    // Clone the selection to avoid borrowing issues
    let selection_clone = document.selection.clone();

    // Get the target layer
    let layer = document
        .get_layer_mut(application.layer_index)
        .ok_or_else(|| anyhow::anyhow!("Layer index {} out of bounds", application.layer_index))?;

    // Get the layer's pixel data
    let pixel_data = layer
        .pixel_data
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("Layer has no pixel data"))?;

    // Apply the adjustment based on scope
    match &application.scope {
        AdjustmentScope::EntireLayer => {
            adjustment.apply(pixel_data)?;
        }
        AdjustmentScope::Selection | AdjustmentScope::Region { .. } => {
            apply_adjustment_with_scope(
                pixel_data,
                adjustment.as_ref(),
                &application.scope,
                Some(&selection_clone),
            )?;
        }
    }

    // Mark the document as dirty
    document.mark_dirty();

    Ok(())
}

/// Apply an adjustment with a specific scope
fn apply_adjustment_with_scope(
    pixel_data: &mut PixelData,
    adjustment: &dyn Adjustment,
    scope: &AdjustmentScope,
    selection: Option<&Selection>,
) -> Result<()> {
    let (width, height) = pixel_data.dimensions();

    // Get the bounds of the scope
    let bounds = scope.get_bounds(width, height, selection);
    if bounds.is_none() {
        return Ok(()); // Nothing to process
    }

    let (start_x, start_y, scope_width, scope_height) = bounds.unwrap();

    // Create a temporary pixel data for the affected region
    let mut temp_data = PixelData::new_rgba(scope_width, scope_height);

    // Copy the affected region to temp data
    for y in 0..scope_height {
        for x in 0..scope_width {
            let src_x = start_x + x;
            let src_y = start_y + y;
            if src_x < width && src_y < height {
                let pixel = pixel_data.get_pixel(src_x, src_y).ok_or_else(|| {
                    anyhow::anyhow!("Failed to get pixel at ({}, {})", src_x, src_y)
                })?;
                temp_data.set_pixel(x, y, pixel)?;
            }
        }
    }

    // Apply the adjustment to the temp data
    adjustment.apply(&mut temp_data)?;

    // Copy the result back, respecting the scope
    for y in 0..scope_height {
        for x in 0..scope_width {
            let dst_x = start_x + x;
            let dst_y = start_y + y;
            if dst_x < width && dst_y < height && scope.contains_point(dst_x, dst_y, selection) {
                let adjusted_pixel = temp_data.get_pixel(x, y).ok_or_else(|| {
                    anyhow::anyhow!("Failed to get adjusted pixel at ({}, {})", x, y)
                })?;
                pixel_data.set_pixel(dst_x, dst_y, adjusted_pixel)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Document, Layer, PixelData, RgbaPixel};

    // Mock adjustment for testing
    #[derive(Debug)]
    struct MockAdjustment {
        brightness: f32,
    }

    impl MockAdjustment {
        fn new(brightness: f32) -> Self {
            Self { brightness }
        }
    }

    impl Adjustment for MockAdjustment {
        fn id(&self) -> &'static str {
            "mock_brightness"
        }

        fn name(&self) -> &'static str {
            "Mock Brightness"
        }

        fn description(&self) -> &'static str {
            "A mock brightness adjustment for testing"
        }

        fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
            let (width, height) = pixel_data.dimensions();
            for y in 0..height {
                for x in 0..width {
                    let mut pixel = pixel_data
                        .get_pixel(x, y)
                        .ok_or_else(|| anyhow::anyhow!("Failed to get pixel at ({}, {})", x, y))?;
                    let brightness_factor = 1.0 + self.brightness;
                    pixel.r = ((pixel.r as f32 * brightness_factor).clamp(0.0, 255.0)) as u8;
                    pixel.g = ((pixel.g as f32 * brightness_factor).clamp(0.0, 255.0)) as u8;
                    pixel.b = ((pixel.b as f32 * brightness_factor).clamp(0.0, 255.0)) as u8;
                    pixel_data.set_pixel(x, y, pixel)?;
                }
            }
            Ok(())
        }

        fn get_parameters(&self) -> serde_json::Value {
            serde_json::json!({ "brightness": self.brightness })
        }

        fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
            if let Some(brightness) = parameters.get("brightness").and_then(|v| v.as_f64()) {
                self.brightness = brightness as f32;
                Ok(())
            } else {
                Err(anyhow::anyhow!("Invalid parameters for MockAdjustment"))
            }
        }

        fn clone_adjustment(&self) -> Box<dyn Adjustment> {
            Box::new(MockAdjustment::new(self.brightness))
        }
    }

    #[test]
    fn test_adjustment_scope_entire_layer() {
        let scope = AdjustmentScope::EntireLayer;
        assert!(scope.contains_point(0, 0, None));
        assert!(scope.contains_point(100, 100, None));

        let bounds = scope.get_bounds(200, 150, None);
        assert_eq!(bounds, Some((0, 0, 200, 150)));
    }

    #[test]
    fn test_adjustment_scope_region() {
        let scope = AdjustmentScope::Region {
            x: 10,
            y: 20,
            width: 50,
            height: 30,
        };

        assert!(scope.contains_point(10, 20, None));
        assert!(scope.contains_point(59, 49, None));
        assert!(!scope.contains_point(9, 20, None));
        assert!(!scope.contains_point(60, 20, None));

        let bounds = scope.get_bounds(200, 150, None);
        assert_eq!(bounds, Some((10, 20, 50, 30)));
    }

    #[test]
    fn test_adjustment_application_creation() {
        let params = serde_json::json!({ "brightness": 0.2 });
        let app = AdjustmentApplication::new(
            "mock_brightness".to_string(),
            params.clone(),
            AdjustmentScope::EntireLayer,
            0,
        );

        assert_eq!(app.adjustment_id, "mock_brightness");
        assert_eq!(app.parameters, params);
        assert_eq!(app.scope, AdjustmentScope::EntireLayer);
        assert_eq!(app.layer_index, 0);
        assert!(!app.create_new_layer);
        assert_eq!(app.opacity, 1.0);
    }

    #[test]
    fn test_adjustment_registry() {
        let mut registry = AdjustmentRegistry::new();
        let adjustment = Box::new(MockAdjustment::new(0.1));

        registry.register(adjustment);

        assert!(registry.get("mock_brightness").is_some());
        assert!(registry.get("nonexistent").is_none());

        let ids = registry.list_ids();
        assert!(ids.contains(&"mock_brightness".to_string()));

        let adjustments = registry.list_adjustments();
        assert_eq!(adjustments.len(), 1);
        assert_eq!(adjustments[0].0, "mock_brightness");
        assert_eq!(adjustments[0].1, "Mock Brightness");
    }

    #[test]
    fn test_mock_adjustment() {
        let mut adjustment = MockAdjustment::new(0.2);

        // Test parameter serialization
        let params = adjustment.get_parameters();
        assert!((params["brightness"].as_f64().unwrap() - 0.2).abs() < 1e-6);

        // Test parameter deserialization
        let new_params = serde_json::json!({ "brightness": 0.5 });
        adjustment.set_parameters(new_params).unwrap();
        assert_eq!(adjustment.brightness, 0.5);

        // Test pixel application
        let pixel = RgbaPixel::new(100, 100, 100, 255);
        let result = adjustment.apply_to_pixel(pixel).unwrap();
        assert!(result.r > pixel.r);
        assert!(result.g > pixel.g);
        assert!(result.b > pixel.b);
        assert_eq!(result.a, pixel.a);
    }

    #[test]
    fn test_apply_adjustment_to_document() {
        let mut document = Document::new("Test".to_string(), 10, 10);
        let mut pixel_data = PixelData::new_rgba(10, 10);
        pixel_data.fill(RgbaPixel::new(100, 100, 100, 255));

        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);
        layer.pixel_data = Some(pixel_data);
        document.add_layer(layer);

        let mut registry = AdjustmentRegistry::new();
        registry.register(Box::new(MockAdjustment::new(0.2)));

        let params = serde_json::json!({ "brightness": 0.2 });
        let application = AdjustmentApplication::new(
            "mock_brightness".to_string(),
            params,
            AdjustmentScope::EntireLayer,
            0,
        );

        apply_adjustment_to_document(&mut document, &application, &registry).unwrap();

        // Verify the adjustment was applied
        let layer = document.get_layer(0).unwrap();
        let pixel_data = layer.pixel_data.as_ref().unwrap();
        let pixel = pixel_data.get_pixel(0, 0).unwrap();
        assert!(pixel.r > 100); // Should be brighter
    }
}
