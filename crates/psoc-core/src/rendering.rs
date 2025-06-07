//! Rendering module for layer composition and image generation
//!
//! This module provides high-performance rendering capabilities for the PSOC image editor.
//! It handles layer composition, blend mode application, and optimized rendering pipelines.

use crate::{adjustment::AdjustmentRegistry, Document, Layer, LayerType, PixelData};
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use tracing::{debug, instrument, trace};

/// Rendering engine for layer composition and image generation
#[derive(Debug)]
pub struct RenderEngine {
    /// Enable parallel processing
    parallel_enabled: bool,
    /// Tile size for parallel processing
    tile_size: u32,
    /// Adjustment registry for applying adjustment layers
    adjustment_registry: AdjustmentRegistry,
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderEngine {
    /// Create a new render engine with default settings
    pub fn new() -> Self {
        let mut adjustment_registry = AdjustmentRegistry::new();
        adjustment_registry.register_default_adjustments();

        Self {
            parallel_enabled: true,
            tile_size: 64,
            adjustment_registry,
        }
    }

    /// Create a render engine with custom settings
    pub fn with_settings(parallel_enabled: bool, tile_size: u32) -> Self {
        let mut adjustment_registry = AdjustmentRegistry::new();
        adjustment_registry.register_default_adjustments();

        Self {
            parallel_enabled,
            tile_size: tile_size.max(16), // Minimum tile size
            adjustment_registry,
        }
    }

    /// Render document to a single flattened image
    #[instrument(skip(self, document))]
    pub fn render_document(&self, document: &Document) -> Result<PixelData> {
        debug!(
            "Rendering document: {}x{} with {} layers",
            document.size.width,
            document.size.height,
            document.layers.len()
        );

        if document.layers.is_empty() {
            // Create empty image with background color
            let mut result =
                PixelData::new_rgba(document.size.width as u32, document.size.height as u32);
            result.fill(document.background_color);
            return Ok(result);
        }

        // Start with background
        let mut result =
            PixelData::new_rgba(document.size.width as u32, document.size.height as u32);
        result.fill(document.background_color);

        // Composite layers from bottom to top
        for layer in &document.layers {
            if !layer.is_effectively_visible() {
                trace!("Skipping invisible layer: {}", layer.name);
                continue;
            }

            // Handle adjustment layers
            if let LayerType::Adjustment {
                adjustment_type,
                parameters,
            } = &layer.layer_type
            {
                self.apply_adjustment_layer(&mut result, layer, adjustment_type, parameters)?;
                continue;
            }

            if let Some(layer_data) = &layer.pixel_data {
                self.composite_layer(&mut result, layer, layer_data)?;
            }
        }

        Ok(result)
    }

    /// Composite a single layer onto the result image
    #[instrument(skip(self, result, layer, layer_data))]
    fn composite_layer(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        layer_data: &PixelData,
    ) -> Result<()> {
        let (result_width, result_height) = result.dimensions();
        let (layer_width, layer_height) = layer_data.dimensions();

        let offset_x = layer.offset.x as i32;
        let offset_y = layer.offset.y as i32;

        trace!(
            "Compositing layer '{}' at offset ({}, {}) with opacity {} and blend mode {:?}",
            layer.name,
            offset_x,
            offset_y,
            layer.effective_opacity(),
            layer.blend_mode
        );

        let params = CompositionParams {
            result_width,
            result_height,
            layer_width,
            layer_height,
            offset_x,
            offset_y,
        };

        if self.parallel_enabled && layer_width * layer_height > self.tile_size * self.tile_size {
            self.composite_layer_parallel(result, layer, layer_data, &params)
        } else {
            self.composite_layer_sequential(result, layer, layer_data, &params)
        }
    }

    /// Sequential layer composition
    fn composite_layer_sequential(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        layer_data: &PixelData,
        params: &CompositionParams,
    ) -> Result<()> {
        for y in 0..params.layer_height {
            for x in 0..params.layer_width {
                let doc_x = x as i32 + params.offset_x;
                let doc_y = y as i32 + params.offset_y;

                // Check bounds
                if doc_x < 0
                    || doc_y < 0
                    || doc_x >= params.result_width as i32
                    || doc_y >= params.result_height as i32
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

    /// Parallel layer composition using tiles
    fn composite_layer_parallel(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        _layer_data: &PixelData,
        params: &CompositionParams,
    ) -> Result<()> {
        // Create tiles for parallel processing
        let tiles = self.create_tiles(params.layer_width, params.layer_height);

        // Process tiles in parallel
        let blend_mode = layer.blend_mode;
        let opacity = layer.effective_opacity();

        // Collect pixel updates
        let updates: Result<Vec<_>> = tiles
            .par_iter()
            .map(|tile| {
                let mut tile_updates = Vec::new();

                for y in tile.y..tile.y + tile.height {
                    for x in tile.x..tile.x + tile.width {
                        if x >= params.layer_width || y >= params.layer_height {
                            continue;
                        }

                        let doc_x = x as i32 + params.offset_x;
                        let doc_y = y as i32 + params.offset_y;

                        // Check bounds
                        if doc_x < 0
                            || doc_y < 0
                            || doc_x >= params.result_width as i32
                            || doc_y >= params.result_height as i32
                        {
                            continue;
                        }

                        // Get pixel with mask applied
                        if let Some(layer_pixel) = layer.get_masked_pixel(x, y) {
                            if let Some(base_pixel) = result.get_pixel(doc_x as u32, doc_y as u32) {
                                let blended = blend_mode.blend(base_pixel, layer_pixel, opacity);
                                tile_updates.push((doc_x as u32, doc_y as u32, blended));
                            }
                        }
                    }
                }

                Ok(tile_updates)
            })
            .collect();

        // Apply updates sequentially to avoid race conditions
        for tile_updates in updates? {
            for (x, y, pixel) in tile_updates {
                result.set_pixel(x, y, pixel)?;
            }
        }

        Ok(())
    }

    /// Create tiles for parallel processing
    fn create_tiles(&self, width: u32, height: u32) -> Vec<Tile> {
        let mut tiles = Vec::new();
        let tile_size = self.tile_size;

        for y in (0..height).step_by(tile_size as usize) {
            for x in (0..width).step_by(tile_size as usize) {
                let tile_width = tile_size.min(width - x);
                let tile_height = tile_size.min(height - y);

                tiles.push(Tile {
                    x,
                    y,
                    width: tile_width,
                    height: tile_height,
                });
            }
        }

        tiles
    }

    /// Render a specific region of the document
    #[instrument(skip(self, document))]
    pub fn render_region(
        &self,
        document: &Document,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<PixelData> {
        debug!("Rendering region: ({}, {}) {}x{}", x, y, width, height);

        // Create result image for the region
        let mut result = PixelData::new_rgba(width, height);
        result.fill(document.background_color);

        // Composite layers in the region
        for layer in &document.layers {
            if !layer.is_effectively_visible() {
                continue;
            }

            if let Some(layer_data) = &layer.pixel_data {
                let region = RegionParams {
                    region_x: x,
                    region_y: y,
                    region_width: width,
                    region_height: height,
                };
                self.composite_layer_region(&mut result, layer, layer_data, &region)?;
            }
        }

        Ok(result)
    }

    /// Composite a layer in a specific region
    fn composite_layer_region(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        layer_data: &PixelData,
        region: &RegionParams,
    ) -> Result<()> {
        let (layer_width, layer_height) = layer_data.dimensions();
        let offset_x = layer.offset.x as i32;
        let offset_y = layer.offset.y as i32;

        for y in 0..region.region_height {
            for x in 0..region.region_width {
                let doc_x = region.region_x + x;
                let doc_y = region.region_y + y;

                let layer_x = doc_x as i32 - offset_x;
                let layer_y = doc_y as i32 - offset_y;

                if layer_x < 0
                    || layer_y < 0
                    || layer_x >= layer_width as i32
                    || layer_y >= layer_height as i32
                {
                    continue;
                }

                // Get pixel with mask applied
                if let Some(layer_pixel) = layer.get_masked_pixel(layer_x as u32, layer_y as u32) {
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

    /// Apply an adjustment layer to the current result
    #[instrument(skip(self, result, layer, adjustment_type, parameters))]
    fn apply_adjustment_layer(
        &self,
        result: &mut PixelData,
        layer: &Layer,
        adjustment_type: &str,
        parameters: &HashMap<String, f32>,
    ) -> Result<()> {
        trace!(
            "Applying adjustment layer '{}' of type '{}' with opacity {}",
            layer.name,
            adjustment_type,
            layer.effective_opacity()
        );

        // Create the adjustment from the registry
        let mut adjustment = self
            .adjustment_registry
            .create(adjustment_type)
            .ok_or_else(|| anyhow::anyhow!("Unknown adjustment type: {}", adjustment_type))?;

        // Convert HashMap<String, f32> to serde_json::Value
        let params_json = serde_json::to_value(parameters)
            .map_err(|e| anyhow::anyhow!("Failed to serialize adjustment parameters: {}", e))?;

        // Set the adjustment parameters
        adjustment.set_parameters(params_json)?;

        // If the adjustment layer has reduced opacity, we need to blend the effect
        let opacity = layer.effective_opacity();
        if (opacity - 1.0).abs() < f32::EPSILON {
            // Full opacity - apply adjustment directly
            adjustment.apply(result)?;
        } else {
            // Reduced opacity - apply to a copy and blend
            let mut adjusted_copy = result.clone();
            adjustment.apply(&mut adjusted_copy)?;

            // Blend the adjusted result back with the original
            self.blend_adjustment_result(result, &adjusted_copy, opacity)?;
        }

        Ok(())
    }

    /// Blend an adjustment result with the original image based on opacity
    fn blend_adjustment_result(
        &self,
        original: &mut PixelData,
        adjusted: &PixelData,
        opacity: f32,
    ) -> Result<()> {
        let (width, height) = original.dimensions();

        for y in 0..height {
            for x in 0..width {
                if let (Some(orig_pixel), Some(adj_pixel)) =
                    (original.get_pixel(x, y), adjusted.get_pixel(x, y))
                {
                    // Linear interpolation between original and adjusted
                    let blended = orig_pixel.lerp(adj_pixel, opacity);
                    original.set_pixel(x, y, blended)?;
                }
            }
        }

        Ok(())
    }
}

/// Tile for parallel processing
#[derive(Debug, Clone)]
struct Tile {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

/// Parameters for layer composition
#[derive(Debug, Clone)]
struct CompositionParams {
    result_width: u32,
    result_height: u32,
    layer_width: u32,
    layer_height: u32,
    offset_x: i32,
    offset_y: i32,
}

/// Parameters for region composition
#[derive(Debug, Clone)]
struct RegionParams {
    region_x: u32,
    region_y: u32,
    region_width: u32,
    region_height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Layer, Point, RgbaPixel};

    #[test]
    fn test_render_engine_creation() {
        let engine = RenderEngine::new();
        assert!(engine.parallel_enabled);
        assert_eq!(engine.tile_size, 64);
    }

    #[test]
    fn test_render_engine_custom_settings() {
        let engine = RenderEngine::with_settings(false, 32);
        assert!(!engine.parallel_enabled);
        assert_eq!(engine.tile_size, 32);
    }

    #[test]
    fn test_render_empty_document() {
        let engine = RenderEngine::new();
        let document = Document::new("Test".to_string(), 100, 100);

        let result = engine.render_document(&document).unwrap();
        let (width, height) = result.dimensions();

        assert_eq!(width, 100);
        assert_eq!(height, 100);
    }

    #[test]
    fn test_render_document_with_layers() {
        let engine = RenderEngine::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        let mut layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        layer.fill(RgbaPixel::new(255, 0, 0, 255)); // Red
        layer.offset = Point::new(25.0, 25.0);

        document.add_layer(layer);

        let result = engine.render_document(&document).unwrap();
        let (width, height) = result.dimensions();

        assert_eq!(width, 100);
        assert_eq!(height, 100);

        // Check that the red pixel is rendered at the correct position
        let pixel = result.get_pixel(50, 50).unwrap();
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn test_tile_creation() {
        let engine = RenderEngine::with_settings(true, 32);
        let tiles = engine.create_tiles(100, 100);

        // Should create 4x4 = 16 tiles for 100x100 image with 32x32 tiles
        assert_eq!(tiles.len(), 16);

        // Check first tile
        assert_eq!(tiles[0].x, 0);
        assert_eq!(tiles[0].y, 0);
        assert_eq!(tiles[0].width, 32);
        assert_eq!(tiles[0].height, 32);

        // Check last tile (should be smaller due to remainder)
        let last_tile = &tiles[tiles.len() - 1];
        assert_eq!(last_tile.x, 96);
        assert_eq!(last_tile.y, 96);
        assert_eq!(last_tile.width, 4);
        assert_eq!(last_tile.height, 4);
    }

    #[test]
    fn test_render_region() {
        let engine = RenderEngine::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        let mut layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        layer.fill(RgbaPixel::new(0, 255, 0, 255)); // Green
        layer.offset = Point::new(10.0, 10.0);

        document.add_layer(layer);

        // Render a 30x30 region starting at (20, 20)
        let result = engine.render_region(&document, 20, 20, 30, 30).unwrap();
        let (width, height) = result.dimensions();

        assert_eq!(width, 30);
        assert_eq!(height, 30);

        // Check that the green pixel is rendered in the region
        let pixel = result.get_pixel(0, 0).unwrap(); // This should be at document position (20, 20)
        assert_eq!(pixel.g, 255);
    }

    #[test]
    fn test_render_with_adjustment_layer() {
        let engine = RenderEngine::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add a base layer with gray pixels
        let mut base_layer = Layer::new_pixel("Base Layer".to_string(), 100, 100);
        base_layer.fill(RgbaPixel::new(128, 128, 128, 255)); // Gray
        document.add_layer(base_layer);

        // Add a brightness adjustment layer
        let mut params = std::collections::HashMap::new();
        params.insert("brightness".to_string(), 0.5); // 50% brighter
        let adjustment_layer = Layer::new_adjustment(
            "Brightness Adjustment".to_string(),
            "brightness".to_string(),
            params,
        );
        document.add_layer(adjustment_layer);

        // Render the document
        let result = engine.render_document(&document).unwrap();
        let (width, height) = result.dimensions();

        assert_eq!(width, 100);
        assert_eq!(height, 100);

        // Check that the pixel is brighter than the original
        let pixel = result.get_pixel(50, 50).unwrap();
        assert!(pixel.r > 128); // Should be brighter than original gray
        assert!(pixel.g > 128);
        assert!(pixel.b > 128);
    }

    #[test]
    fn test_adjustment_layer_opacity() {
        let engine = RenderEngine::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add a base layer with gray pixels
        let mut base_layer = Layer::new_pixel("Base Layer".to_string(), 100, 100);
        base_layer.fill(RgbaPixel::new(128, 128, 128, 255)); // Gray
        document.add_layer(base_layer);

        // Add a brightness adjustment layer with 50% opacity
        let mut params = std::collections::HashMap::new();
        params.insert("brightness".to_string(), 1.0); // 100% brighter
        let mut adjustment_layer = Layer::new_adjustment(
            "Brightness Adjustment".to_string(),
            "brightness".to_string(),
            params,
        );
        adjustment_layer.opacity = 0.5; // 50% opacity
        document.add_layer(adjustment_layer);

        // Render the document
        let result = engine.render_document(&document).unwrap();

        // Check that the effect is reduced due to opacity
        let pixel = result.get_pixel(50, 50).unwrap();
        assert!(pixel.r > 128); // Should be brighter than original
        assert!(pixel.r < 255); // But not fully bright due to opacity
    }
}
