//! Transform commands for applying transformations to layers and selections
//!
//! This module provides commands for applying various transformations such as
//! scaling, rotation, and flipping to layers or selections.

use psoc_core::{Command, CommandMetadata, Document, Layer, Rect, Transform};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

/// Command to apply a transformation to a layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyTransformCommand {
    /// Command metadata
    metadata: CommandMetadata,
    /// Target layer ID
    layer_id: Uuid,
    /// Transformation to apply
    transform: Transform,
    /// Original layer state for undo
    original_transform: Option<Transform>,
    /// Original bounds for undo
    original_bounds: Option<Rect>,
}

impl ApplyTransformCommand {
    /// Create a new apply transform command
    pub fn new(layer_id: Uuid, transform: Transform) -> Self {
        Self {
            metadata: CommandMetadata::new(format!("Apply transformation to layer {}", layer_id)),
            layer_id,
            transform,
            original_transform: None,
            original_bounds: None,
        }
    }

    /// Create command for scaling a layer
    pub fn scale_layer(layer_id: Uuid, scale_x: f32, scale_y: f32) -> Self {
        let transform = Transform::scale(scale_x, scale_y);
        Self::new(layer_id, transform)
    }

    /// Create command for rotating a layer
    pub fn rotate_layer(layer_id: Uuid, angle: f32) -> Self {
        let transform = Transform::rotation(angle);
        Self::new(layer_id, transform)
    }

    /// Create command for translating a layer
    pub fn translate_layer(layer_id: Uuid, dx: f32, dy: f32) -> Self {
        let transform = Transform::translation(dx, dy);
        Self::new(layer_id, transform)
    }

    /// Create command for flipping a layer horizontally
    pub fn flip_horizontal(layer_id: Uuid) -> Self {
        let transform = Transform::scale(-1.0, 1.0);
        Self::new(layer_id, transform)
    }

    /// Create command for flipping a layer vertically
    pub fn flip_vertical(layer_id: Uuid) -> Self {
        let transform = Transform::scale(1.0, -1.0);
        Self::new(layer_id, transform)
    }

    /// Find layer by ID in document
    fn find_layer_mut(document: &mut Document, layer_id: Uuid) -> Option<&mut Layer> {
        document
            .layers
            .iter_mut()
            .find(|layer| layer.id == layer_id)
    }

    /// Find layer by ID in document (immutable)
    #[allow(dead_code)]
    fn find_layer(document: &Document, layer_id: Uuid) -> Option<&Layer> {
        document.layers.iter().find(|layer| layer.id == layer_id)
    }
}

impl Command for ApplyTransformCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> anyhow::Result<()> {
        let layer = Self::find_layer_mut(document, self.layer_id)
            .ok_or_else(|| anyhow::anyhow!("Layer not found: {}", self.layer_id))?;

        // Apply the transformation
        layer.apply_transform(self.transform);
        document.mark_dirty();

        Ok(())
    }

    fn undo(&self, document: &mut Document) -> anyhow::Result<()> {
        // For undo, we need to apply the inverse transformation
        let layer = Self::find_layer_mut(document, self.layer_id)
            .ok_or_else(|| anyhow::anyhow!("Layer not found: {}", self.layer_id))?;

        // Apply inverse transformation
        let inverse_transform = self.transform.inverse();
        layer.apply_transform(inverse_transform);
        document.mark_dirty();

        Ok(())
    }

    fn timestamp(&self) -> SystemTime {
        self.metadata.timestamp
    }

    fn can_merge_with(&self, other: &dyn Command) -> bool {
        // Check if other command is also an ApplyTransformCommand for the same layer
        if let Some(other_transform) = other.as_any().downcast_ref::<ApplyTransformCommand>() {
            self.layer_id == other_transform.layer_id
        } else {
            false
        }
    }

    fn merge_with(&mut self, _other: Box<dyn Command>) -> anyhow::Result<()> {
        // For simplicity, we'll not implement merging for now
        Err(anyhow::anyhow!("Transform command merging not implemented"))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to reset a layer's transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetTransformCommand {
    /// Command metadata
    metadata: CommandMetadata,
    /// Target layer ID
    layer_id: Uuid,
    /// Original transform for undo
    original_transform: Transform,
}

impl ResetTransformCommand {
    /// Create a new reset transform command
    pub fn new(layer_id: Uuid, original_transform: Transform) -> Self {
        Self {
            metadata: CommandMetadata::new(format!("Reset transformation for layer {}", layer_id)),
            layer_id,
            original_transform,
        }
    }
}

impl Command for ResetTransformCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> anyhow::Result<()> {
        let layer = ApplyTransformCommand::find_layer_mut(document, self.layer_id)
            .ok_or_else(|| anyhow::anyhow!("Layer not found: {}", self.layer_id))?;

        // Reset transformation
        layer.reset_transform();
        document.mark_dirty();

        Ok(())
    }

    fn undo(&self, document: &mut Document) -> anyhow::Result<()> {
        let layer = ApplyTransformCommand::find_layer_mut(document, self.layer_id)
            .ok_or_else(|| anyhow::anyhow!("Layer not found: {}", self.layer_id))?;

        // Restore original transformation
        layer.set_transform(self.original_transform);
        document.mark_dirty();

        Ok(())
    }

    fn timestamp(&self) -> SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Document, Layer};

    #[test]
    fn test_apply_transform_command_creation() {
        let layer_id = Uuid::new_v4();
        let transform = Transform::scale(2.0, 2.0);
        let command = ApplyTransformCommand::new(layer_id, transform);

        assert_eq!(command.layer_id, layer_id);
        assert_eq!(command.transform, transform);
        assert!(command.original_transform.is_none());
        assert!(command.original_bounds.is_none());
    }

    #[test]
    fn test_scale_layer_command() {
        let layer_id = Uuid::new_v4();
        let command = ApplyTransformCommand::scale_layer(layer_id, 2.0, 3.0);

        assert_eq!(command.layer_id, layer_id);
        assert_eq!(command.transform, Transform::scale(2.0, 3.0));
    }

    #[test]
    fn test_rotate_layer_command() {
        let layer_id = Uuid::new_v4();
        let angle = std::f32::consts::PI / 4.0; // 45 degrees
        let command = ApplyTransformCommand::rotate_layer(layer_id, angle);

        assert_eq!(command.layer_id, layer_id);
        assert_eq!(command.transform, Transform::rotation(angle));
    }

    #[test]
    fn test_flip_commands() {
        let layer_id = Uuid::new_v4();

        let flip_h = ApplyTransformCommand::flip_horizontal(layer_id);
        assert_eq!(flip_h.transform, Transform::scale(-1.0, 1.0));

        let flip_v = ApplyTransformCommand::flip_vertical(layer_id);
        assert_eq!(flip_v.transform, Transform::scale(1.0, -1.0));
    }

    #[test]
    fn test_apply_transform_execution() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        let layer_id = layer.id;
        document.add_layer(layer);

        let command = ApplyTransformCommand::scale_layer(layer_id, 2.0, 2.0);
        let result = command.execute(&mut document);
        assert!(result.is_ok());
        assert!(document.is_dirty);
    }

    #[test]
    fn test_apply_transform_undo() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        let layer_id = layer.id;
        let original_transform = layer.transform();
        document.add_layer(layer);

        let command = ApplyTransformCommand::scale_layer(layer_id, 2.0, 2.0);

        // Execute and then undo
        command.execute(&mut document).unwrap();
        command.undo(&mut document).unwrap();

        // Check that layer transform was restored (approximately)
        let layer = document.layers.iter().find(|l| l.id == layer_id).unwrap();
        // Due to floating point precision, we check if it's close to identity
        assert_eq!(layer.transform(), original_transform);
    }

    #[test]
    fn test_reset_transform_command() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 50, 50);
        let layer_id = layer.id;

        // Apply some transformation first
        let original_transform = Transform::scale(2.0, 2.0);
        layer.apply_transform(original_transform);
        document.add_layer(layer);

        let command = ResetTransformCommand::new(layer_id, original_transform);
        let result = command.execute(&mut document);
        assert!(result.is_ok());

        // Check that transform was reset
        let layer = document.layers.iter().find(|l| l.id == layer_id).unwrap();
        assert_eq!(layer.transform(), Transform::identity());
    }
}
