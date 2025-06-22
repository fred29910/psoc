//! Basic compilation test for Phase 3 components

use psoc::ui::components::{
    KeyboardNavigationManager, ResponsiveLayoutManager, FocusTarget, NavigationAction,
    PanelId, ScreenSize,
};
use iced::{keyboard, Size};

#[test]
fn test_phase3_components_compile() {
    // Test KeyboardNavigationManager
    let mut nav_manager = KeyboardNavigationManager::new();
    assert!(nav_manager.enabled);
    
    // Test ResponsiveLayoutManager
    let mut layout_manager = ResponsiveLayoutManager::new();
    assert_eq!(layout_manager.screen_size, ScreenSize::Large);
    
    // Test basic operations
    let _target = nav_manager.tab_order.next();
    layout_manager.update_window_size(Size::new(800.0, 600.0));
    
    println!("Phase 3 components compile and work correctly!");
}

#[test]
fn test_keyboard_navigation_basic() {
    let mut nav_manager = KeyboardNavigationManager::new();
    
    // Test tab navigation
    let first = nav_manager.tab_order.next();
    assert_eq!(first, Some(FocusTarget::MenuBar));
    
    let second = nav_manager.tab_order.next();
    assert_eq!(second, Some(FocusTarget::ToolPanel));
}

#[test]
fn test_responsive_layout_basic() {
    let mut layout_manager = ResponsiveLayoutManager::new();
    
    // Test screen size detection
    let small_size = Size::new(600.0, 400.0);
    layout_manager.update_window_size(small_size);
    assert_eq!(layout_manager.screen_size, ScreenSize::Small);
    
    // Test panel operations
    layout_manager.toggle_panel(PanelId::ToolPanel);
    assert!(!layout_manager.is_panel_expanded(PanelId::ToolPanel));
}
