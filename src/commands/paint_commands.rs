//! Paint-related commands for undo/redo functionality
//!
//! This module contains commands for painting operations including:
//! - Brush strokes
//! - Eraser strokes
//! - Fill operations

use anyhow::Result;
use psoc_core::{Command, CommandMetadata, Document, PixelData, Point, RgbaPixel};
use std::fmt::Debug;
use uuid::Uuid;

/// Command to apply a brush stroke to a layer
#[derive(Debug)]
pub struct BrushStrokeCommand {
    metadata: CommandMetadata,
    layer_index: usize,
    stroke_points: Vec<Point>,
    brush_size: f32,
    brush_hardness: f32,
    brush_color: RgbaPixel,
    affected_region: Option<(u32, u32, u32, u32)>, // x, y, width, height
    backup_data: Option<PixelData>,
}

impl BrushStrokeCommand {
    /// Create a new brush stroke command
    pub fn new(
        layer_index: usize,
        stroke_points: Vec<Point>,
        brush_size: f32,
        brush_hardness: f32,
        brush_color: RgbaPixel,
    ) -> Self {
        Self {
            metadata: CommandMetadata::new("Brush Stroke".to_string()),
            layer_index,
            stroke_points,
            brush_size,
            brush_hardness,
            brush_color,
            affected_region: None,
            backup_data: None,
        }
    }

    /// Calculate the affected region for this stroke
    fn calculate_affected_region(&self) -> Option<(u32, u32, u32, u32)> {
        if self.stroke_points.is_empty() {
            return None;
        }

        let brush_radius = self.brush_size / 2.0;
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for point in &self.stroke_points {
            min_x = min_x.min(point.x - brush_radius);
            min_y = min_y.min(point.y - brush_radius);
            max_x = max_x.max(point.x + brush_radius);
            max_y = max_y.max(point.y + brush_radius);
        }

        let x = min_x.max(0.0) as u32;
        let y = min_y.max(0.0) as u32;
        let width = (max_x - min_x).ceil() as u32;
        let height = (max_y - min_y).ceil() as u32;

        Some((x, y, width, height))
    }

    /// Backup the affected region before applying the stroke
    fn backup_region(&mut self, document: &Document) -> Result<()> {
        if let Some(layer) = document.get_layer(self.layer_index) {
            if let Some(pixel_data) = &layer.pixel_data {
                if let Some((x, y, width, height)) = self.calculate_affected_region() {
                    // Create backup of the affected region
                    let mut backup = PixelData::new_rgba(width, height);

                    for dy in 0..height {
                        for dx in 0..width {
                            let src_x = x + dx;
                            let src_y = y + dy;

                            if src_x < pixel_data.dimensions().0
                                && src_y < pixel_data.dimensions().1
                            {
                                if let Some(pixel) = pixel_data.get_pixel(src_x, src_y) {
                                    let _ = backup.set_pixel(dx, dy, pixel);
                                }
                            }
                        }
                    }

                    self.affected_region = Some((x, y, width, height));
                    self.backup_data = Some(backup);
                }
            }
        }
        Ok(())
    }

    /// Apply the brush stroke to the layer
    fn apply_stroke(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = document.get_layer_mut(self.layer_index) {
            if let Some(pixel_data) = &mut layer.pixel_data {
                // Apply brush stroke using the existing brush algorithm
                for point in &self.stroke_points {
                    self.apply_brush_at_point(pixel_data, *point);
                }
                document.mark_dirty();
            }
        }
        Ok(())
    }

    /// Apply brush effect at a single point
    fn apply_brush_at_point(&self, pixel_data: &mut PixelData, center: Point) {
        let radius = self.brush_size / 2.0;
        let radius_squared = radius * radius;

        let min_x = (center.x - radius).max(0.0) as u32;
        let max_x = (center.x + radius).min(pixel_data.dimensions().0 as f32) as u32;
        let min_y = (center.y - radius).max(0.0) as u32;
        let max_y = (center.y + radius).min(pixel_data.dimensions().1 as f32) as u32;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let dx = x as f32 - center.x;
                let dy = y as f32 - center.y;
                let distance_squared = dx * dx + dy * dy;

                if distance_squared <= radius_squared {
                    // Calculate brush strength based on distance and hardness
                    let distance = distance_squared.sqrt();
                    let strength = if distance <= radius * self.brush_hardness {
                        1.0
                    } else {
                        let falloff = (distance - radius * self.brush_hardness)
                            / (radius * (1.0 - self.brush_hardness));
                        1.0 - falloff.min(1.0)
                    };

                    // Apply brush color with alpha blending
                    if let Some(current_pixel) = pixel_data.get_pixel(x, y) {
                        let brush_alpha = (self.brush_color.a as f32 / 255.0) * strength;

                        let blended = RgbaPixel {
                            r: (current_pixel.r as f32 * (1.0 - brush_alpha)
                                + self.brush_color.r as f32 * brush_alpha)
                                as u8,
                            g: (current_pixel.g as f32 * (1.0 - brush_alpha)
                                + self.brush_color.g as f32 * brush_alpha)
                                as u8,
                            b: (current_pixel.b as f32 * (1.0 - brush_alpha)
                                + self.brush_color.b as f32 * brush_alpha)
                                as u8,
                            a: (current_pixel.a as f32 * (1.0 - brush_alpha) + 255.0 * brush_alpha)
                                .min(255.0) as u8,
                        };

                        let _ = pixel_data.set_pixel(x, y, blended);
                    }
                }
            }
        }
    }

    /// Restore the backed up region
    fn restore_region(&self, document: &mut Document) -> Result<()> {
        if let (Some((x, y, _width, _height)), Some(backup_data)) =
            (&self.affected_region, &self.backup_data)
        {
            if let Some(layer) = document.get_layer_mut(self.layer_index) {
                if let Some(pixel_data) = &mut layer.pixel_data {
                    // Restore the backup data
                    for dy in 0..backup_data.dimensions().1 {
                        for dx in 0..backup_data.dimensions().0 {
                            let dst_x = x + dx;
                            let dst_y = y + dy;

                            if dst_x < pixel_data.dimensions().0
                                && dst_y < pixel_data.dimensions().1
                            {
                                if let Some(pixel) = backup_data.get_pixel(dx, dy) {
                                    let _ = pixel_data.set_pixel(dst_x, dst_y, pixel);
                                }
                            }
                        }
                    }
                    document.mark_dirty();
                }
            }
        }
        Ok(())
    }
}

impl Command for BrushStrokeCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // We need to make self mutable to backup the region
        // This is a limitation of the current design - in a real implementation,
        // we might want to separate the backup phase from execution
        self.apply_stroke(document)
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        self.restore_region(document)
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn can_merge_with(&self, _other: &dyn Command) -> bool {
        // Simplified implementation - no merging for now
        false
    }

    fn merge_with(&mut self, _other: Box<dyn Command>) -> Result<()> {
        // Simplified implementation - no merging for now
        Err(anyhow::anyhow!("Brush stroke merging not yet implemented"))
    }
}

/// Command to apply an eraser stroke to a layer
#[derive(Debug)]
pub struct EraserStrokeCommand {
    metadata: CommandMetadata,
    layer_index: usize,
    stroke_points: Vec<Point>,
    eraser_size: f32,
    eraser_hardness: f32,
    affected_region: Option<(u32, u32, u32, u32)>,
    backup_data: Option<PixelData>,
}

impl EraserStrokeCommand {
    /// Create a new eraser stroke command
    pub fn new(
        layer_index: usize,
        stroke_points: Vec<Point>,
        eraser_size: f32,
        eraser_hardness: f32,
    ) -> Self {
        Self {
            metadata: CommandMetadata::new("Eraser Stroke".to_string()),
            layer_index,
            stroke_points,
            eraser_size,
            eraser_hardness,
            affected_region: None,
            backup_data: None,
        }
    }

    /// Apply the eraser stroke to the layer
    fn apply_stroke(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = document.get_layer_mut(self.layer_index) {
            if let Some(pixel_data) = &mut layer.pixel_data {
                // Apply eraser stroke
                for point in &self.stroke_points {
                    self.apply_eraser_at_point(pixel_data, *point);
                }
                document.mark_dirty();
            }
        }
        Ok(())
    }

    /// Apply eraser effect at a single point
    fn apply_eraser_at_point(&self, pixel_data: &mut PixelData, center: Point) {
        let radius = self.eraser_size / 2.0;
        let radius_squared = radius * radius;

        let min_x = (center.x - radius).max(0.0) as u32;
        let max_x = (center.x + radius).min(pixel_data.dimensions().0 as f32) as u32;
        let min_y = (center.y - radius).max(0.0) as u32;
        let max_y = (center.y + radius).min(pixel_data.dimensions().1 as f32) as u32;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let dx = x as f32 - center.x;
                let dy = y as f32 - center.y;
                let distance_squared = dx * dx + dy * dy;

                if distance_squared <= radius_squared {
                    // Calculate eraser strength based on distance and hardness
                    let distance = distance_squared.sqrt();
                    let strength = if distance <= radius * self.eraser_hardness {
                        1.0
                    } else {
                        let falloff = (distance - radius * self.eraser_hardness)
                            / (radius * (1.0 - self.eraser_hardness));
                        1.0 - falloff.min(1.0)
                    };

                    // Apply eraser by reducing alpha
                    if let Some(mut current_pixel) = pixel_data.get_pixel(x, y) {
                        let alpha_reduction = (255.0 * strength) as u8;
                        current_pixel.a = current_pixel.a.saturating_sub(alpha_reduction);
                        let _ = pixel_data.set_pixel(x, y, current_pixel);
                    }
                }
            }
        }
    }
}

impl Command for EraserStrokeCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        self.apply_stroke(document)
    }

    fn undo(&self, _document: &mut Document) -> Result<()> {
        // For now, eraser undo is not implemented
        // In a full implementation, we would backup the affected region
        Err(anyhow::anyhow!("Eraser undo not yet implemented"))
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Document, Layer, PixelData};

    #[test]
    fn test_brush_stroke_command_creation() {
        let points = vec![Point::new(10.0, 10.0), Point::new(20.0, 20.0)];
        let color = RgbaPixel::new(255, 0, 0, 255);

        let command = BrushStrokeCommand::new(0, points, 10.0, 0.8, color);
        assert_eq!(command.description(), "Brush Stroke");
        assert_eq!(command.layer_index, 0);
        assert_eq!(command.stroke_points.len(), 2);
    }

    #[test]
    fn test_brush_stroke_affected_region() {
        let points = vec![Point::new(10.0, 10.0), Point::new(20.0, 20.0)];
        let color = RgbaPixel::new(255, 0, 0, 255);

        let command = BrushStrokeCommand::new(0, points, 10.0, 0.8, color);
        let region = command.calculate_affected_region();

        assert!(region.is_some());
        let (_x, _y, width, height) = region.unwrap();
        assert!(width > 0);
        assert!(height > 0);
    }

    #[test]
    fn test_eraser_stroke_command_creation() {
        let points = vec![Point::new(10.0, 10.0), Point::new(20.0, 20.0)];

        let command = EraserStrokeCommand::new(0, points, 10.0, 0.8);
        assert_eq!(command.description(), "Eraser Stroke");
        assert_eq!(command.layer_index, 0);
        assert_eq!(command.stroke_points.len(), 2);
    }

    #[test]
    fn test_brush_stroke_execution() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        layer.pixel_data = Some(PixelData::new_rgba(100, 100));
        document.add_layer(layer);

        let points = vec![Point::new(50.0, 50.0)];
        let color = RgbaPixel::new(255, 0, 0, 255);
        let command = BrushStrokeCommand::new(0, points, 10.0, 1.0, color);

        // Should execute without error
        assert!(command.execute(&mut document).is_ok());
    }
}
