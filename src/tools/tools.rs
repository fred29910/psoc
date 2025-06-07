//! Concrete tool implementations
//!
//! This module contains the actual implementations of all editing tools,
//! including selection, brush, eraser, and move tools.

use tracing::debug;

use super::tool_trait::{
    Tool, ToolCursor, ToolEvent, ToolOption, ToolOptionType, ToolOptionValue, ToolResult, ToolState,
};
use psoc_core::{Document, Point, RgbaPixel, Selection};
use serde::{Deserialize, Serialize};

/// Tool types available in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolType {
    Select,
    Brush,
    Eraser,
    Move,
    Transform,
}

impl std::fmt::Display for ToolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolType::Select => write!(f, "Select"),
            ToolType::Brush => write!(f, "Brush"),
            ToolType::Eraser => write!(f, "Eraser"),
            ToolType::Move => write!(f, "Move"),
            ToolType::Transform => write!(f, "Transform"),
        }
    }
}

/// Selection tool for making selections
#[derive(Debug)]
pub struct SelectTool {
    selection_start: Option<Point>,
    is_selecting: bool,
    feather_radius: f32,
    anti_alias: bool,
}

impl SelectTool {
    pub fn new() -> Self {
        Self {
            selection_start: None,
            is_selecting: false,
            feather_radius: 0.0,
            anti_alias: true,
        }
    }
}

impl Default for SelectTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for SelectTool {
    fn id(&self) -> &'static str {
        "select"
    }

    fn name(&self) -> &'static str {
        "Selection Tool"
    }

    fn description(&self) -> &'static str {
        "Make rectangular selections"
    }

    fn cursor(&self) -> ToolCursor {
        ToolCursor::Crosshair
    }

    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        state: &mut ToolState,
    ) -> ToolResult<()> {
        match event {
            ToolEvent::MousePressed { position, .. } => {
                debug!("Selection started at: {:?}", position);
                self.selection_start = Some(position);
                self.is_selecting = true;
                state.is_active = true;
                state.last_position = Some(position);
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_selecting {
                    debug!("Selection dragged to: {:?}", position);
                    state.last_position = Some(position);

                    // Update selection rectangle in real-time
                    if let Some(start) = self.selection_start {
                        let selection = Selection::rectangle_from_points(start, position);
                        document.set_selection(selection);
                    }
                }
            }
            ToolEvent::MouseReleased { position, .. } => {
                if self.is_selecting {
                    debug!("Selection completed at: {:?}", position);
                    self.is_selecting = false;
                    state.is_active = false;

                    // Finalize selection
                    if let Some(start) = self.selection_start {
                        let selection = Selection::rectangle_from_points(start, position);
                        debug!("Created selection: {}", selection);
                        document.set_selection(selection);
                    }

                    self.selection_start = None;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "feather".to_string(),
                display_name: "Feather Radius".to_string(),
                description: "Softness of selection edges in pixels".to_string(),
                option_type: ToolOptionType::Float {
                    min: 0.0,
                    max: 50.0,
                },
                default_value: ToolOptionValue::Float(self.feather_radius),
            },
            ToolOption {
                name: "anti_alias".to_string(),
                display_name: "Anti-alias".to_string(),
                description: "Smooth selection edges".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(self.anti_alias),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "feather" => {
                if let ToolOptionValue::Float(radius) = value {
                    self.feather_radius = radius.clamp(0.0, 50.0);
                }
            }
            "anti_alias" => {
                if let ToolOptionValue::Bool(enabled) = value {
                    self.anti_alias = enabled;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "feather" => Some(ToolOptionValue::Float(self.feather_radius)),
            "anti_alias" => Some(ToolOptionValue::Bool(self.anti_alias)),
            _ => None,
        }
    }
}

/// Brush tool for painting
#[derive(Debug)]
pub struct BrushTool {
    brush_size: f32,
    brush_color: RgbaPixel,
    brush_hardness: f32,
    is_painting: bool,
}

impl BrushTool {
    pub fn new() -> Self {
        Self {
            brush_size: 10.0,
            brush_color: RgbaPixel::new(0, 0, 0, 255), // Black
            brush_hardness: 1.0,
            is_painting: false,
        }
    }
}

impl Default for BrushTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for BrushTool {
    fn id(&self) -> &'static str {
        "brush"
    }

    fn name(&self) -> &'static str {
        "Brush Tool"
    }

    fn description(&self) -> &'static str {
        "Paint with a brush"
    }

    fn cursor(&self) -> ToolCursor {
        ToolCursor::Crosshair
    }

    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        state: &mut ToolState,
    ) -> ToolResult<()> {
        match event {
            ToolEvent::MousePressed { position, .. } => {
                debug!("Brush stroke started at: {:?}", position);
                self.is_painting = true;
                state.is_active = true;
                state.last_position = Some(position);

                // Start painting at this position
                self.paint_at_position(position, document)?;
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_painting {
                    debug!("Brush stroke continued to: {:?}", position);

                    // Paint from last position to current position
                    if let Some(last_pos) = state.last_position {
                        self.paint_stroke(last_pos, position, document)?;
                    }

                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased { .. } => {
                if self.is_painting {
                    debug!("Brush stroke completed");
                    self.is_painting = false;
                    state.is_active = false;
                    // TODO: Commit brush stroke to history
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "size".to_string(),
                display_name: "Brush Size".to_string(),
                description: "Size of the brush in pixels".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 100.0,
                },
                default_value: ToolOptionValue::Float(self.brush_size),
            },
            ToolOption {
                name: "hardness".to_string(),
                display_name: "Brush Hardness".to_string(),
                description: "Hardness of the brush edge".to_string(),
                option_type: ToolOptionType::Float { min: 0.0, max: 1.0 },
                default_value: ToolOptionValue::Float(self.brush_hardness),
            },
            ToolOption {
                name: "color".to_string(),
                display_name: "Brush Color".to_string(),
                description: "Color of the brush".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.brush_color.r,
                    self.brush_color.g,
                    self.brush_color.b,
                    self.brush_color.a,
                ]),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "size" => {
                if let ToolOptionValue::Float(size) = value {
                    self.brush_size = size.clamp(1.0, 100.0);
                }
            }
            "hardness" => {
                if let ToolOptionValue::Float(hardness) = value {
                    self.brush_hardness = hardness.clamp(0.0, 1.0);
                }
            }
            "color" => {
                if let ToolOptionValue::Color(rgba) = value {
                    self.brush_color = RgbaPixel::new(rgba[0], rgba[1], rgba[2], rgba[3]);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "size" => Some(ToolOptionValue::Float(self.brush_size)),
            "hardness" => Some(ToolOptionValue::Float(self.brush_hardness)),
            "color" => Some(ToolOptionValue::Color([
                self.brush_color.r,
                self.brush_color.g,
                self.brush_color.b,
                self.brush_color.a,
            ])),
            _ => None,
        }
    }
}

impl BrushTool {
    fn paint_at_position(&self, position: Point, document: &mut Document) -> ToolResult<()> {
        debug!(
            "Painting at position: {:?} with size: {} and color: {:?}",
            position, self.brush_size, self.brush_color
        );

        // Get the active layer
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to paint on");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Paint a circular brush at the position
        self.paint_circular_brush(position, layer)?;
        document.mark_dirty();

        Ok(())
    }

    fn paint_stroke(&self, from: Point, to: Point, document: &mut Document) -> ToolResult<()> {
        debug!("Painting stroke from {:?} to {:?}", from, to);

        // Calculate the distance between points
        let distance = from.distance_to(&to);

        // If points are very close, just paint at the destination
        if distance < 1.0 {
            return self.paint_at_position(to, document);
        }

        // Interpolate points along the stroke for smooth painting
        let steps = (distance / (self.brush_size * 0.25)).ceil() as i32;
        let steps = steps.max(1);

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let interpolated_x = from.x + (to.x - from.x) * t;
            let interpolated_y = from.y + (to.y - from.y) * t;
            let interpolated_pos = Point::new(interpolated_x, interpolated_y);

            self.paint_at_position(interpolated_pos, document)?;
        }

        Ok(())
    }

    /// Paint a circular brush at the given position on the layer
    fn paint_circular_brush(&self, center: Point, layer: &mut psoc_core::Layer) -> ToolResult<()> {
        let radius = self.brush_size / 2.0;
        let layer_dims = layer.dimensions();

        if layer_dims.is_none() {
            return Ok(());
        }

        let (layer_width, layer_height) = layer_dims.unwrap();

        // Calculate the bounding box of the brush
        let min_x = ((center.x - radius).floor() as i32).max(0);
        let max_x = ((center.x + radius).ceil() as i32).min(layer_width as i32 - 1);
        let min_y = ((center.y - radius).floor() as i32).max(0);
        let max_y = ((center.y + radius).ceil() as i32).min(layer_height as i32 - 1);

        // Paint each pixel in the brush area
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pixel_x = x as f32;
                let pixel_y = y as f32;

                // Calculate distance from brush center
                let dx = pixel_x - center.x;
                let dy = pixel_y - center.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    // Calculate brush alpha based on distance and hardness
                    let alpha = self.calculate_brush_alpha(distance, radius);

                    if alpha > 0.0 {
                        // Blend the brush color with the existing pixel
                        self.blend_pixel_at(x as u32, y as u32, alpha, layer)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Calculate the alpha value for a pixel based on distance from brush center
    fn calculate_brush_alpha(&self, distance: f32, radius: f32) -> f32 {
        if distance >= radius {
            return 0.0;
        }

        // Calculate normalized distance (0.0 at center, 1.0 at edge)
        let normalized_distance = distance / radius;

        // Apply hardness - hardness of 1.0 means hard edge, 0.0 means very soft
        if self.brush_hardness >= 1.0 {
            // Hard brush - full opacity within radius
            1.0
        } else if self.brush_hardness <= 0.0 {
            // Very soft brush - gaussian-like falloff
            let falloff = 1.0 - normalized_distance;
            falloff * falloff
        } else {
            // Interpolate between hard and soft based on hardness
            let hard_alpha = 1.0;
            let soft_alpha = {
                let falloff = 1.0 - normalized_distance;
                falloff * falloff
            };

            // Mix hard and soft based on hardness value
            hard_alpha * self.brush_hardness + soft_alpha * (1.0 - self.brush_hardness)
        }
    }

    /// Blend brush color with existing pixel at the given coordinates
    fn blend_pixel_at(
        &self,
        x: u32,
        y: u32,
        alpha: f32,
        layer: &mut psoc_core::Layer,
    ) -> ToolResult<()> {
        // Get the existing pixel
        let existing_pixel = layer
            .get_pixel(x, y)
            .unwrap_or(psoc_core::RgbaPixel::transparent());

        // Create brush pixel with calculated alpha
        let brush_alpha = (alpha * self.brush_color.a as f32 / 255.0 * 255.0) as u8;
        let brush_pixel = psoc_core::RgbaPixel::new(
            self.brush_color.r,
            self.brush_color.g,
            self.brush_color.b,
            brush_alpha,
        );

        // Blend using normal blending mode (alpha compositing)
        let blended_pixel = self.blend_normal(existing_pixel, brush_pixel);

        // Set the blended pixel
        layer.set_pixel(x, y, blended_pixel)?;

        Ok(())
    }

    /// Normal blending mode (alpha compositing)
    fn blend_normal(
        &self,
        base: psoc_core::RgbaPixel,
        overlay: psoc_core::RgbaPixel,
    ) -> psoc_core::RgbaPixel {
        let base_alpha = base.a as f32 / 255.0;
        let overlay_alpha = overlay.a as f32 / 255.0;

        // Alpha compositing formula
        let result_alpha = overlay_alpha + base_alpha * (1.0 - overlay_alpha);

        if result_alpha == 0.0 {
            return psoc_core::RgbaPixel::transparent();
        }

        let inv_result_alpha = 1.0 / result_alpha;

        let result_r = ((overlay.r as f32 * overlay_alpha
            + base.r as f32 * base_alpha * (1.0 - overlay_alpha))
            * inv_result_alpha) as u8;
        let result_g = ((overlay.g as f32 * overlay_alpha
            + base.g as f32 * base_alpha * (1.0 - overlay_alpha))
            * inv_result_alpha) as u8;
        let result_b = ((overlay.b as f32 * overlay_alpha
            + base.b as f32 * base_alpha * (1.0 - overlay_alpha))
            * inv_result_alpha) as u8;
        let result_a = (result_alpha * 255.0) as u8;

        psoc_core::RgbaPixel::new(result_r, result_g, result_b, result_a)
    }
}

/// Eraser tool for erasing pixels
#[derive(Debug)]
pub struct EraserTool {
    eraser_size: f32,
    eraser_hardness: f32,
    is_erasing: bool,
}

impl EraserTool {
    pub fn new() -> Self {
        Self {
            eraser_size: 10.0,
            eraser_hardness: 1.0,
            is_erasing: false,
        }
    }
}

impl Default for EraserTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for EraserTool {
    fn id(&self) -> &'static str {
        "eraser"
    }

    fn name(&self) -> &'static str {
        "Eraser Tool"
    }

    fn description(&self) -> &'static str {
        "Erase pixels by setting alpha to 0"
    }

    fn cursor(&self) -> ToolCursor {
        ToolCursor::Crosshair
    }

    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        state: &mut ToolState,
    ) -> ToolResult<()> {
        match event {
            ToolEvent::MousePressed { position, .. } => {
                debug!("Eraser started at: {:?}", position);
                self.is_erasing = true;
                state.is_active = true;
                state.last_position = Some(position);

                // Start erasing at this position
                self.erase_at_position(position, document)?;
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_erasing {
                    debug!("Eraser continued to: {:?}", position);

                    // Erase from last position to current position
                    if let Some(last_pos) = state.last_position {
                        self.erase_stroke(last_pos, position, document)?;
                    }

                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased { .. } => {
                if self.is_erasing {
                    debug!("Eraser completed");
                    self.is_erasing = false;
                    state.is_active = false;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "size".to_string(),
                display_name: "Eraser Size".to_string(),
                description: "Size of the eraser in pixels".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 100.0,
                },
                default_value: ToolOptionValue::Float(self.eraser_size),
            },
            ToolOption {
                name: "hardness".to_string(),
                display_name: "Eraser Hardness".to_string(),
                description: "Hardness of the eraser edge".to_string(),
                option_type: ToolOptionType::Float { min: 0.0, max: 1.0 },
                default_value: ToolOptionValue::Float(self.eraser_hardness),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "size" => {
                if let ToolOptionValue::Float(size) = value {
                    self.eraser_size = size.clamp(1.0, 100.0);
                }
            }
            "hardness" => {
                if let ToolOptionValue::Float(hardness) = value {
                    self.eraser_hardness = hardness.clamp(0.0, 1.0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "size" => Some(ToolOptionValue::Float(self.eraser_size)),
            "hardness" => Some(ToolOptionValue::Float(self.eraser_hardness)),
            _ => None,
        }
    }
}

impl EraserTool {
    fn erase_at_position(&self, position: Point, document: &mut Document) -> ToolResult<()> {
        debug!(
            "Erasing at position: {:?} with size: {} and hardness: {}",
            position, self.eraser_size, self.eraser_hardness
        );

        // Get the active layer
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to erase on");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Erase a circular area at the position
        self.erase_circular_area(position, layer)?;
        document.mark_dirty();

        Ok(())
    }

    fn erase_stroke(&self, from: Point, to: Point, document: &mut Document) -> ToolResult<()> {
        debug!("Erasing stroke from {:?} to {:?}", from, to);

        // Calculate the distance between points
        let distance = from.distance_to(&to);

        // If points are very close, just erase at the destination
        if distance < 1.0 {
            return self.erase_at_position(to, document);
        }

        // Interpolate points along the stroke for smooth erasing
        let steps = (distance / (self.eraser_size * 0.25)).ceil() as i32;
        let steps = steps.max(1);

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let interpolated_x = from.x + (to.x - from.x) * t;
            let interpolated_y = from.y + (to.y - from.y) * t;
            let interpolated_pos = Point::new(interpolated_x, interpolated_y);

            self.erase_at_position(interpolated_pos, document)?;
        }

        Ok(())
    }

    /// Erase a circular area at the given position on the layer
    fn erase_circular_area(&self, center: Point, layer: &mut psoc_core::Layer) -> ToolResult<()> {
        let radius = self.eraser_size / 2.0;
        let layer_dims = layer.dimensions();

        if layer_dims.is_none() {
            return Ok(());
        }

        let (layer_width, layer_height) = layer_dims.unwrap();

        // Calculate the bounding box of the eraser
        let min_x = ((center.x - radius).floor() as i32).max(0);
        let max_x = ((center.x + radius).ceil() as i32).min(layer_width as i32 - 1);
        let min_y = ((center.y - radius).floor() as i32).max(0);
        let max_y = ((center.y + radius).ceil() as i32).min(layer_height as i32 - 1);

        // Erase each pixel in the eraser area
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pixel_x = x as f32;
                let pixel_y = y as f32;

                // Calculate distance from eraser center
                let dx = pixel_x - center.x;
                let dy = pixel_y - center.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius {
                    // Calculate eraser alpha based on distance and hardness
                    let erase_strength = self.calculate_eraser_alpha(distance, radius);

                    if erase_strength > 0.0 {
                        // Apply erasing to the pixel
                        self.erase_pixel_at(x as u32, y as u32, erase_strength, layer)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Calculate the erasing strength for a pixel based on distance from eraser center
    fn calculate_eraser_alpha(&self, distance: f32, radius: f32) -> f32 {
        if distance >= radius {
            return 0.0;
        }

        // Calculate normalized distance (0.0 at center, 1.0 at edge)
        let normalized_distance = distance / radius;

        // Apply hardness - hardness of 1.0 means hard edge, 0.0 means very soft
        if self.eraser_hardness >= 1.0 {
            // Hard eraser - full strength within radius
            1.0
        } else if self.eraser_hardness <= 0.0 {
            // Very soft eraser - gaussian-like falloff
            let falloff = 1.0 - normalized_distance;
            falloff * falloff
        } else {
            // Interpolate between hard and soft based on hardness
            let hard_alpha = 1.0;
            let soft_alpha = {
                let falloff = 1.0 - normalized_distance;
                falloff * falloff
            };

            // Mix hard and soft based on hardness value
            hard_alpha * self.eraser_hardness + soft_alpha * (1.0 - self.eraser_hardness)
        }
    }

    /// Erase (reduce alpha) of pixel at the given coordinates
    fn erase_pixel_at(
        &self,
        x: u32,
        y: u32,
        erase_strength: f32,
        layer: &mut psoc_core::Layer,
    ) -> ToolResult<()> {
        // Get the existing pixel
        let existing_pixel = layer
            .get_pixel(x, y)
            .unwrap_or(psoc_core::RgbaPixel::transparent());

        // Calculate new alpha after erasing
        let current_alpha = existing_pixel.a as f32 / 255.0;
        let new_alpha = current_alpha * (1.0 - erase_strength);
        let new_alpha_u8 = (new_alpha * 255.0) as u8;

        // Create the erased pixel (same color, reduced alpha)
        let erased_pixel = psoc_core::RgbaPixel::new(
            existing_pixel.r,
            existing_pixel.g,
            existing_pixel.b,
            new_alpha_u8,
        );

        // Set the erased pixel
        layer.set_pixel(x, y, erased_pixel)?;

        Ok(())
    }
}

/// Move tool for moving layers and selections
#[derive(Debug)]
pub struct MoveTool {
    is_moving: bool,
    move_start: Option<Point>,
    snap_to_grid: bool,
    grid_size: f32,
}

impl MoveTool {
    pub fn new() -> Self {
        Self {
            is_moving: false,
            move_start: None,
            snap_to_grid: false,
            grid_size: 10.0,
        }
    }
}

impl Default for MoveTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for MoveTool {
    fn id(&self) -> &'static str {
        "move"
    }

    fn name(&self) -> &'static str {
        "Move Tool"
    }

    fn description(&self) -> &'static str {
        "Move layers and selections"
    }

    fn cursor(&self) -> ToolCursor {
        ToolCursor::Move
    }

    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        state: &mut ToolState,
    ) -> ToolResult<()> {
        match event {
            ToolEvent::MousePressed { position, .. } => {
                debug!("Move started at: {:?}", position);
                self.is_moving = true;
                self.move_start = Some(position);
                state.is_active = true;
                state.last_position = Some(position);
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_moving {
                    if let Some(start_pos) = self.move_start {
                        let delta_x = position.x - start_pos.x;
                        let delta_y = position.y - start_pos.y;
                        debug!("Moving by delta: ({}, {})", delta_x, delta_y);

                        // Apply movement to active layer or selection
                        self.apply_movement(delta_x, delta_y, document)?;

                        // Update start position for continuous movement
                        self.move_start = Some(position);
                    }
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased { .. } => {
                if self.is_moving {
                    debug!("Move completed");
                    self.is_moving = false;
                    state.is_active = false;
                    self.move_start = None;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "snap_to_grid".to_string(),
                display_name: "Snap to Grid".to_string(),
                description: "Snap movement to grid points".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(self.snap_to_grid),
            },
            ToolOption {
                name: "grid_size".to_string(),
                display_name: "Grid Size".to_string(),
                description: "Size of grid cells in pixels".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 100.0,
                },
                default_value: ToolOptionValue::Float(self.grid_size),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "snap_to_grid" => {
                if let ToolOptionValue::Bool(enabled) = value {
                    self.snap_to_grid = enabled;
                }
            }
            "grid_size" => {
                if let ToolOptionValue::Float(size) = value {
                    self.grid_size = size.clamp(1.0, 100.0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "snap_to_grid" => Some(ToolOptionValue::Bool(self.snap_to_grid)),
            "grid_size" => Some(ToolOptionValue::Float(self.grid_size)),
            _ => None,
        }
    }
}

impl MoveTool {
    /// Apply movement to the active layer or selection content
    fn apply_movement(
        &mut self,
        delta_x: f32,
        delta_y: f32,
        document: &mut Document,
    ) -> ToolResult<()> {
        // Check if there's an active selection
        if let Selection::Rectangle(ref selection) = document.selection {
            if !selection.is_empty() {
                // Move selection content
                self.move_selection_content(delta_x, delta_y, document)?;
            } else {
                // Move entire active layer
                self.move_active_layer(delta_x, delta_y, document)?;
            }
        } else {
            // Move entire active layer
            self.move_active_layer(delta_x, delta_y, document)?;
        }

        document.mark_dirty();
        Ok(())
    }

    /// Move the entire active layer
    fn move_active_layer(
        &self,
        delta_x: f32,
        delta_y: f32,
        document: &mut Document,
    ) -> ToolResult<()> {
        if let Some(active_layer) = document.active_layer_mut() {
            active_layer.move_by(delta_x, delta_y);
            debug!("Moved active layer by ({}, {})", delta_x, delta_y);
        }
        Ok(())
    }

    /// Move content within the selection area
    fn move_selection_content(
        &self,
        delta_x: f32,
        delta_y: f32,
        document: &mut Document,
    ) -> ToolResult<()> {
        // For now, we'll implement a simple approach: move the entire layer
        // In a more advanced implementation, we would:
        // 1. Extract pixels from the selection area
        // 2. Clear the original selection area
        // 3. Paste the pixels at the new location

        // For P3.5, we'll move the entire layer as a starting point
        self.move_active_layer(delta_x, delta_y, document)?;
        debug!("Moved selection content by ({}, {})", delta_x, delta_y);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Document, Layer, Point, RgbaPixel};

    #[test]
    fn test_brush_tool_creation() {
        let brush = BrushTool::new();

        assert_eq!(brush.brush_size, 10.0);
        assert_eq!(brush.brush_color, RgbaPixel::new(0, 0, 0, 255));
        assert_eq!(brush.brush_hardness, 1.0);
        assert!(!brush.is_painting);
    }

    #[test]
    fn test_brush_tool_options() {
        let brush = BrushTool::new();
        let options = brush.options();

        assert_eq!(options.len(), 3);

        // Check size option
        assert_eq!(options[0].name, "size");
        assert_eq!(options[0].display_name, "Brush Size");
        assert_eq!(options[0].default_value, ToolOptionValue::Float(10.0));

        // Check hardness option
        assert_eq!(options[1].name, "hardness");
        assert_eq!(options[1].display_name, "Brush Hardness");
        assert_eq!(options[1].default_value, ToolOptionValue::Float(1.0));

        // Check color option
        assert_eq!(options[2].name, "color");
        assert_eq!(options[2].display_name, "Brush Color");
        assert_eq!(
            options[2].default_value,
            ToolOptionValue::Color([0, 0, 0, 255])
        );
    }

    #[test]
    fn test_brush_tool_set_options() {
        let mut brush = BrushTool::new();

        // Test size option
        brush
            .set_option("size", ToolOptionValue::Float(20.0))
            .unwrap();
        assert_eq!(brush.brush_size, 20.0);

        // Test hardness option
        brush
            .set_option("hardness", ToolOptionValue::Float(0.5))
            .unwrap();
        assert_eq!(brush.brush_hardness, 0.5);

        // Test color option
        brush
            .set_option("color", ToolOptionValue::Color([255, 128, 64, 200]))
            .unwrap();
        assert_eq!(brush.brush_color, RgbaPixel::new(255, 128, 64, 200));

        // Test clamping
        brush
            .set_option("size", ToolOptionValue::Float(150.0))
            .unwrap();
        assert_eq!(brush.brush_size, 100.0); // Should be clamped to max

        brush
            .set_option("hardness", ToolOptionValue::Float(-0.5))
            .unwrap();
        assert_eq!(brush.brush_hardness, 0.0); // Should be clamped to min
    }

    #[test]
    fn test_brush_tool_get_options() {
        let mut brush = BrushTool::new();
        brush.brush_size = 25.0;
        brush.brush_hardness = 0.7;
        brush.brush_color = RgbaPixel::new(100, 150, 200, 180);

        assert_eq!(brush.get_option("size"), Some(ToolOptionValue::Float(25.0)));
        assert_eq!(
            brush.get_option("hardness"),
            Some(ToolOptionValue::Float(0.7))
        );
        assert_eq!(
            brush.get_option("color"),
            Some(ToolOptionValue::Color([100, 150, 200, 180]))
        );
        assert_eq!(brush.get_option("invalid"), None);
    }

    #[test]
    fn test_brush_tool_event_handling() {
        use super::super::tool_trait::{KeyModifiers, MouseButton, ToolEvent, ToolState};

        let mut brush = BrushTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Add a layer to paint on
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Test mouse pressed event
        let press_event = ToolEvent::MousePressed {
            position: Point::new(50.0, 50.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        brush
            .handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(brush.is_painting);
        assert!(state.is_active);
        assert_eq!(state.last_position, Some(Point::new(50.0, 50.0)));
        assert!(document.is_dirty);

        // Test mouse dragged event
        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(60.0, 60.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        brush
            .handle_event(drag_event, &mut document, &mut state)
            .unwrap();
        assert!(brush.is_painting);
        assert_eq!(state.last_position, Some(Point::new(60.0, 60.0)));

        // Test mouse released event
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(60.0, 60.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        brush
            .handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!brush.is_painting);
        assert!(!state.is_active);
    }

    #[test]
    fn test_brush_alpha_calculation() {
        let brush = BrushTool::new();

        // Test hard brush (hardness = 1.0)
        assert_eq!(brush.calculate_brush_alpha(0.0, 10.0), 1.0); // Center
        assert_eq!(brush.calculate_brush_alpha(5.0, 10.0), 1.0); // Within radius
        assert_eq!(brush.calculate_brush_alpha(10.0, 10.0), 0.0); // At edge
        assert_eq!(brush.calculate_brush_alpha(15.0, 10.0), 0.0); // Outside

        // Test soft brush (hardness = 0.0)
        let mut soft_brush = BrushTool::new();
        soft_brush.brush_hardness = 0.0;
        assert_eq!(soft_brush.calculate_brush_alpha(0.0, 10.0), 1.0); // Center
        assert!(soft_brush.calculate_brush_alpha(5.0, 10.0) > 0.0); // Within radius
        assert!(soft_brush.calculate_brush_alpha(5.0, 10.0) < 1.0); // But less than full
        assert_eq!(soft_brush.calculate_brush_alpha(10.0, 10.0), 0.0); // At edge
    }

    #[test]
    fn test_brush_normal_blending() {
        let brush = BrushTool::new();

        // Test blending with transparent background
        let transparent = RgbaPixel::transparent();
        let red = RgbaPixel::new(255, 0, 0, 255);
        let result = brush.blend_normal(transparent, red);
        assert_eq!(result, red);

        // Test blending with opaque background
        let blue = RgbaPixel::new(0, 0, 255, 255);
        let semi_red = RgbaPixel::new(255, 0, 0, 128);
        let result = brush.blend_normal(blue, semi_red);

        // Result should be a mix of blue and red
        assert!(result.r > 0);
        assert!(result.b > 0);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_brush_painting_on_layer() {
        let mut brush = BrushTool::new();
        brush.brush_size = 4.0; // Small brush for testing
        brush.brush_color = RgbaPixel::new(255, 0, 0, 255); // Red

        let mut document = Document::new("Test".to_string(), 20, 20);
        let layer = Layer::new_pixel("Test Layer".to_string(), 20, 20);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Paint at center
        let center = Point::new(10.0, 10.0);
        brush.paint_at_position(center, &mut document).unwrap();

        // Check that pixels were painted
        let active_layer = document.active_layer().unwrap();
        let center_pixel = active_layer.get_pixel(10, 10).unwrap();

        // Center pixel should be red (or close to it due to blending)
        assert!(center_pixel.r > 0);
        assert!(center_pixel.a > 0);
    }

    #[test]
    fn test_brush_stroke_painting() {
        let mut brush = BrushTool::new();
        brush.brush_size = 2.0;
        brush.brush_color = RgbaPixel::new(0, 255, 0, 255); // Green

        let mut document = Document::new("Test".to_string(), 50, 50);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Paint a stroke from (10, 10) to (20, 20)
        let from = Point::new(10.0, 10.0);
        let to = Point::new(20.0, 20.0);
        brush.paint_stroke(from, to, &mut document).unwrap();

        // Check that pixels along the stroke were painted
        let active_layer = document.active_layer().unwrap();

        // Start point should be painted
        let start_pixel = active_layer.get_pixel(10, 10).unwrap();
        assert!(start_pixel.g > 0);

        // End point should be painted
        let end_pixel = active_layer.get_pixel(20, 20).unwrap();
        assert!(end_pixel.g > 0);

        // Some point in between should be painted
        let mid_pixel = active_layer.get_pixel(15, 15).unwrap();
        assert!(mid_pixel.g > 0);
    }

    // Eraser Tool Tests
    #[test]
    fn test_eraser_tool_creation() {
        let eraser = EraserTool::new();

        assert_eq!(eraser.eraser_size, 10.0);
        assert_eq!(eraser.eraser_hardness, 1.0);
        assert!(!eraser.is_erasing);
    }

    #[test]
    fn test_eraser_tool_options() {
        let eraser = EraserTool::new();
        let options = eraser.options();

        assert_eq!(options.len(), 2);

        // Check size option
        assert_eq!(options[0].name, "size");
        assert_eq!(options[0].display_name, "Eraser Size");
        assert_eq!(options[0].default_value, ToolOptionValue::Float(10.0));

        // Check hardness option
        assert_eq!(options[1].name, "hardness");
        assert_eq!(options[1].display_name, "Eraser Hardness");
        assert_eq!(options[1].default_value, ToolOptionValue::Float(1.0));
    }

    #[test]
    fn test_eraser_tool_set_options() {
        let mut eraser = EraserTool::new();

        // Test size option
        eraser
            .set_option("size", ToolOptionValue::Float(25.0))
            .unwrap();
        assert_eq!(eraser.eraser_size, 25.0);

        // Test hardness option
        eraser
            .set_option("hardness", ToolOptionValue::Float(0.3))
            .unwrap();
        assert_eq!(eraser.eraser_hardness, 0.3);

        // Test clamping
        eraser
            .set_option("size", ToolOptionValue::Float(150.0))
            .unwrap();
        assert_eq!(eraser.eraser_size, 100.0); // Should be clamped to max

        eraser
            .set_option("hardness", ToolOptionValue::Float(-0.5))
            .unwrap();
        assert_eq!(eraser.eraser_hardness, 0.0); // Should be clamped to min
    }

    #[test]
    fn test_eraser_tool_get_options() {
        let mut eraser = EraserTool::new();
        eraser.eraser_size = 30.0;
        eraser.eraser_hardness = 0.8;

        assert_eq!(
            eraser.get_option("size"),
            Some(ToolOptionValue::Float(30.0))
        );
        assert_eq!(
            eraser.get_option("hardness"),
            Some(ToolOptionValue::Float(0.8))
        );
        assert_eq!(eraser.get_option("invalid"), None);
    }

    #[test]
    fn test_eraser_tool_event_handling() {
        use super::super::tool_trait::{KeyModifiers, MouseButton, ToolEvent, ToolState};

        let mut eraser = EraserTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Add a layer to erase on
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Test mouse pressed event
        let press_event = ToolEvent::MousePressed {
            position: Point::new(50.0, 50.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        eraser
            .handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(eraser.is_erasing);
        assert!(state.is_active);
        assert_eq!(state.last_position, Some(Point::new(50.0, 50.0)));
        assert!(document.is_dirty);

        // Test mouse dragged event
        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(60.0, 60.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        eraser
            .handle_event(drag_event, &mut document, &mut state)
            .unwrap();
        assert!(eraser.is_erasing);
        assert_eq!(state.last_position, Some(Point::new(60.0, 60.0)));

        // Test mouse released event
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(60.0, 60.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        eraser
            .handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!eraser.is_erasing);
        assert!(!state.is_active);
    }

    #[test]
    fn test_eraser_alpha_calculation() {
        let eraser = EraserTool::new();

        // Test hard eraser (hardness = 1.0)
        assert_eq!(eraser.calculate_eraser_alpha(0.0, 10.0), 1.0); // Center
        assert_eq!(eraser.calculate_eraser_alpha(5.0, 10.0), 1.0); // Within radius
        assert_eq!(eraser.calculate_eraser_alpha(10.0, 10.0), 0.0); // At edge
        assert_eq!(eraser.calculate_eraser_alpha(15.0, 10.0), 0.0); // Outside

        // Test soft eraser (hardness = 0.0)
        let mut soft_eraser = EraserTool::new();
        soft_eraser.eraser_hardness = 0.0;
        assert_eq!(soft_eraser.calculate_eraser_alpha(0.0, 10.0), 1.0); // Center
        assert!(soft_eraser.calculate_eraser_alpha(5.0, 10.0) > 0.0); // Within radius
        assert!(soft_eraser.calculate_eraser_alpha(5.0, 10.0) < 1.0); // But less than full
        assert_eq!(soft_eraser.calculate_eraser_alpha(10.0, 10.0), 0.0); // At edge
    }

    #[test]
    fn test_eraser_pixel_erasing() {
        let eraser = EraserTool::new();

        // Create a test layer with some opaque pixels
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);

        // Fill with red pixels
        for y in 0..10 {
            for x in 0..10 {
                layer
                    .set_pixel(x, y, RgbaPixel::new(255, 0, 0, 255))
                    .unwrap();
            }
        }

        // Test full erasing (strength = 1.0)
        eraser.erase_pixel_at(5, 5, 1.0, &mut layer).unwrap();
        let erased_pixel = layer.get_pixel(5, 5).unwrap();
        assert_eq!(erased_pixel.a, 0); // Should be fully transparent
        assert_eq!(erased_pixel.r, 255); // Color should remain

        // Test partial erasing (strength = 0.5)
        eraser.erase_pixel_at(6, 6, 0.5, &mut layer).unwrap();
        let partial_pixel = layer.get_pixel(6, 6).unwrap();
        assert!(partial_pixel.a > 0); // Should be partially transparent
        assert!(partial_pixel.a < 255); // But not fully opaque
        assert_eq!(partial_pixel.r, 255); // Color should remain
    }

    #[test]
    fn test_eraser_on_layer() {
        let mut eraser = EraserTool::new();
        eraser.eraser_size = 4.0; // Small eraser for testing

        let mut document = Document::new("Test".to_string(), 20, 20);
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 20, 20);

        // Fill layer with blue pixels
        for y in 0..20 {
            for x in 0..20 {
                layer
                    .set_pixel(x, y, RgbaPixel::new(0, 0, 255, 255))
                    .unwrap();
            }
        }

        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Erase at center
        let center = Point::new(10.0, 10.0);
        eraser.erase_at_position(center, &mut document).unwrap();

        // Check that pixels were erased
        let active_layer = document.active_layer().unwrap();
        let center_pixel = active_layer.get_pixel(10, 10).unwrap();

        // Center pixel should have reduced alpha
        assert!(center_pixel.a < 255);
        assert_eq!(center_pixel.b, 255); // Color should remain blue
    }

    #[test]
    fn test_eraser_stroke() {
        let mut eraser = EraserTool::new();
        eraser.eraser_size = 2.0;

        let mut document = Document::new("Test".to_string(), 50, 50);
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);

        // Fill layer with green pixels
        for y in 0..50 {
            for x in 0..50 {
                layer
                    .set_pixel(x, y, RgbaPixel::new(0, 255, 0, 255))
                    .unwrap();
            }
        }

        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Erase a stroke from (10, 10) to (20, 20)
        let from = Point::new(10.0, 10.0);
        let to = Point::new(20.0, 20.0);
        eraser.erase_stroke(from, to, &mut document).unwrap();

        // Check that pixels along the stroke were erased
        let active_layer = document.active_layer().unwrap();

        // Start point should be erased
        let start_pixel = active_layer.get_pixel(10, 10).unwrap();
        assert!(start_pixel.a < 255);

        // End point should be erased
        let end_pixel = active_layer.get_pixel(20, 20).unwrap();
        assert!(end_pixel.a < 255);

        // Some point in between should be erased
        let mid_pixel = active_layer.get_pixel(15, 15).unwrap();
        assert!(mid_pixel.a < 255);
    }

    // Move Tool Tests
    #[test]
    fn test_move_tool_creation() {
        let move_tool = MoveTool::new();

        assert!(!move_tool.is_moving);
        assert!(move_tool.move_start.is_none());
    }

    #[test]
    fn test_move_tool_properties() {
        let move_tool = MoveTool::new();

        assert_eq!(move_tool.id(), "move");
        assert_eq!(move_tool.name(), "Move Tool");
        assert_eq!(move_tool.description(), "Move layers and selections");
        assert_eq!(move_tool.cursor(), ToolCursor::Move);
        assert_eq!(move_tool.options().len(), 2);
    }

    #[test]
    fn test_move_tool_event_handling() {
        use super::super::tool_trait::{KeyModifiers, MouseButton, ToolEvent, ToolState};

        let mut move_tool = MoveTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Add a layer to move
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Test mouse pressed event
        let press_event = ToolEvent::MousePressed {
            position: Point::new(50.0, 50.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        move_tool
            .handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(move_tool.is_moving);
        assert!(state.is_active);
        assert_eq!(state.last_position, Some(Point::new(50.0, 50.0)));
        assert_eq!(move_tool.move_start, Some(Point::new(50.0, 50.0)));

        // Test mouse dragged event
        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(60.0, 60.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        move_tool
            .handle_event(drag_event, &mut document, &mut state)
            .unwrap();
        assert!(move_tool.is_moving);
        assert_eq!(state.last_position, Some(Point::new(60.0, 60.0)));
        assert_eq!(move_tool.move_start, Some(Point::new(60.0, 60.0))); // Updated for continuous movement
        assert!(document.is_dirty);

        // Test mouse released event
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(60.0, 60.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        move_tool
            .handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!move_tool.is_moving);
        assert!(!state.is_active);
        assert!(move_tool.move_start.is_none());
    }

    #[test]
    fn test_move_tool_layer_movement() {
        let move_tool = MoveTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add a layer to move
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Get initial layer position
        let initial_offset = document.active_layer().unwrap().offset;

        // Apply movement
        move_tool
            .move_active_layer(10.0, 20.0, &mut document)
            .unwrap();

        // Check that layer was moved
        let final_offset = document.active_layer().unwrap().offset;
        assert_eq!(final_offset.x, initial_offset.x + 10.0);
        assert_eq!(final_offset.y, initial_offset.y + 20.0);
    }

    #[test]
    fn test_move_tool_with_selection() {
        let mut move_tool = MoveTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add a layer to move
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Create a selection
        let selection = Selection::rectangle(10.0, 10.0, 40.0, 40.0);
        document.set_selection(selection);

        // Get initial layer position
        let initial_offset = document.active_layer().unwrap().offset;

        // Apply movement (should move selection content, which for now moves the entire layer)
        move_tool.apply_movement(15.0, 25.0, &mut document).unwrap();

        // Check that layer was moved
        let final_offset = document.active_layer().unwrap().offset;
        assert_eq!(final_offset.x, initial_offset.x + 15.0);
        assert_eq!(final_offset.y, initial_offset.y + 25.0);
        assert!(document.is_dirty);
    }

    #[test]
    fn test_move_tool_without_active_layer() {
        let move_tool = MoveTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // No layers added, so no active layer
        let result = move_tool.move_active_layer(10.0, 20.0, &mut document);
        assert!(result.is_ok()); // Should not fail, just do nothing
    }

    #[test]
    fn test_transform_tool_creation() {
        let transform = TransformTool::new();

        assert_eq!(transform.transform_mode(), TransformMode::Free);
        assert!(!transform.is_transforming);
        assert_eq!(
            transform.current_transform,
            psoc_core::Transform::identity()
        );
        assert!(transform.original_bounds.is_none());
    }

    #[test]
    fn test_transform_tool_properties() {
        let transform = TransformTool::new();

        assert_eq!(transform.id(), "transform");
        assert_eq!(transform.name(), "Transform Tool");
        assert_eq!(
            transform.description(),
            "Scale, rotate, and flip layers or selections"
        );
        assert_eq!(transform.cursor(), ToolCursor::Default);
    }

    #[test]
    fn test_transform_mode_setting() {
        let mut transform = TransformTool::new();

        transform.set_transform_mode(TransformMode::Scale);
        assert_eq!(transform.transform_mode(), TransformMode::Scale);

        transform.set_transform_mode(TransformMode::Rotate);
        assert_eq!(transform.transform_mode(), TransformMode::Rotate);

        transform.set_transform_mode(TransformMode::FlipHorizontal);
        assert_eq!(transform.transform_mode(), TransformMode::FlipHorizontal);
    }

    #[test]
    fn test_transform_tool_options() {
        let transform = TransformTool::new();
        let options = transform.options();

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].name, "mode");
        assert_eq!(options[0].display_name, "Transform Mode");

        // Test getting option
        let mode_option = transform.get_option("mode");
        assert!(mode_option.is_some());
        if let Some(ToolOptionValue::String(mode)) = mode_option {
            assert_eq!(mode, "Free");
        } else {
            panic!("Expected String option value");
        }
    }

    #[test]
    fn test_transform_tool_set_options() {
        let mut transform = TransformTool::new();

        // Set scale mode
        let result = transform.set_option("mode", ToolOptionValue::String("Scale".to_string()));
        assert!(result.is_ok());
        assert_eq!(transform.transform_mode(), TransformMode::Scale);

        // Set rotate mode
        let result = transform.set_option("mode", ToolOptionValue::String("Rotate".to_string()));
        assert!(result.is_ok());
        assert_eq!(transform.transform_mode(), TransformMode::Rotate);

        // Set invalid mode (should default to Free)
        let result = transform.set_option("mode", ToolOptionValue::String("Invalid".to_string()));
        assert!(result.is_ok());
        assert_eq!(transform.transform_mode(), TransformMode::Free);
    }

    #[test]
    fn test_transform_tool_with_layer() {
        let mut transform = TransformTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Start transformation
        let result = transform.start_transform(&document);
        assert!(result.is_ok());
        assert!(transform.is_transforming);
        assert!(transform.original_bounds.is_some());

        // Check original bounds
        if let Some(bounds) = transform.original_bounds {
            assert_eq!(bounds.width, 50.0);
            assert_eq!(bounds.height, 50.0);
        }
    }

    #[test]
    fn test_transform_tool_without_layer() {
        let mut transform = TransformTool::new();
        let document = Document::new("Test".to_string(), 100, 100);

        // Try to start transformation without active layer
        let result = transform.start_transform(&document);
        assert!(result.is_err());
        assert!(!transform.is_transforming);
    }

    #[test]
    fn test_transform_tool_event_handling() {
        let mut transform = TransformTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        let mut state = ToolState::default();

        // Mouse press should start transformation
        let press_event = ToolEvent::MousePressed {
            position: Point::new(25.0, 25.0),
            button: crate::tools::tool_trait::MouseButton::Left,
            modifiers: crate::tools::tool_trait::KeyModifiers::default(),
        };

        let result = transform.handle_event(press_event, &mut document, &mut state);
        assert!(result.is_ok());
        assert!(transform.is_transforming);
        assert!(state.is_active);

        // Mouse release should stop active state but keep transforming
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(30.0, 30.0),
            button: crate::tools::tool_trait::MouseButton::Left,
            modifiers: crate::tools::tool_trait::KeyModifiers::default(),
        };

        let result = transform.handle_event(release_event, &mut document, &mut state);
        assert!(result.is_ok());
        assert!(transform.is_transforming); // Still transforming
        assert!(!state.is_active); // But not actively dragging

        // Enter key should commit transformation
        let enter_event = ToolEvent::KeyPressed {
            key: crate::tools::tool_trait::Key::Enter,
            modifiers: crate::tools::tool_trait::KeyModifiers::default(),
        };

        let result = transform.handle_event(enter_event, &mut document, &mut state);
        assert!(result.is_ok());
        assert!(!transform.is_transforming); // Transformation committed
    }

    #[test]
    fn test_transform_tool_cancel() {
        let mut transform = TransformTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        let mut state = ToolState::default();

        // Start transformation
        let press_event = ToolEvent::MousePressed {
            position: Point::new(25.0, 25.0),
            button: crate::tools::tool_trait::MouseButton::Left,
            modifiers: crate::tools::tool_trait::KeyModifiers::default(),
        };

        let result = transform.handle_event(press_event, &mut document, &mut state);
        assert!(result.is_ok());
        assert!(transform.is_transforming);

        // Escape key should cancel transformation
        let escape_event = ToolEvent::KeyPressed {
            key: crate::tools::tool_trait::Key::Escape,
            modifiers: crate::tools::tool_trait::KeyModifiers::default(),
        };

        let result = transform.handle_event(escape_event, &mut document, &mut state);
        assert!(result.is_ok());
        assert!(!transform.is_transforming); // Transformation cancelled
    }
}

/// Transform tool for scaling, rotating, and flipping layers
#[derive(Debug)]
pub struct TransformTool {
    /// Current transformation being applied
    current_transform: psoc_core::Transform,
    /// Whether we're currently transforming
    is_transforming: bool,
    /// Transform mode (scale, rotate, etc.)
    transform_mode: TransformMode,
    /// Transform anchor point
    anchor_point: Point,
    /// Original bounds of the transform target
    original_bounds: Option<psoc_core::Rect>,
    /// Transform handles for UI interaction
    transform_handles: TransformHandles,
}

/// Transform modes available
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformMode {
    /// Free transform (scale and rotate)
    Free,
    /// Scale only
    Scale,
    /// Rotate only
    Rotate,
    /// Flip horizontal
    FlipHorizontal,
    /// Flip vertical
    FlipVertical,
}

/// Transform handles for UI interaction
#[derive(Debug, Clone)]
pub struct TransformHandles {
    /// Corner handles for scaling
    pub corners: [Point; 4],
    /// Edge handles for scaling
    pub edges: [Point; 4],
    /// Rotation handle
    pub rotation: Point,
    /// Center handle for moving anchor
    pub center: Point,
}

impl TransformTool {
    pub fn new() -> Self {
        Self {
            current_transform: psoc_core::Transform::identity(),
            is_transforming: false,
            transform_mode: TransformMode::Free,
            anchor_point: Point::new(0.0, 0.0),
            original_bounds: None,
            transform_handles: TransformHandles::default(),
        }
    }

    /// Set the transform mode
    pub fn set_transform_mode(&mut self, mode: TransformMode) {
        self.transform_mode = mode;
    }

    /// Get the current transform mode
    pub fn transform_mode(&self) -> TransformMode {
        self.transform_mode
    }

    /// Start a new transformation
    fn start_transform(&mut self, document: &Document) -> ToolResult<()> {
        // Get the bounds of the current selection or active layer
        if let psoc_core::Selection::Rectangle(rect_selection) = &document.selection {
            self.original_bounds = Some(rect_selection.bounds());
            self.anchor_point = rect_selection.bounds().center();
        } else if let Some(layer) = document.active_layer() {
            self.original_bounds = Some(layer.bounds);
            self.anchor_point = layer.bounds.center();
        } else {
            return Err(crate::PsocError::Tool {
                message: "No selection or active layer to transform".to_string(),
            });
        }

        self.current_transform = psoc_core::Transform::identity();
        self.update_transform_handles();
        self.is_transforming = true;

        Ok(())
    }

    /// Update transform handles based on current bounds
    fn update_transform_handles(&mut self) {
        if let Some(bounds) = self.original_bounds {
            let transformed_bounds = self.current_transform.transform_rect(bounds);

            // Corner handles
            self.transform_handles.corners = [
                transformed_bounds.top_left(),
                transformed_bounds.top_right(),
                transformed_bounds.bottom_right(),
                transformed_bounds.bottom_left(),
            ];

            // Edge handles (midpoints of edges)
            self.transform_handles.edges = [
                Point::new(
                    (transformed_bounds.x + transformed_bounds.x + transformed_bounds.width) / 2.0,
                    transformed_bounds.y,
                ), // Top
                Point::new(
                    transformed_bounds.x + transformed_bounds.width,
                    (transformed_bounds.y + transformed_bounds.y + transformed_bounds.height) / 2.0,
                ), // Right
                Point::new(
                    (transformed_bounds.x + transformed_bounds.x + transformed_bounds.width) / 2.0,
                    transformed_bounds.y + transformed_bounds.height,
                ), // Bottom
                Point::new(
                    transformed_bounds.x,
                    (transformed_bounds.y + transformed_bounds.y + transformed_bounds.height) / 2.0,
                ), // Left
            ];

            // Rotation handle (above the top edge)
            self.transform_handles.rotation = Point::new(
                (transformed_bounds.x + transformed_bounds.x + transformed_bounds.width) / 2.0,
                transformed_bounds.y - 20.0,
            );

            // Center handle
            self.transform_handles.center = transformed_bounds.center();
        }
    }

    /// Apply scale transformation
    fn apply_scale(&mut self, scale_x: f32, scale_y: f32) {
        let scale_transform = psoc_core::Transform::scale(scale_x, scale_y);
        self.current_transform = self.current_transform.then(&scale_transform);
        self.update_transform_handles();
    }

    /// Apply rotation transformation
    fn apply_rotation(&mut self, angle: f32) {
        let rotation_transform = psoc_core::Transform::rotation(angle);
        self.current_transform = self.current_transform.then(&rotation_transform);
        self.update_transform_handles();
    }

    /// Apply flip transformation
    fn apply_flip(&mut self, horizontal: bool) {
        let (scale_x, scale_y) = if horizontal { (-1.0, 1.0) } else { (1.0, -1.0) };
        self.apply_scale(scale_x, scale_y);
    }

    /// Commit the current transformation
    fn commit_transform(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_transforming {
            return Ok(());
        }

        // Apply transformation to the target (selection or active layer)
        if !document.selection.is_select_all() {
            // TODO: Apply transform to selection content
            debug!("Applying transform to selection (not yet implemented)");
        } else if let Some(layer) = document.active_layer_mut() {
            layer.apply_transform(self.current_transform);
            document.mark_dirty();
        }

        self.reset_transform();
        Ok(())
    }

    /// Cancel the current transformation
    fn cancel_transform(&mut self) {
        self.reset_transform();
    }

    /// Reset transformation state
    fn reset_transform(&mut self) {
        self.current_transform = psoc_core::Transform::identity();
        self.is_transforming = false;
        self.original_bounds = None;
        self.transform_handles = TransformHandles::default();
    }
}

impl Default for TransformTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TransformHandles {
    fn default() -> Self {
        let origin = Point::new(0.0, 0.0);
        Self {
            corners: [origin; 4],
            edges: [origin; 4],
            rotation: origin,
            center: origin,
        }
    }
}

impl Tool for TransformTool {
    fn id(&self) -> &'static str {
        "transform"
    }

    fn name(&self) -> &'static str {
        "Transform Tool"
    }

    fn description(&self) -> &'static str {
        "Scale, rotate, and flip layers or selections"
    }

    fn cursor(&self) -> ToolCursor {
        if self.is_transforming {
            match self.transform_mode {
                TransformMode::Scale => ToolCursor::Resize,
                TransformMode::Rotate => ToolCursor::Crosshair,
                _ => ToolCursor::Move,
            }
        } else {
            ToolCursor::Default
        }
    }

    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        state: &mut ToolState,
    ) -> ToolResult<()> {
        match event {
            ToolEvent::MousePressed { position, .. } => {
                debug!("Transform tool mouse pressed at: {:?}", position);

                if !self.is_transforming {
                    // Start a new transformation
                    self.start_transform(document)?;
                    state.is_active = true;
                } else {
                    // Check if clicking on transform handles
                    // For now, just continue with the transformation
                    state.is_active = true;
                }

                state.last_position = Some(position);
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_transforming && state.is_active {
                    debug!("Transform tool dragging to: {:?}", position);

                    if let Some(last_pos) = state.last_position {
                        // Calculate transformation based on drag
                        let dx = position.x - last_pos.x;
                        let dy = position.y - last_pos.y;

                        match self.transform_mode {
                            TransformMode::Scale => {
                                // Simple uniform scaling based on drag distance
                                let scale_factor = 1.0 + (dx + dy) * 0.01;
                                self.apply_scale(scale_factor, scale_factor);
                            }
                            TransformMode::Rotate => {
                                // Rotation based on angle from center
                                if let Some(bounds) = self.original_bounds {
                                    let center = bounds.center();
                                    let angle = (position.y - center.y)
                                        .atan2(position.x - center.x)
                                        - (last_pos.y - center.y).atan2(last_pos.x - center.x);
                                    self.apply_rotation(angle);
                                }
                            }
                            TransformMode::Free => {
                                // Free transform - scale based on drag
                                let scale_factor = 1.0 + (dx + dy) * 0.01;
                                self.apply_scale(scale_factor, scale_factor);
                            }
                            TransformMode::FlipHorizontal => {
                                self.apply_flip(true);
                                self.transform_mode = TransformMode::Free; // Reset to free after flip
                            }
                            TransformMode::FlipVertical => {
                                self.apply_flip(false);
                                self.transform_mode = TransformMode::Free; // Reset to free after flip
                            }
                        }
                    }

                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased { .. } => {
                if self.is_transforming && state.is_active {
                    debug!("Transform tool mouse released");
                    // Keep transformation active until explicitly committed or cancelled
                    state.is_active = false;
                }
            }
            ToolEvent::KeyPressed { key, .. } => {
                match key {
                    super::tool_trait::Key::Enter => {
                        // Commit transformation
                        if self.is_transforming {
                            debug!("Committing transformation");
                            self.commit_transform(document)?;
                        }
                    }
                    super::tool_trait::Key::Escape => {
                        // Cancel transformation
                        if self.is_transforming {
                            debug!("Cancelling transformation");
                            self.cancel_transform();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![ToolOption {
            name: "mode".to_string(),
            display_name: "Transform Mode".to_string(),
            description: "Type of transformation to apply".to_string(),
            option_type: ToolOptionType::Enum(vec![
                "Free".to_string(),
                "Scale".to_string(),
                "Rotate".to_string(),
                "Flip Horizontal".to_string(),
                "Flip Vertical".to_string(),
            ]),
            default_value: ToolOptionValue::String("Free".to_string()),
        }]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "mode" => {
                if let ToolOptionValue::String(mode_str) = value {
                    self.transform_mode = match mode_str.as_str() {
                        "Scale" => TransformMode::Scale,
                        "Rotate" => TransformMode::Rotate,
                        "Flip Horizontal" => TransformMode::FlipHorizontal,
                        "Flip Vertical" => TransformMode::FlipVertical,
                        _ => TransformMode::Free,
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "mode" => {
                let mode_str = match self.transform_mode {
                    TransformMode::Free => "Free",
                    TransformMode::Scale => "Scale",
                    TransformMode::Rotate => "Rotate",
                    TransformMode::FlipHorizontal => "Flip Horizontal",
                    TransformMode::FlipVertical => "Flip Vertical",
                };
                Some(ToolOptionValue::String(mode_str.to_string()))
            }
            _ => None,
        }
    }
}
