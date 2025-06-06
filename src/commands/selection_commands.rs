//! Selection-related commands for undo/redo functionality
//!
//! This module contains commands for selection operations including:
//! - Creating and modifying selections
//! - Moving selection contents
//! - Selection transformations

use anyhow::Result;
use psoc_core::{Command, CommandMetadata, Document, Point, Selection};
use std::fmt::Debug;
use uuid::Uuid;

/// Command to create or modify a selection
#[derive(Debug)]
pub struct SetSelectionCommand {
    metadata: CommandMetadata,
    old_selection: Selection,
    new_selection: Selection,
}

impl SetSelectionCommand {
    /// Create a new set selection command
    pub fn new(new_selection: Selection, document: &Document) -> Self {
        Self {
            metadata: CommandMetadata::new("Set Selection".to_string()),
            old_selection: document.selection.clone(),
            new_selection,
        }
    }
}

impl Command for SetSelectionCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.new_selection.clone());
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.old_selection.clone());
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to clear the current selection
#[derive(Debug)]
pub struct ClearSelectionCommand {
    metadata: CommandMetadata,
    old_selection: Selection,
}

impl ClearSelectionCommand {
    /// Create a new clear selection command
    pub fn new(document: &Document) -> Self {
        Self {
            metadata: CommandMetadata::new("Clear Selection".to_string()),
            old_selection: document.selection.clone(),
        }
    }
}

impl Command for ClearSelectionCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.set_selection(Selection::None);
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.old_selection.clone());
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to move selection contents
#[derive(Debug)]
pub struct MoveSelectionCommand {
    metadata: CommandMetadata,
    layer_index: usize,
    offset: Point,
    old_selection: Selection,
    new_selection: Selection,
}

impl MoveSelectionCommand {
    /// Create a new move selection command
    pub fn new(layer_index: usize, offset: Point, document: &Document) -> Result<Self> {
        let old_selection = document.selection.clone();
        let new_selection = match &old_selection {
            Selection::Rectangle(rect_sel) => {
                let mut new_rect = rect_sel.clone();
                new_rect.rect.x += offset.x;
                new_rect.rect.y += offset.y;
                Selection::Rectangle(new_rect)
            }
            Selection::None => Selection::None,
        };

        Ok(Self {
            metadata: CommandMetadata::new("Move Selection".to_string()),
            layer_index,
            offset,
            old_selection,
            new_selection,
        })
    }
}

impl Command for MoveSelectionCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // Move the selection
        document.set_selection(self.new_selection.clone());

        // TODO: In a full implementation, we would also move the actual pixel content
        // within the selection area. This would require:
        // 1. Extracting pixels from the old selection area
        // 2. Clearing the old area
        // 3. Placing pixels in the new area

        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        // Restore the original selection
        document.set_selection(self.old_selection.clone());

        // TODO: Restore the original pixel content

        Ok(())
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
        Err(anyhow::anyhow!(
            "Selection move merging not yet implemented"
        ))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to invert the current selection
#[derive(Debug)]
pub struct InvertSelectionCommand {
    metadata: CommandMetadata,
    old_selection: Selection,
    document_bounds: (f32, f32), // width, height
}

impl InvertSelectionCommand {
    /// Create a new invert selection command
    pub fn new(document: &Document) -> Self {
        Self {
            metadata: CommandMetadata::new("Invert Selection".to_string()),
            old_selection: document.selection.clone(),
            document_bounds: (document.size.width, document.size.height),
        }
    }
}

impl Command for InvertSelectionCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // TODO: Implement proper selection inversion
        // For now, we'll just clear the selection as a placeholder
        match &self.old_selection {
            Selection::None => {
                // If no selection, select all
                let full_rect = psoc_core::RectangleSelection {
                    rect: psoc_core::Rect {
                        x: 0.0,
                        y: 0.0,
                        width: self.document_bounds.0,
                        height: self.document_bounds.1,
                    },
                    inverted: false,
                };
                document.set_selection(Selection::Rectangle(full_rect));
            }
            Selection::Rectangle(_) => {
                // If there's a selection, clear it (simplified inversion)
                document.set_selection(Selection::None);
            }
        }
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.old_selection.clone());
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to select all content in the document
#[derive(Debug)]
pub struct SelectAllCommand {
    metadata: CommandMetadata,
    old_selection: Selection,
    document_bounds: (f32, f32),
}

impl SelectAllCommand {
    /// Create a new select all command
    pub fn new(document: &Document) -> Self {
        Self {
            metadata: CommandMetadata::new("Select All".to_string()),
            old_selection: document.selection.clone(),
            document_bounds: (document.size.width, document.size.height),
        }
    }
}

impl Command for SelectAllCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        let full_rect = psoc_core::RectangleSelection {
            rect: psoc_core::Rect {
                x: 0.0,
                y: 0.0,
                width: self.document_bounds.0,
                height: self.document_bounds.1,
            },
            inverted: false,
        };
        document.set_selection(Selection::Rectangle(full_rect));
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.old_selection.clone());
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to transform a selection (resize, rotate, etc.)
#[derive(Debug)]
pub struct TransformSelectionCommand {
    metadata: CommandMetadata,
    old_selection: Selection,
    new_selection: Selection,
    transform_type: String,
}

impl TransformSelectionCommand {
    /// Create a new transform selection command
    pub fn new(new_selection: Selection, transform_type: String, document: &Document) -> Self {
        Self {
            metadata: CommandMetadata::new(format!("Transform Selection ({})", transform_type)),
            old_selection: document.selection.clone(),
            new_selection,
            transform_type,
        }
    }
}

impl Command for TransformSelectionCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.new_selection.clone());
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.set_selection(self.old_selection.clone());
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Document, RectangleSelection};

    #[test]
    fn test_set_selection_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let new_selection = Selection::Rectangle(RectangleSelection {
            rect: psoc_core::Rect {
                x: 10.0,
                y: 10.0,
                width: 50.0,
                height: 50.0,
            },
            inverted: false,
        });

        let command = SetSelectionCommand::new(new_selection.clone(), &document);
        assert!(command.execute(&mut document).is_ok());

        // Check that selection was set
        match &document.selection {
            Selection::Rectangle(rect) => {
                assert_eq!(rect.rect.x, 10.0);
                assert_eq!(rect.rect.y, 10.0);
                assert_eq!(rect.rect.width, 50.0);
                assert_eq!(rect.rect.height, 50.0);
            }
            _ => panic!("Expected rectangle selection"),
        }

        // Test undo
        assert!(command.undo(&mut document).is_ok());
        assert!(matches!(document.selection, Selection::None));
    }

    #[test]
    fn test_clear_selection_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let selection = Selection::Rectangle(RectangleSelection {
            rect: psoc_core::Rect {
                x: 10.0,
                y: 10.0,
                width: 50.0,
                height: 50.0,
            },
            inverted: false,
        });
        document.set_selection(selection);

        let command = ClearSelectionCommand::new(&document);
        assert!(command.execute(&mut document).is_ok());
        assert!(matches!(document.selection, Selection::None));

        // Test undo
        assert!(command.undo(&mut document).is_ok());
        assert!(matches!(document.selection, Selection::Rectangle(_)));
    }

    #[test]
    fn test_select_all_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let command = SelectAllCommand::new(&document);

        assert!(command.execute(&mut document).is_ok());

        // Check that entire document is selected
        match &document.selection {
            Selection::Rectangle(rect) => {
                assert_eq!(rect.rect.x, 0.0);
                assert_eq!(rect.rect.y, 0.0);
                assert_eq!(rect.rect.width, 100.0);
                assert_eq!(rect.rect.height, 100.0);
            }
            _ => panic!("Expected rectangle selection"),
        }

        // Test undo
        assert!(command.undo(&mut document).is_ok());
        assert!(matches!(document.selection, Selection::None));
    }

    #[test]
    fn test_move_selection_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let selection = Selection::Rectangle(RectangleSelection {
            rect: psoc_core::Rect {
                x: 10.0,
                y: 10.0,
                width: 50.0,
                height: 50.0,
            },
            inverted: false,
        });
        document.set_selection(selection);

        let offset = Point::new(20.0, 30.0);
        let command = MoveSelectionCommand::new(0, offset, &document).unwrap();

        assert!(command.execute(&mut document).is_ok());

        // Check that selection was moved
        match &document.selection {
            Selection::Rectangle(rect) => {
                assert_eq!(rect.rect.x, 30.0); // 10.0 + 20.0
                assert_eq!(rect.rect.y, 40.0); // 10.0 + 30.0
                assert_eq!(rect.rect.width, 50.0);
                assert_eq!(rect.rect.height, 50.0);
            }
            _ => panic!("Expected rectangle selection"),
        }

        // Test undo
        assert!(command.undo(&mut document).is_ok());
        match &document.selection {
            Selection::Rectangle(rect) => {
                assert_eq!(rect.rect.x, 10.0);
                assert_eq!(rect.rect.y, 10.0);
            }
            _ => panic!("Expected rectangle selection"),
        }
    }

    #[test]
    fn test_invert_selection_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let command = InvertSelectionCommand::new(&document);

        // With no selection, invert should select all
        assert!(command.execute(&mut document).is_ok());
        assert!(matches!(document.selection, Selection::Rectangle(_)));

        // Test undo
        assert!(command.undo(&mut document).is_ok());
        assert!(matches!(document.selection, Selection::None));
    }
}
