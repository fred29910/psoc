//! Adjustment-related commands for undo/redo functionality
//!
//! This module contains commands for image adjustments and filters including:
//! - Apply adjustment commands
//! - Adjustment layer commands
//! - Filter application commands

use anyhow::Result;
use psoc_core::{adjustment::AdjustmentApplication, Command, CommandMetadata, Document, PixelData};
use std::fmt::Debug;
use uuid::Uuid;

/// Command to apply an adjustment to a layer
#[derive(Debug)]
pub struct ApplyAdjustmentCommand {
    metadata: CommandMetadata,
    application: AdjustmentApplication,
    backup_data: Option<PixelData>,
}

impl ApplyAdjustmentCommand {
    /// Create a new apply adjustment command
    pub fn new(application: AdjustmentApplication) -> Self {
        let description = format!(
            "Apply {} to layer {}",
            application.adjustment_id, application.layer_index
        );

        Self {
            metadata: CommandMetadata::new(description),
            application,
            backup_data: None,
        }
    }

    /// Backup the affected region before applying the adjustment
    fn backup_region(&mut self, document: &Document) -> Result<()> {
        let layer = document
            .get_layer(self.application.layer_index)
            .ok_or_else(|| {
                anyhow::anyhow!("Layer index {} out of bounds", self.application.layer_index)
            })?;

        if let Some(pixel_data) = &layer.pixel_data {
            // For now, backup the entire layer
            // In a more sophisticated implementation, we could backup only the affected region
            self.backup_data = Some(pixel_data.clone());
        }

        Ok(())
    }

    /// Restore the backed up region
    fn restore_region(&self, document: &mut Document) -> Result<()> {
        if let Some(backup_data) = &self.backup_data {
            let layer = document
                .get_layer_mut(self.application.layer_index)
                .ok_or_else(|| {
                    anyhow::anyhow!("Layer index {} out of bounds", self.application.layer_index)
                })?;

            layer.pixel_data = Some(backup_data.clone());
            document.mark_dirty();
        }

        Ok(())
    }
}

impl Command for ApplyAdjustmentCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // We need to make self mutable to backup the region
        // This is a limitation of the current design - in a real implementation,
        // we might want to separate the backup phase from execution
        let mut cmd = ApplyAdjustmentCommand {
            metadata: self.metadata.clone(),
            application: self.application.clone(),
            backup_data: None,
        };

        // Backup the region before applying
        cmd.backup_region(document)?;

        // Apply the adjustment using a global registry
        // In a real implementation, the registry would be passed as a parameter
        // or stored in the document/application state
        let registry = get_global_adjustment_registry();
        psoc_core::adjustment::apply_adjustment_to_document(
            document,
            &self.application,
            &registry,
        )?;

        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        self.restore_region(document)
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn can_merge_with(&self, _other: &dyn Command) -> bool {
        // For now, don't merge adjustment commands
        // In a more sophisticated implementation, we could merge similar adjustments
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to create an adjustment layer
#[derive(Debug)]
pub struct CreateAdjustmentLayerCommand {
    metadata: CommandMetadata,
    layer_name: String,
    adjustment_id: String,
    parameters: serde_json::Value,
    insert_index: usize,
    created_layer_id: Option<Uuid>,
}

impl CreateAdjustmentLayerCommand {
    /// Create a new adjustment layer command
    pub fn new(
        layer_name: String,
        adjustment_id: String,
        parameters: serde_json::Value,
        insert_index: usize,
    ) -> Self {
        let description = format!("Create {} adjustment layer '{}'", adjustment_id, layer_name);

        Self {
            metadata: CommandMetadata::new(description),
            layer_name,
            adjustment_id,
            parameters,
            insert_index,
            created_layer_id: None,
        }
    }
}

impl Command for CreateAdjustmentLayerCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // Create an adjustment layer
        // For now, we'll create a regular layer with metadata indicating it's an adjustment layer
        // In a full implementation, we would have a dedicated AdjustmentLayer type

        let layer = psoc_core::Layer::new_pixel(self.layer_name.clone(), 1, 1);
        let _layer_id = layer.id;

        document.insert_layer(self.insert_index, layer)?;

        // Store the created layer ID for undo
        // Note: This is a hack since we can't modify self in execute()
        // In a real implementation, we'd handle this differently

        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        // Remove the created adjustment layer
        if self.insert_index < document.layers.len() {
            document.remove_layer(self.insert_index)?;
        }
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Command to modify adjustment layer parameters
#[derive(Debug)]
pub struct ModifyAdjustmentCommand {
    metadata: CommandMetadata,
    layer_index: usize,
    old_parameters: serde_json::Value,
    new_parameters: serde_json::Value,
}

impl ModifyAdjustmentCommand {
    /// Create a new modify adjustment command
    pub fn new(
        layer_index: usize,
        old_parameters: serde_json::Value,
        new_parameters: serde_json::Value,
    ) -> Self {
        let description = format!("Modify adjustment layer {}", layer_index);

        Self {
            metadata: CommandMetadata::new(description),
            layer_index,
            old_parameters,
            new_parameters,
        }
    }
}

impl Command for ModifyAdjustmentCommand {
    fn id(&self) -> Uuid {
        self.metadata.id
    }

    fn description(&self) -> &str {
        &self.metadata.description
    }

    fn execute(&self, document: &mut Document) -> Result<()> {
        // Apply new parameters to the adjustment layer
        // This would update the adjustment layer's parameters and re-render
        // For now, this is a placeholder implementation
        document.mark_dirty();
        Ok(())
    }

    fn undo(&self, document: &mut Document) -> Result<()> {
        // Restore old parameters to the adjustment layer
        // This would update the adjustment layer's parameters and re-render
        // For now, this is a placeholder implementation
        document.mark_dirty();
        Ok(())
    }

    fn timestamp(&self) -> std::time::SystemTime {
        self.metadata.timestamp
    }

    fn can_merge_with(&self, other: &dyn Command) -> bool {
        // Check if we can merge with another ModifyAdjustmentCommand for the same layer
        if let Some(other_cmd) = other.as_any().downcast_ref::<ModifyAdjustmentCommand>() {
            self.layer_index == other_cmd.layer_index
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Global adjustment registry for commands
/// In a real implementation, this would be managed by the application state
fn get_global_adjustment_registry() -> psoc_core::adjustment::AdjustmentRegistry {
    let mut registry = psoc_core::adjustment::AdjustmentRegistry::new();

    // Register built-in adjustments
    registry.register(Box::new(psoc_core::BrightnessAdjustment::identity()));
    registry.register(Box::new(psoc_core::ContrastAdjustment::identity()));

    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{
        adjustment::{AdjustmentApplication, AdjustmentScope},
        Document, Layer,
    };

    #[test]
    fn test_apply_adjustment_command_creation() {
        let params = serde_json::json!({ "brightness": 0.2 });
        let application = AdjustmentApplication::new(
            "brightness".to_string(),
            params,
            AdjustmentScope::EntireLayer,
            0,
        );

        let command = ApplyAdjustmentCommand::new(application);

        assert!(command
            .description()
            .contains("Apply brightness to layer 0"));
        assert!(command.backup_data.is_none());
    }

    #[test]
    fn test_create_adjustment_layer_command() {
        let params = serde_json::json!({ "brightness": 0.2 });
        let command = CreateAdjustmentLayerCommand::new(
            "Brightness".to_string(),
            "brightness".to_string(),
            params,
            1,
        );

        assert!(command
            .description()
            .contains("Create brightness adjustment layer 'Brightness'"));
        assert_eq!(command.insert_index, 1);
    }

    #[test]
    fn test_modify_adjustment_command() {
        let old_params = serde_json::json!({ "brightness": 0.2 });
        let new_params = serde_json::json!({ "brightness": 0.5 });

        let command = ModifyAdjustmentCommand::new(0, old_params, new_params);

        assert!(command.description().contains("Modify adjustment layer 0"));
        assert_eq!(command.layer_index, 0);
    }

    #[test]
    fn test_create_adjustment_layer_execution() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Base Layer".to_string(), 100, 100);
        document.add_layer(layer);

        let params = serde_json::json!({ "brightness": 0.2 });
        let command = CreateAdjustmentLayerCommand::new(
            "Brightness".to_string(),
            "brightness".to_string(),
            params,
            1,
        );

        let initial_layer_count = document.layers.len();
        command.execute(&mut document).unwrap();

        assert_eq!(document.layers.len(), initial_layer_count + 1);
        assert_eq!(document.layers[1].name, "Brightness");
    }

    #[test]
    fn test_create_adjustment_layer_undo() {
        let mut document = Document::new("Test".to_string(), 100, 100);
        let layer = Layer::new_pixel("Base Layer".to_string(), 100, 100);
        document.add_layer(layer);

        let params = serde_json::json!({ "brightness": 0.2 });
        let command = CreateAdjustmentLayerCommand::new(
            "Brightness".to_string(),
            "brightness".to_string(),
            params,
            1,
        );

        // Execute and then undo
        command.execute(&mut document).unwrap();
        let layer_count_after_execute = document.layers.len();

        command.undo(&mut document).unwrap();
        let layer_count_after_undo = document.layers.len();

        assert_eq!(layer_count_after_execute, 2);
        assert_eq!(layer_count_after_undo, 1);
    }

    #[test]
    fn test_modify_adjustment_command_merge() {
        let old_params1 = serde_json::json!({ "brightness": 0.2 });
        let new_params1 = serde_json::json!({ "brightness": 0.5 });
        let old_params2 = serde_json::json!({ "brightness": 0.5 });
        let new_params2 = serde_json::json!({ "brightness": 0.8 });

        let command1 = ModifyAdjustmentCommand::new(0, old_params1.clone(), new_params1.clone());
        let command2 = ModifyAdjustmentCommand::new(0, old_params2, new_params2);
        let command3 = ModifyAdjustmentCommand::new(1, old_params1, new_params1);

        // Commands for the same layer should be mergeable
        assert!(command1.can_merge_with(&command2));

        // Commands for different layers should not be mergeable
        assert!(!command1.can_merge_with(&command3));
    }
}
