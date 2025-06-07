use psoc::{
    tools::{
        tool_trait::{KeyModifiers, MouseButton},
        ToolEvent, ToolManager, ToolType,
    },
    ui::application::AppState,
};
use psoc_core::{Document, Layer, Point, RgbaPixel};

#[test]
fn test_brush_tool_mask_editing_mode() {
    let mut tool_manager = ToolManager::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a layer with a mask
    let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
    layer.create_mask(100, 100).unwrap();
    document.add_layer(layer);

    // Set brush tool as active
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Create a mouse press event
    let event = ToolEvent::MousePressed {
        position: Point::new(50.0, 50.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Handle event in mask editing mode (should not crash)
    let result = tool_manager.handle_event_with_mask_mode(event, &mut document, true, None);
    assert!(result.is_ok());

    // Verify the layer still has a mask
    let layer = document.get_layer(0).unwrap();
    assert!(layer.has_mask());
}

#[test]
fn test_eraser_tool_mask_editing_mode() {
    let mut tool_manager = ToolManager::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a layer with a mask
    let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
    layer.create_mask(100, 100).unwrap();

    // Set initial mask to gray
    for y in 0..100 {
        for x in 0..100 {
            layer
                .set_mask_pixel(x, y, RgbaPixel::new(128, 128, 128, 255))
                .unwrap();
        }
    }

    document.add_layer(layer);

    // Set eraser tool as active
    tool_manager.set_active_tool(ToolType::Eraser).unwrap();

    // Create a mouse press event
    let event = ToolEvent::MousePressed {
        position: Point::new(50.0, 50.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Handle event in mask editing mode (should not crash)
    let result = tool_manager.handle_event_with_mask_mode(event, &mut document, true, None);
    assert!(result.is_ok());

    // Verify the layer still has a mask
    let layer = document.get_layer(0).unwrap();
    assert!(layer.has_mask());
}

#[test]
fn test_mask_editing_without_mask() {
    let mut tool_manager = ToolManager::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a layer WITHOUT a mask
    let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
    document.add_layer(layer);

    // Set brush tool as active
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Create a mouse press event
    let event = ToolEvent::MousePressed {
        position: Point::new(50.0, 50.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Handle event in mask editing mode (should not crash)
    let result = tool_manager.handle_event_with_mask_mode(event, &mut document, true, None);
    assert!(result.is_ok());

    // Verify no changes were made since there's no mask
    let layer = document.get_layer(0).unwrap();
    assert!(!layer.has_mask());
}

#[test]
fn test_normal_tool_operation_in_mask_mode() {
    let mut tool_manager = ToolManager::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a layer
    let layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
    document.add_layer(layer);

    // Set select tool as active (non-mask-aware tool)
    tool_manager.set_active_tool(ToolType::Select).unwrap();

    // Create a mouse press event
    let event = ToolEvent::MousePressed {
        position: Point::new(50.0, 50.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Handle event in mask editing mode (should work normally for non-mask-aware tools)
    let result = tool_manager.handle_event_with_mask_mode(event, &mut document, true, None);
    assert!(result.is_ok());
}

#[test]
fn test_brush_stroke_in_mask_mode() {
    let mut tool_manager = ToolManager::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a layer with a mask
    let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
    layer.create_mask(100, 100).unwrap();
    document.add_layer(layer);

    // Set brush tool as active
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    // Create a mouse press event
    let press_event = ToolEvent::MousePressed {
        position: Point::new(40.0, 40.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Create a mouse drag event
    let drag_event = ToolEvent::MouseDragged {
        position: Point::new(60.0, 60.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Create a mouse release event
    let release_event = ToolEvent::MouseReleased {
        position: Point::new(60.0, 60.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Handle stroke in mask editing mode (should not crash)
    tool_manager
        .handle_event_with_mask_mode(press_event, &mut document, true, None)
        .unwrap();
    tool_manager
        .handle_event_with_mask_mode(drag_event, &mut document, true, None)
        .unwrap();
    tool_manager
        .handle_event_with_mask_mode(release_event, &mut document, true, None)
        .unwrap();

    // Verify the layer still has a mask
    let layer = document.get_layer(0).unwrap();
    assert!(layer.has_mask());
}

#[test]
fn test_app_state_mask_editing_fields() {
    let app_state = AppState::default();

    // Test default values
    assert!(!app_state.mask_editing_mode);
    assert!(app_state.mask_editing_layer.is_none());
}

#[test]
fn test_mask_editing_mode_toggle() {
    let mut app_state = AppState::default();

    // Initially not in mask editing mode
    assert!(!app_state.mask_editing_mode);

    // Toggle mask editing mode
    app_state.mask_editing_mode = true;
    app_state.mask_editing_layer = Some(0);

    assert!(app_state.mask_editing_mode);
    assert_eq!(app_state.mask_editing_layer, Some(0));

    // Toggle back
    app_state.mask_editing_mode = false;
    app_state.mask_editing_layer = None;

    assert!(!app_state.mask_editing_mode);
    assert!(app_state.mask_editing_layer.is_none());
}

#[test]
fn test_tool_manager_mask_aware_event_handling() {
    let mut tool_manager = ToolManager::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a layer with mask
    let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 100);
    layer.create_mask(100, 100).unwrap();
    document.add_layer(layer);

    // Test with brush tool
    tool_manager.set_active_tool(ToolType::Brush).unwrap();

    let event = ToolEvent::MousePressed {
        position: Point::new(25.0, 25.0),
        button: MouseButton::Left,
        modifiers: KeyModifiers::default(),
    };

    // Test normal mode
    let result = tool_manager.handle_event(event.clone(), &mut document);
    assert!(result.is_ok());

    // Test mask editing mode
    let result = tool_manager.handle_event_with_mask_mode(event, &mut document, true, Some(0));
    assert!(result.is_ok());
}
