//! Tools module - Core editing tools and tool management system
//!
//! This module provides the foundation for all editing tools in PSOC, including:
//! - Tool trait definition for consistent tool behavior
//! - Tool manager for switching between tools
//! - Concrete tool implementations (Select, Brush, Eraser, Move)
//! - Tool event handling and state management

pub mod tool_manager;
pub mod tool_trait;
#[allow(clippy::module_inception)]
pub mod tools;

// Re-export commonly used types
pub use tool_manager::{ToolManager, ToolManagerError};
pub use tool_trait::{Tool, ToolEvent, ToolResult, ToolState};
pub use tools::{
    BrushTool, EllipseTool, EraserTool, LassoTool, MagicWandTool, MoveTool, SelectTool, TextTool,
    ToolType, TransformTool,
};
