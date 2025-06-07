//! Unit tests for P5.4 Rulers, Grid, and Guides functionality

use psoc::ui::application::{Message, ViewMessage};
use psoc::ui::canvas::ImageCanvas;

#[test]
fn test_canvas_rulers_toggle() {
    let mut canvas = ImageCanvas::new();

    // Initially rulers should be visible (default)
    assert!(canvas.state().show_rulers);

    // Toggle rulers off
    canvas.toggle_rulers();
    assert!(!canvas.state().show_rulers);

    // Toggle rulers back on
    canvas.toggle_rulers();
    assert!(canvas.state().show_rulers);
}

#[test]
fn test_canvas_rulers_set_visibility() {
    let mut canvas = ImageCanvas::new();

    // Set rulers visible
    canvas.set_rulers_visible(true);
    assert!(canvas.state().show_rulers);

    // Set rulers hidden
    canvas.set_rulers_visible(false);
    assert!(!canvas.state().show_rulers);
}

#[test]
fn test_canvas_grid_toggle() {
    let mut canvas = ImageCanvas::new();

    // Initially grid should be hidden (default)
    assert!(!canvas.state().show_grid);

    // Toggle grid on
    canvas.toggle_grid();
    assert!(canvas.state().show_grid);

    // Toggle grid back off
    canvas.toggle_grid();
    assert!(!canvas.state().show_grid);
}

#[test]
fn test_canvas_grid_set_visibility() {
    let mut canvas = ImageCanvas::new();

    // Set grid visible
    canvas.set_grid_visible(true);
    assert!(canvas.state().show_grid);

    // Set grid hidden
    canvas.set_grid_visible(false);
    assert!(!canvas.state().show_grid);
}

#[test]
fn test_canvas_grid_size() {
    let mut canvas = ImageCanvas::new();

    // Default grid size should be 20.0
    assert_eq!(canvas.state().grid_size, 20.0);

    // Set grid size
    canvas.set_grid_size(30.0);
    assert_eq!(canvas.state().grid_size, 30.0);

    // Test bounds checking - too small
    canvas.set_grid_size(1.0);
    assert_eq!(canvas.state().grid_size, 5.0); // Should be clamped to minimum

    // Test bounds checking - too large
    canvas.set_grid_size(200.0);
    assert_eq!(canvas.state().grid_size, 100.0); // Should be clamped to maximum
}

#[test]
fn test_canvas_guides_toggle() {
    let mut canvas = ImageCanvas::new();

    // Initially guides should be visible (default)
    assert!(canvas.state().show_guides);

    // Toggle guides off
    canvas.toggle_guides();
    assert!(!canvas.state().show_guides);

    // Toggle guides back on
    canvas.toggle_guides();
    assert!(canvas.state().show_guides);
}

#[test]
fn test_canvas_guides_set_visibility() {
    let mut canvas = ImageCanvas::new();

    // Set guides visible
    canvas.set_guides_visible(true);
    assert!(canvas.state().show_guides);

    // Set guides hidden
    canvas.set_guides_visible(false);
    assert!(!canvas.state().show_guides);
}

#[test]
fn test_canvas_horizontal_guides() {
    let mut canvas = ImageCanvas::new();

    // Initially no guides
    assert_eq!(canvas.state().horizontal_guides.len(), 0);

    // Add horizontal guides
    canvas.add_horizontal_guide(100.0);
    canvas.add_horizontal_guide(200.0);
    assert_eq!(canvas.state().horizontal_guides.len(), 2);
    assert_eq!(canvas.state().horizontal_guides[0], 100.0);
    assert_eq!(canvas.state().horizontal_guides[1], 200.0);

    // Remove horizontal guide
    canvas.remove_horizontal_guide(0);
    assert_eq!(canvas.state().horizontal_guides.len(), 1);
    assert_eq!(canvas.state().horizontal_guides[0], 200.0);

    // Clear all guides
    canvas.clear_guides();
    assert_eq!(canvas.state().horizontal_guides.len(), 0);
}

#[test]
fn test_canvas_vertical_guides() {
    let mut canvas = ImageCanvas::new();

    // Initially no guides
    assert_eq!(canvas.state().vertical_guides.len(), 0);

    // Add vertical guides
    canvas.add_vertical_guide(150.0);
    canvas.add_vertical_guide(300.0);
    assert_eq!(canvas.state().vertical_guides.len(), 2);
    assert_eq!(canvas.state().vertical_guides[0], 150.0);
    assert_eq!(canvas.state().vertical_guides[1], 300.0);

    // Remove vertical guide
    canvas.remove_vertical_guide(1);
    assert_eq!(canvas.state().vertical_guides.len(), 1);
    assert_eq!(canvas.state().vertical_guides[0], 150.0);

    // Clear all guides
    canvas.clear_guides();
    assert_eq!(canvas.state().vertical_guides.len(), 0);
}

#[test]
fn test_canvas_guides_bounds_checking() {
    let mut canvas = ImageCanvas::new();

    // Add some guides
    canvas.add_horizontal_guide(100.0);
    canvas.add_vertical_guide(200.0);

    // Try to remove guide with invalid index
    let initial_h_count = canvas.state().horizontal_guides.len();
    let initial_v_count = canvas.state().vertical_guides.len();

    canvas.remove_horizontal_guide(999); // Invalid index
    canvas.remove_vertical_guide(999); // Invalid index

    // Should not have changed
    assert_eq!(canvas.state().horizontal_guides.len(), initial_h_count);
    assert_eq!(canvas.state().vertical_guides.len(), initial_v_count);
}

#[test]
fn test_view_message_types() {
    // Test that ViewMessage variants can be created
    let _toggle_rulers = ViewMessage::ToggleRulers;
    let _set_rulers = ViewMessage::SetRulersVisible(true);
    let _toggle_grid = ViewMessage::ToggleGrid;
    let _set_grid = ViewMessage::SetGridVisible(false);
    let _set_grid_size = ViewMessage::SetGridSize(25.0);
    let _toggle_guides = ViewMessage::ToggleGuides;
    let _set_guides = ViewMessage::SetGuidesVisible(true);
    let _add_h_guide = ViewMessage::AddHorizontalGuide(100.0);
    let _add_v_guide = ViewMessage::AddVerticalGuide(150.0);
    let _remove_h_guide = ViewMessage::RemoveHorizontalGuide(0);
    let _remove_v_guide = ViewMessage::RemoveVerticalGuide(1);
    let _clear_guides = ViewMessage::ClearGuides;

    // Test that Message::View can be created
    let _view_msg = Message::View(ViewMessage::ToggleRulers);
}

#[test]
fn test_canvas_default_state() {
    let canvas = ImageCanvas::new();
    let state = canvas.state();

    // Test default values
    assert!(state.show_rulers); // Rulers visible by default
    assert!(!state.show_grid); // Grid hidden by default
    assert!(state.show_guides); // Guides visible by default
    assert_eq!(state.grid_size, 20.0); // Default grid size
    assert_eq!(state.ruler_size, 20.0); // Default ruler size
    assert_eq!(state.horizontal_guides.len(), 0); // No guides initially
    assert_eq!(state.vertical_guides.len(), 0); // No guides initially
}

#[test]
fn test_canvas_state_consistency() {
    let mut canvas = ImageCanvas::new();

    // Test that multiple operations maintain consistency
    canvas.set_grid_visible(true);
    canvas.set_grid_size(40.0);
    canvas.add_horizontal_guide(50.0);
    canvas.add_vertical_guide(75.0);
    canvas.set_rulers_visible(false);
    canvas.set_guides_visible(false);

    let state = canvas.state();
    assert!(!state.show_rulers);
    assert!(state.show_grid);
    assert!(!state.show_guides);
    assert_eq!(state.grid_size, 40.0);
    assert_eq!(state.horizontal_guides.len(), 1);
    assert_eq!(state.vertical_guides.len(), 1);
    assert_eq!(state.horizontal_guides[0], 50.0);
    assert_eq!(state.vertical_guides[0], 75.0);
}
