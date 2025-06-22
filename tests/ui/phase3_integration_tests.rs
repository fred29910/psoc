//! Integration tests for Phase 3: Interactive Experience Optimization
//! Tests keyboard navigation, responsive layout, and enhanced user interaction features

use psoc::ui::components::{
    KeyboardNavigationManager, ResponsiveLayoutManager, FocusTarget, NavigationAction,
    PanelId, ScreenSize,
};
use psoc::shortcuts::{ShortcutKey, ShortcutModifiers};
use iced::{keyboard, Size};

#[test]
fn test_keyboard_navigation_manager_creation() {
    let nav_manager = KeyboardNavigationManager::new();
    
    assert!(nav_manager.enabled);
    assert!(nav_manager.show_focus_indicators);
    assert!(!nav_manager.alt_pressed);
    assert!(nav_manager.tab_order.current().is_none());
}

#[test]
fn test_tab_navigation_order() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    // Test forward navigation
    let first_target = nav_manager.tab_order.next();
    assert_eq!(first_target, Some(FocusTarget::MenuBar));
    
    let second_target = nav_manager.tab_order.next();
    assert_eq!(second_target, Some(FocusTarget::ToolPanel));
    
    let third_target = nav_manager.tab_order.next();
    assert_eq!(third_target, Some(FocusTarget::Canvas));
    
    // Test backward navigation
    let prev_target = nav_manager.tab_order.previous();
    assert_eq!(prev_target, Some(FocusTarget::ToolPanel));
}

#[test]
fn test_keyboard_navigation_key_handling() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    // Test Tab key
    let tab_key = keyboard::Key::Named(keyboard::key::Named::Tab);
    let no_modifiers = keyboard::Modifiers::default();
    
    let action = nav_manager.handle_key_press(tab_key, no_modifiers);
    assert_eq!(action, Some(NavigationAction::FocusNext));
    
    // Test Shift+Tab
    let shift_modifiers = keyboard::Modifiers::SHIFT;
    let action = nav_manager.handle_key_press(tab_key, shift_modifiers);
    assert_eq!(action, Some(NavigationAction::FocusPrevious));
    
    // Test Enter key
    let enter_key = keyboard::Key::Named(keyboard::key::Named::Enter);
    let action = nav_manager.handle_key_press(enter_key, no_modifiers);
    assert_eq!(action, Some(NavigationAction::Activate));
    
    // Test Escape key
    let escape_key = keyboard::Key::Named(keyboard::key::Named::Escape);
    let action = nav_manager.handle_key_press(escape_key, no_modifiers);
    assert_eq!(action, Some(NavigationAction::Cancel));
}

#[test]
fn test_focus_target_management() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    // Test focusing specific target
    let success = nav_manager.tab_order.focus(FocusTarget::Canvas);
    assert!(success);
    assert_eq!(nav_manager.tab_order.current(), Some(FocusTarget::Canvas));
    
    // Test clearing focus
    nav_manager.tab_order.clear();
    assert!(nav_manager.tab_order.current().is_none());
}

#[test]
fn test_responsive_layout_manager_creation() {
    let layout_manager = ResponsiveLayoutManager::new();
    
    assert_eq!(layout_manager.screen_size, ScreenSize::Large);
    assert!(!layout_manager.compact_mode);
    assert_eq!(layout_manager.window_size, Size::new(1200.0, 800.0));
    
    // Test panel states
    assert!(layout_manager.is_panel_expanded(PanelId::ToolPanel));
    assert!(layout_manager.is_panel_expanded(PanelId::PropertiesPanel));
}

#[test]
fn test_screen_size_detection() {
    // Test small screen
    let small_size = Size::new(600.0, 400.0);
    assert_eq!(ScreenSize::from_window_size(small_size), ScreenSize::Small);
    
    // Test medium screen
    let medium_size = Size::new(900.0, 600.0);
    assert_eq!(ScreenSize::from_window_size(medium_size), ScreenSize::Medium);
    
    // Test large screen
    let large_size = Size::new(1200.0, 800.0);
    assert_eq!(ScreenSize::from_window_size(large_size), ScreenSize::Large);
    
    // Test extra large screen
    let xl_size = Size::new(1600.0, 1000.0);
    assert_eq!(ScreenSize::from_window_size(xl_size), ScreenSize::ExtraLarge);
}

#[test]
fn test_responsive_layout_window_resize() {
    let mut layout_manager = ResponsiveLayoutManager::new();
    
    // Start with large screen
    assert_eq!(layout_manager.screen_size, ScreenSize::Large);
    assert!(!layout_manager.compact_mode);
    
    // Resize to small screen
    let small_size = Size::new(600.0, 400.0);
    layout_manager.update_window_size(small_size);
    
    assert_eq!(layout_manager.screen_size, ScreenSize::Small);
    assert!(layout_manager.compact_mode);
    
    // Check that panels are minimized/hidden
    assert!(!layout_manager.is_panel_expanded(PanelId::LayersPanel));
    assert!(!layout_manager.is_panel_expanded(PanelId::HistoryPanel));
}

#[test]
fn test_panel_width_recommendations() {
    assert_eq!(ScreenSize::Small.panel_width(), 200.0);
    assert_eq!(ScreenSize::Medium.panel_width(), 250.0);
    assert_eq!(ScreenSize::Large.panel_width(), 280.0);
    assert_eq!(ScreenSize::ExtraLarge.panel_width(), 320.0);
}

#[test]
fn test_panel_collapse_behavior() {
    assert!(ScreenSize::Small.should_collapse_panels());
    assert!(ScreenSize::Medium.should_collapse_panels());
    assert!(!ScreenSize::Large.should_collapse_panels());
    assert!(!ScreenSize::ExtraLarge.should_collapse_panels());
}

#[test]
fn test_panel_toggle_operations() {
    let mut layout_manager = ResponsiveLayoutManager::new();
    
    // Test panel visibility toggle
    assert!(layout_manager.is_panel_expanded(PanelId::ToolPanel));
    layout_manager.toggle_panel(PanelId::ToolPanel);
    assert!(!layout_manager.is_panel_expanded(PanelId::ToolPanel));
    
    // Test panel minimized toggle
    layout_manager.toggle_panel(PanelId::ToolPanel); // Make visible again
    assert!(layout_manager.is_panel_expanded(PanelId::ToolPanel));
    
    layout_manager.toggle_panel_minimized(PanelId::ToolPanel);
    assert!(!layout_manager.is_panel_expanded(PanelId::ToolPanel));
}

#[test]
fn test_panel_resizing() {
    let mut layout_manager = ResponsiveLayoutManager::new();
    
    let original_width = layout_manager.get_effective_panel_width(PanelId::ToolPanel);
    assert!(original_width > 0.0);
    
    // Test resizing within bounds
    layout_manager.resize_panel(PanelId::ToolPanel, 300.0);
    let new_width = layout_manager.get_effective_panel_width(PanelId::ToolPanel);
    assert_eq!(new_width, 300.0);
    
    // Test resizing beyond max bounds (should clamp)
    layout_manager.resize_panel(PanelId::ToolPanel, 500.0);
    let clamped_width = layout_manager.get_effective_panel_width(PanelId::ToolPanel);
    assert!(clamped_width <= 300.0); // Should be clamped to max_width
    
    // Test resizing below min bounds (should clamp)
    layout_manager.resize_panel(PanelId::ToolPanel, 50.0);
    let min_clamped_width = layout_manager.get_effective_panel_width(PanelId::ToolPanel);
    assert!(min_clamped_width >= 150.0); // Should be clamped to min_width
}

#[test]
fn test_canvas_width_calculation() {
    let mut layout_manager = ResponsiveLayoutManager::new();
    layout_manager.update_window_size(Size::new(1200.0, 800.0));
    
    let canvas_width = layout_manager.get_canvas_width();
    assert!(canvas_width > 300.0); // Should have minimum width
    
    // Hide panels and check canvas width increases
    layout_manager.toggle_panel(PanelId::ToolPanel);
    layout_manager.toggle_panel(PanelId::PropertiesPanel);
    
    let expanded_canvas_width = layout_manager.get_canvas_width();
    assert!(expanded_canvas_width > canvas_width);
}

#[test]
fn test_compact_mode_transitions() {
    let mut layout_manager = ResponsiveLayoutManager::new();
    
    // Start in normal mode
    assert!(!layout_manager.compact_mode);
    assert!(layout_manager.is_panel_expanded(PanelId::LayersPanel));
    
    // Enter compact mode
    layout_manager.enter_compact_mode();
    assert!(layout_manager.compact_mode);
    
    // Exit compact mode
    layout_manager.exit_compact_mode();
    assert!(!layout_manager.compact_mode);
    assert!(layout_manager.is_panel_expanded(PanelId::LayersPanel));
}

#[test]
fn test_keyboard_navigation_disabled_state() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    // Disable navigation
    nav_manager.set_enabled(false);
    assert!(!nav_manager.enabled);
    
    // Test that key handling returns None when disabled
    let tab_key = keyboard::Key::Named(keyboard::key::Named::Tab);
    let no_modifiers = keyboard::Modifiers::default();
    
    let action = nav_manager.handle_key_press(tab_key, no_modifiers);
    assert!(action.is_none());
}

#[test]
fn test_custom_key_bindings() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    // Add custom key binding
    let custom_key = ShortcutKey::Character('h');
    let custom_modifiers = ShortcutModifiers::new().with_ctrl(true);
    let custom_action = NavigationAction::FocusTarget(FocusTarget::HistoryPanel);
    
    nav_manager.add_key_binding(custom_key, custom_modifiers, custom_action);
    
    // Test custom key binding
    let ctrl_h_key = keyboard::Key::Character("h".into());
    let ctrl_modifiers = keyboard::Modifiers::COMMAND;
    
    let action = nav_manager.handle_key_press(ctrl_h_key, ctrl_modifiers);
    assert_eq!(action, Some(custom_action));
    
    // Remove custom key binding
    nav_manager.remove_key_binding(custom_key, custom_modifiers);
    
    let action_after_removal = nav_manager.handle_key_press(ctrl_h_key, ctrl_modifiers);
    assert!(action_after_removal.is_none());
}

#[test]
fn test_focus_indicators_toggle() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    assert!(nav_manager.show_focus_indicators);
    
    nav_manager.set_show_focus_indicators(false);
    assert!(!nav_manager.show_focus_indicators);
    
    nav_manager.set_show_focus_indicators(true);
    assert!(nav_manager.show_focus_indicators);
}
