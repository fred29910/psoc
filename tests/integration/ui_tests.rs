//! Integration tests for UI components

use psoc::ui::{AppState, LayerMessage, Message, PsocTheme, Tool};

#[test]
fn test_app_state_creation() {
    let state = AppState::default();
    assert_eq!(state.current_tool, Tool::Select);
    assert_eq!(state.zoom_level, 1.0);
    assert_eq!(state.pan_offset, (0.0, 0.0));
    assert!(!state.document_open);
    assert!(matches!(state.theme, PsocTheme::Dark));
}

#[test]
fn test_app_state_tool_changes() {
    let mut state = AppState::default();

    state.current_tool = Tool::Brush;
    assert_eq!(state.current_tool, Tool::Brush);

    state.current_tool = Tool::Eraser;
    assert_eq!(state.current_tool, Tool::Eraser);

    state.current_tool = Tool::Move;
    assert_eq!(state.current_tool, Tool::Move);
}

#[test]
fn test_app_state_zoom_operations() {
    let mut state = AppState::default();

    // Test zoom in
    state.zoom_level = 1.0;
    let new_zoom = (state.zoom_level * 1.2).min(10.0);
    state.zoom_level = new_zoom;
    assert!((state.zoom_level - 1.2).abs() < f32::EPSILON);

    // Test zoom out
    state.zoom_level = 1.0;
    let new_zoom = (state.zoom_level / 1.2).max(0.1);
    state.zoom_level = new_zoom;
    assert!((state.zoom_level - 0.8333333).abs() < 0.001);

    // Test zoom limits
    state.zoom_level = 15.0;
    let new_zoom = (state.zoom_level * 1.2).min(10.0);
    state.zoom_level = new_zoom;
    assert_eq!(state.zoom_level, 10.0);

    state.zoom_level = 0.05;
    let new_zoom = (state.zoom_level / 1.2).max(0.1);
    state.zoom_level = new_zoom;
    assert_eq!(state.zoom_level, 0.1);
}

#[test]
fn test_app_state_pan_operations() {
    let mut state = AppState::default();

    // Test panning
    state.pan_offset = (0.0, 0.0);
    state.pan_offset.0 += 10.0;
    state.pan_offset.1 += 5.0;
    assert_eq!(state.pan_offset, (10.0, 5.0));

    // Test negative panning
    state.pan_offset.0 -= 20.0;
    state.pan_offset.1 -= 10.0;
    assert_eq!(state.pan_offset, (-10.0, -5.0));
}

#[test]
fn test_app_state_document_operations() {
    let mut state = AppState::default();

    // Initially no document
    assert!(!state.document_open);

    // Open document
    state.document_open = true;
    assert!(state.document_open);

    // Reset state
    state.document_open = false;
    state.zoom_level = 1.0;
    state.pan_offset = (0.0, 0.0);
    assert!(!state.document_open);
    assert_eq!(state.zoom_level, 1.0);
    assert_eq!(state.pan_offset, (0.0, 0.0));
}

#[test]
fn test_tool_display() {
    assert_eq!(Tool::Select.to_string(), "Select");
    assert_eq!(Tool::Brush.to_string(), "Brush");
    assert_eq!(Tool::Eraser.to_string(), "Eraser");
    assert_eq!(Tool::Move.to_string(), "Move");
}

#[test]
fn test_theme_variants() {
    let dark = PsocTheme::Dark;
    let light = PsocTheme::Light;
    let high_contrast = PsocTheme::HighContrast;

    // Test that themes are different
    assert_ne!(format!("{:?}", dark), format!("{:?}", light));
    assert_ne!(format!("{:?}", dark), format!("{:?}", high_contrast));
    assert_ne!(format!("{:?}", light), format!("{:?}", high_contrast));
}

#[test]
fn test_message_variants() {
    // Test that different message types can be created
    let _new_doc = Message::NewDocument;
    let _open_doc = Message::OpenDocument;
    let _save_doc = Message::SaveDocument;
    let _tool_change = Message::ToolChanged(Tool::Brush);
    let _zoom_in = Message::ZoomIn;
    let _zoom_out = Message::ZoomOut;
    let _error = Message::Error("Test error".to_string());

    // Test layer messages
    let _add_layer = Message::Layer(LayerMessage::AddEmptyLayer);
    let _delete_layer = Message::Layer(LayerMessage::DeleteLayer(0));
    let _select_layer = Message::Layer(LayerMessage::SelectLayer(1));
    let _toggle_visibility = Message::Layer(LayerMessage::ToggleLayerVisibility(0));
    let _change_opacity = Message::Layer(LayerMessage::ChangeLayerOpacity(0, 0.5));
}

#[cfg(test)]
mod canvas_tests {
    use psoc::ui::application::CanvasMessage;
    use psoc::ImageCanvas;

    #[test]
    fn test_canvas_message_creation() {
        let mouse_moved = CanvasMessage::MouseMoved { x: 10.0, y: 20.0 };
        let mouse_pressed = CanvasMessage::MousePressed { x: 15.0, y: 25.0 };
        let mouse_released = CanvasMessage::MouseReleased { x: 20.0, y: 30.0 };
        let scrolled = CanvasMessage::Scrolled {
            delta_x: 5.0,
            delta_y: -3.0,
        };

        // Test that messages contain expected data
        match mouse_moved {
            CanvasMessage::MouseMoved { x, y } => {
                assert_eq!(x, 10.0);
                assert_eq!(y, 20.0);
            }
            _ => panic!("Expected MouseMoved message"),
        }

        match scrolled {
            CanvasMessage::Scrolled { delta_x, delta_y } => {
                assert_eq!(delta_x, 5.0);
                assert_eq!(delta_y, -3.0);
            }
            _ => panic!("Expected Scrolled message"),
        }
    }

    #[test]
    fn test_canvas_creation() {
        let canvas = ImageCanvas::new();

        // Test initial state
        assert_eq!(canvas.zoom(), 1.0);
        assert_eq!(canvas.pan_offset().x, 0.0);
        assert_eq!(canvas.pan_offset().y, 0.0);
    }

    #[test]
    fn test_canvas_zoom_operations() {
        let mut canvas = ImageCanvas::new();

        // Test zoom in
        canvas.set_zoom(2.0);
        assert_eq!(canvas.zoom(), 2.0);

        // Test zoom limits
        canvas.set_zoom(15.0); // Should be clamped to 10.0
        assert_eq!(canvas.zoom(), 10.0);

        canvas.set_zoom(0.05); // Should be clamped to 0.1
        assert_eq!(canvas.zoom(), 0.1);
    }

    #[test]
    fn test_canvas_pan_operations() {
        let mut canvas = ImageCanvas::new();

        // Test pan offset
        let offset = iced::Vector::new(50.0, -25.0);
        canvas.set_pan_offset(offset);

        assert_eq!(canvas.pan_offset().x, 50.0);
        assert_eq!(canvas.pan_offset().y, -25.0);
    }
}

#[cfg(test)]
mod icon_tests {
    use psoc::ui::icons::Icon;

    #[test]
    fn test_icon_unicode_mapping() {
        assert_eq!(Icon::New.unicode(), '+');
        assert_eq!(Icon::Open.unicode(), 'O');
        assert_eq!(Icon::Save.unicode(), 'S');
        assert_eq!(Icon::Close.unicode(), '×');
        assert_eq!(Icon::Info.unicode(), 'i');
    }

    #[test]
    fn test_icon_string_mapping() {
        assert_eq!(Icon::New.as_str(), "New");
        assert_eq!(Icon::Open.as_str(), "Open");
        assert_eq!(Icon::Save.as_str(), "Save");
        assert_eq!(Icon::Close.as_str(), "Close");
        assert_eq!(Icon::Info.as_str(), "Information");
    }
}

#[cfg(test)]
mod layer_panel_tests {
    use psoc::ui::{AppState, LayerMessage};
    use psoc_core::{Document, Layer};

    #[test]
    fn test_layer_message_creation() {
        // Test that layer messages can be created with different parameters
        let add_empty = LayerMessage::AddEmptyLayer;
        let delete_layer = LayerMessage::DeleteLayer(0);
        let duplicate_layer = LayerMessage::DuplicateLayer(1);
        let select_layer = LayerMessage::SelectLayer(2);
        let toggle_visibility = LayerMessage::ToggleLayerVisibility(0);
        let change_opacity = LayerMessage::ChangeLayerOpacity(1, 0.75);
        let move_up = LayerMessage::MoveLayerUp(1);
        let move_down = LayerMessage::MoveLayerDown(0);
        let rename = LayerMessage::RenameLayer(0, "New Name".to_string());

        // Test that messages contain expected data
        match delete_layer {
            LayerMessage::DeleteLayer(index) => assert_eq!(index, 0),
            _ => panic!("Expected DeleteLayer message"),
        }

        match change_opacity {
            LayerMessage::ChangeLayerOpacity(index, opacity) => {
                assert_eq!(index, 1);
                assert!((opacity - 0.75).abs() < f32::EPSILON);
            }
            _ => panic!("Expected ChangeLayerOpacity message"),
        }

        match rename {
            LayerMessage::RenameLayer(index, name) => {
                assert_eq!(index, 0);
                assert_eq!(name, "New Name");
            }
            _ => panic!("Expected RenameLayer message"),
        }
    }

    #[test]
    fn test_app_state_with_document() {
        let mut state = AppState::default();

        // Initially no document
        assert!(state.current_document.is_none());
        assert!(!state.document_open);

        // Create a document
        let document = Document::new("Test Document".to_string(), 800, 600);
        state.current_document = Some(document);
        state.document_open = true;

        // Verify document is set
        assert!(state.current_document.is_some());
        assert!(state.document_open);

        let doc = state.current_document.as_ref().unwrap();
        assert_eq!(doc.metadata.title, "Test Document");
        assert_eq!(doc.dimensions(), (800, 600));
    }

    #[test]
    fn test_document_layer_operations() {
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add some layers
        let layer1 = Layer::new_pixel("Layer 1".to_string(), 100, 100);
        let layer2 = Layer::new_pixel("Layer 2".to_string(), 100, 100);

        document.add_layer(layer1);
        document.add_layer(layer2);

        assert_eq!(document.layer_count(), 2);

        // Test layer selection
        document.set_active_layer(1).unwrap();
        assert_eq!(document.active_layer_index, Some(1));

        // Test layer visibility
        document.layers[0].visible = false;
        assert!(!document.layers[0].visible);
        assert!(document.layers[1].visible);

        // Test layer opacity
        document.layers[1].opacity = 0.5;
        assert!((document.layers[1].opacity - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_layer_ordering_operations() {
        let mut document = Document::new("Test".to_string(), 100, 100);

        // Add layers with different names
        let layer1 = Layer::new_pixel("Bottom".to_string(), 100, 100);
        let layer2 = Layer::new_pixel("Middle".to_string(), 100, 100);
        let layer3 = Layer::new_pixel("Top".to_string(), 100, 100);

        document.add_layer(layer1);
        document.add_layer(layer2);
        document.add_layer(layer3);

        // Initial order: Bottom(0), Middle(1), Top(2)
        assert_eq!(document.layers[0].name, "Bottom");
        assert_eq!(document.layers[1].name, "Middle");
        assert_eq!(document.layers[2].name, "Top");

        // Swap middle and top (simulate move up)
        document.layers.swap(1, 2);

        // New order: Bottom(0), Top(1), Middle(2)
        assert_eq!(document.layers[0].name, "Bottom");
        assert_eq!(document.layers[1].name, "Top");
        assert_eq!(document.layers[2].name, "Middle");
    }
}
