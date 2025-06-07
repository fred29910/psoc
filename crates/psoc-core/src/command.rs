//! Command system for undo/redo functionality
//!
//! This module implements the Command pattern to provide comprehensive undo/redo
//! functionality for all document operations in PSOC. It includes:
//! - Core Command trait for all reversible operations
//! - CommandHistory for managing undo/redo stacks
//! - Specific command implementations for various operations

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::{debug, info};
use uuid::Uuid;

use crate::Document;

/// Maximum number of commands to keep in history
pub const MAX_COMMAND_HISTORY: usize = 100;

/// Core trait for all reversible commands
///
/// Every operation that can be undone must implement this trait.
/// Commands should be self-contained and include all necessary data
/// to perform both the operation and its reverse.
pub trait Command: Debug + Send + Sync {
    /// Get a unique identifier for this command
    fn id(&self) -> Uuid;

    /// Get a human-readable description of this command
    fn description(&self) -> &str;

    /// Execute the command, modifying the document
    fn execute(&self, document: &mut Document) -> Result<()>;

    /// Undo the command, reverting the document to its previous state
    fn undo(&self, document: &mut Document) -> Result<()>;

    /// Check if this command can be merged with another command
    /// This is useful for combining similar operations (e.g., multiple brush strokes)
    fn can_merge_with(&self, _other: &dyn Command) -> bool {
        false
    }

    /// Merge this command with another command
    /// Only called if can_merge_with returns true
    fn merge_with(&mut self, _other: Box<dyn Command>) -> Result<()> {
        Err(anyhow::anyhow!("Command merging not implemented"))
    }

    /// Get the timestamp when this command was created
    fn timestamp(&self) -> std::time::SystemTime;

    /// Check if this command modifies the document
    fn modifies_document(&self) -> bool {
        true
    }

    /// Get a reference to this command as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Command history manager for undo/redo operations
#[derive(Debug)]
pub struct CommandHistory {
    /// Stack of executed commands (for undo)
    undo_stack: Vec<Box<dyn Command>>,
    /// Stack of undone commands (for redo)
    redo_stack: Vec<Box<dyn Command>>,
    /// Current position in history (for navigation)
    current_position: usize,
    /// Maximum number of commands to keep
    max_history: usize,
    /// Whether command merging is enabled
    merge_enabled: bool,
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for CommandHistory {
    fn clone(&self) -> Self {
        // Note: We can't clone Box<dyn Command> directly, so we create a new empty history
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_position: 0,
            max_history: self.max_history,
            merge_enabled: self.merge_enabled,
        }
    }
}

impl CommandHistory {
    /// Create a new command history
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_position: 0,
            max_history: MAX_COMMAND_HISTORY,
            merge_enabled: true,
        }
    }

    /// Create a new command history with custom settings
    pub fn with_settings(max_history: usize, merge_enabled: bool) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_position: 0,
            max_history,
            merge_enabled,
        }
    }

    /// Execute a command and add it to the history
    pub fn execute_command(
        &mut self,
        command: Box<dyn Command>,
        document: &mut Document,
    ) -> Result<()> {
        debug!("Executing command: {}", command.description());

        // Execute the command
        command.execute(document)?;

        // Clear redo stack when a new command is executed
        self.redo_stack.clear();

        // Add command to undo stack
        self.undo_stack.push(command);
        self.current_position = self.undo_stack.len();

        // Limit history size
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
            self.current_position = self.undo_stack.len();
        }

        info!("Command executed and added to history");
        Ok(())
    }

    /// Undo the last command
    pub fn undo(&mut self, document: &mut Document) -> Result<bool> {
        if self.undo_stack.is_empty() || self.current_position == 0 {
            debug!("No commands to undo");
            return Ok(false);
        }

        // Get the command to undo
        let command = self
            .undo_stack
            .get(self.current_position - 1)
            .ok_or_else(|| anyhow::anyhow!("Invalid undo position"))?;

        debug!("Undoing command: {}", command.description());

        // Undo the command
        command.undo(document)?;

        // Move the command to redo stack (we need to clone the reference)
        // Note: This is a simplified approach - in a full implementation,
        // we might need a different strategy for command storage
        self.current_position -= 1;

        info!("Command undone successfully");
        Ok(true)
    }

    /// Redo the last undone command
    pub fn redo(&mut self, document: &mut Document) -> Result<bool> {
        if self.current_position >= self.undo_stack.len() {
            debug!("No commands to redo");
            return Ok(false);
        }

        // Get the command to redo
        let command = self
            .undo_stack
            .get(self.current_position)
            .ok_or_else(|| anyhow::anyhow!("Invalid redo position"))?;

        debug!("Redoing command: {}", command.description());

        // Execute the command again
        command.execute(document)?;
        self.current_position += 1;

        info!("Command redone successfully");
        Ok(true)
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty() && self.current_position > 0
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.current_position < self.undo_stack.len()
    }

    /// Get the description of the next command that would be undone
    pub fn undo_description(&self) -> Option<&str> {
        if self.can_undo() {
            self.undo_stack
                .get(self.current_position - 1)
                .map(|cmd| cmd.description())
        } else {
            None
        }
    }

    /// Get the description of the next command that would be redone
    pub fn redo_description(&self) -> Option<&str> {
        if self.can_redo() {
            self.undo_stack
                .get(self.current_position)
                .map(|cmd| cmd.description())
        } else {
            None
        }
    }

    /// Clear all command history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_position = 0;
        info!("Command history cleared");
    }

    /// Get the number of commands in undo stack
    pub fn undo_count(&self) -> usize {
        self.current_position
    }

    /// Get the number of commands in redo stack
    pub fn redo_count(&self) -> usize {
        self.undo_stack.len() - self.current_position
    }

    /// Enable or disable command merging
    pub fn set_merge_enabled(&mut self, enabled: bool) {
        self.merge_enabled = enabled;
    }

    /// Check if command merging is enabled
    pub fn is_merge_enabled(&self) -> bool {
        self.merge_enabled
    }

    /// Set maximum history size
    pub fn set_max_history(&mut self, max_history: usize) {
        self.max_history = max_history;
    }

    /// Get maximum history size
    pub fn max_history(&self) -> usize {
        self.max_history
    }

    /// Get all commands in history for display purposes
    pub fn get_history_entries(&self) -> Vec<HistoryEntry> {
        self.undo_stack
            .iter()
            .enumerate()
            .map(|(index, command)| HistoryEntry {
                index,
                description: command.description().to_string(),
                timestamp: command.timestamp(),
                is_current: index == self.current_position.saturating_sub(1),
                can_navigate_to: true,
            })
            .collect()
    }

    /// Navigate to a specific position in history
    /// Note: This method should be called from Document to avoid borrowing issues
    pub fn should_navigate_to_position(&self, position: usize) -> Option<NavigationDirection> {
        if position > self.undo_stack.len() || position == self.current_position {
            return None;
        }

        if position < self.current_position {
            Some(NavigationDirection::Backward(
                self.current_position - position,
            ))
        } else {
            Some(NavigationDirection::Forward(
                position - self.current_position,
            ))
        }
    }

    /// Get current position in history
    pub fn current_position(&self) -> usize {
        self.current_position
    }

    /// Get total number of commands in history
    pub fn total_commands(&self) -> usize {
        self.undo_stack.len()
    }
}

/// Navigation direction for history
#[derive(Debug, Clone)]
pub enum NavigationDirection {
    /// Move backward (undo) by the specified number of steps
    Backward(usize),
    /// Move forward (redo) by the specified number of steps
    Forward(usize),
}

/// Represents an entry in the history panel
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// Index in the history stack
    pub index: usize,
    /// Description of the command
    pub description: String,
    /// When the command was executed
    pub timestamp: std::time::SystemTime,
    /// Whether this is the current position
    pub is_current: bool,
    /// Whether user can navigate to this position
    pub can_navigate_to: bool,
}

/// Command execution result
#[derive(Debug, Clone, PartialEq)]
pub enum CommandResult {
    /// Command executed successfully
    Success,
    /// Command was merged with previous command
    Merged,
    /// Command execution failed
    Failed(String),
}

/// Base command data that all commands should include
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    /// Unique command identifier
    pub id: Uuid,
    /// Human-readable description
    pub description: String,
    /// Timestamp when command was created
    pub timestamp: std::time::SystemTime,
    /// Whether this command modifies the document
    pub modifies_document: bool,
}

impl CommandMetadata {
    /// Create new command metadata
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            timestamp: std::time::SystemTime::now(),
            modifies_document: true,
        }
    }

    /// Create new command metadata that doesn't modify the document
    pub fn new_non_modifying(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            timestamp: std::time::SystemTime::now(),
            modifies_document: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Document;

    /// Mock command for testing
    #[derive(Debug)]
    struct MockCommand {
        metadata: CommandMetadata,
        executed: std::sync::Arc<std::sync::Mutex<bool>>,
        undone: std::sync::Arc<std::sync::Mutex<bool>>,
    }

    impl MockCommand {
        fn new(description: &str) -> Self {
            Self {
                metadata: CommandMetadata::new(description.to_string()),
                executed: std::sync::Arc::new(std::sync::Mutex::new(false)),
                undone: std::sync::Arc::new(std::sync::Mutex::new(false)),
            }
        }

        fn was_executed(&self) -> bool {
            *self.executed.lock().unwrap()
        }

        fn was_undone(&self) -> bool {
            *self.undone.lock().unwrap()
        }
    }

    impl Command for MockCommand {
        fn id(&self) -> Uuid {
            self.metadata.id
        }

        fn description(&self) -> &str {
            &self.metadata.description
        }

        fn execute(&self, _document: &mut Document) -> Result<()> {
            *self.executed.lock().unwrap() = true;
            Ok(())
        }

        fn undo(&self, _document: &mut Document) -> Result<()> {
            *self.undone.lock().unwrap() = true;
            Ok(())
        }

        fn timestamp(&self) -> std::time::SystemTime {
            self.metadata.timestamp
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_command_history_creation() {
        let history = CommandHistory::new();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 0);
        assert_eq!(history.redo_count(), 0);
    }

    #[test]
    fn test_command_execution() {
        let mut history = CommandHistory::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        let command = Box::new(MockCommand::new("Test Command"));
        let command_ptr = command.as_ref() as *const MockCommand;

        history.execute_command(command, &mut document).unwrap();

        // After executing a command, undo should be available
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(history.undo_count(), 1);

        // Safety: We know the command was executed
        unsafe {
            assert!((*command_ptr).was_executed());
        }
    }

    #[test]
    fn test_undo_redo() {
        let mut history = CommandHistory::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        let command = Box::new(MockCommand::new("Test Command"));

        // Execute command
        history.execute_command(command, &mut document).unwrap();

        // Should be able to undo
        assert!(history.can_undo());
        let undone = history.undo(&mut document).unwrap();
        assert!(undone);
        assert!(!history.can_undo());
        assert!(history.can_redo());

        // Should be able to redo
        let redone = history.redo(&mut document).unwrap();
        assert!(redone);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_command_descriptions() {
        let mut history = CommandHistory::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        let command = Box::new(MockCommand::new("Test Command"));
        history.execute_command(command, &mut document).unwrap();

        // Should return command description for undo
        assert_eq!(history.undo_description(), Some("Test Command"));
        assert_eq!(history.redo_description(), None);

        history.undo(&mut document).unwrap();
        assert_eq!(history.undo_description(), None);
        assert_eq!(history.redo_description(), Some("Test Command"));
    }

    #[test]
    fn test_history_limits() {
        let mut history = CommandHistory::with_settings(2, false);
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add 3 commands
        for i in 0..3 {
            let command = Box::new(MockCommand::new(&format!("Command {}", i)));
            history.execute_command(command, &mut document).unwrap();
        }

        // Should only keep 2 commands due to limit
        assert_eq!(history.undo_count(), 2);
        assert_eq!(history.undo_description(), Some("Command 2"));
    }

    #[test]
    fn test_clear_history() {
        let mut history = CommandHistory::new();
        let mut document = Document::new("Test".to_string(), 100, 100);

        let command = Box::new(MockCommand::new("Test Command"));
        history.execute_command(command, &mut document).unwrap();

        // Should support undo after command execution
        assert!(history.can_undo());
        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
}
