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
}

impl std::fmt::Display for ToolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolType::Select => write!(f, "Select"),
            ToolType::Brush => write!(f, "Brush"),
            ToolType::Eraser => write!(f, "Eraser"),
            ToolType::Move => write!(f, "Move"),
        }
    }
}

/// Selection tool for making selections
#[derive(Debug)]
pub struct SelectTool {
    selection_start: Option<Point>,
    is_selecting: bool,
}

impl SelectTool {
    pub fn new() -> Self {
        Self {
            selection_start: None,
            is_selecting: false,
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
}

/// Brush tool for painting
#[derive(Debug)]
pub struct BrushTool {
    brush_size: f32,
    #[allow(dead_code)]
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
            _ => {}
        }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<ToolOptionValue> {
        match name {
            "size" => Some(ToolOptionValue::Float(self.brush_size)),
            "hardness" => Some(ToolOptionValue::Float(self.brush_hardness)),
            _ => None,
        }
    }
}

impl BrushTool {
    fn paint_at_position(&self, position: Point, document: &mut Document) -> ToolResult<()> {
        // TODO: Implement actual painting logic
        debug!(
            "Painting at position: {:?} with size: {}",
            position, self.brush_size
        );

        // For now, just mark the document as dirty
        document.mark_dirty();

        Ok(())
    }

    fn paint_stroke(&self, from: Point, to: Point, document: &mut Document) -> ToolResult<()> {
        // TODO: Implement stroke painting logic
        debug!("Painting stroke from {:?} to {:?}", from, to);

        // For now, just mark the document as dirty
        document.mark_dirty();

        Ok(())
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
}

impl EraserTool {
    fn erase_at_position(&self, position: Point, document: &mut Document) -> ToolResult<()> {
        // TODO: Implement actual erasing logic
        debug!(
            "Erasing at position: {:?} with size: {}",
            position, self.eraser_size
        );

        // For now, just mark the document as dirty
        document.mark_dirty();

        Ok(())
    }

    fn erase_stroke(&self, from: Point, to: Point, document: &mut Document) -> ToolResult<()> {
        // TODO: Implement stroke erasing logic
        debug!("Erasing stroke from {:?} to {:?}", from, to);

        // For now, just mark the document as dirty
        document.mark_dirty();

        Ok(())
    }
}

/// Move tool for moving layers and selections
#[derive(Debug)]
pub struct MoveTool {
    is_moving: bool,
    move_start: Option<Point>,
}

impl MoveTool {
    pub fn new() -> Self {
        Self {
            is_moving: false,
            move_start: None,
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

                        // TODO: Apply movement to active layer
                        document.mark_dirty();
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
}
