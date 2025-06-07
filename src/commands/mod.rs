//! Command implementations for undo/redo functionality
//!
//! This module contains concrete implementations of the Command trait for various
//! operations in PSOC. Each command encapsulates a specific operation that can
//! be executed and undone.

pub mod adjustment_commands;
pub mod layer_commands;
pub mod paint_commands;
pub mod selection_commands;
pub mod transform_commands;

// Re-export commonly used command types
pub use adjustment_commands::*;
pub use layer_commands::*;
pub use paint_commands::*;
pub use selection_commands::*;
pub use transform_commands::*;

use anyhow::Result;
use psoc_core::{Command, CommandMetadata, Document};
use std::fmt::Debug;
use uuid::Uuid;

/// A composite command that groups multiple commands together
///
/// This is useful for operations that consist of multiple atomic operations
/// but should be treated as a single undoable action.
#[derive(Debug)]
pub struct CompositeCommand {
    metadata: CommandMetadata,
    commands: Vec<Box<dyn Command>>,
}

impl CompositeCommand {
    /// Create a new composite command
    pub fn new(description: String, commands: Vec<Box<dyn Command>>) -> Self {
        Self {
            metadata: CommandMetadata::new(description),
            commands,
        }
    }

    /// Add a command to the composite
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    /// Get the number of sub-commands
    pub fn command_count(&self) -> usize {
        self.commands.len()
    }
}

impl Command for CompositeCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // Execute all commands in order
        for command in &self.commands {
            command.execute(document)?;
        }
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        // Undo all commands in reverse order
        for command in self.commands.iter().rev() {
            command.undo(document)?;
        }
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn modifies_document(&self) -> bool {
        self.commands.iter().any(|cmd| cmd.modifies_document())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// A no-op command that does nothing
///
/// Useful for testing and as a placeholder.
#[derive(Debug)]
pub struct NoOpCommand {
    metadata: CommandMetadata,
}

impl NoOpCommand {
    /// Create a new no-op command
    pub fn new(description: String) -> Self {
        Self {
            metadata: CommandMetadata::new_non_modifying(description),
        }
    }
}

impl Command for NoOpCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, _document: &mut Document) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn undo(&self, _document: &mut Document) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn modifies_document(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::Document;

    #[test]
    fn test_composite_command_creation() {
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(NoOpCommand::new("First".to_string())),
            Box::new(NoOpCommand::new("Second".to_string())),
        ];

        let composite = CompositeCommand::new("Composite Test".to_string(), commands);
        assert_eq!(composite.description(), "Composite Test");
        assert_eq!(composite.command_count(), 2);
        assert!(!composite.modifies_document()); // NoOp commands don't modify
    }

    #[test]
    fn test_composite_command_execution() {
        let mut document = Document::new("Test".to_string(), 100, 100);

        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(NoOpCommand::new("First".to_string())),
            Box::new(NoOpCommand::new("Second".to_string())),
        ];

        let composite = CompositeCommand::new("Composite Test".to_string(), commands);

        // Should execute without error
        assert!(composite.execute(&mut document).is_ok());
        assert!(composite.undo(&mut document).is_ok());
    }

    #[test]
    fn test_noop_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let noop = NoOpCommand::new("No Operation".to_string());

        assert_eq!(noop.description(), "No Operation");
        assert!(!noop.modifies_document());
        assert!(noop.execute(&mut document).is_ok());
        assert!(noop.undo(&mut document).is_ok());
    }

    #[test]
    fn test_composite_command_add() {
        let mut composite = CompositeCommand::new("Test".to_string(), vec![]);
        assert_eq!(composite.command_count(), 0);

        composite.add_command(Box::new(NoOpCommand::new("Added".to_string())));
        assert_eq!(composite.command_count(), 1);
    }
}
