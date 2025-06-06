//! Tool manager for handling tool switching and coordination
//!
//! The ToolManager is responsible for managing the active tool, handling tool
//! switching, and coordinating tool events with the document and application state.

use std::collections::HashMap;

use tracing::{debug, info, warn};

use super::tool_trait::{Tool, ToolEvent, ToolResult, ToolState};
use super::tools::ToolType;
use crate::PsocError;
use psoc_core::Document;

/// Manages the active tool and handles tool switching
#[derive(Debug)]
pub struct ToolManager {
    /// Currently active tool type
    active_tool_type: Option<ToolType>,
    /// Available tools by their type
    tools: HashMap<ToolType, Box<dyn Tool>>,
    /// Current tool state
    tool_state: ToolState,
    /// Tool history for undo/redo operations
    tool_history: Vec<ToolHistoryEntry>,
}

/// Entry in the tool history for undo/redo
#[derive(Debug, Clone)]
pub struct ToolHistoryEntry {
    pub tool_type: ToolType,
    pub timestamp: std::time::Instant,
}

impl Default for ToolManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolManager {
    /// Create a new tool manager
    pub fn new() -> Self {
        let mut manager = Self {
            active_tool_type: None,
            tools: HashMap::new(),
            tool_state: ToolState::default(),
            tool_history: Vec::new(),
        };

        // Register default tools
        manager.register_default_tools();

        // Set default active tool
        if let Err(e) = manager.set_active_tool(ToolType::Select) {
            warn!("Failed to set default tool: {}", e);
        }

        manager
    }

    /// Register all default tools
    fn register_default_tools(&mut self) {
        use super::tools::*;

        self.register_tool(ToolType::Select, Box::new(SelectTool::new()));
        self.register_tool(ToolType::Brush, Box::new(BrushTool::new()));
        self.register_tool(ToolType::Eraser, Box::new(EraserTool::new()));
        self.register_tool(ToolType::Move, Box::new(MoveTool::new()));
    }

    /// Register a tool with the manager
    pub fn register_tool(&mut self, tool_type: ToolType, tool: Box<dyn Tool>) {
        debug!("Registering tool: {:?}", tool_type);
        self.tools.insert(tool_type, tool);
    }

    /// Get the currently active tool type
    pub fn active_tool_type(&self) -> Option<ToolType> {
        self.active_tool_type
    }

    /// Set the active tool
    pub fn set_active_tool(&mut self, tool_type: ToolType) -> ToolResult<()> {
        info!("Switching to tool: {:?}", tool_type);

        // Deactivate current tool if any
        if let Some(current_type) = self.active_tool_type {
            if let Some(current_tool) = self.tools.get_mut(&current_type) {
                debug!("Deactivating current tool: {}", current_tool.name());
                current_tool.deactivate()?;
            }
        }

        // Check if the new tool exists
        if !self.tools.contains_key(&tool_type) {
            return Err(ToolManagerError::ToolNotFound {
                tool_type: format!("{:?}", tool_type),
            }
            .into());
        }

        // Activate the new tool
        if let Some(new_tool) = self.tools.get_mut(&tool_type) {
            debug!("Activating new tool: {}", new_tool.name());
            new_tool.activate()?;
        }

        // Update active tool type
        self.active_tool_type = Some(tool_type);

        // Reset tool state for new tool
        self.tool_state = ToolState::default();

        // Add to history
        self.tool_history.push(ToolHistoryEntry {
            tool_type,
            timestamp: std::time::Instant::now(),
        });

        // Keep history size reasonable
        if self.tool_history.len() > 100 {
            self.tool_history.remove(0);
        }

        info!("Successfully switched to tool: {:?}", tool_type);
        Ok(())
    }

    /// Handle a tool event
    pub fn handle_event(&mut self, event: ToolEvent, document: &mut Document) -> ToolResult<()> {
        if let Some(tool_type) = self.active_tool_type {
            if let Some(tool) = self.tools.get_mut(&tool_type) {
                // Check if the tool can handle this event
                if tool.can_handle_event(&event) {
                    debug!("Handling event with tool: {}", tool.name());
                    tool.handle_event(event, document, &mut self.tool_state)?;
                } else {
                    debug!("Tool {} cannot handle event: {:?}", tool.name(), event);
                }
            }
        } else {
            warn!("No active tool to handle event: {:?}", event);
        }
        Ok(())
    }

    /// Get the current tool's cursor
    pub fn cursor(&self) -> super::tool_trait::ToolCursor {
        if let Some(tool_type) = self.active_tool_type {
            if let Some(tool) = self.tools.get(&tool_type) {
                return tool.cursor();
            }
        }
        super::tool_trait::ToolCursor::Default
    }

    /// Get available tool types
    pub fn available_tools(&self) -> Vec<ToolType> {
        self.tools.keys().copied().collect()
    }

    /// Get tool information
    pub fn tool_info(&self, tool_type: ToolType) -> Option<ToolInfo> {
        self.tools.get(&tool_type).map(|tool| ToolInfo {
            id: tool.id().to_string(),
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            tool_type,
        })
    }

    /// Get current tool state
    pub fn tool_state(&self) -> &ToolState {
        &self.tool_state
    }

    /// Get tool history
    pub fn tool_history(&self) -> &[ToolHistoryEntry] {
        &self.tool_history
    }

    /// Clear tool history
    pub fn clear_history(&mut self) {
        self.tool_history.clear();
    }
}

/// Information about a tool
#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tool_type: ToolType,
}

/// Errors specific to tool manager operations
#[derive(Debug, thiserror::Error)]
pub enum ToolManagerError {
    #[error("Tool not found: {tool_type}")]
    ToolNotFound { tool_type: String },
    #[error("Tool manager error: {message}")]
    General { message: String },
}

impl From<ToolManagerError> for PsocError {
    fn from(error: ToolManagerError) -> Self {
        PsocError::tool(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_manager_creation() {
        let manager = ToolManager::new();
        assert!(manager.active_tool_type.is_some());
        assert_eq!(manager.available_tools().len(), 4); // Select, Brush, Eraser, Move
    }

    #[test]
    fn test_tool_switching() {
        let mut manager = ToolManager::new();

        // Switch to brush tool
        assert!(manager.set_active_tool(ToolType::Brush).is_ok());
        assert_eq!(manager.active_tool_type(), Some(ToolType::Brush));

        // Switch to eraser tool
        assert!(manager.set_active_tool(ToolType::Eraser).is_ok());
        assert_eq!(manager.active_tool_type(), Some(ToolType::Eraser));
    }

    #[test]
    fn test_tool_history() {
        let mut manager = ToolManager::new();
        let initial_history_len = manager.tool_history().len();

        manager.set_active_tool(ToolType::Brush).unwrap();
        assert_eq!(manager.tool_history().len(), initial_history_len + 1);

        manager.set_active_tool(ToolType::Eraser).unwrap();
        assert_eq!(manager.tool_history().len(), initial_history_len + 2);

        manager.clear_history();
        assert_eq!(manager.tool_history().len(), 0);
    }

    #[test]
    fn test_tool_info() {
        let manager = ToolManager::new();

        let info = manager.tool_info(ToolType::Select).unwrap();
        assert_eq!(info.tool_type, ToolType::Select);
        assert!(!info.name.is_empty());
        assert!(!info.description.is_empty());
    }

    #[test]
    fn test_selection_tool_events() {
        use super::super::tool_trait::{KeyModifiers, MouseButton, ToolEvent};
        use psoc_core::{Document, Point};

        let mut manager = ToolManager::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Switch to selection tool
        manager.set_active_tool(ToolType::Select).unwrap();

        // Simulate selection creation
        let start_event = ToolEvent::MousePressed {
            position: Point::new(10.0, 20.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        let drag_event = ToolEvent::MouseDragged {
            position: Point::new(60.0, 50.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        let release_event = ToolEvent::MouseReleased {
            position: Point::new(60.0, 50.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        // Process events
        manager.handle_event(start_event, &mut document).unwrap();
        manager.handle_event(drag_event, &mut document).unwrap();
        manager.handle_event(release_event, &mut document).unwrap();

        // Check that selection was created
        assert!(document.has_selection());
        let bounds = document.selection_bounds().unwrap();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 50.0);
        assert_eq!(bounds.height, 30.0);
    }
}
