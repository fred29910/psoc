//! Tests for the keyboard shortcuts system

use psoc::shortcuts::{Shortcut, ShortcutAction, ShortcutKey, ShortcutManager, ShortcutModifiers};

#[test]
fn test_shortcut_creation() {
    let shortcut = Shortcut::new(
        ShortcutKey::Character('n'),
        ShortcutModifiers::new().with_ctrl(true),
        ShortcutAction::NewDocument,
    );

    assert_eq!(shortcut.key, ShortcutKey::Character('n'));
    assert!(shortcut.modifiers.ctrl);
    assert!(!shortcut.modifiers.shift);
    assert_eq!(shortcut.action, ShortcutAction::NewDocument);
    assert!(shortcut.enabled);
}

#[test]
fn test_shortcut_with_description() {
    let shortcut = Shortcut::with_description(
        ShortcutKey::Character('s'),
        ShortcutModifiers::new().with_ctrl(true),
        ShortcutAction::SaveDocument,
        "Save the current document".to_string(),
    );

    assert_eq!(
        shortcut.description,
        Some("Save the current document".to_string())
    );
}

#[test]
fn test_shortcut_matches() {
    let shortcut = Shortcut::new(
        ShortcutKey::Character('z'),
        ShortcutModifiers::new().with_ctrl(true),
        ShortcutAction::Undo,
    );

    let modifiers = ShortcutModifiers::new().with_ctrl(true);
    assert!(shortcut.matches(ShortcutKey::Character('z'), &modifiers));

    let wrong_modifiers = ShortcutModifiers::new().with_shift(true);
    assert!(!shortcut.matches(ShortcutKey::Character('z'), &wrong_modifiers));

    assert!(!shortcut.matches(ShortcutKey::Character('y'), &modifiers));
}

#[test]
fn test_shortcut_display_string() {
    let shortcut = Shortcut::new(
        ShortcutKey::Character('s'),
        ShortcutModifiers::new().with_ctrl(true).with_shift(true),
        ShortcutAction::SaveAsDocument,
    );

    let display = shortcut.display_string();
    assert!(display.contains("Ctrl"));
    assert!(display.contains("Shift"));
    assert!(display.contains("S"));
}

#[test]
fn test_shortcut_manager_creation() {
    let manager = ShortcutManager::new();
    assert!(manager.is_enabled());
    assert!(manager.count() > 0); // Should have default shortcuts
}

#[test]
fn test_shortcut_manager_empty() {
    let manager = ShortcutManager::empty();
    assert!(manager.is_enabled());
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_shortcut_registration() {
    let mut manager = ShortcutManager::empty();

    let shortcut = Shortcut::new(
        ShortcutKey::Function(1),
        ShortcutModifiers::new(),
        ShortcutAction::ShowAbout,
    );

    assert!(manager.register_shortcut(shortcut).is_ok());
    assert_eq!(manager.count(), 1);
}

#[test]
fn test_shortcut_conflict_detection() {
    let mut manager = ShortcutManager::empty();

    let shortcut1 = Shortcut::new(
        ShortcutKey::Character('n'),
        ShortcutModifiers::new().with_ctrl(true),
        ShortcutAction::NewDocument,
    );

    let shortcut2 = Shortcut::new(
        ShortcutKey::Character('n'),
        ShortcutModifiers::new().with_ctrl(true),
        ShortcutAction::OpenDocument, // Different action, same key combo
    );

    assert!(manager.register_shortcut(shortcut1).is_ok());
    assert!(manager.register_shortcut(shortcut2).is_err());
}

#[test]
fn test_shortcut_action_lookup() {
    let manager = ShortcutManager::new();

    let modifiers = ShortcutModifiers::new().with_ctrl(true);
    let action = manager.find_action(ShortcutKey::Character('n'), &modifiers);
    assert_eq!(action, Some(ShortcutAction::NewDocument));

    let no_action = manager.find_action(ShortcutKey::Character('x'), &ShortcutModifiers::new());
    assert!(no_action.is_none());
}

#[test]
fn test_shortcut_unregistration() {
    let mut manager = ShortcutManager::new();
    let initial_count = manager.count();

    assert!(manager.unregister_shortcut(&ShortcutAction::NewDocument));
    assert_eq!(manager.count(), initial_count - 1);

    assert!(!manager.unregister_shortcut(&ShortcutAction::NewDocument)); // Already removed
}

#[test]
fn test_shortcut_enable_disable() {
    let mut manager = ShortcutManager::new();

    manager.set_enabled(false);
    assert!(!manager.is_enabled());

    let modifiers = ShortcutModifiers::new().with_ctrl(true);
    let action = manager.find_action(ShortcutKey::Character('n'), &modifiers);
    assert!(action.is_none()); // Should be None when disabled

    manager.set_enabled(true);
    assert!(manager.is_enabled());

    let action = manager.find_action(ShortcutKey::Character('n'), &modifiers);
    assert_eq!(action, Some(ShortcutAction::NewDocument));
}

#[test]
fn test_shortcut_validation() {
    let mut manager = ShortcutManager::empty();

    // Add valid shortcuts
    let shortcut1 = Shortcut::new(
        ShortcutKey::Character('a'),
        ShortcutModifiers::new(),
        ShortcutAction::SelectAll,
    );
    let shortcut2 = Shortcut::new(
        ShortcutKey::Character('b'),
        ShortcutModifiers::new(),
        ShortcutAction::BrushTool,
    );

    manager.register_shortcut(shortcut1).unwrap();
    manager.register_shortcut(shortcut2).unwrap();

    assert!(manager.validate().is_ok());
}

#[test]
fn test_shortcut_categories() {
    let manager = ShortcutManager::new();
    let categories = manager.get_shortcuts_by_category();

    assert!(categories.contains_key("File"));
    assert!(categories.contains_key("Edit"));
    assert!(categories.contains_key("Tools"));
    assert!(categories.contains_key("View"));
}

#[test]
fn test_shortcut_modifiers() {
    let modifiers = ShortcutModifiers::new()
        .with_ctrl(true)
        .with_shift(true)
        .with_alt(false)
        .with_meta(false);

    assert!(modifiers.ctrl);
    assert!(modifiers.shift);
    assert!(!modifiers.alt);
    assert!(!modifiers.meta);
    assert!(modifiers.has_any());

    let empty_modifiers = ShortcutModifiers::new();
    assert!(!empty_modifiers.has_any());
}

#[test]
fn test_shortcut_key_types() {
    let char_key = ShortcutKey::Character('a');
    let func_key = ShortcutKey::Function(1);
    let special_key = ShortcutKey::Escape;

    assert_ne!(char_key, func_key);
    assert_ne!(func_key, special_key);
    assert_ne!(char_key, special_key);
}

#[test]
fn test_shortcut_action_display() {
    let action = ShortcutAction::NewDocument;
    let display = format!("{}", action);
    assert_eq!(display, "New Document");

    let custom_action = ShortcutAction::Custom("My Custom Action".to_string());
    let custom_display = format!("{}", custom_action);
    assert_eq!(custom_display, "My Custom Action");
}

#[test]
fn test_default_shortcuts_coverage() {
    let manager = ShortcutManager::new();

    // Test that common shortcuts are registered
    let ctrl = ShortcutModifiers::new().with_ctrl(true);

    assert_eq!(
        manager.find_action(ShortcutKey::Character('n'), &ctrl),
        Some(ShortcutAction::NewDocument)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('o'), &ctrl),
        Some(ShortcutAction::OpenDocument)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('s'), &ctrl),
        Some(ShortcutAction::SaveDocument)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('z'), &ctrl),
        Some(ShortcutAction::Undo)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('y'), &ctrl),
        Some(ShortcutAction::Redo)
    );

    // Test tool shortcuts (no modifiers)
    let no_mod = ShortcutModifiers::new();
    assert_eq!(
        manager.find_action(ShortcutKey::Character('v'), &no_mod),
        Some(ShortcutAction::SelectTool)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('b'), &no_mod),
        Some(ShortcutAction::BrushTool)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('e'), &no_mod),
        Some(ShortcutAction::EraserTool)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('m'), &no_mod),
        Some(ShortcutAction::MoveTool)
    );
    assert_eq!(
        manager.find_action(ShortcutKey::Character('t'), &no_mod),
        Some(ShortcutAction::TransformTool)
    );
}

#[test]
fn test_shortcut_clear() {
    let mut manager = ShortcutManager::new();
    let initial_count = manager.count();
    assert!(initial_count > 0);

    manager.clear();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_shortcut_has_shortcut() {
    let manager = ShortcutManager::new();

    let ctrl = ShortcutModifiers::new().with_ctrl(true);
    assert!(manager.has_shortcut(ShortcutKey::Character('n'), &ctrl));
    assert!(!manager.has_shortcut(ShortcutKey::Character('x'), &ShortcutModifiers::new()));
}
