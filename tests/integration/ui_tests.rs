//! Integration tests for UI components

use psoc::ui::{AppState, Message, PsocTheme, Tool};

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
}

#[cfg(test)]
mod canvas_tests {
    use psoc::ui::application::CanvasMessage;

    #[test]
    fn test_canvas_message_creation() {
        let _mouse_moved = CanvasMessage::MouseMoved { x: 10.0, y: 20.0 };
        let _mouse_pressed = CanvasMessage::MousePressed { x: 15.0, y: 25.0 };
        let _mouse_released = CanvasMessage::MouseReleased { x: 20.0, y: 30.0 };
        let _scrolled = CanvasMessage::Scrolled {
            delta_x: 5.0,
            delta_y: -3.0,
        };
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
