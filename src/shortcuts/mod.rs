//! Keyboard shortcuts system for PSOC
//!
//! This module provides a comprehensive keyboard shortcuts system that allows
//! users to quickly access common operations through key combinations.

pub mod keyboard_events;
pub mod shortcut_manager;
pub mod shortcut_types;

pub use keyboard_events::{iced_key_to_shortcut_key, iced_modifiers_to_shortcut_modifiers};
pub use shortcut_manager::ShortcutManager;
pub use shortcut_types::{Shortcut, ShortcutAction, ShortcutKey, ShortcutModifiers};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcuts_module_exports() {
        // Test that all main types are properly exported
        let _manager = ShortcutManager::new();
        let _shortcut = Shortcut::new(
            ShortcutKey::Character('n'),
            ShortcutModifiers::new().with_ctrl(true),
            ShortcutAction::NewDocument,
        );
    }
}
