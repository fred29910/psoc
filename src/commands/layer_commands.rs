//! Layer-related commands for undo/redo functionality
//!
//! This module contains commands for all layer operations including:
//! - Adding and removing layers
//! - Moving layers up/down
//! - Changing layer properties (visibility, opacity, blend mode)
//! - Duplicating layers

use anyhow::Result;
use psoc_core::{Command, CommandMetadata, Document, Layer};
use std::fmt::Debug;
use uuid::Uuid;

/// Command to add a new layer to the document
#[derive(Debug)]
pub struct AddLayerCommand {
    metadata: CommandMetadata,
    layer: Layer,
    insert_index: usize,
}

impl AddLayerCommand {
    /// Create a new add layer command
    pub fn new(layer: Layer, insert_index: usize) -> Self {
        Self {
            metadata: CommandMetadata::new(format!("Add Layer '{}'", layer.name)),
            layer,
            insert_index,
        }
    }
}

impl Command for AddLayerCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.insert_layer(self.insert_index, self.layer.clone())?;
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.remove_layer(self.insert_index)?;
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
}

/// Command to remove a layer from the document
#[derive(Debug)]
pub struct RemoveLayerCommand {
    metadata: CommandMetadata,
    layer: Option<Layer>,
    layer_index: usize,
    was_active: bool,
}

impl RemoveLayerCommand {
    /// Create a new remove layer command
    pub fn new(layer_index: usize, document: &Document) -> Result<Self> {
        let layer = document
            .get_layer(layer_index)
            .ok_or_else(|| anyhow::anyhow!("Layer index out of bounds"))?;

        let was_active = document.active_layer_index == Some(layer_index);

        Ok(Self {
            metadata: CommandMetadata::new(format!("Remove Layer '{}'", layer.name)),
            layer: Some(layer.clone()),
            layer_index,
            was_active,
        })
    }
}

impl Command for RemoveLayerCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.remove_layer(self.layer_index)?;
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = &self.layer {
            document.insert_layer(self.layer_index, layer.clone())?;
            if self.was_active {
                document.set_active_layer(self.layer_index)?;
            }
        }
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
}

/// Command to move a layer to a different position
#[derive(Debug)]
pub struct MoveLayerCommand {
    metadata: CommandMetadata,
    from_index: usize,
    to_index: usize,
}

impl MoveLayerCommand {
    /// Create a new move layer command
    pub fn new(from_index: usize, to_index: usize, document: &Document) -> Result<Self> {
        let layer = document
            .get_layer(from_index)
            .ok_or_else(|| anyhow::anyhow!("Layer index out of bounds"))?;

        Ok(Self {
            metadata: CommandMetadata::new(format!("Move Layer '{}'", layer.name)),
            from_index,
            to_index,
        })
    }
}

impl Command for MoveLayerCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.move_layer(self.from_index, self.to_index)?;
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.move_layer(self.to_index, self.from_index)?;
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
}

/// Command to change layer visibility
#[derive(Debug)]
pub struct ToggleLayerVisibilityCommand {
    metadata: CommandMetadata,
    layer_index: usize,
    old_visibility: bool,
}

impl ToggleLayerVisibilityCommand {
    /// Create a new toggle layer visibility command
    pub fn new(layer_index: usize, document: &Document) -> Result<Self> {
        let layer = document
            .get_layer(layer_index)
            .ok_or_else(|| anyhow::anyhow!("Layer index out of bounds"))?;

        Ok(Self {
            metadata: CommandMetadata::new(format!("Toggle Visibility '{}'", layer.name)),
            layer_index,
            old_visibility: layer.visible,
        })
    }
}

impl Command for ToggleLayerVisibilityCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = document.get_layer_mut(self.layer_index) {
            layer.visible = !self.old_visibility;
            document.mark_dirty();
        }
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = document.get_layer_mut(self.layer_index) {
            layer.visible = self.old_visibility;
            document.mark_dirty();
        }
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
}

/// Command to change layer opacity
#[derive(Debug)]
pub struct ChangeLayerOpacityCommand {
    metadata: CommandMetadata,
    layer_index: usize,
    old_opacity: f32,
    new_opacity: f32,
}

impl ChangeLayerOpacityCommand {
    /// Create a new change layer opacity command
    pub fn new(layer_index: usize, new_opacity: f32, document: &Document) -> Result<Self> {
        let layer = document
            .get_layer(layer_index)
            .ok_or_else(|| anyhow::anyhow!("Layer index out of bounds"))?;

        Ok(Self {
            metadata: CommandMetadata::new(format!("Change Opacity '{}'", layer.name)),
            layer_index,
            old_opacity: layer.opacity,
            new_opacity,
        })
    }
}

impl Command for ChangeLayerOpacityCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = document.get_layer_mut(self.layer_index) {
            layer.opacity = self.new_opacity;
            document.mark_dirty();
        }
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        if let Some(layer) = document.get_layer_mut(self.layer_index) {
            layer.opacity = self.old_opacity;
            document.mark_dirty();
        }
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
        Err(anyhow::anyhow!("Command merging not implemented"))
    }
}

/// Command to duplicate a layer
#[derive(Debug)]
pub struct DuplicateLayerCommand {
    metadata: CommandMetadata,
    source_index: usize,
    insert_index: usize,
}

impl DuplicateLayerCommand {
    /// Create a new duplicate layer command
    pub fn new(source_index: usize, document: &Document) -> Result<Self> {
        let layer = document
            .get_layer(source_index)
            .ok_or_else(|| anyhow::anyhow!("Layer index out of bounds"))?;

        Ok(Self {
            metadata: CommandMetadata::new(format!("Duplicate Layer '{}'", layer.name)),
            source_index,
            insert_index: source_index + 1,
        })
    }
}

impl Command for DuplicateLayerCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        document.duplicate_layer(self.source_index)?;
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        document.remove_layer(self.insert_index)?;
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Document, Layer};

    #[test]
    fn test_add_layer_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);

        let command = AddLayerCommand::new(layer, 0);
        assert!(command.execute(&mut document).is_ok());
        assert_eq!(document.layer_count(), 1);

        assert!(command.undo(&mut document).is_ok());
        assert_eq!(document.layer_count(), 0);
    }

    #[test]
    fn test_remove_layer_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);

        let command = RemoveLayerCommand::new(0, &document).unwrap();
        assert!(command.execute(&mut document).is_ok());
        assert_eq!(document.layer_count(), 0);

        assert!(command.undo(&mut document).is_ok());
        assert_eq!(document.layer_count(), 1);
    }

    #[test]
    fn test_toggle_visibility_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);

        let original_visibility = document.get_layer(0).unwrap().visible;
        let command = ToggleLayerVisibilityCommand::new(0, &document).unwrap();

        assert!(command.execute(&mut document).is_ok());
        assert_ne!(document.get_layer(0).unwrap().visible, original_visibility);

        assert!(command.undo(&mut document).is_ok());
        assert_eq!(document.get_layer(0).unwrap().visible, original_visibility);
    }

    #[test]
    fn test_change_opacity_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);

        let original_opacity = document.get_layer(0).unwrap().opacity;
        let new_opacity = 0.5;
        let command = ChangeLayerOpacityCommand::new(0, new_opacity, &document).unwrap();

        assert!(command.execute(&mut document).is_ok());
        assert_eq!(document.get_layer(0).unwrap().opacity, new_opacity);

        assert!(command.undo(&mut document).is_ok());
        assert_eq!(document.get_layer(0).unwrap().opacity, original_opacity);
    }

    #[test]
    fn test_duplicate_layer_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        document.add_layer(layer);

        let command = DuplicateLayerCommand::new(0, &document).unwrap();
        assert!(command.execute(&mut document).is_ok());
        assert_eq!(document.layer_count(), 2);

        assert!(command.undo(&mut document).is_ok());
        assert_eq!(document.layer_count(), 1);
    }
}
