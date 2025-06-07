//! Crop commands for image cropping operations
//!
//! This module contains commands for cropping images and layers.

use anyhow::Result;
use psoc_core::{Command, CommandMetadata, Document, Rect};
use std::fmt::Debug;
use uuid::Uuid;

/// Command to crop the entire document to a specified rectangle
#[derive(Debug)]
pub struct CropDocumentCommand {
    metadata: CommandMetadata,
    crop_rect: Rect,
    #[allow(dead_code)]
    original_size: Option<(u32, u32)>,
}

impl CropDocumentCommand {
    /// Create a new crop document command
    pub fn new(crop_rect: Rect) -> Self {
        Self {
            metadata: CommandMetadata::new(format!(
                "Crop Document to {}x{} at ({}, {})",
                crop_rect.width, crop_rect.height, crop_rect.x, crop_rect.y
            )),
            crop_rect,
            original_size: None,
        }
    }

    /// Get the crop rectangle
    pub fn crop_rect(&self) -> Rect {
        self.crop_rect
    }
}

impl Command for CropDocumentCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // TODO: Store original size for undo in a proper way
        // For now, we'll implement a simple version without storing state

        // TODO: Implement actual crop operation
        // This would involve:
        // 1. Cropping all layers to the specified rectangle
        // 2. Updating document dimensions
        // 3. Adjusting layer positions if needed

        // For now, just mark as dirty
        document.mark_dirty();

        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        // TODO: Implement undo operation
        // This would involve:
        // 1. Restoring original document size
        // 2. Restoring original layer content
        // 3. Repositioning layers

        document.mark_dirty();
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn modifies_document(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn merge_with(&mut self, other: Box<dyn Command>) -> Result<()> {
        // Crop commands generally don't merge well
        // Each crop operation should be separate
        let _ = other; // Suppress unused warning
        Ok(())
    }
}

/// Command to crop a specific layer to a rectangle
#[derive(Debug)]
pub struct CropLayerCommand {
    metadata: CommandMetadata,
    layer_id: Uuid,
    crop_rect: Rect,
}

impl CropLayerCommand {
    /// Create a new crop layer command
    pub fn new(layer_id: Uuid, crop_rect: Rect) -> Self {
        Self {
            metadata: CommandMetadata::new(format!(
                "Crop Layer to {}x{} at ({}, {})",
                crop_rect.width, crop_rect.height, crop_rect.x, crop_rect.y
            )),
            layer_id,
            crop_rect,
        }
    }

    /// Get the layer ID
    pub fn layer_id(&self) -> Uuid {
        self.layer_id
    }

    /// Get the crop rectangle
    pub fn crop_rect(&self) -> Rect {
        self.crop_rect
    }
}

impl Command for CropLayerCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // TODO: Implement layer-specific crop operation
        // This would involve:
        // 1. Finding the layer by ID
        // 2. Cropping the layer's pixel data
        // 3. Updating layer bounds

        document.mark_dirty();
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        // TODO: Implement undo operation
        // This would involve restoring the original layer content

        document.mark_dirty();
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn modifies_document(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn merge_with(&mut self, other: Box<dyn Command>) -> Result<()> {
        // Layer crop commands generally don't merge
        let _ = other; // Suppress unused warning
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Document, Rect};

    #[test]
    fn test_crop_document_command_creation() {
        let crop_rect = Rect::new(10.0, 20.0, 100.0, 80.0);
        let command = CropDocumentCommand::new(crop_rect);

        assert_eq!(command.crop_rect(), crop_rect);
        assert!(command.modifies_document());
        assert!(command.description().contains("Crop Document"));
    }

    #[test]
    fn test_crop_document_command_execution() {
        let crop_rect = Rect::new(10.0, 20.0, 100.0, 80.0);
        let command = CropDocumentCommand::new(crop_rect);
        let mut document = Document::new("Test".to_string(), 200, 150);

        // Should execute without error
        assert!(command.execute(&mut document).is_ok());
        assert!(command.undo(&mut document).is_ok());
    }

    #[test]
    fn test_crop_layer_command_creation() {
        let layer_id = Uuid::new_v4();
        let crop_rect = Rect::new(5.0, 10.0, 50.0, 40.0);
        let command = CropLayerCommand::new(layer_id, crop_rect);

        assert_eq!(command.layer_id(), layer_id);
        assert_eq!(command.crop_rect(), crop_rect);
        assert!(command.modifies_document());
        assert!(command.description().contains("Crop Layer"));
    }

    #[test]
    fn test_crop_layer_command_execution() {
        let layer_id = Uuid::new_v4();
        let crop_rect = Rect::new(5.0, 10.0, 50.0, 40.0);
        let command = CropLayerCommand::new(layer_id, crop_rect);
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Should execute without error
        assert!(command.execute(&mut document).is_ok());
        assert!(command.undo(&mut document).is_ok());
    }
}
