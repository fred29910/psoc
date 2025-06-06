//! Tool trait definition and related types
//!
//! This module defines the core `Tool` trait that all editing tools must implement,
//! providing a consistent interface for tool activation, deactivation, and event handling.

use std::fmt::Debug;

use psoc_core::{Document, Point};
use serde::{Deserialize, Serialize};

use crate::{PsocError, Result};

/// Core trait that all editing tools must implement
///
/// This trait provides a consistent interface for all tools in PSOC, ensuring
/// that tools can be activated, deactivated, and handle various events in a
/// standardized way.
pub trait Tool: Debug + Send + Sync {
    /// Get the tool's unique identifier
    fn id(&self) -> &'static str;

    /// Get the tool's display name
    fn name(&self) -> &'static str;

    /// Get the tool's description
    fn description(&self) -> &'static str;

    /// Activate the tool
    ///
    /// Called when the tool becomes the active tool. This is where the tool
    /// should initialize any state or resources it needs.
    fn activate(&mut self) -> ToolResult<()> {
        Ok(())
    }

    /// Deactivate the tool
    ///
    /// Called when the tool is no longer the active tool. This is where the tool
    /// should clean up any state or resources.
    fn deactivate(&mut self) -> ToolResult<()> {
        Ok(())
    }

    /// Handle a tool event
    ///
    /// This is the main entry point for tool interaction. All mouse events,
    /// keyboard events, and other tool-specific events are handled here.
    fn handle_event(
        &mut self,
        event: ToolEvent,
        document: &mut Document,
        state: &mut ToolState,
    ) -> ToolResult<()>;

    /// Get the current cursor for this tool
    ///
    /// Returns the cursor that should be displayed when this tool is active.
    fn cursor(&self) -> ToolCursor {
        ToolCursor::Default
    }

    /// Check if the tool can handle the given event
    ///
    /// This allows tools to filter events they're interested in.
    fn can_handle_event(&self, event: &ToolEvent) -> bool {
        matches!(
            event,
            ToolEvent::MousePressed { .. }
                | ToolEvent::MouseReleased { .. }
                | ToolEvent::MouseMoved { .. }
                | ToolEvent::MouseDragged { .. }
        )
    }

    /// Get tool-specific options
    ///
    /// Returns a list of configurable options for this tool.
    fn options(&self) -> Vec<ToolOption> {
        Vec::new()
    }

    /// Set a tool option value
    fn set_option(&mut self, _name: &str, _value: ToolOptionValue) -> ToolResult<()> {
        Ok(())
    }

    /// Get a tool option value
    fn get_option(&self, _name: &str) -> Option<ToolOptionValue> {
        None
    }
}

/// Events that can be sent to tools
#[derive(Debug, Clone, PartialEq)]
pub enum ToolEvent {
    /// Mouse button pressed
    MousePressed {
        position: Point,
        button: MouseButton,
        modifiers: KeyModifiers,
    },
    /// Mouse button released
    MouseReleased {
        position: Point,
        button: MouseButton,
        modifiers: KeyModifiers,
    },
    /// Mouse moved (without button pressed)
    MouseMoved {
        position: Point,
        modifiers: KeyModifiers,
    },
    /// Mouse dragged (with button pressed)
    MouseDragged {
        position: Point,
        button: MouseButton,
        modifiers: KeyModifiers,
    },
    /// Key pressed
    KeyPressed { key: Key, modifiers: KeyModifiers },
    /// Key released
    KeyReleased { key: Key, modifiers: KeyModifiers },
    /// Tool-specific custom event
    Custom(String),
}

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Keyboard keys (simplified set for now)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    Escape,
    Enter,
    Space,
    Shift,
    Control,
    Alt,
    Delete,
    Backspace,
    Tab,
    Character(char),
}

/// Keyboard modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Tool cursor types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCursor {
    Default,
    Crosshair,
    Hand,
    Move,
    Resize,
    Text,
    Custom(&'static str),
}

/// Tool state that persists across events
#[derive(Debug, Clone, Default)]
pub struct ToolState {
    /// Whether the tool is currently active (e.g., dragging)
    pub is_active: bool,
    /// Last known mouse position
    pub last_position: Option<Point>,
    /// Tool-specific state data
    pub data: std::collections::HashMap<String, ToolStateValue>,
}

/// Values that can be stored in tool state
#[derive(Debug, Clone)]
pub enum ToolStateValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Point(Point),
}

/// Tool configuration options
#[derive(Debug, Clone)]
pub struct ToolOption {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub option_type: ToolOptionType,
    pub default_value: ToolOptionValue,
}

/// Types of tool options
#[derive(Debug, Clone)]
pub enum ToolOptionType {
    Bool,
    Int { min: i32, max: i32 },
    Float { min: f32, max: f32 },
    String,
    Color,
    Enum(Vec<String>),
}

/// Values for tool options
#[derive(Debug, Clone, PartialEq)]
pub enum ToolOptionValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Color([u8; 4]), // RGBA
}

/// Result type for tool operations
pub type ToolResult<T> = Result<T>;

/// Error types specific to tool operations
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Tool not found: {name}")]
    ToolNotFound { name: String },
    #[error("Invalid tool state: {message}")]
    InvalidState { message: String },
    #[error("Tool operation failed: {message}")]
    OperationFailed { message: String },
    #[error("Invalid option: {name}")]
    InvalidOption { name: String },
}

impl From<ToolError> for PsocError {
    fn from(error: ToolError) -> Self {
        PsocError::tool(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_modifiers_default() {
        let modifiers = KeyModifiers::default();
        assert!(!modifiers.shift);
        assert!(!modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(!modifiers.meta);
    }

    #[test]
    fn test_tool_state_default() {
        let state = ToolState::default();
        assert!(!state.is_active);
        assert!(state.last_position.is_none());
        assert!(state.data.is_empty());
    }

    #[test]
    fn test_mouse_button_serialization() {
        let button = MouseButton::Left;
        let serialized = serde_json::to_string(&button).unwrap();
        let deserialized: MouseButton = serde_json::from_str(&serialized).unwrap();
        assert_eq!(button, deserialized);
    }

    #[test]
    fn test_tool_option_value_equality() {
        assert_eq!(ToolOptionValue::Bool(true), ToolOptionValue::Bool(true));
        assert_eq!(ToolOptionValue::Int(42), ToolOptionValue::Int(42));
        assert_eq!(
            ToolOptionValue::Float(std::f32::consts::PI),
            ToolOptionValue::Float(std::f32::consts::PI)
        );
        assert_ne!(ToolOptionValue::Bool(true), ToolOptionValue::Bool(false));
    }

    #[test]
    fn test_tool_event_creation() {
        use psoc_core::Point;

        let event = ToolEvent::MousePressed {
            position: Point::new(10.0, 20.0),
            button: MouseButton::Left,
            modifiers: KeyModifiers::default(),
        };

        match event {
            ToolEvent::MousePressed {
                position, button, ..
            } => {
                assert_eq!(position.x, 10.0);
                assert_eq!(position.y, 20.0);
                assert_eq!(button, MouseButton::Left);
            }
            _ => panic!("Expected MousePressed event"),
        }
    }

    #[test]
    fn test_tool_cursor_types() {
        assert_eq!(ToolCursor::Default, ToolCursor::Default);
        assert_ne!(ToolCursor::Default, ToolCursor::Crosshair);
        assert_ne!(ToolCursor::Hand, ToolCursor::Move);
    }
}
