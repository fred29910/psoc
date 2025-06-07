//! Tests for the history panel functionality

use psoc_core::{Command, CommandHistory, Document, NavigationDirection};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Mock command for testing
#[derive(Debug)]
struct MockCommand {
    id: Uuid,
    description: String,
    executed: Arc<Mutex<bool>>,
    undone: Arc<Mutex<bool>>,
}

impl MockCommand {
    fn new(description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.to_string(),
            executed: Arc::new(Mutex::new(false)),
            undone: Arc::new(Mutex::new(false)),
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
        self.id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn execute(&self, _document: &mut Document) -> anyhow::Result<()> {
        *self.executed.lock().unwrap() = true;
        Ok(())
    }

    fn undo(&self, _document: &mut Document) -> anyhow::Result<()> {
        *self.undone.lock().unwrap() = true;
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        std::time::SystemTime::now()
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
    assert_eq!(history.current_position(), 0);
    assert_eq!(history.total_commands(), 0);
}

#[test]
fn test_command_execution_and_history() {
    let mut history = CommandHistory::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Execute first command
    let command1 = Box::new(MockCommand::new("First Command"));
    let command1_ptr = command1.as_ref() as *const MockCommand;
    history.execute_command(command1, &mut document).unwrap();

    assert!(history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.undo_count(), 1);
    assert_eq!(history.redo_count(), 0);
    assert_eq!(history.current_position(), 1);
    assert_eq!(history.total_commands(), 1);

    // Execute second command
    let command2 = Box::new(MockCommand::new("Second Command"));
    history.execute_command(command2, &mut document).unwrap();

    assert!(history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.undo_count(), 2);
    assert_eq!(history.redo_count(), 0);
    assert_eq!(history.current_position(), 2);
    assert_eq!(history.total_commands(), 2);

    // Verify first command was executed
    unsafe {
        assert!((*command1_ptr).was_executed());
    }
}

#[test]
fn test_undo_redo_operations() {
    let mut history = CommandHistory::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Execute commands
    let command1 = Box::new(MockCommand::new("Command 1"));
    let command2 = Box::new(MockCommand::new("Command 2"));
    let command1_ptr = command1.as_ref() as *const MockCommand;
    let command2_ptr = command2.as_ref() as *const MockCommand;

    history.execute_command(command1, &mut document).unwrap();
    history.execute_command(command2, &mut document).unwrap();

    // Test undo
    assert!(history.undo(&mut document).unwrap());
    assert_eq!(history.current_position(), 1);
    assert!(history.can_undo());
    assert!(history.can_redo());

    // Test redo
    assert!(history.redo(&mut document).unwrap());
    assert_eq!(history.current_position(), 2);
    assert!(history.can_undo());
    assert!(!history.can_redo());

    // Verify commands were executed and undone
    unsafe {
        assert!((*command1_ptr).was_executed());
        assert!((*command2_ptr).was_executed());
        assert!((*command2_ptr).was_undone());
    }
}

#[test]
fn test_history_descriptions() {
    let mut history = CommandHistory::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    let command = Box::new(MockCommand::new("Test Command"));
    history.execute_command(command, &mut document).unwrap();

    assert_eq!(history.undo_description(), Some("Test Command"));
    assert_eq!(history.redo_description(), None);

    history.undo(&mut document).unwrap();
    assert_eq!(history.undo_description(), None);
    assert_eq!(history.redo_description(), Some("Test Command"));
}

#[test]
fn test_history_entries() {
    let mut history = CommandHistory::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add some commands
    let command1 = Box::new(MockCommand::new("First"));
    let command2 = Box::new(MockCommand::new("Second"));
    let command3 = Box::new(MockCommand::new("Third"));

    history.execute_command(command1, &mut document).unwrap();
    history.execute_command(command2, &mut document).unwrap();
    history.execute_command(command3, &mut document).unwrap();

    let entries = history.get_history_entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].description, "First");
    assert_eq!(entries[1].description, "Second");
    assert_eq!(entries[2].description, "Third");
    assert!(entries[2].is_current); // Last command is current
    assert!(!entries[0].is_current);
    assert!(!entries[1].is_current);
}

#[test]
fn test_navigation_direction() {
    let mut history = CommandHistory::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add commands
    for i in 1..=5 {
        let command = Box::new(MockCommand::new(&format!("Command {}", i)));
        history.execute_command(command, &mut document).unwrap();
    }

    // Test navigation direction calculation
    assert_eq!(history.current_position(), 5);

    // Should navigate backward
    if let Some(NavigationDirection::Backward(steps)) = history.should_navigate_to_position(2) {
        assert_eq!(steps, 3);
    } else {
        panic!("Expected backward navigation");
    }

    // Undo some commands
    history.undo(&mut document).unwrap();
    history.undo(&mut document).unwrap();
    assert_eq!(history.current_position(), 3);

    // Should navigate forward
    if let Some(NavigationDirection::Forward(steps)) = history.should_navigate_to_position(4) {
        assert_eq!(steps, 1);
    } else {
        panic!("Expected forward navigation");
    }

    // No navigation needed
    assert!(history.should_navigate_to_position(3).is_none());
}

#[test]
fn test_document_history_navigation() {
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Test navigation with empty history
    assert!(!document.navigate_to_history_position(1).unwrap());
    assert_eq!(document.command_history.current_position(), 0);

    // Test navigation to same position
    assert!(!document.navigate_to_history_position(0).unwrap());
    assert_eq!(document.command_history.current_position(), 0);
}

#[test]
fn test_clear_history() {
    let mut history = CommandHistory::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add some commands
    let command1 = Box::new(MockCommand::new("Command 1"));
    let command2 = Box::new(MockCommand::new("Command 2"));
    history.execute_command(command1, &mut document).unwrap();
    history.execute_command(command2, &mut document).unwrap();

    assert_eq!(history.total_commands(), 2);
    assert!(history.can_undo());

    // Clear history
    history.clear();

    assert_eq!(history.total_commands(), 0);
    assert!(!history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.current_position(), 0);
    assert_eq!(history.undo_count(), 0);
    assert_eq!(history.redo_count(), 0);
}

#[test]
fn test_history_limits() {
    let mut history = CommandHistory::with_settings(3, false);
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add more commands than the limit
    for i in 1..=5 {
        let command = Box::new(MockCommand::new(&format!("Command {}", i)));
        history.execute_command(command, &mut document).unwrap();
    }

    // Should only keep the last 3 commands
    assert_eq!(history.total_commands(), 3);
    assert_eq!(history.current_position(), 3);

    let entries = history.get_history_entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].description, "Command 3");
    assert_eq!(entries[1].description, "Command 4");
    assert_eq!(entries[2].description, "Command 5");
}
