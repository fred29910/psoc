//! Concrete tool implementations
//!
//! This module contains the actual implementations of all editing tools,
//! including selection, brush, eraser, and move tools.

use tracing::debug;

use super::tool_trait::{
    Key, Tool, ToolCursor, ToolEvent, ToolOption, ToolOptionType, ToolOptionValue, ToolResult,
    ToolState,
};
use psoc_core::{Document, Point, RgbaPixel, Selection};
use serde::{Deserialize, Serialize};

/// Tool types available in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolType {
    Select,
    EllipseSelect,
    LassoSelect,
    MagicWand,
    Brush,
    Eraser,
    Move,
    Transform,
    Text,
    Gradient,
    // Shape tools
    Rectangle,
    Ellipse,
    Line,
    Polygon,
    // Crop tool
    Crop,
    // Eyedropper tool
    Eyedropper,
}

impl std::fmt::Display for ToolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolType::Select => write!(f, "Select"),
            ToolType::EllipseSelect => write!(f, "Ellipse Select"),
            ToolType::LassoSelect => write!(f, "Lasso Select"),
            ToolType::MagicWand => write!(f, "Magic Wand"),
            ToolType::Brush => write!(f, "Brush"),
            ToolType::Eraser => write!(f, "Eraser"),
            ToolType::Move => write!(f, "Move"),
            ToolType::Transform => write!(f, "Transform"),
            ToolType::Text => write!(f, "Text"),
            ToolType::Gradient => write!(f, "Gradient"),
            ToolType::Rectangle => write!(f, "Rectangle"),
            ToolType::Ellipse => write!(f, "Ellipse"),
            ToolType::Line => write!(f, "Line"),
            ToolType::Polygon => write!(f, "Polygon"),
            ToolType::Crop => write!(f, "Crop"),
            ToolType::Eyedropper => write!(f, "Eyedropper"),
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

/// Ellipse selection tool for making elliptical selections
#[derive(Debug)]
pub struct EllipseTool {
    selection_start: Option<Point>,
    is_selecting: bool,
    feather_radius: f32,
    anti_alias: bool,
}

impl EllipseTool {
    pub fn new() -> Self {
        Self {
            selection_start: None,
            is_selecting: false,
            feather_radius: 0.0,
            anti_alias: true,
        }
    }
}

impl Default for EllipseTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for EllipseTool {
    fn id(&self) -> &'static str {
        "ellipse_select"
    }

    fn name(&self) -> &'static str {
        "Ellipse Select Tool"
    }

    fn description(&self) -> &'static str {
        "Make elliptical selections"
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
                debug!("Ellipse selection started at: {:?}", position);
                self.selection_start = Some(position);
                self.is_selecting = true;
                state.is_active = true;
                state.last_position = Some(position);
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_selecting {
                    debug!("Ellipse selection dragged to: {:?}", position);
                    state.last_position = Some(position);

                    // Update preview selection
                    if let Some(start) = self.selection_start {
                        let selection = Selection::ellipse_from_points(start, position);
                        document.set_selection(selection);
                    }
                }
            }
            ToolEvent::MouseReleased { position, .. } => {
                if self.is_selecting {
                    debug!("Ellipse selection completed at: {:?}", position);
                    self.is_selecting = false;
                    state.is_active = false;

                    // Finalize selection
                    if let Some(start) = self.selection_start {
                        let selection = Selection::ellipse_from_points(start, position);
                        debug!("Created ellipse selection: {}", selection);
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

        // Check if we should paint on the mask or the layer
        // For now, we'll always paint on the layer data
        // TODO: Add mask editing support based on application state
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

/// Lasso selection tool for making freeform selections
#[derive(Debug)]
pub struct LassoTool {
    current_path: Vec<Point>,
    is_selecting: bool,
    feather_radius: f32,
    anti_alias: bool,
}

impl LassoTool {
    pub fn new() -> Self {
        Self {
            current_path: Vec::new(),
            is_selecting: false,
            feather_radius: 0.0,
            anti_alias: true,
        }
    }
}

impl Default for LassoTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for LassoTool {
    fn id(&self) -> &'static str {
        "lasso_select"
    }

    fn name(&self) -> &'static str {
        "Lasso Select Tool"
    }

    fn description(&self) -> &'static str {
        "Make freeform selections by drawing"
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
                debug!("Lasso selection started at: {:?}", position);
                self.current_path.clear();
                self.current_path.push(position);
                self.is_selecting = true;
                state.is_active = true;
                state.last_position = Some(position);
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_selecting {
                    debug!("Lasso selection continued to: {:?}", position);

                    // Add point to path if it's far enough from the last point
                    if let Some(last_point) = self.current_path.last() {
                        let distance = last_point.distance_to(&position);
                        if distance > 2.0 {
                            // Minimum distance to avoid too many points
                            self.current_path.push(position);
                        }
                    } else {
                        self.current_path.push(position);
                    }

                    state.last_position = Some(position);

                    // Update preview selection if we have enough points
                    if self.current_path.len() >= 3 {
                        let mut preview_path = self.current_path.clone();
                        // Close the path for preview
                        if let Some(first_point) = self.current_path.first() {
                            preview_path.push(*first_point);
                        }
                        let selection = Selection::lasso(preview_path);
                        document.set_selection(selection);
                    }
                }
            }
            ToolEvent::MouseReleased { position, .. } => {
                if self.is_selecting {
                    debug!("Lasso selection completed at: {:?}", position);
                    self.is_selecting = false;
                    state.is_active = false;

                    // Add final point if different from last
                    if let Some(last_point) = self.current_path.last() {
                        if last_point.distance_to(&position) > 1.0 {
                            self.current_path.push(position);
                        }
                    }

                    // Finalize selection if we have enough points
                    if self.current_path.len() >= 3 {
                        let mut final_path = self.current_path.clone();
                        // Close the path
                        if let Some(first_point) = self.current_path.first() {
                            final_path.push(*first_point);
                        }
                        let selection = Selection::lasso(final_path);
                        debug!("Created lasso selection: {}", selection);
                        document.set_selection(selection);
                    } else {
                        // Not enough points, clear selection
                        document.set_selection(Selection::None);
                    }

                    self.current_path.clear();
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

/// Magic Wand selection tool for selecting similar colors
#[derive(Debug)]
pub struct MagicWandTool {
    tolerance: f32,
    contiguous: bool,
    anti_alias: bool,
    sample_merged: bool,
}

impl MagicWandTool {
    pub fn new() -> Self {
        Self {
            tolerance: 32.0,
            contiguous: true,
            anti_alias: true,
            sample_merged: false,
        }
    }
}

impl Default for MagicWandTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for MagicWandTool {
    fn id(&self) -> &'static str {
        "magic_wand"
    }

    fn name(&self) -> &'static str {
        "Magic Wand Tool"
    }

    fn description(&self) -> &'static str {
        "Select areas of similar color"
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
                debug!("Magic wand selection at: {:?}", position);
                state.is_active = true;
                state.last_position = Some(position);

                // Perform magic wand selection
                self.perform_magic_wand_selection(position, document)?;
            }
            ToolEvent::MouseReleased { .. } => {
                if state.is_active {
                    debug!("Magic wand selection completed");
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
                name: "tolerance".to_string(),
                display_name: "Tolerance".to_string(),
                description: "How similar colors must be to be selected".to_string(),
                option_type: ToolOptionType::Float {
                    min: 0.0,
                    max: 255.0,
                },
                default_value: ToolOptionValue::Float(self.tolerance),
            },
            ToolOption {
                name: "contiguous".to_string(),
                display_name: "Contiguous".to_string(),
                description: "Only select connected areas".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(self.contiguous),
            },
            ToolOption {
                name: "anti_alias".to_string(),
                display_name: "Anti-alias".to_string(),
                description: "Smooth selection edges".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(self.anti_alias),
            },
            ToolOption {
                name: "sample_merged".to_string(),
                display_name: "Sample Merged".to_string(),
                description: "Sample from all visible layers".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(self.sample_merged),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "tolerance" => {
                if let ToolOptionValue::Float(tolerance) = value {
                    self.tolerance = tolerance.clamp(0.0, 255.0);
                }
            }
            "contiguous" => {
                if let ToolOptionValue::Bool(enabled) = value {
                    self.contiguous = enabled;
                }
            }
            "anti_alias" => {
                if let ToolOptionValue::Bool(enabled) = value {
                    self.anti_alias = enabled;
                }
            }
            "sample_merged" => {
                if let ToolOptionValue::Bool(enabled) = value {
                    self.sample_merged = enabled;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "tolerance" => Some(ToolOptionValue::Float(self.tolerance)),
            "contiguous" => Some(ToolOptionValue::Bool(self.contiguous)),
            "anti_alias" => Some(ToolOptionValue::Bool(self.anti_alias)),
            "sample_merged" => Some(ToolOptionValue::Bool(self.sample_merged)),
            _ => None,
        }
    }
}

impl MagicWandTool {
    /// Perform magic wand selection at the given position
    fn perform_magic_wand_selection(
        &self,
        position: Point,
        document: &mut Document,
    ) -> ToolResult<()> {
        // Get the active layer
        let active_layer = document.active_layer();
        if active_layer.is_none() {
            debug!("No active layer for magic wand selection");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        let layer_dims = layer.dimensions();
        if layer_dims.is_none() {
            return Ok(());
        }

        let (width, height) = layer_dims.unwrap();
        let x = position.x as u32;
        let y = position.y as u32;

        // Check bounds
        if x >= width || y >= height {
            debug!("Magic wand position out of bounds");
            return Ok(());
        }

        // Get the target color at the clicked position
        let target_color = layer
            .get_pixel(x, y)
            .unwrap_or(psoc_core::RgbaPixel::transparent());
        debug!("Magic wand target color: {:?}", target_color);

        // Create a mask for the selection
        let mut mask_data = vec![0u8; (width * height) as usize];

        if self.contiguous {
            // Flood fill algorithm for contiguous selection
            self.flood_fill_selection(x, y, target_color, layer, &mut mask_data, width, height)?;
        } else {
            // Select all similar colors regardless of connectivity
            self.select_all_similar(target_color, layer, &mut mask_data, width, height)?;
        }

        // Create mask selection
        let selection = Selection::mask(width, height, mask_data);
        debug!("Created magic wand selection: {}", selection);
        document.set_selection(selection);

        Ok(())
    }

    /// Flood fill algorithm for contiguous selection
    #[allow(clippy::too_many_arguments)]
    fn flood_fill_selection(
        &self,
        start_x: u32,
        start_y: u32,
        target_color: psoc_core::RgbaPixel,
        layer: &psoc_core::Layer,
        mask_data: &mut [u8],
        width: u32,
        height: u32,
    ) -> ToolResult<()> {
        let mut stack = Vec::new();
        stack.push((start_x, start_y));

        while let Some((x, y)) = stack.pop() {
            if x >= width || y >= height {
                continue;
            }

            let index = (y * width + x) as usize;
            if index >= mask_data.len() || mask_data[index] > 0 {
                continue; // Already processed
            }

            let pixel = layer
                .get_pixel(x, y)
                .unwrap_or(psoc_core::RgbaPixel::transparent());
            if !self.colors_similar(pixel, target_color) {
                continue;
            }

            // Mark as selected
            mask_data[index] = 255;

            // Add neighbors to stack
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < width - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < height - 1 {
                stack.push((x, y + 1));
            }
        }

        Ok(())
    }

    /// Select all similar colors regardless of connectivity
    fn select_all_similar(
        &self,
        target_color: psoc_core::RgbaPixel,
        layer: &psoc_core::Layer,
        mask_data: &mut [u8],
        width: u32,
        height: u32,
    ) -> ToolResult<()> {
        for y in 0..height {
            for x in 0..width {
                let pixel = layer
                    .get_pixel(x, y)
                    .unwrap_or(psoc_core::RgbaPixel::transparent());
                if self.colors_similar(pixel, target_color) {
                    let index = (y * width + x) as usize;
                    if index < mask_data.len() {
                        mask_data[index] = 255;
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if two colors are similar within tolerance
    fn colors_similar(&self, color1: psoc_core::RgbaPixel, color2: psoc_core::RgbaPixel) -> bool {
        let dr = (color1.r as f32 - color2.r as f32).abs();
        let dg = (color1.g as f32 - color2.g as f32).abs();
        let db = (color1.b as f32 - color2.b as f32).abs();
        let da = (color1.a as f32 - color2.a as f32).abs();

        // Calculate color distance (simple Euclidean distance in RGBA space)
        let distance = (dr * dr + dg * dg + db * db + da * da).sqrt();

        distance <= self.tolerance
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
        if name == "mode" {
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

#[cfg(test)]
mod new_selection_tools_tests {
    use super::*;
    use crate::tools::tool_trait::{KeyModifiers, MouseButton};
    use psoc_core::{Document, Point, Selection};

    #[test]
    fn test_ellipse_tool_creation() {
        let tool = EllipseTool::new();
        assert_eq!(tool.id(), "ellipse_select");
        assert_eq!(tool.name(), "Ellipse Select Tool");
        assert!(!tool.is_selecting);
        assert_eq!(tool.feather_radius, 0.0);
        assert!(tool.anti_alias);
    }

    #[test]
    fn test_ellipse_tool_options() {
        let tool = EllipseTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 2);

        let feather_option = &options[0];
        assert_eq!(feather_option.name, "feather");
        assert_eq!(feather_option.display_name, "Feather Radius");

        let anti_alias_option = &options[1];
        assert_eq!(anti_alias_option.name, "anti_alias");
        assert_eq!(anti_alias_option.display_name, "Anti-alias");
    }

    #[test]
    fn test_ellipse_tool_event_handling() {
        let mut tool = EllipseTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Test mouse press
        let press_event = ToolEvent::MousePressed {
            position: Point::new(20.0, 30.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_selecting);
        assert!(state.is_active);

        // Test mouse drag
        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(60.0, 70.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(drag_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_selecting);

        // Test mouse release
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(60.0, 70.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!tool.is_selecting);
        assert!(!state.is_active);

        // Check that ellipse selection was created
        assert!(document.has_selection());
        if let Selection::Ellipse(ellipse) = &document.selection {
            assert_eq!(ellipse.center.x, 40.0); // (20 + 60) / 2
            assert_eq!(ellipse.center.y, 50.0); // (30 + 70) / 2
            assert_eq!(ellipse.radius_x, 20.0); // |60 - 20| / 2
            assert_eq!(ellipse.radius_y, 20.0); // |70 - 30| / 2
        } else {
            panic!("Expected ellipse selection");
        }
    }

    #[test]
    fn test_lasso_tool_creation() {
        let tool = LassoTool::new();
        assert_eq!(tool.id(), "lasso_select");
        assert_eq!(tool.name(), "Lasso Select Tool");
        assert!(tool.current_path.is_empty());
        assert!(!tool.is_selecting);
        assert_eq!(tool.feather_radius, 0.0);
        assert!(tool.anti_alias);
    }

    #[test]
    fn test_lasso_tool_options() {
        let tool = LassoTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 2);

        let feather_option = &options[0];
        assert_eq!(feather_option.name, "feather");
        assert_eq!(feather_option.display_name, "Feather Radius");

        let anti_alias_option = &options[1];
        assert_eq!(anti_alias_option.name, "anti_alias");
        assert_eq!(anti_alias_option.display_name, "Anti-alias");
    }

    #[test]
    fn test_lasso_tool_event_handling() {
        let mut tool = LassoTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Test mouse press
        let press_event = ToolEvent::MousePressed {
            position: Point::new(10.0, 10.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_selecting);
        assert!(state.is_active);
        assert_eq!(tool.current_path.len(), 1);

        // Test mouse drag (multiple points)
        let drag_event1 = ToolEvent::MouseDragged {
            position: Point::new(15.0, 20.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(drag_event1, &mut document, &mut state)
            .unwrap();
        assert_eq!(tool.current_path.len(), 2);

        let drag_event2 = ToolEvent::MouseDragged {
            position: Point::new(25.0, 15.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(drag_event2, &mut document, &mut state)
            .unwrap();
        assert_eq!(tool.current_path.len(), 3);

        // Test mouse release
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(30.0, 10.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };
        tool.handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!tool.is_selecting);
        assert!(!state.is_active);
        assert!(tool.current_path.is_empty()); // Path should be cleared after completion

        // Check that lasso selection was created
        assert!(document.has_selection());
        if let Selection::Lasso(lasso) = &document.selection {
            assert!(lasso.points.len() >= 4); // At least 4 points (3 drawn + closing point)
        } else {
            panic!("Expected lasso selection");
        }
    }

    #[test]
    fn test_magic_wand_tool_creation() {
        let tool = MagicWandTool::new();
        assert_eq!(tool.id(), "magic_wand");
        assert_eq!(tool.name(), "Magic Wand Tool");
        assert_eq!(tool.tolerance, 32.0);
        assert!(tool.contiguous);
        assert!(tool.anti_alias);
        assert!(!tool.sample_merged);
    }

    #[test]
    fn test_magic_wand_tool_options() {
        let tool = MagicWandTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 4);

        let tolerance_option = &options[0];
        assert_eq!(tolerance_option.name, "tolerance");
        assert_eq!(tolerance_option.display_name, "Tolerance");

        let contiguous_option = &options[1];
        assert_eq!(contiguous_option.name, "contiguous");
        assert_eq!(contiguous_option.display_name, "Contiguous");

        let anti_alias_option = &options[2];
        assert_eq!(anti_alias_option.name, "anti_alias");
        assert_eq!(anti_alias_option.display_name, "Anti-alias");

        let sample_merged_option = &options[3];
        assert_eq!(sample_merged_option.name, "sample_merged");
        assert_eq!(sample_merged_option.display_name, "Sample Merged");
    }

    #[test]
    fn test_magic_wand_tool_set_options() {
        let mut tool = MagicWandTool::new();

        // Test tolerance setting
        tool.set_option("tolerance", ToolOptionValue::Float(50.0))
            .unwrap();
        assert_eq!(tool.tolerance, 50.0);

        // Test contiguous setting
        tool.set_option("contiguous", ToolOptionValue::Bool(false))
            .unwrap();
        assert!(!tool.contiguous);

        // Test anti_alias setting
        tool.set_option("anti_alias", ToolOptionValue::Bool(false))
            .unwrap();
        assert!(!tool.anti_alias);

        // Test sample_merged setting
        tool.set_option("sample_merged", ToolOptionValue::Bool(true))
            .unwrap();
        assert!(tool.sample_merged);
    }

    #[test]
    fn test_magic_wand_tool_get_options() {
        let tool = MagicWandTool::new();

        assert_eq!(
            tool.get_option("tolerance"),
            Some(ToolOptionValue::Float(32.0))
        );
        assert_eq!(
            tool.get_option("contiguous"),
            Some(ToolOptionValue::Bool(true))
        );
        assert_eq!(
            tool.get_option("anti_alias"),
            Some(ToolOptionValue::Bool(true))
        );
        assert_eq!(
            tool.get_option("sample_merged"),
            Some(ToolOptionValue::Bool(false))
        );
        assert_eq!(tool.get_option("nonexistent"), None);
    }

    #[test]
    fn test_magic_wand_colors_similar() {
        let tool = MagicWandTool::new();

        let color1 = psoc_core::RgbaPixel::new(100, 100, 100, 255);
        let color2 = psoc_core::RgbaPixel::new(110, 110, 110, 255);
        let color3 = psoc_core::RgbaPixel::new(200, 200, 200, 255);

        // Similar colors within tolerance
        assert!(tool.colors_similar(color1, color2));

        // Different colors outside tolerance
        assert!(!tool.colors_similar(color1, color3));

        // Same color
        assert!(tool.colors_similar(color1, color1));
    }

    // Text Tool Tests
    #[test]
    fn test_text_tool_creation() {
        let text_tool = TextTool::new();

        assert_eq!(text_tool.font_family, "Arial");
        assert_eq!(text_tool.font_size, 24.0);
        assert_eq!(text_tool.text_color, RgbaPixel::new(0, 0, 0, 255));
        assert_eq!(text_tool.text_alignment, TextAlignment::Left);
        assert!(!text_tool.is_editing);
        assert!(text_tool.current_text.is_empty());
        assert!(text_tool.text_position.is_none());
    }

    #[test]
    fn test_text_tool_properties() {
        let text_tool = TextTool::new();

        assert_eq!(text_tool.id(), "text");
        assert_eq!(text_tool.name(), "Text Tool");
        assert_eq!(text_tool.description(), "Add and edit text layers");
        assert_eq!(text_tool.cursor(), ToolCursor::Crosshair);
    }

    #[test]
    fn test_text_tool_options() {
        let text_tool = TextTool::new();
        let options = text_tool.options();

        assert_eq!(options.len(), 4);

        // Check font family option
        assert_eq!(options[0].name, "font_family");
        assert_eq!(options[0].display_name, "Font Family");

        // Check font size option
        assert_eq!(options[1].name, "font_size");
        assert_eq!(options[1].display_name, "Font Size");
        assert_eq!(options[1].default_value, ToolOptionValue::Float(24.0));

        // Check text color option
        assert_eq!(options[2].name, "text_color");
        assert_eq!(options[2].display_name, "Text Color");

        // Check alignment option
        assert_eq!(options[3].name, "alignment");
        assert_eq!(options[3].display_name, "Text Alignment");
    }

    #[test]
    fn test_text_tool_set_options() {
        let mut text_tool = TextTool::new();

        // Test font family option
        text_tool
            .set_option(
                "font_family",
                ToolOptionValue::String("Times New Roman".to_string()),
            )
            .unwrap();
        assert_eq!(text_tool.font_family, "Times New Roman");

        // Test font size option
        text_tool
            .set_option("font_size", ToolOptionValue::Float(36.0))
            .unwrap();
        assert_eq!(text_tool.font_size, 36.0);

        // Test text color option
        text_tool
            .set_option("text_color", ToolOptionValue::Color([255, 0, 0, 255]))
            .unwrap();
        assert_eq!(text_tool.text_color, RgbaPixel::new(255, 0, 0, 255));

        // Test alignment option
        text_tool
            .set_option("alignment", ToolOptionValue::String("Center".to_string()))
            .unwrap();
        assert_eq!(text_tool.text_alignment, TextAlignment::Center);

        // Test font size clamping
        text_tool
            .set_option("font_size", ToolOptionValue::Float(300.0))
            .unwrap();
        assert_eq!(text_tool.font_size, 200.0); // Should be clamped to max

        text_tool
            .set_option("font_size", ToolOptionValue::Float(5.0))
            .unwrap();
        assert_eq!(text_tool.font_size, 8.0); // Should be clamped to min
    }

    #[test]
    fn test_text_tool_get_options() {
        let mut text_tool = TextTool::new();
        text_tool.font_family = "Helvetica".to_string();
        text_tool.font_size = 18.0;
        text_tool.text_color = RgbaPixel::new(100, 150, 200, 255);
        text_tool.text_alignment = TextAlignment::Right;

        assert_eq!(
            text_tool.get_option("font_family"),
            Some(ToolOptionValue::String("Helvetica".to_string()))
        );
        assert_eq!(
            text_tool.get_option("font_size"),
            Some(ToolOptionValue::Float(18.0))
        );
        assert_eq!(
            text_tool.get_option("text_color"),
            Some(ToolOptionValue::Color([100, 150, 200, 255]))
        );
        assert_eq!(
            text_tool.get_option("alignment"),
            Some(ToolOptionValue::String("Right".to_string()))
        );
        assert_eq!(text_tool.get_option("invalid"), None);
    }

    #[test]
    fn test_text_tool_editing_workflow() {
        let mut text_tool = TextTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Start text editing
        let position = Point::new(50.0, 50.0);
        text_tool.start_text_editing(position);

        assert!(text_tool.is_editing);
        assert_eq!(text_tool.text_position, Some(position));
        assert!(text_tool.current_text.is_empty());
        assert_eq!(text_tool.cursor(), ToolCursor::Text);

        // Add some characters
        text_tool.add_character('H');
        text_tool.add_character('e');
        text_tool.add_character('l');
        text_tool.add_character('l');
        text_tool.add_character('o');

        assert_eq!(text_tool.current_text, "Hello");

        // Remove a character
        text_tool.remove_character();
        assert_eq!(text_tool.current_text, "Hell");

        // Finish editing
        let initial_layer_count = document.layer_count();
        text_tool.finish_text_editing(&mut document).unwrap();

        assert!(!text_tool.is_editing);
        assert!(text_tool.current_text.is_empty());
        assert!(text_tool.text_position.is_none());
        assert_eq!(document.layer_count(), initial_layer_count + 1);
        assert!(document.is_dirty);
    }

    #[test]
    fn test_text_tool_cancel_editing() {
        let mut text_tool = TextTool::new();
        let document = Document::new("Test".to_string(), 100, 100);

        // Start text editing and add some text
        text_tool.start_text_editing(Point::new(25.0, 25.0));
        text_tool.add_character('T');
        text_tool.add_character('e');
        text_tool.add_character('s');
        text_tool.add_character('t');

        assert!(text_tool.is_editing);
        assert_eq!(text_tool.current_text, "Test");

        // Cancel editing
        let initial_layer_count = document.layer_count();
        text_tool.cancel_text_editing();

        assert!(!text_tool.is_editing);
        assert!(text_tool.current_text.is_empty());
        assert!(text_tool.text_position.is_none());
        assert_eq!(document.layer_count(), initial_layer_count); // No layer should be added
    }

    #[test]
    fn test_text_tool_event_handling() {
        use super::super::tool_trait::{KeyModifiers, MouseButton, ToolEvent, ToolState};

        let mut text_tool = TextTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Test mouse pressed event (start editing)
        let press_event = ToolEvent::MousePressed {
            position: Point::new(30.0, 40.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        text_tool
            .handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(text_tool.is_editing);
        assert!(state.is_active);
        assert_eq!(text_tool.text_position, Some(Point::new(30.0, 40.0)));

        // Test character input
        let char_event = ToolEvent::KeyPressed {
            key: super::super::tool_trait::Key::Character('A'),
            modifiers: KeyModifiers::default(),
        };

        text_tool
            .handle_event(char_event, &mut document, &mut state)
            .unwrap();
        assert_eq!(text_tool.current_text, "A");

        // Test backspace
        let backspace_event = ToolEvent::KeyPressed {
            key: super::super::tool_trait::Key::Backspace,
            modifiers: KeyModifiers::default(),
        };

        text_tool
            .handle_event(backspace_event, &mut document, &mut state)
            .unwrap();
        assert!(text_tool.current_text.is_empty());

        // Add text again
        text_tool.add_character('H');
        text_tool.add_character('i');

        // Test Enter key (finish editing)
        let enter_event = ToolEvent::KeyPressed {
            key: super::super::tool_trait::Key::Enter,
            modifiers: KeyModifiers::default(),
        };

        let initial_layer_count = document.layer_count();
        text_tool
            .handle_event(enter_event, &mut document, &mut state)
            .unwrap();
        assert!(!text_tool.is_editing);
        assert!(!state.is_active);
        assert_eq!(document.layer_count(), initial_layer_count + 1);
    }

    #[test]
    fn test_text_tool_escape_cancel() {
        use super::super::tool_trait::{KeyModifiers, ToolEvent, ToolState};

        let mut text_tool = TextTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Start editing and add text
        text_tool.start_text_editing(Point::new(10.0, 20.0));
        text_tool.add_character('T');
        text_tool.add_character('e');
        text_tool.add_character('s');
        text_tool.add_character('t');
        state.is_active = true;

        assert!(text_tool.is_editing);
        assert_eq!(text_tool.current_text, "Test");

        // Test Escape key (cancel editing)
        let escape_event = ToolEvent::KeyPressed {
            key: super::super::tool_trait::Key::Escape,
            modifiers: KeyModifiers::default(),
        };

        let initial_layer_count = document.layer_count();
        text_tool
            .handle_event(escape_event, &mut document, &mut state)
            .unwrap();
        assert!(!text_tool.is_editing);
        assert!(!state.is_active);
        assert!(text_tool.current_text.is_empty());
        assert_eq!(document.layer_count(), initial_layer_count); // No layer added
    }
}

/// Text tool for adding and editing text layers
#[derive(Debug)]
pub struct TextTool {
    /// Current font family
    font_family: String,
    /// Current font size
    font_size: f32,
    /// Current text color
    text_color: RgbaPixel,
    /// Text alignment
    text_alignment: TextAlignment,
    /// Whether we're currently editing text
    is_editing: bool,
    /// Current text content being edited
    current_text: String,
    /// Position where text will be placed
    text_position: Option<Point>,
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl TextTool {
    pub fn new() -> Self {
        Self {
            font_family: "Arial".to_string(),
            font_size: 24.0,
            text_color: RgbaPixel::new(0, 0, 0, 255), // Black
            text_alignment: TextAlignment::Left,
            is_editing: false,
            current_text: String::new(),
            text_position: None,
        }
    }

    /// Start text editing at the given position
    fn start_text_editing(&mut self, position: Point) {
        self.text_position = Some(position);
        self.is_editing = true;
        self.current_text.clear();
        debug!("Started text editing at position: {:?}", position);
    }

    /// Add character to current text
    fn add_character(&mut self, ch: char) {
        if self.is_editing {
            self.current_text.push(ch);
            debug!("Added character '{}' to text: '{}'", ch, self.current_text);
        }
    }

    /// Remove last character from current text
    fn remove_character(&mut self) {
        if self.is_editing && !self.current_text.is_empty() {
            self.current_text.pop();
            debug!("Removed character, text now: '{}'", self.current_text);
        }
    }

    /// Finish text editing and create text layer
    fn finish_text_editing(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_editing || self.current_text.is_empty() {
            self.cancel_text_editing();
            return Ok(());
        }

        if let Some(position) = self.text_position {
            // Create a new text layer
            let layer_name = format!(
                "Text: {}",
                if self.current_text.len() > 20 {
                    format!("{}...", &self.current_text[..17])
                } else {
                    self.current_text.clone()
                }
            );

            let text_layer = psoc_core::Layer::new_text(
                layer_name,
                self.current_text.clone(),
                self.font_family.clone(),
                self.font_size,
                self.text_color,
                position,
            );

            document.add_layer(text_layer);
            document.mark_dirty();

            debug!("Created text layer with content: '{}'", self.current_text);
        }

        self.cancel_text_editing();
        Ok(())
    }

    /// Cancel text editing without creating layer
    fn cancel_text_editing(&mut self) {
        self.is_editing = false;
        self.current_text.clear();
        self.text_position = None;
        debug!("Cancelled text editing");
    }
}

impl Default for TextTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for TextTool {
    fn id(&self) -> &'static str {
        "text"
    }

    fn name(&self) -> &'static str {
        "Text Tool"
    }

    fn description(&self) -> &'static str {
        "Add and edit text layers"
    }

    fn cursor(&self) -> ToolCursor {
        if self.is_editing {
            ToolCursor::Text
        } else {
            ToolCursor::Crosshair
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
                debug!("Text tool mouse pressed at: {:?}", position);

                if self.is_editing {
                    // Finish current text editing
                    self.finish_text_editing(document)?;
                }

                // Start new text editing at clicked position
                self.start_text_editing(position);
                state.is_active = true;
                state.last_position = Some(position);
            }
            ToolEvent::KeyPressed { key, .. } => {
                if self.is_editing {
                    match key {
                        super::tool_trait::Key::Enter => {
                            // Finish text editing
                            self.finish_text_editing(document)?;
                            state.is_active = false;
                        }
                        super::tool_trait::Key::Escape => {
                            // Cancel text editing
                            self.cancel_text_editing();
                            state.is_active = false;
                        }
                        super::tool_trait::Key::Backspace => {
                            // Remove last character
                            self.remove_character();
                        }
                        super::tool_trait::Key::Character(ch) => {
                            // Add character to text
                            self.add_character(ch);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "font_family".to_string(),
                display_name: "Font Family".to_string(),
                description: "Font family for text".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Arial".to_string(),
                    "Times New Roman".to_string(),
                    "Helvetica".to_string(),
                    "Courier New".to_string(),
                    "Georgia".to_string(),
                    "Verdana".to_string(),
                ]),
                default_value: ToolOptionValue::String(self.font_family.clone()),
            },
            ToolOption {
                name: "font_size".to_string(),
                display_name: "Font Size".to_string(),
                description: "Size of the text in points".to_string(),
                option_type: ToolOptionType::Float {
                    min: 8.0,
                    max: 200.0,
                },
                default_value: ToolOptionValue::Float(self.font_size),
            },
            ToolOption {
                name: "text_color".to_string(),
                display_name: "Text Color".to_string(),
                description: "Color of the text".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.text_color.r,
                    self.text_color.g,
                    self.text_color.b,
                    self.text_color.a,
                ]),
            },
            ToolOption {
                name: "alignment".to_string(),
                display_name: "Text Alignment".to_string(),
                description: "Text alignment".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Left".to_string(),
                    "Center".to_string(),
                    "Right".to_string(),
                ]),
                default_value: ToolOptionValue::String("Left".to_string()),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "font_family" => {
                if let ToolOptionValue::String(family) = value {
                    self.font_family = family;
                }
            }
            "font_size" => {
                if let ToolOptionValue::Float(size) = value {
                    self.font_size = size.clamp(8.0, 200.0);
                }
            }
            "text_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.text_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "alignment" => {
                if let ToolOptionValue::String(align) = value {
                    self.text_alignment = match align.as_str() {
                        "Center" => TextAlignment::Center,
                        "Right" => TextAlignment::Right,
                        _ => TextAlignment::Left,
                    };
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "font_family" => Some(ToolOptionValue::String(self.font_family.clone())),
            "font_size" => Some(ToolOptionValue::Float(self.font_size)),
            "text_color" => Some(ToolOptionValue::Color([
                self.text_color.r,
                self.text_color.g,
                self.text_color.b,
                self.text_color.a,
            ])),
            "alignment" => {
                let align_str = match self.text_alignment {
                    TextAlignment::Left => "Left",
                    TextAlignment::Center => "Center",
                    TextAlignment::Right => "Right",
                };
                Some(ToolOptionValue::String(align_str.to_string()))
            }
            _ => None,
        }
    }
}

/// Gradient tool for creating and applying gradients
#[derive(Debug)]
pub struct GradientTool {
    /// Current gradient being edited
    current_gradient: psoc_core::Gradient,
    /// Whether we're currently creating a gradient
    is_creating: bool,
    /// Start point of gradient creation
    gradient_start: Option<Point>,
    /// End point of gradient creation
    gradient_end: Option<Point>,
    /// Gradient manager for storing gradients
    gradient_manager: psoc_core::GradientManager,
    /// Whether to apply gradient to selection only
    apply_to_selection: bool,
}

impl GradientTool {
    pub fn new() -> Self {
        Self {
            current_gradient: psoc_core::Gradient::default(),
            is_creating: false,
            gradient_start: None,
            gradient_end: None,
            gradient_manager: psoc_core::GradientManager::new(),
            apply_to_selection: false,
        }
    }

    /// Set the current gradient
    pub fn set_gradient(&mut self, gradient: psoc_core::Gradient) {
        self.current_gradient = gradient;
    }

    /// Get the current gradient
    pub fn current_gradient(&self) -> &psoc_core::Gradient {
        &self.current_gradient
    }

    /// Get mutable reference to current gradient
    pub fn current_gradient_mut(&mut self) -> &mut psoc_core::Gradient {
        &mut self.current_gradient
    }

    /// Get the gradient manager
    pub fn gradient_manager(&self) -> &psoc_core::GradientManager {
        &self.gradient_manager
    }

    /// Get mutable gradient manager
    pub fn gradient_manager_mut(&mut self) -> &mut psoc_core::GradientManager {
        &mut self.gradient_manager
    }

    /// Start creating a gradient
    fn start_gradient_creation(&mut self, start_point: Point) {
        self.is_creating = true;
        self.gradient_start = Some(start_point);
        self.gradient_end = None;

        // Set gradient start point
        self.current_gradient.start_point = start_point;
    }

    /// Update gradient creation
    fn update_gradient_creation(&mut self, end_point: Point) {
        if self.is_creating {
            self.gradient_end = Some(end_point);
            self.current_gradient.end_point = end_point;
        }
    }

    /// Finish gradient creation and apply to document
    fn finish_gradient_creation(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_creating {
            return Ok(());
        }

        if let (Some(start), Some(end)) = (self.gradient_start, self.gradient_end) {
            // Update gradient geometry
            match self.current_gradient.gradient_type {
                psoc_core::GradientType::Linear => {
                    self.current_gradient.set_linear_geometry(start, end);
                }
                psoc_core::GradientType::Radial => {
                    let radius = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt();
                    self.current_gradient.set_radial_geometry(start, radius);
                }
                _ => {
                    self.current_gradient.set_linear_geometry(start, end);
                }
            }

            // Apply gradient to document
            self.apply_gradient_to_document(document)?;
        }

        self.is_creating = false;
        self.gradient_start = None;
        self.gradient_end = None;

        Ok(())
    }

    /// Apply gradient to the document
    fn apply_gradient_to_document(&self, document: &mut Document) -> ToolResult<()> {
        // First, determine the region to apply gradient to
        let apply_region = if self.apply_to_selection && document.has_selection() {
            // Apply to selection bounds
            match &document.selection {
                psoc_core::Selection::Rectangle(rect_sel) => rect_sel.bounds(),
                psoc_core::Selection::Ellipse(ellipse_sel) => ellipse_sel.bounds(),
                psoc_core::Selection::Lasso(lasso_sel) => lasso_sel.bounds(),
                _ => {
                    // Get layer bounds without borrowing mutably
                    if let Some(layer) = document.active_layer() {
                        layer.bounds
                    } else {
                        debug!("No active layer to apply gradient to");
                        return Ok(());
                    }
                }
            }
        } else {
            // Apply to entire layer
            if let Some(layer) = document.active_layer() {
                layer.bounds
            } else {
                debug!("No active layer to apply gradient to");
                return Ok(());
            }
        };

        // Now get mutable access to the layer
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to apply gradient to");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Render gradient to the region
        let gradient_pixels = self
            .current_gradient
            .render_to_region(apply_region)
            .map_err(|e| crate::PsocError::Tool {
                message: format!("Failed to render gradient: {}", e),
            })?;

        // Apply gradient pixels to layer
        let mut pixel_index = 0;
        for y in 0..(apply_region.height as u32) {
            for x in 0..(apply_region.width as u32) {
                let layer_x = apply_region.x as u32 + x;
                let layer_y = apply_region.y as u32 + y;

                if pixel_index < gradient_pixels.len() {
                    let gradient_pixel = gradient_pixels[pixel_index];

                    // Blend gradient pixel with existing layer pixel
                    if let Some(existing_pixel) = layer.get_pixel(layer_x, layer_y) {
                        let blended = self.blend_gradient_pixel(existing_pixel, gradient_pixel);
                        layer.set_pixel(layer_x, layer_y, blended)?;
                    }

                    pixel_index += 1;
                }
            }
        }

        document.mark_dirty();
        Ok(())
    }

    /// Blend gradient pixel with existing pixel
    fn blend_gradient_pixel(&self, base: RgbaPixel, gradient: RgbaPixel) -> RgbaPixel {
        // Alpha blending formula: result = src * src_alpha + dst * (1 - src_alpha)
        let src_alpha = gradient.a as f32 / 255.0;
        let dst_alpha = base.a as f32 / 255.0;
        let inv_src_alpha = 1.0 - src_alpha;

        // Calculate final alpha
        let final_alpha = src_alpha + dst_alpha * inv_src_alpha;

        if final_alpha == 0.0 {
            return RgbaPixel::transparent();
        }

        // Calculate color components
        let r = (gradient.r as f32 * src_alpha + base.r as f32 * dst_alpha * inv_src_alpha)
            / final_alpha;
        let g = (gradient.g as f32 * src_alpha + base.g as f32 * dst_alpha * inv_src_alpha)
            / final_alpha;
        let b = (gradient.b as f32 * src_alpha + base.b as f32 * dst_alpha * inv_src_alpha)
            / final_alpha;

        RgbaPixel::new(
            r.clamp(0.0, 255.0) as u8,
            g.clamp(0.0, 255.0) as u8,
            b.clamp(0.0, 255.0) as u8,
            (final_alpha * 255.0).clamp(0.0, 255.0) as u8,
        )
    }

    /// Cancel gradient creation
    fn cancel_gradient_creation(&mut self) {
        self.is_creating = false;
        self.gradient_start = None;
        self.gradient_end = None;
    }
}

impl Default for GradientTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for GradientTool {
    fn id(&self) -> &'static str {
        "gradient"
    }

    fn name(&self) -> &'static str {
        "Gradient Tool"
    }

    fn description(&self) -> &'static str {
        "Create and apply linear and radial gradients"
    }

    fn cursor(&self) -> ToolCursor {
        if self.is_creating {
            ToolCursor::Crosshair
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
            ToolEvent::MousePressed {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left {
                    debug!("Gradient tool mouse pressed at: {:?}", position);
                    self.start_gradient_creation(position);
                    state.is_active = true;
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseDragged {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_creating {
                    debug!("Gradient tool dragging to: {:?}", position);
                    self.update_gradient_creation(position);
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_creating {
                    debug!("Gradient tool mouse released at: {:?}", position);
                    self.update_gradient_creation(position);
                    self.finish_gradient_creation(document)?;
                    state.is_active = false;
                }
            }
            ToolEvent::KeyPressed { key, .. } => {
                if key == super::tool_trait::Key::Escape {
                    if self.is_creating {
                        debug!("Cancelling gradient creation");
                        self.cancel_gradient_creation();
                        state.is_active = false;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "gradient_type".to_string(),
                display_name: "Gradient Type".to_string(),
                description: "Type of gradient to create".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Linear".to_string(),
                    "Radial".to_string(),
                    "Angular".to_string(),
                    "Diamond".to_string(),
                ]),
                default_value: ToolOptionValue::String("Linear".to_string()),
            },
            ToolOption {
                name: "interpolation".to_string(),
                display_name: "Interpolation".to_string(),
                description: "Color interpolation method".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Linear".to_string(),
                    "HSL".to_string(),
                    "HSV".to_string(),
                    "Smooth".to_string(),
                ]),
                default_value: ToolOptionValue::String("Linear".to_string()),
            },
            ToolOption {
                name: "repeat".to_string(),
                display_name: "Repeat".to_string(),
                description: "Whether the gradient repeats".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(false),
            },
            ToolOption {
                name: "apply_to_selection".to_string(),
                display_name: "Apply to Selection".to_string(),
                description: "Apply gradient only to selected area".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(false),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "gradient_type" => {
                if let ToolOptionValue::String(type_str) = value {
                    self.current_gradient.gradient_type = match type_str.as_str() {
                        "Radial" => psoc_core::GradientType::Radial,
                        "Angular" => psoc_core::GradientType::Angular,
                        "Diamond" => psoc_core::GradientType::Diamond,
                        _ => psoc_core::GradientType::Linear,
                    };
                }
            }
            "interpolation" => {
                if let ToolOptionValue::String(interp_str) = value {
                    self.current_gradient.interpolation = match interp_str.as_str() {
                        "HSL" => psoc_core::InterpolationMethod::Hsl,
                        "HSV" => psoc_core::InterpolationMethod::Hsv,
                        "Smooth" => psoc_core::InterpolationMethod::Smooth,
                        _ => psoc_core::InterpolationMethod::Linear,
                    };
                }
            }
            "repeat" => {
                if let ToolOptionValue::Bool(repeat) = value {
                    self.current_gradient.repeat = repeat;
                }
            }
            "apply_to_selection" => {
                if let ToolOptionValue::Bool(apply) = value {
                    self.apply_to_selection = apply;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "gradient_type" => {
                let type_str = match self.current_gradient.gradient_type {
                    psoc_core::GradientType::Linear => "Linear",
                    psoc_core::GradientType::Radial => "Radial",
                    psoc_core::GradientType::Angular => "Angular",
                    psoc_core::GradientType::Diamond => "Diamond",
                };
                Some(ToolOptionValue::String(type_str.to_string()))
            }
            "interpolation" => {
                let interp_str = match self.current_gradient.interpolation {
                    psoc_core::InterpolationMethod::Linear => "Linear",
                    psoc_core::InterpolationMethod::Hsl => "HSL",
                    psoc_core::InterpolationMethod::Hsv => "HSV",
                    psoc_core::InterpolationMethod::Smooth => "Smooth",
                };
                Some(ToolOptionValue::String(interp_str.to_string()))
            }
            "repeat" => Some(ToolOptionValue::Bool(self.current_gradient.repeat)),
            "apply_to_selection" => Some(ToolOptionValue::Bool(self.apply_to_selection)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod gradient_tool_tests {
    use super::*;
    use crate::tools::tool_trait::{KeyModifiers, MouseButton, ToolEvent, ToolState};
    use psoc_core::{Document, Layer, Point, RgbaPixel};

    #[test]
    fn test_gradient_tool_creation() {
        let tool = GradientTool::new();
        assert_eq!(tool.id(), "gradient");
        assert_eq!(tool.name(), "Gradient Tool");
        assert_eq!(
            tool.description(),
            "Create and apply linear and radial gradients"
        );
        assert!(!tool.is_creating);
        assert!(tool.gradient_start.is_none());
        assert!(tool.gradient_end.is_none());
        assert!(!tool.apply_to_selection);
    }

    #[test]
    fn test_gradient_tool_options() {
        let tool = GradientTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 4);

        let gradient_type_option = &options[0];
        assert_eq!(gradient_type_option.name, "gradient_type");
        assert_eq!(gradient_type_option.display_name, "Gradient Type");

        let interpolation_option = &options[1];
        assert_eq!(interpolation_option.name, "interpolation");
        assert_eq!(interpolation_option.display_name, "Interpolation");

        let repeat_option = &options[2];
        assert_eq!(repeat_option.name, "repeat");
        assert_eq!(repeat_option.display_name, "Repeat");

        let apply_to_selection_option = &options[3];
        assert_eq!(apply_to_selection_option.name, "apply_to_selection");
        assert_eq!(apply_to_selection_option.display_name, "Apply to Selection");
    }

    #[test]
    fn test_gradient_tool_set_options() {
        let mut tool = GradientTool::new();

        // Test gradient type option
        tool.set_option(
            "gradient_type",
            ToolOptionValue::String("Radial".to_string()),
        )
        .unwrap();
        assert_eq!(
            tool.current_gradient.gradient_type,
            psoc_core::GradientType::Radial
        );

        // Test interpolation option
        tool.set_option("interpolation", ToolOptionValue::String("HSL".to_string()))
            .unwrap();
        assert_eq!(
            tool.current_gradient.interpolation,
            psoc_core::InterpolationMethod::Hsl
        );

        // Test repeat option
        tool.set_option("repeat", ToolOptionValue::Bool(true))
            .unwrap();
        assert!(tool.current_gradient.repeat);

        // Test apply to selection option
        tool.set_option("apply_to_selection", ToolOptionValue::Bool(true))
            .unwrap();
        assert!(tool.apply_to_selection);
    }

    #[test]
    fn test_gradient_tool_get_options() {
        let mut tool = GradientTool::new();
        tool.current_gradient.gradient_type = psoc_core::GradientType::Radial;
        tool.current_gradient.interpolation = psoc_core::InterpolationMethod::Hsv;
        tool.current_gradient.repeat = true;
        tool.apply_to_selection = true;

        assert_eq!(
            tool.get_option("gradient_type"),
            Some(ToolOptionValue::String("Radial".to_string()))
        );
        assert_eq!(
            tool.get_option("interpolation"),
            Some(ToolOptionValue::String("HSV".to_string()))
        );
        assert_eq!(tool.get_option("repeat"), Some(ToolOptionValue::Bool(true)));
        assert_eq!(
            tool.get_option("apply_to_selection"),
            Some(ToolOptionValue::Bool(true))
        );
        assert_eq!(tool.get_option("invalid"), None);
    }

    #[test]
    fn test_gradient_tool_event_handling() {
        let mut tool = GradientTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Add a layer to apply gradient to
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Test mouse pressed event
        let press_event = ToolEvent::MousePressed {
            position: Point::new(20.0, 30.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        tool.handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_creating);
        assert!(state.is_active);
        assert_eq!(tool.gradient_start, Some(Point::new(20.0, 30.0)));
        assert_eq!(state.last_position, Some(Point::new(20.0, 30.0)));

        // Test mouse dragged event
        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(60.0, 70.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        tool.handle_event(drag_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_creating);
        assert_eq!(tool.gradient_end, Some(Point::new(60.0, 70.0)));
        assert_eq!(state.last_position, Some(Point::new(60.0, 70.0)));

        // Test mouse released event
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(60.0, 70.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        tool.handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!tool.is_creating);
        assert!(!state.is_active);
        assert!(tool.gradient_start.is_none());
        assert!(tool.gradient_end.is_none());
        assert!(document.is_dirty);
    }

    #[test]
    fn test_gradient_tool_cancel() {
        let mut tool = GradientTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Start gradient creation
        let press_event = ToolEvent::MousePressed {
            position: Point::new(10.0, 10.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        tool.handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_creating);

        // Cancel with Escape key
        let escape_event = ToolEvent::KeyPressed {
            key: crate::tools::tool_trait::Key::Escape,
            modifiers: KeyModifiers::default(),
        };

        tool.handle_event(escape_event, &mut document, &mut state)
            .unwrap();
        assert!(!tool.is_creating);
        assert!(!state.is_active);
        assert!(tool.gradient_start.is_none());
        assert!(tool.gradient_end.is_none());
    }

    #[test]
    fn test_gradient_tool_cursor() {
        let mut tool = GradientTool::new();
        assert_eq!(tool.cursor(), ToolCursor::Default);

        tool.is_creating = true;
        assert_eq!(tool.cursor(), ToolCursor::Crosshair);
    }

    #[test]
    fn test_gradient_tool_manager_integration() {
        let tool = GradientTool::new();
        assert!(tool.gradient_manager().count() > 0);

        let gradient_names = tool.gradient_manager().gradient_names();
        assert!(gradient_names.contains(&"Black to White".to_string()));
        assert!(gradient_names.contains(&"Rainbow".to_string()));
    }

    #[test]
    fn test_gradient_tool_blend_pixel() {
        let tool = GradientTool::new();

        // Test blending with transparent gradient
        let base = RgbaPixel::new(255, 0, 0, 255); // Red
        let gradient = RgbaPixel::new(0, 255, 0, 128); // Semi-transparent green
        let result = tool.blend_gradient_pixel(base, gradient);

        // Result should be a blend of red and green
        assert!(result.r > 0);
        assert!(result.g > 0);
        assert!(result.a > 0);

        // Test blending with opaque gradient
        let opaque_gradient = RgbaPixel::new(0, 0, 255, 255); // Blue
        let result = tool.blend_gradient_pixel(base, opaque_gradient);
        assert_eq!(result, opaque_gradient); // Should be completely blue

        // Test blending with transparent base
        let transparent_base = RgbaPixel::new(255, 0, 0, 0); // Transparent red
        let solid_gradient = RgbaPixel::new(0, 255, 0, 255); // Solid green
        let result = tool.blend_gradient_pixel(transparent_base, solid_gradient);
        assert_eq!(result, solid_gradient); // Should be completely green
    }
}

// ============================================================================
// SHAPE TOOLS IMPLEMENTATION
// ============================================================================

/// Shape drawing mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeMode {
    /// Draw shape outline only
    Stroke,
    /// Fill shape with solid color
    Fill,
    /// Both stroke and fill
    Both,
}

impl std::fmt::Display for ShapeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeMode::Stroke => write!(f, "Stroke"),
            ShapeMode::Fill => write!(f, "Fill"),
            ShapeMode::Both => write!(f, "Both"),
        }
    }
}

/// Rectangle tool for drawing rectangles
#[derive(Debug)]
pub struct RectangleTool {
    /// Start point of rectangle
    start_point: Option<Point>,
    /// End point of rectangle
    end_point: Option<Point>,
    /// Whether we're currently drawing
    is_drawing: bool,
    /// Shape drawing mode
    shape_mode: ShapeMode,
    /// Stroke color
    stroke_color: RgbaPixel,
    /// Fill color
    fill_color: RgbaPixel,
    /// Stroke width
    stroke_width: f32,
    /// Whether to maintain aspect ratio
    maintain_aspect_ratio: bool,
}

impl RectangleTool {
    pub fn new() -> Self {
        Self {
            start_point: None,
            end_point: None,
            is_drawing: false,
            shape_mode: ShapeMode::Stroke,
            stroke_color: RgbaPixel::new(0, 0, 0, 255), // Black
            fill_color: RgbaPixel::new(255, 255, 255, 255), // White
            stroke_width: 2.0,
            maintain_aspect_ratio: false,
        }
    }

    /// Start drawing a rectangle
    fn start_drawing(&mut self, start: Point) {
        self.start_point = Some(start);
        self.end_point = Some(start);
        self.is_drawing = true;
        debug!("Started drawing rectangle at: {:?}", start);
    }

    /// Update rectangle drawing
    fn update_drawing(&mut self, end: Point) {
        if self.is_drawing {
            let adjusted_end = if self.maintain_aspect_ratio {
                if let Some(start) = self.start_point {
                    // Make it a square by using the smaller dimension
                    let dx = (end.x - start.x).abs();
                    let dy = (end.y - start.y).abs();
                    let size = dx.min(dy);

                    let sign_x = if end.x >= start.x { 1.0 } else { -1.0 };
                    let sign_y = if end.y >= start.y { 1.0 } else { -1.0 };

                    Point::new(start.x + size * sign_x, start.y + size * sign_y)
                } else {
                    end
                }
            } else {
                end
            };

            self.end_point = Some(adjusted_end);
            debug!("Updated rectangle to: {:?}", adjusted_end);
        }
    }

    /// Finish drawing and create rectangle on layer
    fn finish_drawing(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_drawing {
            return Ok(());
        }

        if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
            self.draw_rectangle_on_layer(start, end, document)?;
        }

        self.is_drawing = false;
        self.start_point = None;
        self.end_point = None;
        Ok(())
    }

    /// Draw rectangle on the active layer
    fn draw_rectangle_on_layer(
        &self,
        start: Point,
        end: Point,
        document: &mut Document,
    ) -> ToolResult<()> {
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to draw rectangle on");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Calculate rectangle bounds
        let min_x = start.x.min(end.x) as i32;
        let max_x = start.x.max(end.x) as i32;
        let min_y = start.y.min(end.y) as i32;
        let max_y = start.y.max(end.y) as i32;

        // Draw based on shape mode
        match self.shape_mode {
            ShapeMode::Fill => {
                self.fill_rectangle(layer, min_x, min_y, max_x, max_y)?;
            }
            ShapeMode::Stroke => {
                self.stroke_rectangle(layer, min_x, min_y, max_x, max_y)?;
            }
            ShapeMode::Both => {
                self.fill_rectangle(layer, min_x, min_y, max_x, max_y)?;
                self.stroke_rectangle(layer, min_x, min_y, max_x, max_y)?;
            }
        }

        document.mark_dirty();
        Ok(())
    }

    /// Fill rectangle with solid color
    fn fill_rectangle(
        &self,
        layer: &mut psoc_core::Layer,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> ToolResult<()> {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x >= 0 && y >= 0 {
                    layer.set_pixel(x as u32, y as u32, self.fill_color)?;
                }
            }
        }
        Ok(())
    }

    /// Draw rectangle outline
    fn stroke_rectangle(
        &self,
        layer: &mut psoc_core::Layer,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> ToolResult<()> {
        let stroke_width = self.stroke_width as i32;

        // Draw top and bottom edges
        for x in min_x..=max_x {
            for offset in 0..stroke_width {
                // Top edge
                let top_y = min_y + offset;
                if x >= 0 && top_y >= 0 {
                    layer.set_pixel(x as u32, top_y as u32, self.stroke_color)?;
                }

                // Bottom edge
                let bottom_y = max_y - offset;
                if x >= 0 && bottom_y >= 0 && bottom_y != top_y {
                    layer.set_pixel(x as u32, bottom_y as u32, self.stroke_color)?;
                }
            }
        }

        // Draw left and right edges
        for y in min_y..=max_y {
            for offset in 0..stroke_width {
                // Left edge
                let left_x = min_x + offset;
                if left_x >= 0 && y >= 0 {
                    layer.set_pixel(left_x as u32, y as u32, self.stroke_color)?;
                }

                // Right edge
                let right_x = max_x - offset;
                if right_x >= 0 && y >= 0 && right_x != left_x {
                    layer.set_pixel(right_x as u32, y as u32, self.stroke_color)?;
                }
            }
        }

        Ok(())
    }

    /// Cancel drawing
    fn cancel_drawing(&mut self) {
        self.is_drawing = false;
        self.start_point = None;
        self.end_point = None;
        debug!("Cancelled rectangle drawing");
    }
}

impl Default for RectangleTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for RectangleTool {
    fn id(&self) -> &'static str {
        "rectangle"
    }

    fn name(&self) -> &'static str {
        "Rectangle Tool"
    }

    fn description(&self) -> &'static str {
        "Draw rectangles and squares"
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
            ToolEvent::MousePressed {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left {
                    debug!("Rectangle tool mouse pressed at: {:?}", position);

                    // Check for shift key to maintain aspect ratio
                    self.maintain_aspect_ratio = modifiers.shift;

                    self.start_drawing(position);
                    state.is_active = true;
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseDragged {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_drawing {
                    debug!("Rectangle tool dragging to: {:?}", position);

                    // Update aspect ratio constraint
                    self.maintain_aspect_ratio = modifiers.shift;

                    self.update_drawing(position);
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_drawing {
                    debug!("Rectangle tool mouse released at: {:?}", position);
                    self.update_drawing(position);
                    self.finish_drawing(document)?;
                    state.is_active = false;
                }
            }
            ToolEvent::KeyPressed { key, .. } => match key {
                super::tool_trait::Key::Escape => {
                    if self.is_drawing {
                        debug!("Cancelling rectangle drawing");
                        self.cancel_drawing();
                        state.is_active = false;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "shape_mode".to_string(),
                display_name: "Shape Mode".to_string(),
                description: "How to draw the rectangle".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Stroke".to_string(),
                    "Fill".to_string(),
                    "Both".to_string(),
                ]),
                default_value: ToolOptionValue::String("Stroke".to_string()),
            },
            ToolOption {
                name: "stroke_color".to_string(),
                display_name: "Stroke Color".to_string(),
                description: "Color of the rectangle outline".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.stroke_color.r,
                    self.stroke_color.g,
                    self.stroke_color.b,
                    self.stroke_color.a,
                ]),
            },
            ToolOption {
                name: "fill_color".to_string(),
                display_name: "Fill Color".to_string(),
                description: "Color to fill the rectangle".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.fill_color.r,
                    self.fill_color.g,
                    self.fill_color.b,
                    self.fill_color.a,
                ]),
            },
            ToolOption {
                name: "stroke_width".to_string(),
                display_name: "Stroke Width".to_string(),
                description: "Width of the rectangle outline".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 20.0,
                },
                default_value: ToolOptionValue::Float(self.stroke_width),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "shape_mode" => {
                if let ToolOptionValue::String(mode) = value {
                    self.shape_mode = match mode.as_str() {
                        "Fill" => ShapeMode::Fill,
                        "Both" => ShapeMode::Both,
                        _ => ShapeMode::Stroke,
                    };
                }
            }
            "stroke_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.stroke_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "fill_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.fill_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "stroke_width" => {
                if let ToolOptionValue::Float(width) = value {
                    self.stroke_width = width.clamp(1.0, 20.0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "shape_mode" => Some(ToolOptionValue::String(self.shape_mode.to_string())),
            "stroke_color" => Some(ToolOptionValue::Color([
                self.stroke_color.r,
                self.stroke_color.g,
                self.stroke_color.b,
                self.stroke_color.a,
            ])),
            "fill_color" => Some(ToolOptionValue::Color([
                self.fill_color.r,
                self.fill_color.g,
                self.fill_color.b,
                self.fill_color.a,
            ])),
            "stroke_width" => Some(ToolOptionValue::Float(self.stroke_width)),
            _ => None,
        }
    }
}

/// Ellipse tool for drawing ellipses and circles
#[derive(Debug)]
pub struct EllipseShapeTool {
    /// Start point of ellipse
    start_point: Option<Point>,
    /// End point of ellipse
    end_point: Option<Point>,
    /// Whether we're currently drawing
    is_drawing: bool,
    /// Shape drawing mode
    shape_mode: ShapeMode,
    /// Stroke color
    stroke_color: RgbaPixel,
    /// Fill color
    fill_color: RgbaPixel,
    /// Stroke width
    stroke_width: f32,
    /// Whether to maintain aspect ratio (circle)
    maintain_aspect_ratio: bool,
}

impl EllipseShapeTool {
    pub fn new() -> Self {
        Self {
            start_point: None,
            end_point: None,
            is_drawing: false,
            shape_mode: ShapeMode::Stroke,
            stroke_color: RgbaPixel::new(0, 0, 0, 255), // Black
            fill_color: RgbaPixel::new(255, 255, 255, 255), // White
            stroke_width: 2.0,
            maintain_aspect_ratio: false,
        }
    }

    /// Start drawing an ellipse
    fn start_drawing(&mut self, start: Point) {
        self.start_point = Some(start);
        self.end_point = Some(start);
        self.is_drawing = true;
        debug!("Started drawing ellipse at: {:?}", start);
    }

    /// Update ellipse drawing
    fn update_drawing(&mut self, end: Point) {
        if self.is_drawing {
            let adjusted_end = if self.maintain_aspect_ratio {
                if let Some(start) = self.start_point {
                    // Make it a circle by using the smaller dimension
                    let dx = (end.x - start.x).abs();
                    let dy = (end.y - start.y).abs();
                    let size = dx.min(dy);

                    let sign_x = if end.x >= start.x { 1.0 } else { -1.0 };
                    let sign_y = if end.y >= start.y { 1.0 } else { -1.0 };

                    Point::new(start.x + size * sign_x, start.y + size * sign_y)
                } else {
                    end
                }
            } else {
                end
            };

            self.end_point = Some(adjusted_end);
            debug!("Updated ellipse to: {:?}", adjusted_end);
        }
    }

    /// Finish drawing and create ellipse on layer
    fn finish_drawing(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_drawing {
            return Ok(());
        }

        if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
            self.draw_ellipse_on_layer(start, end, document)?;
        }

        self.is_drawing = false;
        self.start_point = None;
        self.end_point = None;
        Ok(())
    }

    /// Draw ellipse on the active layer
    fn draw_ellipse_on_layer(
        &self,
        start: Point,
        end: Point,
        document: &mut Document,
    ) -> ToolResult<()> {
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to draw ellipse on");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Calculate ellipse parameters
        let center_x = (start.x + end.x) / 2.0;
        let center_y = (start.y + end.y) / 2.0;
        let radius_x = (end.x - start.x).abs() / 2.0;
        let radius_y = (end.y - start.y).abs() / 2.0;

        // Draw based on shape mode
        match self.shape_mode {
            ShapeMode::Fill => {
                self.fill_ellipse(layer, center_x, center_y, radius_x, radius_y)?;
            }
            ShapeMode::Stroke => {
                self.stroke_ellipse(layer, center_x, center_y, radius_x, radius_y)?;
            }
            ShapeMode::Both => {
                self.fill_ellipse(layer, center_x, center_y, radius_x, radius_y)?;
                self.stroke_ellipse(layer, center_x, center_y, radius_x, radius_y)?;
            }
        }

        document.mark_dirty();
        Ok(())
    }

    /// Fill ellipse with solid color
    fn fill_ellipse(
        &self,
        layer: &mut psoc_core::Layer,
        center_x: f32,
        center_y: f32,
        radius_x: f32,
        radius_y: f32,
    ) -> ToolResult<()> {
        let min_x = (center_x - radius_x).floor() as i32;
        let max_x = (center_x + radius_x).ceil() as i32;
        let min_y = (center_y - radius_y).floor() as i32;
        let max_y = (center_y + radius_y).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x >= 0 && y >= 0 {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;

                    // Check if point is inside ellipse
                    let ellipse_eq =
                        (dx * dx) / (radius_x * radius_x) + (dy * dy) / (radius_y * radius_y);
                    if ellipse_eq <= 1.0 {
                        layer.set_pixel(x as u32, y as u32, self.fill_color)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Draw ellipse outline using Bresenham-like algorithm
    fn stroke_ellipse(
        &self,
        layer: &mut psoc_core::Layer,
        center_x: f32,
        center_y: f32,
        radius_x: f32,
        radius_y: f32,
    ) -> ToolResult<()> {
        let stroke_width = self.stroke_width;

        // Draw ellipse outline by checking distance from ellipse edge
        let min_x = (center_x - radius_x - stroke_width).floor() as i32;
        let max_x = (center_x + radius_x + stroke_width).ceil() as i32;
        let min_y = (center_y - radius_y - stroke_width).floor() as i32;
        let max_y = (center_y + radius_y + stroke_width).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x >= 0 && y >= 0 {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;

                    // Calculate distance from ellipse edge
                    let ellipse_eq =
                        (dx * dx) / (radius_x * radius_x) + (dy * dy) / (radius_y * radius_y);
                    let distance_from_edge =
                        (ellipse_eq.sqrt() - 1.0).abs() * radius_x.min(radius_y);

                    if distance_from_edge <= stroke_width / 2.0 {
                        layer.set_pixel(x as u32, y as u32, self.stroke_color)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Cancel drawing
    fn cancel_drawing(&mut self) {
        self.is_drawing = false;
        self.start_point = None;
        self.end_point = None;
        debug!("Cancelled ellipse drawing");
    }
}

impl Default for EllipseShapeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for EllipseShapeTool {
    fn id(&self) -> &'static str {
        "ellipse_shape"
    }

    fn name(&self) -> &'static str {
        "Ellipse Tool"
    }

    fn description(&self) -> &'static str {
        "Draw ellipses and circles"
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
            ToolEvent::MousePressed {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left {
                    debug!("Ellipse tool mouse pressed at: {:?}", position);

                    // Check for shift key to maintain aspect ratio (circle)
                    self.maintain_aspect_ratio = modifiers.shift;

                    self.start_drawing(position);
                    state.is_active = true;
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseDragged {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_drawing {
                    debug!("Ellipse tool dragging to: {:?}", position);

                    // Update aspect ratio constraint
                    self.maintain_aspect_ratio = modifiers.shift;

                    self.update_drawing(position);
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_drawing {
                    debug!("Ellipse tool mouse released at: {:?}", position);
                    self.update_drawing(position);
                    self.finish_drawing(document)?;
                    state.is_active = false;
                }
            }
            ToolEvent::KeyPressed { key, .. } => match key {
                super::tool_trait::Key::Escape => {
                    if self.is_drawing {
                        debug!("Cancelling ellipse drawing");
                        self.cancel_drawing();
                        state.is_active = false;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "shape_mode".to_string(),
                display_name: "Shape Mode".to_string(),
                description: "How to draw the ellipse".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Stroke".to_string(),
                    "Fill".to_string(),
                    "Both".to_string(),
                ]),
                default_value: ToolOptionValue::String("Stroke".to_string()),
            },
            ToolOption {
                name: "stroke_color".to_string(),
                display_name: "Stroke Color".to_string(),
                description: "Color of the ellipse outline".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.stroke_color.r,
                    self.stroke_color.g,
                    self.stroke_color.b,
                    self.stroke_color.a,
                ]),
            },
            ToolOption {
                name: "fill_color".to_string(),
                display_name: "Fill Color".to_string(),
                description: "Color to fill the ellipse".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.fill_color.r,
                    self.fill_color.g,
                    self.fill_color.b,
                    self.fill_color.a,
                ]),
            },
            ToolOption {
                name: "stroke_width".to_string(),
                display_name: "Stroke Width".to_string(),
                description: "Width of the ellipse outline".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 20.0,
                },
                default_value: ToolOptionValue::Float(self.stroke_width),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "shape_mode" => {
                if let ToolOptionValue::String(mode) = value {
                    self.shape_mode = match mode.as_str() {
                        "Fill" => ShapeMode::Fill,
                        "Both" => ShapeMode::Both,
                        _ => ShapeMode::Stroke,
                    };
                }
            }
            "stroke_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.stroke_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "fill_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.fill_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "stroke_width" => {
                if let ToolOptionValue::Float(width) = value {
                    self.stroke_width = width.clamp(1.0, 20.0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "shape_mode" => Some(ToolOptionValue::String(self.shape_mode.to_string())),
            "stroke_color" => Some(ToolOptionValue::Color([
                self.stroke_color.r,
                self.stroke_color.g,
                self.stroke_color.b,
                self.stroke_color.a,
            ])),
            "fill_color" => Some(ToolOptionValue::Color([
                self.fill_color.r,
                self.fill_color.g,
                self.fill_color.b,
                self.fill_color.a,
            ])),
            "stroke_width" => Some(ToolOptionValue::Float(self.stroke_width)),
            _ => None,
        }
    }
}

/// Line tool for drawing straight lines
#[derive(Debug)]
pub struct LineTool {
    /// Start point of line
    start_point: Option<Point>,
    /// End point of line
    end_point: Option<Point>,
    /// Whether we're currently drawing
    is_drawing: bool,
    /// Line color
    line_color: RgbaPixel,
    /// Line width
    line_width: f32,
    /// Whether to constrain to 45-degree angles
    constrain_angle: bool,
}

impl LineTool {
    pub fn new() -> Self {
        Self {
            start_point: None,
            end_point: None,
            is_drawing: false,
            line_color: RgbaPixel::new(0, 0, 0, 255), // Black
            line_width: 2.0,
            constrain_angle: false,
        }
    }

    /// Start drawing a line
    fn start_drawing(&mut self, start: Point) {
        self.start_point = Some(start);
        self.end_point = Some(start);
        self.is_drawing = true;
        debug!("Started drawing line at: {:?}", start);
    }

    /// Update line drawing
    fn update_drawing(&mut self, end: Point) {
        if self.is_drawing {
            let adjusted_end = if self.constrain_angle {
                if let Some(start) = self.start_point {
                    // Constrain to 45-degree angles
                    let dx = end.x - start.x;
                    let dy = end.y - start.y;
                    let abs_dx = dx.abs();
                    let abs_dy = dy.abs();

                    if abs_dx > abs_dy * 2.0 {
                        // Horizontal line
                        Point::new(end.x, start.y)
                    } else if abs_dy > abs_dx * 2.0 {
                        // Vertical line
                        Point::new(start.x, end.y)
                    } else {
                        // Diagonal line (45 degrees)
                        let size = abs_dx.min(abs_dy);
                        let sign_x = if dx >= 0.0 { 1.0 } else { -1.0 };
                        let sign_y = if dy >= 0.0 { 1.0 } else { -1.0 };
                        Point::new(start.x + size * sign_x, start.y + size * sign_y)
                    }
                } else {
                    end
                }
            } else {
                end
            };

            self.end_point = Some(adjusted_end);
            debug!("Updated line to: {:?}", adjusted_end);
        }
    }

    /// Finish drawing and create line on layer
    fn finish_drawing(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_drawing {
            return Ok(());
        }

        if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
            self.draw_line_on_layer(start, end, document)?;
        }

        self.is_drawing = false;
        self.start_point = None;
        self.end_point = None;
        Ok(())
    }

    /// Draw line on the active layer using Bresenham's algorithm
    fn draw_line_on_layer(
        &self,
        start: Point,
        end: Point,
        document: &mut Document,
    ) -> ToolResult<()> {
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to draw line on");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Use Bresenham's line algorithm with anti-aliasing
        self.draw_thick_line(layer, start, end)?;

        document.mark_dirty();
        Ok(())
    }

    /// Draw a thick line using multiple parallel thin lines
    fn draw_thick_line(
        &self,
        layer: &mut psoc_core::Layer,
        start: Point,
        end: Point,
    ) -> ToolResult<()> {
        let thickness = self.line_width;
        let half_thickness = thickness / 2.0;

        // Calculate perpendicular vector for thickness
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let length = (dx * dx + dy * dy).sqrt();

        if length == 0.0 {
            // Single point
            self.draw_circle_at(layer, start, half_thickness)?;
            return Ok(());
        }

        let perp_x = -dy / length * half_thickness;
        let perp_y = dx / length * half_thickness;

        // Draw multiple parallel lines to create thickness
        let steps = (thickness as i32).max(1);
        for i in 0..steps {
            let t = if steps == 1 {
                0.0
            } else {
                (i as f32) / (steps - 1) as f32 - 0.5
            };
            let offset_x = perp_x * t * 2.0;
            let offset_y = perp_y * t * 2.0;

            let line_start = Point::new(start.x + offset_x, start.y + offset_y);
            let line_end = Point::new(end.x + offset_x, end.y + offset_y);

            self.draw_thin_line(layer, line_start, line_end)?;
        }

        // Draw rounded end caps
        self.draw_circle_at(layer, start, half_thickness)?;
        self.draw_circle_at(layer, end, half_thickness)?;

        Ok(())
    }

    /// Draw a thin line using Bresenham's algorithm
    fn draw_thin_line(
        &self,
        layer: &mut psoc_core::Layer,
        start: Point,
        end: Point,
    ) -> ToolResult<()> {
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                layer.set_pixel(x as u32, y as u32, self.line_color)?;
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }

        Ok(())
    }

    /// Draw a filled circle at a point (for line caps)
    fn draw_circle_at(
        &self,
        layer: &mut psoc_core::Layer,
        center: Point,
        radius: f32,
    ) -> ToolResult<()> {
        let min_x = (center.x - radius).floor() as i32;
        let max_x = (center.x + radius).ceil() as i32;
        let min_y = (center.y - radius).floor() as i32;
        let max_y = (center.y + radius).ceil() as i32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x >= 0 && y >= 0 {
                    let dx = x as f32 - center.x;
                    let dy = y as f32 - center.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance <= radius {
                        layer.set_pixel(x as u32, y as u32, self.line_color)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Cancel drawing
    fn cancel_drawing(&mut self) {
        self.is_drawing = false;
        self.start_point = None;
        self.end_point = None;
        debug!("Cancelled line drawing");
    }
}

impl Default for LineTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for LineTool {
    fn id(&self) -> &'static str {
        "line"
    }

    fn name(&self) -> &'static str {
        "Line Tool"
    }

    fn description(&self) -> &'static str {
        "Draw straight lines"
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
            ToolEvent::MousePressed {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left {
                    debug!("Line tool mouse pressed at: {:?}", position);

                    // Check for shift key to constrain angles
                    self.constrain_angle = modifiers.shift;

                    self.start_drawing(position);
                    state.is_active = true;
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseDragged {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_drawing {
                    debug!("Line tool dragging to: {:?}", position);

                    // Update angle constraint
                    self.constrain_angle = modifiers.shift;

                    self.update_drawing(position);
                    state.last_position = Some(position);
                }
            }
            ToolEvent::MouseReleased {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left && self.is_drawing {
                    debug!("Line tool mouse released at: {:?}", position);
                    self.update_drawing(position);
                    self.finish_drawing(document)?;
                    state.is_active = false;
                }
            }
            ToolEvent::KeyPressed { key, .. } => match key {
                super::tool_trait::Key::Escape => {
                    if self.is_drawing {
                        debug!("Cancelling line drawing");
                        self.cancel_drawing();
                        state.is_active = false;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "line_color".to_string(),
                display_name: "Line Color".to_string(),
                description: "Color of the line".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.line_color.r,
                    self.line_color.g,
                    self.line_color.b,
                    self.line_color.a,
                ]),
            },
            ToolOption {
                name: "line_width".to_string(),
                display_name: "Line Width".to_string(),
                description: "Width of the line".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 50.0,
                },
                default_value: ToolOptionValue::Float(self.line_width),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "line_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.line_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "line_width" => {
                if let ToolOptionValue::Float(width) = value {
                    self.line_width = width.clamp(1.0, 50.0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "line_color" => Some(ToolOptionValue::Color([
                self.line_color.r,
                self.line_color.g,
                self.line_color.b,
                self.line_color.a,
            ])),
            "line_width" => Some(ToolOptionValue::Float(self.line_width)),
            _ => None,
        }
    }
}

/// Polygon tool for drawing polygons
#[derive(Debug)]
pub struct PolygonTool {
    /// Points of the polygon being drawn
    points: Vec<Point>,
    /// Whether we're currently drawing
    is_drawing: bool,
    /// Shape drawing mode
    shape_mode: ShapeMode,
    /// Stroke color
    stroke_color: RgbaPixel,
    /// Fill color
    fill_color: RgbaPixel,
    /// Stroke width
    stroke_width: f32,
    /// Minimum distance between points
    min_point_distance: f32,
}

impl PolygonTool {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            is_drawing: false,
            shape_mode: ShapeMode::Stroke,
            stroke_color: RgbaPixel::new(0, 0, 0, 255), // Black
            fill_color: RgbaPixel::new(255, 255, 255, 255), // White
            stroke_width: 2.0,
            min_point_distance: 5.0,
        }
    }

    /// Start drawing a polygon
    fn start_drawing(&mut self, start: Point) {
        self.points.clear();
        self.points.push(start);
        self.is_drawing = true;
        debug!("Started drawing polygon at: {:?}", start);
    }

    /// Add a point to the polygon
    fn add_point(&mut self, point: Point) -> bool {
        if !self.is_drawing {
            return false;
        }

        // Check if point is close to the first point (to close polygon)
        if self.points.len() >= 3 {
            let first_point = self.points[0];
            let distance =
                ((point.x - first_point.x).powi(2) + (point.y - first_point.y).powi(2)).sqrt();
            if distance <= self.min_point_distance * 2.0 {
                // Close the polygon
                return true;
            }
        }

        // Check minimum distance from last point
        if let Some(last_point) = self.points.last() {
            let distance =
                ((point.x - last_point.x).powi(2) + (point.y - last_point.y).powi(2)).sqrt();
            if distance < self.min_point_distance {
                return false; // Too close to last point
            }
        }

        self.points.push(point);
        debug!(
            "Added polygon point: {:?}, total points: {}",
            point,
            self.points.len()
        );
        false
    }

    /// Finish drawing and create polygon on layer
    fn finish_drawing(&mut self, document: &mut Document) -> ToolResult<()> {
        if !self.is_drawing || self.points.len() < 3 {
            self.cancel_drawing();
            return Ok(());
        }

        self.draw_polygon_on_layer(document)?;

        self.is_drawing = false;
        self.points.clear();
        Ok(())
    }

    /// Draw polygon on the active layer
    fn draw_polygon_on_layer(&self, document: &mut Document) -> ToolResult<()> {
        let active_layer = document.active_layer_mut();
        if active_layer.is_none() {
            debug!("No active layer to draw polygon on");
            return Ok(());
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(());
        }

        // Draw based on shape mode
        match self.shape_mode {
            ShapeMode::Fill => {
                self.fill_polygon(layer)?;
            }
            ShapeMode::Stroke => {
                self.stroke_polygon(layer)?;
            }
            ShapeMode::Both => {
                self.fill_polygon(layer)?;
                self.stroke_polygon(layer)?;
            }
        }

        document.mark_dirty();
        Ok(())
    }

    /// Fill polygon using scanline algorithm
    fn fill_polygon(&self, layer: &mut psoc_core::Layer) -> ToolResult<()> {
        if self.points.len() < 3 {
            return Ok(());
        }

        // Find bounding box
        let _min_x = self.points.iter().map(|p| p.x as i32).min().unwrap_or(0);
        let _max_x = self.points.iter().map(|p| p.x as i32).max().unwrap_or(0);
        let min_y = self.points.iter().map(|p| p.y as i32).min().unwrap_or(0);
        let max_y = self.points.iter().map(|p| p.y as i32).max().unwrap_or(0);

        // Scanline fill algorithm
        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            // Find intersections with polygon edges
            for i in 0..self.points.len() {
                let p1 = self.points[i];
                let p2 = self.points[(i + 1) % self.points.len()];

                let y1 = p1.y as i32;
                let y2 = p2.y as i32;

                if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                    // Edge crosses scanline
                    let x_intersect = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x_intersect as i32);
                }
            }

            // Sort intersections
            intersections.sort();

            // Fill between pairs of intersections
            for chunk in intersections.chunks(2) {
                if chunk.len() == 2 {
                    let start_x = chunk[0].max(0);
                    let end_x = chunk[1];

                    for x in start_x..=end_x {
                        if x >= 0 && y >= 0 {
                            layer.set_pixel(x as u32, y as u32, self.fill_color)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Draw polygon outline
    fn stroke_polygon(&self, layer: &mut psoc_core::Layer) -> ToolResult<()> {
        if self.points.len() < 2 {
            return Ok(());
        }

        // Draw lines between consecutive points
        for i in 0..self.points.len() {
            let start = self.points[i];
            let end = self.points[(i + 1) % self.points.len()];
            self.draw_thick_line(layer, start, end)?;
        }

        Ok(())
    }

    /// Draw a thick line (similar to LineTool implementation)
    fn draw_thick_line(
        &self,
        layer: &mut psoc_core::Layer,
        start: Point,
        end: Point,
    ) -> ToolResult<()> {
        let thickness = self.stroke_width;
        let half_thickness = thickness / 2.0;

        // Calculate perpendicular vector for thickness
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let length = (dx * dx + dy * dy).sqrt();

        if length == 0.0 {
            return Ok(());
        }

        let perp_x = -dy / length * half_thickness;
        let perp_y = dx / length * half_thickness;

        // Draw multiple parallel lines to create thickness
        let steps = (thickness as i32).max(1);
        for i in 0..steps {
            let t = if steps == 1 {
                0.0
            } else {
                (i as f32) / (steps - 1) as f32 - 0.5
            };
            let offset_x = perp_x * t * 2.0;
            let offset_y = perp_y * t * 2.0;

            let line_start = Point::new(start.x + offset_x, start.y + offset_y);
            let line_end = Point::new(end.x + offset_x, end.y + offset_y);

            self.draw_thin_line(layer, line_start, line_end)?;
        }

        Ok(())
    }

    /// Draw a thin line using Bresenham's algorithm
    fn draw_thin_line(
        &self,
        layer: &mut psoc_core::Layer,
        start: Point,
        end: Point,
    ) -> ToolResult<()> {
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                layer.set_pixel(x as u32, y as u32, self.stroke_color)?;
            }

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }

        Ok(())
    }

    /// Cancel drawing
    fn cancel_drawing(&mut self) {
        self.is_drawing = false;
        self.points.clear();
        debug!("Cancelled polygon drawing");
    }
}

impl Default for PolygonTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for PolygonTool {
    fn id(&self) -> &'static str {
        "polygon"
    }

    fn name(&self) -> &'static str {
        "Polygon Tool"
    }

    fn description(&self) -> &'static str {
        "Draw polygons by clicking points"
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
            ToolEvent::MousePressed {
                position, button, ..
            } => {
                if button == super::tool_trait::MouseButton::Left {
                    debug!("Polygon tool mouse pressed at: {:?}", position);

                    if !self.is_drawing {
                        // Start new polygon
                        self.start_drawing(position);
                        state.is_active = true;
                    } else {
                        // Add point or close polygon
                        let should_close = self.add_point(position);
                        if should_close {
                            self.finish_drawing(document)?;
                            state.is_active = false;
                        }
                    }
                    state.last_position = Some(position);
                }
            }
            // Note: Double-click functionality removed as MouseDoubleClicked event is not available
            // Users can use Enter key or click near start point to finish polygon
            ToolEvent::KeyPressed { key, .. } => match key {
                super::tool_trait::Key::Escape => {
                    if self.is_drawing {
                        debug!("Cancelling polygon drawing");
                        self.cancel_drawing();
                        state.is_active = false;
                    }
                }
                super::tool_trait::Key::Enter => {
                    if self.is_drawing {
                        debug!("Enter pressed, finishing polygon");
                        self.finish_drawing(document)?;
                        state.is_active = false;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "shape_mode".to_string(),
                display_name: "Shape Mode".to_string(),
                description: "How to draw the polygon".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Stroke".to_string(),
                    "Fill".to_string(),
                    "Both".to_string(),
                ]),
                default_value: ToolOptionValue::String("Stroke".to_string()),
            },
            ToolOption {
                name: "stroke_color".to_string(),
                display_name: "Stroke Color".to_string(),
                description: "Color of the polygon outline".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.stroke_color.r,
                    self.stroke_color.g,
                    self.stroke_color.b,
                    self.stroke_color.a,
                ]),
            },
            ToolOption {
                name: "fill_color".to_string(),
                display_name: "Fill Color".to_string(),
                description: "Color to fill the polygon".to_string(),
                option_type: ToolOptionType::Color,
                default_value: ToolOptionValue::Color([
                    self.fill_color.r,
                    self.fill_color.g,
                    self.fill_color.b,
                    self.fill_color.a,
                ]),
            },
            ToolOption {
                name: "stroke_width".to_string(),
                display_name: "Stroke Width".to_string(),
                description: "Width of the polygon outline".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 20.0,
                },
                default_value: ToolOptionValue::Float(self.stroke_width),
            },
            ToolOption {
                name: "min_point_distance".to_string(),
                display_name: "Min Point Distance".to_string(),
                description: "Minimum distance between polygon points".to_string(),
                option_type: ToolOptionType::Float {
                    min: 1.0,
                    max: 50.0,
                },
                default_value: ToolOptionValue::Float(self.min_point_distance),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "shape_mode" => {
                if let ToolOptionValue::String(mode) = value {
                    self.shape_mode = match mode.as_str() {
                        "Fill" => ShapeMode::Fill,
                        "Both" => ShapeMode::Both,
                        _ => ShapeMode::Stroke,
                    };
                }
            }
            "stroke_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.stroke_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "fill_color" => {
                if let ToolOptionValue::Color([r, g, b, a]) = value {
                    self.fill_color = RgbaPixel::new(r, g, b, a);
                }
            }
            "stroke_width" => {
                if let ToolOptionValue::Float(width) = value {
                    self.stroke_width = width.clamp(1.0, 20.0);
                }
            }
            "min_point_distance" => {
                if let ToolOptionValue::Float(distance) = value {
                    self.min_point_distance = distance.clamp(1.0, 50.0);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "shape_mode" => Some(ToolOptionValue::String(self.shape_mode.to_string())),
            "stroke_color" => Some(ToolOptionValue::Color([
                self.stroke_color.r,
                self.stroke_color.g,
                self.stroke_color.b,
                self.stroke_color.a,
            ])),
            "fill_color" => Some(ToolOptionValue::Color([
                self.fill_color.r,
                self.fill_color.g,
                self.fill_color.b,
                self.fill_color.a,
            ])),
            "stroke_width" => Some(ToolOptionValue::Float(self.stroke_width)),
            "min_point_distance" => Some(ToolOptionValue::Float(self.min_point_distance)),
            _ => None,
        }
    }
}

// ============================================================================
// SHAPE TOOLS TESTS
// ============================================================================

#[cfg(test)]
mod shape_tool_tests {
    use super::*;
    use psoc_core::{Point, RgbaPixel};

    #[test]
    fn test_rectangle_tool_creation() {
        let tool = RectangleTool::new();
        assert_eq!(tool.id(), "rectangle");
        assert_eq!(tool.name(), "Rectangle Tool");
        assert!(!tool.is_drawing);
        assert_eq!(tool.shape_mode, ShapeMode::Stroke);
    }

    #[test]
    fn test_rectangle_tool_options() {
        let tool = RectangleTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 4);

        // Check that all expected options are present
        let option_names: Vec<&str> = options.iter().map(|o| o.name.as_str()).collect();
        assert!(option_names.contains(&"shape_mode"));
        assert!(option_names.contains(&"stroke_color"));
        assert!(option_names.contains(&"fill_color"));
        assert!(option_names.contains(&"stroke_width"));
    }

    #[test]
    fn test_rectangle_tool_set_option() {
        let mut tool = RectangleTool::new();

        // Test setting shape mode
        tool.set_option("shape_mode", ToolOptionValue::String("Fill".to_string()))
            .unwrap();
        assert_eq!(tool.shape_mode, ShapeMode::Fill);

        // Test setting stroke color
        tool.set_option("stroke_color", ToolOptionValue::Color([255, 0, 0, 255]))
            .unwrap();
        assert_eq!(tool.stroke_color, RgbaPixel::new(255, 0, 0, 255));

        // Test setting stroke width
        tool.set_option("stroke_width", ToolOptionValue::Float(5.0))
            .unwrap();
        assert_eq!(tool.stroke_width, 5.0);
    }

    #[test]
    fn test_ellipse_tool_creation() {
        let tool = EllipseShapeTool::new();
        assert_eq!(tool.id(), "ellipse_shape");
        assert_eq!(tool.name(), "Ellipse Tool");
        assert!(!tool.is_drawing);
        assert_eq!(tool.shape_mode, ShapeMode::Stroke);
    }

    #[test]
    fn test_ellipse_tool_options() {
        let tool = EllipseShapeTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 4);

        // Check that all expected options are present
        let option_names: Vec<&str> = options.iter().map(|o| o.name.as_str()).collect();
        assert!(option_names.contains(&"shape_mode"));
        assert!(option_names.contains(&"stroke_color"));
        assert!(option_names.contains(&"fill_color"));
        assert!(option_names.contains(&"stroke_width"));
    }

    #[test]
    fn test_line_tool_creation() {
        let tool = LineTool::new();
        assert_eq!(tool.id(), "line");
        assert_eq!(tool.name(), "Line Tool");
        assert!(!tool.is_drawing);
        assert_eq!(tool.line_color, RgbaPixel::new(0, 0, 0, 255));
        assert_eq!(tool.line_width, 2.0);
    }

    #[test]
    fn test_line_tool_options() {
        let tool = LineTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 2);

        // Check that all expected options are present
        let option_names: Vec<&str> = options.iter().map(|o| o.name.as_str()).collect();
        assert!(option_names.contains(&"line_color"));
        assert!(option_names.contains(&"line_width"));
    }

    #[test]
    fn test_polygon_tool_creation() {
        let tool = PolygonTool::new();
        assert_eq!(tool.id(), "polygon");
        assert_eq!(tool.name(), "Polygon Tool");
        assert!(!tool.is_drawing);
        assert_eq!(tool.shape_mode, ShapeMode::Stroke);
        assert!(tool.points.is_empty());
    }

    #[test]
    fn test_polygon_tool_options() {
        let tool = PolygonTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 5);

        // Check that all expected options are present
        let option_names: Vec<&str> = options.iter().map(|o| o.name.as_str()).collect();
        assert!(option_names.contains(&"shape_mode"));
        assert!(option_names.contains(&"stroke_color"));
        assert!(option_names.contains(&"fill_color"));
        assert!(option_names.contains(&"stroke_width"));
        assert!(option_names.contains(&"min_point_distance"));
    }

    #[test]
    fn test_shape_mode_display() {
        assert_eq!(ShapeMode::Stroke.to_string(), "Stroke");
        assert_eq!(ShapeMode::Fill.to_string(), "Fill");
        assert_eq!(ShapeMode::Both.to_string(), "Both");
    }

    #[test]
    fn test_rectangle_tool_drawing_state() {
        let mut tool = RectangleTool::new();
        let start_point = Point::new(10.0, 20.0);

        // Test start drawing
        tool.start_drawing(start_point);
        assert!(tool.is_drawing);
        assert_eq!(tool.start_point, Some(start_point));
        assert_eq!(tool.end_point, Some(start_point));

        // Test update drawing
        let end_point = Point::new(50.0, 60.0);
        tool.update_drawing(end_point);
        assert_eq!(tool.end_point, Some(end_point));

        // Test cancel drawing
        tool.cancel_drawing();
        assert!(!tool.is_drawing);
        assert_eq!(tool.start_point, None);
        assert_eq!(tool.end_point, None);
    }

    #[test]
    fn test_line_tool_angle_constraint() {
        let mut tool = LineTool::new();
        let start_point = Point::new(10.0, 10.0);

        tool.start_drawing(start_point);
        tool.constrain_angle = true;

        // Test horizontal constraint
        let horizontal_end = Point::new(50.0, 15.0); // Slight vertical offset
        tool.update_drawing(horizontal_end);
        if let Some(end) = tool.end_point {
            assert_eq!(end.y, start_point.y); // Should be constrained to horizontal
        }
    }

    #[test]
    fn test_polygon_tool_point_management() {
        let mut tool = PolygonTool::new();
        let start_point = Point::new(10.0, 10.0);

        // Start drawing
        tool.start_drawing(start_point);
        assert!(tool.is_drawing);
        assert_eq!(tool.points.len(), 1);
        assert_eq!(tool.points[0], start_point);

        // Add second point
        let second_point = Point::new(20.0, 10.0);
        let should_close = tool.add_point(second_point);
        assert!(!should_close);
        assert_eq!(tool.points.len(), 2);

        // Add third point
        let third_point = Point::new(15.0, 20.0);
        let should_close = tool.add_point(third_point);
        assert!(!should_close);
        assert_eq!(tool.points.len(), 3);

        // Try to close by clicking near start point
        let close_point = Point::new(11.0, 11.0); // Close to start point
        let should_close = tool.add_point(close_point);
        assert!(should_close); // Should trigger close
    }
}

/// Crop tool for cropping images
#[derive(Debug)]
pub struct CropTool {
    crop_start: Option<Point>,
    crop_end: Option<Point>,
    is_cropping: bool,
    aspect_ratio_constraint: Option<f32>,
    show_preview: bool,
    crop_mode: CropMode,
}

/// Crop mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropMode {
    Free,
    FixedRatio(u32, u32), // width, height ratio
    Square,
}

impl std::fmt::Display for CropMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CropMode::Free => write!(f, "Free"),
            CropMode::FixedRatio(w, h) => write!(f, "{}:{}", w, h),
            CropMode::Square => write!(f, "Square"),
        }
    }
}

impl CropTool {
    pub fn new() -> Self {
        Self {
            crop_start: None,
            crop_end: None,
            is_cropping: false,
            aspect_ratio_constraint: None,
            show_preview: true,
            crop_mode: CropMode::Free,
        }
    }

    /// Calculate constrained crop rectangle based on aspect ratio
    fn calculate_constrained_rect(&self, start: Point, end: Point) -> (Point, Point) {
        match self.crop_mode {
            CropMode::Free => (start, end),
            CropMode::Square => {
                let width = (end.x - start.x).abs();
                let height = (end.y - start.y).abs();
                let size = width.min(height);

                let new_end = Point::new(
                    start.x + if end.x >= start.x { size } else { -size },
                    start.y + if end.y >= start.y { size } else { -size },
                );
                (start, new_end)
            }
            CropMode::FixedRatio(ratio_w, ratio_h) => {
                let ratio = ratio_w as f32 / ratio_h as f32;
                let width = (end.x - start.x).abs();
                let height = width / ratio;

                let new_end = Point::new(
                    start.x + if end.x >= start.x { width } else { -width },
                    start.y + if end.y >= start.y { height } else { -height },
                );
                (start, new_end)
            }
        }
    }

    /// Apply crop to the document
    fn apply_crop(&self, document: &mut Document) -> ToolResult<()> {
        if let (Some(start), Some(end)) = (self.crop_start, self.crop_end) {
            let (constrained_start, constrained_end) = self.calculate_constrained_rect(start, end);

            // Calculate crop rectangle
            let x = constrained_start.x.min(constrained_end.x);
            let y = constrained_start.y.min(constrained_end.y);
            let width = (constrained_end.x - constrained_start.x).abs();
            let height = (constrained_end.y - constrained_start.y).abs();

            debug!(
                "Applying crop: x={}, y={}, width={}, height={}",
                x, y, width, height
            );

            // TODO: Implement actual crop operation on document
            // This would involve:
            // 1. Creating a crop command
            // 2. Applying it to all layers or selected layer
            // 3. Updating document dimensions

            document.mark_dirty();
        }
        Ok(())
    }
}

impl Default for CropTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool for CropTool {
    fn id(&self) -> &'static str {
        "crop"
    }

    fn name(&self) -> &'static str {
        "Crop Tool"
    }

    fn description(&self) -> &'static str {
        "Crop image to selected area"
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
                debug!("Crop selection started at: {:?}", position);
                self.crop_start = Some(position);
                self.crop_end = None;
                self.is_cropping = true;
                state.is_active = true;
                state.last_position = Some(position);
            }
            ToolEvent::MouseDragged { position, .. } => {
                if self.is_cropping {
                    debug!("Crop selection dragged to: {:?}", position);
                    self.crop_end = Some(position);
                    state.last_position = Some(position);

                    // Update preview if enabled
                    if self.show_preview {
                        // TODO: Update crop preview visualization
                    }
                }
            }
            ToolEvent::MouseReleased { position, .. } => {
                if self.is_cropping {
                    debug!("Crop selection completed at: {:?}", position);
                    self.crop_end = Some(position);
                    self.is_cropping = false;
                    state.is_active = false;

                    // Apply crop operation
                    self.apply_crop(document)?;

                    // Reset crop selection
                    self.crop_start = None;
                    self.crop_end = None;
                }
            }
            ToolEvent::KeyPressed { key, .. } => {
                // ESC key cancels crop operation
                if key == Key::Escape {
                    debug!("Crop operation cancelled");
                    self.crop_start = None;
                    self.crop_end = None;
                    self.is_cropping = false;
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
                name: "crop_mode".to_string(),
                display_name: "Crop Mode".to_string(),
                description: "Constraint mode for crop selection".to_string(),
                option_type: ToolOptionType::Enum(vec![
                    "Free".to_string(),
                    "Square".to_string(),
                    "16:9".to_string(),
                    "4:3".to_string(),
                    "3:2".to_string(),
                ]),
                default_value: ToolOptionValue::String("Free".to_string()),
            },
            ToolOption {
                name: "show_preview".to_string(),
                display_name: "Show Preview".to_string(),
                description: "Show crop preview overlay".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(self.show_preview),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "crop_mode" => {
                if let ToolOptionValue::String(mode_str) = value {
                    self.crop_mode = match mode_str.as_str() {
                        "Free" => CropMode::Free,
                        "Square" => CropMode::Square,
                        "16:9" => CropMode::FixedRatio(16, 9),
                        "4:3" => CropMode::FixedRatio(4, 3),
                        "3:2" => CropMode::FixedRatio(3, 2),
                        _ => CropMode::Free,
                    };

                    // Update aspect ratio constraint
                    self.aspect_ratio_constraint = match self.crop_mode {
                        CropMode::Free => None,
                        CropMode::Square => Some(1.0),
                        CropMode::FixedRatio(w, h) => Some(w as f32 / h as f32),
                    };
                }
            }
            "show_preview" => {
                if let ToolOptionValue::Bool(enabled) = value {
                    self.show_preview = enabled;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "crop_mode" => Some(ToolOptionValue::String(self.crop_mode.to_string())),
            "show_preview" => Some(ToolOptionValue::Bool(self.show_preview)),
            _ => None,
        }
    }
}

// Crop tool tests
#[cfg(test)]
mod crop_tool_tests {
    use super::*;
    use psoc_core::{Document, Point};

    #[test]
    fn test_crop_tool_creation() {
        let tool = CropTool::new();
        assert_eq!(tool.id(), "crop");
        assert_eq!(tool.name(), "Crop Tool");
        assert!(!tool.is_cropping);
        assert!(tool.show_preview);
        assert!(matches!(tool.crop_mode, CropMode::Free));
    }

    #[test]
    fn test_crop_tool_options() {
        let tool = CropTool::new();
        let options = tool.options();
        assert_eq!(options.len(), 2);

        let crop_mode_option = &options[0];
        assert_eq!(crop_mode_option.name, "crop_mode");
        assert_eq!(crop_mode_option.display_name, "Crop Mode");

        let preview_option = &options[1];
        assert_eq!(preview_option.name, "show_preview");
        assert_eq!(preview_option.display_name, "Show Preview");
    }

    #[test]
    fn test_crop_tool_set_options() {
        let mut tool = CropTool::new();

        // Test crop mode setting
        tool.set_option("crop_mode", ToolOptionValue::String("Square".to_string()))
            .unwrap();
        assert!(matches!(tool.crop_mode, CropMode::Square));
        assert_eq!(tool.aspect_ratio_constraint, Some(1.0));

        tool.set_option("crop_mode", ToolOptionValue::String("16:9".to_string()))
            .unwrap();
        assert!(matches!(tool.crop_mode, CropMode::FixedRatio(16, 9)));
        assert_eq!(tool.aspect_ratio_constraint, Some(16.0 / 9.0));

        // Test preview setting
        tool.set_option("show_preview", ToolOptionValue::Bool(false))
            .unwrap();
        assert!(!tool.show_preview);
    }

    #[test]
    fn test_crop_tool_get_options() {
        let tool = CropTool::new();

        let crop_mode = tool.get_option("crop_mode").unwrap();
        assert_eq!(crop_mode, ToolOptionValue::String("Free".to_string()));

        let show_preview = tool.get_option("show_preview").unwrap();
        assert_eq!(show_preview, ToolOptionValue::Bool(true));

        assert!(tool.get_option("invalid_option").is_none());
    }

    #[test]
    fn test_crop_mode_display() {
        assert_eq!(CropMode::Free.to_string(), "Free");
        assert_eq!(CropMode::Square.to_string(), "Square");
        assert_eq!(CropMode::FixedRatio(16, 9).to_string(), "16:9");
        assert_eq!(CropMode::FixedRatio(4, 3).to_string(), "4:3");
    }

    #[test]
    fn test_crop_tool_constrained_rect() {
        let mut tool = CropTool::new();
        let start = Point::new(10.0, 10.0);
        let end = Point::new(50.0, 30.0);

        // Free mode - no constraint
        let (constrained_start, constrained_end) = tool.calculate_constrained_rect(start, end);
        assert_eq!(constrained_start, start);
        assert_eq!(constrained_end, end);

        // Square mode - should constrain to square
        tool.crop_mode = CropMode::Square;
        let (constrained_start, constrained_end) = tool.calculate_constrained_rect(start, end);
        assert_eq!(constrained_start, start);
        // Should be square with smaller dimension
        let _expected_size = 20.0; // min(40, 20)
        assert_eq!(constrained_end, Point::new(30.0, 30.0));
    }

    #[test]
    fn test_crop_tool_event_handling() {
        let mut tool = CropTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Test mouse press
        let press_event = ToolEvent::MousePressed {
            position: Point::new(10.0, 10.0),
            button: super::super::tool_trait::MouseButton::Left,
            modifiers: super::super::tool_trait::KeyModifiers::default(),
        };

        tool.handle_event(press_event, &mut document, &mut state)
            .unwrap();
        assert!(tool.is_cropping);
        assert_eq!(tool.crop_start, Some(Point::new(10.0, 10.0)));
        assert!(state.is_active);

        // Test mouse drag
        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(50.0, 30.0),
            button: super::super::tool_trait::MouseButton::Left,
            modifiers: super::super::tool_trait::KeyModifiers::default(),
        };

        tool.handle_event(drag_event, &mut document, &mut state)
            .unwrap();
        assert_eq!(tool.crop_end, Some(Point::new(50.0, 30.0)));

        // Test mouse release
        let release_event = ToolEvent::MouseReleased {
            position: Point::new(50.0, 30.0),
            button: super::super::tool_trait::MouseButton::Left,
            modifiers: super::super::tool_trait::KeyModifiers::default(),
        };

        tool.handle_event(release_event, &mut document, &mut state)
            .unwrap();
        assert!(!tool.is_cropping);
        assert!(!state.is_active);
        assert_eq!(tool.crop_start, None);
        assert_eq!(tool.crop_end, None);
    }

    #[test]
    fn test_crop_tool_escape_cancel() {
        let mut tool = CropTool::new();
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut state = ToolState::default();

        // Start cropping
        tool.crop_start = Some(Point::new(10.0, 10.0));
        tool.is_cropping = true;
        state.is_active = true;

        // Press escape
        let escape_event = ToolEvent::KeyPressed {
            key: Key::Escape,
            modifiers: super::super::tool_trait::KeyModifiers::default(),
        };

        tool.handle_event(escape_event, &mut document, &mut state)
            .unwrap();
        assert!(!tool.is_cropping);
        assert!(!state.is_active);
        assert_eq!(tool.crop_start, None);
        assert_eq!(tool.crop_end, None);
    }
}

/// Eyedropper tool for picking colors from the canvas
#[derive(Debug, Clone)]
pub struct EyedropperTool {
    /// Whether to pick color to foreground (true) or background (false)
    pick_to_foreground: bool,
    /// Sample size for color picking (1x1, 3x3, 5x5)
    sample_size: u32,
}

impl EyedropperTool {
    /// Create a new eyedropper tool
    pub fn new() -> Self {
        Self {
            pick_to_foreground: true,
            sample_size: 1,
        }
    }

    /// Pick color from the document at the given position
    fn pick_color_at_position(
        &self,
        position: Point,
        document: &Document,
    ) -> ToolResult<Option<RgbaPixel>> {
        let x = position.x as u32;
        let y = position.y as u32;

        // Get the active layer
        let active_layer = document.active_layer();
        if active_layer.is_none() {
            debug!("No active layer to pick color from");
            return Ok(None);
        }

        let layer = active_layer.unwrap();
        if !layer.has_pixel_data() {
            debug!("Active layer has no pixel data");
            return Ok(None);
        }

        let layer_dims = layer.dimensions();
        if layer_dims.is_none() {
            return Ok(None);
        }

        let (width, height) = layer_dims.unwrap();

        // Check bounds
        if x >= width || y >= height {
            debug!(
                "Position out of bounds: ({}, {}) vs ({}, {})",
                x, y, width, height
            );
            return Ok(None);
        }

        // Sample color based on sample size
        let color = match self.sample_size {
            1 => {
                // Single pixel sampling
                layer.get_pixel(x, y).unwrap_or(RgbaPixel::transparent())
            }
            size => {
                // Multi-pixel sampling (average)
                self.sample_average_color(x, y, size, layer)?
            }
        };

        debug!("Picked color: {:?} at position ({}, {})", color, x, y);
        Ok(Some(color))
    }

    /// Sample average color from a square area
    fn sample_average_color(
        &self,
        center_x: u32,
        center_y: u32,
        size: u32,
        layer: &psoc_core::Layer,
    ) -> ToolResult<RgbaPixel> {
        let layer_dims = layer.dimensions().unwrap_or((0, 0));
        let (width, height) = layer_dims;

        let half_size = size / 2;
        let mut total_r = 0u32;
        let mut total_g = 0u32;
        let mut total_b = 0u32;
        let mut total_a = 0u32;
        let mut count = 0u32;

        // Sample pixels in the square area
        for dy in 0..size {
            for dx in 0..size {
                let sample_x = center_x.saturating_sub(half_size).saturating_add(dx);
                let sample_y = center_y.saturating_sub(half_size).saturating_add(dy);

                if sample_x < width && sample_y < height {
                    if let Some(pixel) = layer.get_pixel(sample_x, sample_y) {
                        total_r += pixel.r as u32;
                        total_g += pixel.g as u32;
                        total_b += pixel.b as u32;
                        total_a += pixel.a as u32;
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            Ok(RgbaPixel::new(
                (total_r / count) as u8,
                (total_g / count) as u8,
                (total_b / count) as u8,
                (total_a / count) as u8,
            ))
        } else {
            Ok(RgbaPixel::transparent())
        }
    }
}

impl Tool for EyedropperTool {
    fn id(&self) -> &'static str {
        "eyedropper"
    }

    fn name(&self) -> &'static str {
        "Eyedropper Tool"
    }

    fn description(&self) -> &'static str {
        "Pick colors from the canvas"
    }

    fn cursor(&self) -> ToolCursor {
        ToolCursor::Crosshair
    }

    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        _state: &mut ToolState,
    ) -> ToolResult<()> {
        match event {
            ToolEvent::MousePressed {
                position,
                button,
                modifiers,
                ..
            } => {
                if button == super::tool_trait::MouseButton::Left {
                    debug!("Eyedropper tool picking color at: {:?}", position);

                    // Check for Alt key to pick to background color
                    self.pick_to_foreground = !modifiers.alt;

                    // Pick color from the document
                    if let Ok(Some(color)) = self.pick_color_at_position(position, document) {
                        debug!(
                            "Picked color: {:?} to {}",
                            color,
                            if self.pick_to_foreground {
                                "foreground"
                            } else {
                                "background"
                            }
                        );

                        // TODO: Apply the picked color to foreground/background
                        // This would typically involve sending a message to the application
                        // For now, we'll just log the action
                        debug!("Color picked successfully: {:?}", color);
                    } else {
                        debug!("Failed to pick color at position: {:?}", position);
                    }
                }
            }
            _ => {
                // Eyedropper tool doesn't handle other events
            }
        }

        Ok(())
    }

    fn options(&self) -> Vec<ToolOption> {
        vec![
            ToolOption {
                name: "pick_to_foreground".to_string(),
                display_name: "Pick to Foreground".to_string(),
                description: "Pick color to foreground (true) or background (false)".to_string(),
                option_type: ToolOptionType::Bool,
                default_value: ToolOptionValue::Bool(true),
            },
            ToolOption {
                name: "sample_size".to_string(),
                display_name: "Sample Size".to_string(),
                description: "Size of the sampling area (1x1, 3x3, 5x5)".to_string(),
                option_type: ToolOptionType::Choice(vec![
                    "1x1".to_string(),
                    "3x3".to_string(),
                    "5x5".to_string(),
                ]),
                default_value: ToolOptionValue::Choice(0),
            },
        ]
    }

    fn set_option(&mut self, name: &str, value: ToolOptionValue) -> ToolResult<()> {
        match name {
            "pick_to_foreground" => {
                if let ToolOptionValue::Bool(pick_to_fg) = value {
                    self.pick_to_foreground = pick_to_fg;
                    debug!("Set pick_to_foreground to: {}", pick_to_fg);
                } else {
                    return Err(super::tool_trait::ToolError::InvalidOptionValue {
                        option: name.to_string(),
                        value: format!("{:?}", value),
                    }
                    .into());
                }
            }
            "sample_size" => {
                if let ToolOptionValue::Choice(choice) = value {
                    self.sample_size = match choice {
                        0 => 1, // 1x1
                        1 => 3, // 3x3
                        2 => 5, // 5x5
                        _ => 1, // Default to 1x1
                    };
                    debug!(
                        "Set sample_size to: {}x{}",
                        self.sample_size, self.sample_size
                    );
                } else {
                    return Err(super::tool_trait::ToolError::InvalidOptionValue {
                        option: name.to_string(),
                        value: format!("{:?}", value),
                    }
                    .into());
                }
            }
            _ => {
                return Err(super::tool_trait::ToolError::UnknownOption {
                    option: name.to_string(),
                }
                .into());
            }
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "pick_to_foreground" => Some(ToolOptionValue::Bool(self.pick_to_foreground)),
            "sample_size" => {
                let choice = match self.sample_size {
                    1 => 0, // 1x1
                    3 => 1, // 3x3
                    5 => 2, // 5x5
                    _ => 0, // Default to 1x1
                };
                Some(ToolOptionValue::Choice(choice))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod eyedropper_tests {
    use super::super::tool_trait::{KeyModifiers, MouseButton, ToolEvent, ToolState};
    use super::*;
    use psoc_core::{Document, Layer, Point, RgbaPixel};

    #[test]
    fn test_eyedropper_tool_creation() {
        let tool = EyedropperTool::new();
        assert_eq!(tool.id(), "eyedropper");
        assert_eq!(tool.name(), "Eyedropper Tool");
        assert!(tool.pick_to_foreground);
        assert_eq!(tool.sample_size, 1);
    }

    #[test]
    fn test_eyedropper_tool_options() {
        let mut tool = EyedropperTool::new();

        // Test setting pick_to_foreground option
        tool.set_option("pick_to_foreground", ToolOptionValue::Bool(false))
            .unwrap();
        assert!(!tool.pick_to_foreground);

        // Test getting pick_to_foreground option
        let value = tool.get_option("pick_to_foreground").unwrap();
        assert_eq!(value, ToolOptionValue::Bool(false));

        // Test setting sample_size option
        tool.set_option("sample_size", ToolOptionValue::Choice(1))
            .unwrap();
        assert_eq!(tool.sample_size, 3);

        // Test getting sample_size option
        let value = tool.get_option("sample_size").unwrap();
        assert_eq!(value, ToolOptionValue::Choice(1));
    }

    #[test]
    fn test_eyedropper_tool_reset_options() {
        let mut tool = EyedropperTool::new();

        // Change options
        tool.set_option("pick_to_foreground", ToolOptionValue::Bool(false))
            .unwrap();
        tool.set_option("sample_size", ToolOptionValue::Choice(2))
            .unwrap();

        // Verify options were changed
        assert!(!tool.pick_to_foreground);
        assert_eq!(tool.sample_size, 5);

        // Reset manually (since reset_options is not in the trait)
        tool.pick_to_foreground = true;
        tool.sample_size = 1;

        // Check that options are reset to defaults
        assert!(tool.pick_to_foreground);
        assert_eq!(tool.sample_size, 1);
    }

    #[test]
    fn test_eyedropper_color_picking() {
        let tool = EyedropperTool::new();
        let mut document = Document::new("Test".to_string(), 10, 10);

        // Create a layer with known pixel data
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);

        // Set a specific color at position (5, 5)
        let test_color = RgbaPixel::new(255, 128, 64, 255);
        layer.set_pixel(5, 5, test_color).unwrap();

        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Pick color at position (5, 5)
        let picked_color = tool
            .pick_color_at_position(Point::new(5.0, 5.0), &document)
            .unwrap();

        assert!(picked_color.is_some());
        assert_eq!(picked_color.unwrap(), test_color);
    }

    #[test]
    fn test_eyedropper_out_of_bounds() {
        let tool = EyedropperTool::new();
        let mut document = Document::new("Test".to_string(), 10, 10);

        let layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Try to pick color outside bounds
        let picked_color = tool
            .pick_color_at_position(Point::new(15.0, 15.0), &document)
            .unwrap();

        assert!(picked_color.is_none());
    }

    #[test]
    fn test_eyedropper_sample_average_color() {
        let tool = EyedropperTool::new();
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);

        // Set colors in a 3x3 area around (5, 5)
        let colors = [
            RgbaPixel::new(255, 0, 0, 255), // Red
            RgbaPixel::new(0, 255, 0, 255), // Green
            RgbaPixel::new(0, 0, 255, 255), // Blue
        ];

        // Set pixels in a pattern
        layer.set_pixel(4, 4, colors[0]).unwrap();
        layer.set_pixel(5, 4, colors[1]).unwrap();
        layer.set_pixel(6, 4, colors[2]).unwrap();
        layer.set_pixel(4, 5, colors[1]).unwrap();
        layer.set_pixel(5, 5, colors[2]).unwrap();
        layer.set_pixel(6, 5, colors[0]).unwrap();
        layer.set_pixel(4, 6, colors[2]).unwrap();
        layer.set_pixel(5, 6, colors[0]).unwrap();
        layer.set_pixel(6, 6, colors[1]).unwrap();

        // Sample average color
        let avg_color = tool.sample_average_color(5, 5, 3, &layer).unwrap();

        // The average should be approximately (85, 85, 85, 255)
        // since we have equal amounts of red, green, and blue
        assert!(avg_color.r > 80 && avg_color.r < 90);
        assert!(avg_color.g > 80 && avg_color.g < 90);
        assert!(avg_color.b > 80 && avg_color.b < 90);
        assert_eq!(avg_color.a, 255);
    }

    #[test]
    fn test_eyedropper_mouse_event_handling() {
        let mut tool = EyedropperTool::new();
        let mut document = Document::new("Test".to_string(), 10, 10);
        let mut state = ToolState::default();

        let layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Test mouse press event
        let event = ToolEvent::MousePressed {
            position: Point::new(5.0, 5.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        // Should not panic and should handle the event
        tool.handle_event(event, &mut document, &mut state).unwrap();
    }

    #[test]
    fn test_eyedropper_alt_modifier() {
        let mut tool = EyedropperTool::new();
        let mut document = Document::new("Test".to_string(), 10, 10);
        let mut state = ToolState::default();

        let layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);
        document.add_layer(layer);
        document.set_active_layer(0).unwrap();

        // Test mouse press with Alt modifier (should pick to background)
        let event = ToolEvent::MousePressed {
            position: Point::new(5.0, 5.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers {
                alt: true,
                ..Default::default()
            },
        };

        tool.handle_event(event, &mut document, &mut state).unwrap();

        // pick_to_foreground should be false when Alt is pressed
        assert!(!tool.pick_to_foreground);
    }
}
