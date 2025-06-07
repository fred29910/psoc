//! Shortcut manager for handling keyboard shortcuts
//!
//! This module provides the ShortcutManager which handles registration,
//! lookup, and conflict detection for keyboard shortcuts.

use super::shortcut_types::{Shortcut, ShortcutAction, ShortcutKey, ShortcutModifiers};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Manages keyboard shortcuts for the application
#[derive(Debug, Clone)]
pub struct ShortcutManager {
    /// Map from (key, modifiers) to shortcut
    shortcuts: HashMap<(ShortcutKey, ShortcutModifiers), Shortcut>,
    /// Map from action to shortcut for reverse lookup
    action_map: HashMap<ShortcutAction, Shortcut>,
    /// Whether shortcuts are enabled globally
    enabled: bool,
}

impl Default for ShortcutManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShortcutManager {
    /// Create a new shortcut manager with default shortcuts
    pub fn new() -> Self {
        let mut manager = Self {
            shortcuts: HashMap::new(),
            action_map: HashMap::new(),
            enabled: true,
        };

        manager.register_default_shortcuts();
        manager
    }

    /// Create an empty shortcut manager
    pub fn empty() -> Self {
        Self {
            shortcuts: HashMap::new(),
            action_map: HashMap::new(),
            enabled: true,
        }
    }

    /// Register a shortcut
    pub fn register_shortcut(&mut self, shortcut: Shortcut) -> Result<(), String> {
        let key_combo = (shortcut.key, shortcut.modifiers);

        // Check for conflicts
        if let Some(existing) = self.shortcuts.get(&key_combo) {
            if existing.action != shortcut.action {
                return Err(format!(
                    "Shortcut conflict: {} is already assigned to {}",
                    shortcut.display_string(),
                    existing.action
                ));
            }
        }

        debug!("Registering shortcut: {}", shortcut);

        // Remove any existing shortcut for this action
        if let Some(old_shortcut) = self.action_map.get(&shortcut.action) {
            let old_key_combo = (old_shortcut.key, old_shortcut.modifiers);
            self.shortcuts.remove(&old_key_combo);
        }

        // Register the new shortcut
        self.action_map
            .insert(shortcut.action.clone(), shortcut.clone());
        self.shortcuts.insert(key_combo, shortcut);

        Ok(())
    }

    /// Unregister a shortcut by action
    pub fn unregister_shortcut(&mut self, action: &ShortcutAction) -> bool {
        if let Some(shortcut) = self.action_map.remove(action) {
            let key_combo = (shortcut.key, shortcut.modifiers);
            self.shortcuts.remove(&key_combo);
            debug!("Unregistered shortcut for action: {}", action);
            true
        } else {
            false
        }
    }

    /// Find a shortcut action for the given key and modifiers
    pub fn find_action(
        &self,
        key: ShortcutKey,
        modifiers: &ShortcutModifiers,
    ) -> Option<ShortcutAction> {
        if !self.enabled {
            return None;
        }

        let key_combo = (key, *modifiers);
        self.shortcuts
            .get(&key_combo)
            .filter(|shortcut| shortcut.enabled)
            .map(|shortcut| shortcut.action.clone())
    }

    /// Get the shortcut for a specific action
    pub fn get_shortcut_for_action(&self, action: &ShortcutAction) -> Option<&Shortcut> {
        self.action_map.get(action)
    }

    /// Get all registered shortcuts
    pub fn get_all_shortcuts(&self) -> Vec<&Shortcut> {
        self.shortcuts.values().collect()
    }

    /// Get shortcuts grouped by category
    pub fn get_shortcuts_by_category(&self) -> HashMap<String, Vec<&Shortcut>> {
        let mut categories = HashMap::new();

        for shortcut in self.shortcuts.values() {
            let category = self.get_action_category(&shortcut.action);
            categories
                .entry(category)
                .or_insert_with(Vec::new)
                .push(shortcut);
        }

        categories
    }

    /// Enable or disable all shortcuts
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!("Shortcuts {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Check if shortcuts are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Clear all shortcuts
    pub fn clear(&mut self) {
        self.shortcuts.clear();
        self.action_map.clear();
        info!("All shortcuts cleared");
    }

    /// Get the number of registered shortcuts
    pub fn count(&self) -> usize {
        self.shortcuts.len()
    }

    /// Check if a shortcut exists for the given key combination
    pub fn has_shortcut(&self, key: ShortcutKey, modifiers: &ShortcutModifiers) -> bool {
        let key_combo = (key, *modifiers);
        self.shortcuts.contains_key(&key_combo)
    }

    /// Validate that there are no conflicts in the current shortcuts
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut key_combos: HashMap<(ShortcutKey, ShortcutModifiers), &ShortcutAction> =
            HashMap::new();

        for shortcut in self.shortcuts.values() {
            let key_combo = (shortcut.key, shortcut.modifiers);
            if let Some(existing_action) = key_combos.get(&key_combo) {
                if **existing_action != shortcut.action {
                    errors.push(format!(
                        "Conflict: {} assigned to both {} and {}",
                        shortcut.display_string(),
                        existing_action,
                        shortcut.action
                    ));
                }
            } else {
                key_combos.insert(key_combo, &shortcut.action);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Register default shortcuts for common operations
    fn register_default_shortcuts(&mut self) {
        let default_shortcuts = vec![
            // File operations
            Shortcut::new(
                ShortcutKey::Character('n'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::NewDocument,
            ),
            Shortcut::new(
                ShortcutKey::Character('o'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::OpenDocument,
            ),
            Shortcut::new(
                ShortcutKey::Character('s'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::SaveDocument,
            ),
            Shortcut::new(
                ShortcutKey::Character('s'),
                ShortcutModifiers::new().with_ctrl(true).with_shift(true),
                ShortcutAction::SaveAsDocument,
            ),
            // Edit operations
            Shortcut::new(
                ShortcutKey::Character('z'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::Undo,
            ),
            Shortcut::new(
                ShortcutKey::Character('y'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::Redo,
            ),
            Shortcut::new(
                ShortcutKey::Character('x'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::Cut,
            ),
            Shortcut::new(
                ShortcutKey::Character('c'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::Copy,
            ),
            Shortcut::new(
                ShortcutKey::Character('v'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::Paste,
            ),
            Shortcut::new(
                ShortcutKey::Character('a'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::SelectAll,
            ),
            Shortcut::new(
                ShortcutKey::Character('d'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::DeselectAll,
            ),
            // Tool shortcuts
            Shortcut::new(
                ShortcutKey::Character('v'),
                ShortcutModifiers::new(),
                ShortcutAction::SelectTool,
            ),
            Shortcut::new(
                ShortcutKey::Character('b'),
                ShortcutModifiers::new(),
                ShortcutAction::BrushTool,
            ),
            Shortcut::new(
                ShortcutKey::Character('e'),
                ShortcutModifiers::new(),
                ShortcutAction::EraserTool,
            ),
            Shortcut::new(
                ShortcutKey::Character('m'),
                ShortcutModifiers::new(),
                ShortcutAction::MoveTool,
            ),
            Shortcut::new(
                ShortcutKey::Character('t'),
                ShortcutModifiers::new(),
                ShortcutAction::TransformTool,
            ),
            // View operations
            Shortcut::new(
                ShortcutKey::Character('+'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::ZoomIn,
            ),
            Shortcut::new(
                ShortcutKey::Character('-'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::ZoomOut,
            ),
            Shortcut::new(
                ShortcutKey::Character('0'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::ZoomReset,
            ),
            // Layer operations
            Shortcut::new(
                ShortcutKey::Character('j'),
                ShortcutModifiers::new().with_ctrl(true).with_shift(true),
                ShortcutAction::NewLayer,
            ),
            Shortcut::new(
                ShortcutKey::Character('j'),
                ShortcutModifiers::new().with_ctrl(true),
                ShortcutAction::DuplicateLayer,
            ),
        ];

        for shortcut in default_shortcuts {
            if let Err(e) = self.register_shortcut(shortcut) {
                warn!("Failed to register default shortcut: {}", e);
            }
        }

        info!("Registered {} default shortcuts", self.count());
    }

    /// Get the category for an action (for UI organization)
    fn get_action_category(&self, action: &ShortcutAction) -> String {
        match action {
            ShortcutAction::NewDocument
            | ShortcutAction::OpenDocument
            | ShortcutAction::SaveDocument
            | ShortcutAction::SaveAsDocument
            | ShortcutAction::ExportDocument => "File".to_string(),

            ShortcutAction::Undo
            | ShortcutAction::Redo
            | ShortcutAction::Cut
            | ShortcutAction::Copy
            | ShortcutAction::Paste
            | ShortcutAction::SelectAll
            | ShortcutAction::DeselectAll
            | ShortcutAction::InvertSelection => "Edit".to_string(),

            ShortcutAction::SelectTool
            | ShortcutAction::BrushTool
            | ShortcutAction::EraserTool
            | ShortcutAction::MoveTool
            | ShortcutAction::TransformTool
            | ShortcutAction::EyedropperTool => "Tools".to_string(),

            ShortcutAction::ZoomIn
            | ShortcutAction::ZoomOut
            | ShortcutAction::ZoomReset
            | ShortcutAction::ZoomFit
            | ShortcutAction::ToggleRulers
            | ShortcutAction::ToggleGrid
            | ShortcutAction::ToggleGuides => "View".to_string(),

            ShortcutAction::NewLayer
            | ShortcutAction::DuplicateLayer
            | ShortcutAction::DeleteLayer
            | ShortcutAction::MergeDown
            | ShortcutAction::FlattenImage => "Layer".to_string(),

            ShortcutAction::BrightnessContrast
            | ShortcutAction::HueSaturation
            | ShortcutAction::Levels
            | ShortcutAction::Curves => "Adjustments".to_string(),

            ShortcutAction::ShowPreferences | ShortcutAction::ShowAbout | ShortcutAction::Exit => {
                "Application".to_string()
            }

            ShortcutAction::Custom(_) => "Custom".to_string(),
        }
    }
}
