//! Smart object functionality tests
//!
//! This module contains comprehensive tests for smart object features including:
//! - Smart object layer creation and management
//! - Content loading and caching
//! - Non-destructive transformations
//! - Rendering integration
//! - Command system integration

use psoc_core::{
    geometry::{Point, Size},
    layer::{InterpolationQuality, Layer, LayerType, SmartObjectContentType, SmartTransform},
    // pixel::{PixelData, RgbaPixel}, // For future pixel-based tests
    smart_object::SmartObjectManager,
    Document,
    RenderEngine,
};
use std::path::PathBuf;
// use tempfile::NamedTempFile; // For future file-based tests

#[test]
fn test_smart_object_layer_creation() {
    let content_type = SmartObjectContentType::EmbeddedImage {
        original_path: Some(PathBuf::from("test.png")),
        image_data: vec![1, 2, 3, 4], // Dummy data
        format: "png".to_string(),
    };

    let original_size = Size::new(100.0, 200.0);
    let position = Point::new(10.0, 20.0);

    let layer = Layer::new_smart_object(
        "Test Smart Object".to_string(),
        content_type.clone(),
        original_size,
        position,
    );

    assert_eq!(layer.name, "Test Smart Object");
    assert!(layer.is_smart_object());
    assert_eq!(layer.smart_object_original_size(), Some(original_size));
    assert_eq!(layer.offset, position);
    assert!(!layer.smart_object_needs_update());

    // Check layer type
    if let LayerType::SmartObject {
        content_type: layer_content,
        original_size: layer_size,
        smart_transform,
        needs_update,
    } = &layer.layer_type
    {
        assert_eq!(*layer_content, content_type);
        assert_eq!(*layer_size, original_size);
        assert_eq!(*smart_transform, SmartTransform::default());
        assert!(!needs_update);
    } else {
        panic!("Layer should be a smart object");
    }
}

#[test]
fn test_smart_object_content_types() {
    // Test embedded image
    let embedded = SmartObjectContentType::EmbeddedImage {
        original_path: Some(PathBuf::from("test.png")),
        image_data: vec![1, 2, 3, 4],
        format: "png".to_string(),
    };

    // Test linked image
    let linked = SmartObjectContentType::LinkedImage {
        file_path: PathBuf::from("linked.jpg"),
        last_modified: None,
    };

    // Test embedded document
    let document = SmartObjectContentType::EmbeddedDocument {
        document_data: vec![5, 6, 7, 8],
    };

    // All should be different
    assert_ne!(embedded, linked);
    assert_ne!(linked, document);
    assert_ne!(embedded, document);
}

#[test]
fn test_smart_transform_default() {
    let transform = SmartTransform::default();

    assert_eq!(transform.scale, (1.0, 1.0));
    assert_eq!(transform.rotation, 0.0);
    assert_eq!(transform.translation, Point::origin());
    assert!(transform.maintain_aspect_ratio);
    assert_eq!(transform.interpolation_quality, InterpolationQuality::High);
}

#[test]
fn test_smart_transform_custom() {
    let transform = SmartTransform {
        scale: (2.0, 1.5),
        rotation: std::f32::consts::PI / 4.0, // 45 degrees
        translation: Point::new(10.0, 20.0),
        maintain_aspect_ratio: false,
        interpolation_quality: InterpolationQuality::Linear,
    };

    assert_eq!(transform.scale, (2.0, 1.5));
    assert_eq!(transform.rotation, std::f32::consts::PI / 4.0);
    assert_eq!(transform.translation, Point::new(10.0, 20.0));
    assert!(!transform.maintain_aspect_ratio);
    assert_eq!(
        transform.interpolation_quality,
        InterpolationQuality::Linear
    );
}

#[test]
fn test_interpolation_quality_variants() {
    assert_eq!(InterpolationQuality::default(), InterpolationQuality::High);

    let qualities = [
        InterpolationQuality::Nearest,
        InterpolationQuality::Linear,
        InterpolationQuality::High,
    ];

    // All should be different
    for (i, quality1) in qualities.iter().enumerate() {
        for (j, quality2) in qualities.iter().enumerate() {
            if i != j {
                assert_ne!(quality1, quality2);
            }
        }
    }
}

#[test]
fn test_smart_object_layer_methods() {
    let content_type = SmartObjectContentType::EmbeddedImage {
        original_path: Some(PathBuf::from("test.png")),
        image_data: vec![1, 2, 3, 4],
        format: "png".to_string(),
    };

    let mut layer = Layer::new_smart_object(
        "Test".to_string(),
        content_type.clone(),
        Size::new(100.0, 100.0),
        Point::origin(),
    );

    // Test smart object methods
    assert!(layer.is_smart_object());
    assert!(!layer.smart_object_needs_update());

    // Test marking for update
    layer.mark_smart_object_for_update();
    assert!(layer.smart_object_needs_update());

    // Test clearing update flag
    layer.clear_smart_object_update_flag();
    assert!(!layer.smart_object_needs_update());

    // Test transform update
    let new_transform = SmartTransform {
        scale: (2.0, 2.0),
        ..SmartTransform::default()
    };

    assert!(layer
        .update_smart_object_transform(new_transform.clone())
        .is_ok());
    assert_eq!(layer.smart_object_transform(), Some(&new_transform));

    // Test transform reset
    assert!(layer.reset_smart_object_transform().is_ok());
    assert_eq!(
        layer.smart_object_transform(),
        Some(&SmartTransform::default())
    );
}

#[test]
fn test_smart_object_manager_creation() {
    let manager = SmartObjectManager::new();
    let (content_cache_size, render_cache_size) = manager.cache_stats();
    assert_eq!(content_cache_size, 0);
    assert_eq!(render_cache_size, 0);

    let manager_with_size = SmartObjectManager::with_cache_size(50);
    let (content_cache_size, render_cache_size) = manager_with_size.cache_stats();
    assert_eq!(content_cache_size, 0);
    assert_eq!(render_cache_size, 0);
}

#[test]
fn test_smart_object_manager_cache_operations() {
    let mut manager = SmartObjectManager::new();

    // Test cache clearing
    manager.clear_caches();
    let (content_cache_size, render_cache_size) = manager.cache_stats();
    assert_eq!(content_cache_size, 0);
    assert_eq!(render_cache_size, 0);

    manager.clear_content_cache();
    manager.clear_render_cache();
    let (content_cache_size, render_cache_size) = manager.cache_stats();
    assert_eq!(content_cache_size, 0);
    assert_eq!(render_cache_size, 0);
}

#[test]
fn test_smart_object_manager_linked_file_update_check() {
    let manager = SmartObjectManager::new();

    // Test with non-linked content (should return false)
    let embedded_content = SmartObjectContentType::EmbeddedImage {
        original_path: Some(PathBuf::from("test.png")),
        image_data: vec![1, 2, 3, 4],
        format: "png".to_string(),
    };

    assert!(!manager.check_linked_file_update(&embedded_content).unwrap());

    // Test with linked content with no stored time (should return true)
    let linked_content_no_time = SmartObjectContentType::LinkedImage {
        file_path: PathBuf::from("non_existent_file.png"),
        last_modified: None,
    };

    // When no time is stored, it should return true (needs update check)
    assert!(manager
        .check_linked_file_update(&linked_content_no_time)
        .unwrap());

    // Test with linked content with stored time but non-existent file (should error)
    let linked_content_with_time = SmartObjectContentType::LinkedImage {
        file_path: PathBuf::from("non_existent_file.png"),
        last_modified: Some(std::time::SystemTime::now()),
    };

    // This should return an error since the file doesn't exist but we have a stored time
    assert!(manager
        .check_linked_file_update(&linked_content_with_time)
        .is_err());
}

#[test]
fn test_render_engine_smart_object_integration() {
    let mut engine = RenderEngine::new();

    // Test that the engine has a smart object manager
    let (content_cache_size, render_cache_size) = engine.smart_object_manager().cache_stats();
    assert_eq!(content_cache_size, 0);
    assert_eq!(render_cache_size, 0);

    // Test mutable access
    engine.smart_object_manager_mut().clear_caches();
    let (content_cache_size, render_cache_size) = engine.smart_object_manager().cache_stats();
    assert_eq!(content_cache_size, 0);
    assert_eq!(render_cache_size, 0);
}

#[test]
fn test_smart_object_in_document() {
    let mut document = Document::new("Test Document".to_string(), 200, 200);

    let content_type = SmartObjectContentType::EmbeddedImage {
        original_path: Some(PathBuf::from("test.png")),
        image_data: vec![1, 2, 3, 4],
        format: "png".to_string(),
    };

    let smart_object_layer = Layer::new_smart_object(
        "Smart Object Layer".to_string(),
        content_type,
        Size::new(100.0, 100.0),
        Point::new(50.0, 50.0),
    );

    document.add_layer(smart_object_layer);

    assert_eq!(document.layer_count(), 1);

    let layer = document.get_layer(0).unwrap();
    assert!(layer.is_smart_object());
    assert_eq!(layer.name, "Smart Object Layer");
}

#[test]
fn test_non_smart_object_layer_methods() {
    let mut pixel_layer = Layer::new_pixel("Pixel Layer".to_string(), 100, 100);

    // Non-smart object layers should return appropriate defaults
    assert!(!pixel_layer.is_smart_object());
    assert!(pixel_layer.smart_object_content_type().is_none());
    assert!(pixel_layer.smart_object_original_size().is_none());
    assert!(pixel_layer.smart_object_transform().is_none());
    assert!(!pixel_layer.smart_object_needs_update());

    // These operations should have no effect on non-smart object layers
    pixel_layer.mark_smart_object_for_update();
    assert!(!pixel_layer.smart_object_needs_update());

    pixel_layer.clear_smart_object_update_flag();
    assert!(!pixel_layer.smart_object_needs_update());

    // These operations should return errors
    let new_transform = SmartTransform::default();
    assert!(pixel_layer
        .update_smart_object_transform(new_transform)
        .is_err());
    assert!(pixel_layer.reset_smart_object_transform().is_err());
}
