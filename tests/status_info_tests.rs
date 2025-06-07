//! Tests for status information and enhanced status bar functionality

use psoc::ui::application::{AppState, StatusInfo};
use psoc_core::{Document, RgbaPixel};

#[test]
fn test_status_info_creation_empty_state() {
    let app_state = AppState::default();
    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.document_status, "No document");
    assert!(status_info.image_size.is_none());
    assert!(status_info.color_mode.is_none());
    assert_eq!(status_info.zoom_level, 1.0);
    assert!(status_info.mouse_position.is_none());
    assert!(status_info.pixel_color.is_none());
}

#[test]
fn test_status_info_with_document() {
    let mut app_state = AppState::default();
    let document = Document::new("Test Document".to_string(), 800, 600);

    app_state.current_document = Some(document);
    app_state.document_open = true;
    app_state.zoom_level = 1.5;

    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.document_status, "Unsaved");
    assert_eq!(status_info.image_size, Some((800, 600)));
    assert!(status_info.color_mode.is_some());
    assert_eq!(status_info.zoom_level, 1.5);
}

#[test]
fn test_status_info_with_saved_document() {
    let mut app_state = AppState::default();
    let document = Document::new("Test Document".to_string(), 1024, 768);

    app_state.current_document = Some(document);
    app_state.document_open = true;
    app_state.current_file_path = Some(std::path::PathBuf::from("test.psoc"));

    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.document_status, "Saved");
    assert_eq!(status_info.image_size, Some((1024, 768)));
}

#[test]
fn test_status_info_with_mouse_position() {
    let mut app_state = AppState::default();
    app_state.mouse_position = Some((150.5, 200.3));

    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.mouse_position, Some((150.5, 200.3)));
}

#[test]
fn test_status_info_with_pixel_color() {
    let mut app_state = AppState::default();
    let pixel_color = RgbaPixel::new(255, 128, 64, 200);
    app_state.current_pixel_color = Some(pixel_color);

    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.pixel_color, Some(pixel_color));
}

#[test]
fn test_status_info_with_image() {
    let mut app_state = AppState::default();

    // Create a simple test image
    let test_image = image::DynamicImage::new_rgb8(640, 480);
    app_state.current_image = Some(test_image);
    app_state.document_open = true;

    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.image_size, Some((640, 480)));
    assert!(status_info.color_mode.is_some());
    assert!(status_info.color_mode.unwrap().contains("RGB"));
}

#[test]
fn test_status_info_zoom_levels() {
    let mut app_state = AppState::default();

    // Test different zoom levels
    let zoom_levels = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0];

    for zoom in zoom_levels.iter() {
        app_state.zoom_level = *zoom;
        let status_info = StatusInfo::from_app_state(&app_state);
        assert_eq!(status_info.zoom_level, *zoom);
    }
}

#[test]
fn test_status_info_complete_state() {
    let mut app_state = AppState::default();
    let document = Document::new("Complete Test".to_string(), 1920, 1080);
    let pixel_color = RgbaPixel::new(100, 150, 200, 255);

    app_state.current_document = Some(document);
    app_state.document_open = true;
    app_state.current_file_path = Some(std::path::PathBuf::from("complete.psoc"));
    app_state.zoom_level = 0.75;
    app_state.mouse_position = Some((960.0, 540.0));
    app_state.current_pixel_color = Some(pixel_color);

    let status_info = StatusInfo::from_app_state(&app_state);

    assert_eq!(status_info.document_status, "Saved");
    assert_eq!(status_info.image_size, Some((1920, 1080)));
    assert!(status_info.color_mode.is_some());
    assert_eq!(status_info.zoom_level, 0.75);
    assert_eq!(status_info.mouse_position, Some((960.0, 540.0)));
    assert_eq!(status_info.pixel_color, Some(pixel_color));
}
