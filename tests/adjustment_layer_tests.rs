//! Tests for adjustment layer functionality

use psoc_core::{rendering::RenderEngine, Document, Layer, LayerType, RgbaPixel};
use std::collections::HashMap;

#[test]
fn test_adjustment_layer_creation() {
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 0.2);

    let layer = Layer::new_adjustment(
        "Brightness Adjustment".to_string(),
        "brightness".to_string(),
        params.clone(),
    );

    assert_eq!(layer.name, "Brightness Adjustment");
    assert!(layer.visible);
    assert_eq!(layer.opacity, 1.0);
    assert!(layer.pixel_data.is_none());

    if let LayerType::Adjustment {
        adjustment_type,
        parameters,
    } = &layer.layer_type
    {
        assert_eq!(adjustment_type, "brightness");
        assert_eq!(parameters.get("brightness"), Some(&0.2));
    } else {
        panic!("Expected adjustment layer type");
    }
}

#[test]
fn test_adjustment_layer_in_document() {
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add base layer
    let base_layer = Layer::new_pixel("Base".to_string(), 100, 100);
    document.add_layer(base_layer);

    // Add adjustment layer
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 0.5);
    let adj_layer =
        Layer::new_adjustment("Brightness".to_string(), "brightness".to_string(), params);
    document.add_layer(adj_layer);

    assert_eq!(document.layers.len(), 2);

    // Check that the adjustment layer is properly stored
    let adj_layer = &document.layers[1];
    if let LayerType::Adjustment {
        adjustment_type, ..
    } = &adj_layer.layer_type
    {
        assert_eq!(adjustment_type, "brightness");
    } else {
        panic!("Expected adjustment layer");
    }
}

#[test]
fn test_adjustment_layer_rendering() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 10, 10);

    // Add base layer with known color
    let mut base_layer = Layer::new_pixel("Base".to_string(), 10, 10);
    base_layer.fill(RgbaPixel::new(100, 100, 100, 255));
    document.add_layer(base_layer);

    // Add brightness adjustment
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 0.5); // 50% brighter
    let adj_layer =
        Layer::new_adjustment("Brightness".to_string(), "brightness".to_string(), params);
    document.add_layer(adj_layer);

    // Render and check result
    let result = engine.render_document(&document).unwrap();
    let pixel = result.get_pixel(5, 5).unwrap();

    // Should be brighter than original
    assert!(pixel.r > 100);
    assert!(pixel.g > 100);
    assert!(pixel.b > 100);
}

#[test]
fn test_multiple_adjustment_layers() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 10, 10);

    // Add base layer
    let mut base_layer = Layer::new_pixel("Base".to_string(), 10, 10);
    base_layer.fill(RgbaPixel::new(100, 100, 100, 255));
    document.add_layer(base_layer);

    // Add brightness adjustment
    let mut brightness_params = HashMap::new();
    brightness_params.insert("brightness".to_string(), 0.2);
    let brightness_layer = Layer::new_adjustment(
        "Brightness".to_string(),
        "brightness".to_string(),
        brightness_params,
    );
    document.add_layer(brightness_layer);

    // Add contrast adjustment
    let mut contrast_params = HashMap::new();
    contrast_params.insert("contrast".to_string(), 0.3);
    let contrast_layer = Layer::new_adjustment(
        "Contrast".to_string(),
        "contrast".to_string(),
        contrast_params,
    );
    document.add_layer(contrast_layer);

    // Should render without errors
    let result = engine.render_document(&document);
    assert!(result.is_ok());
}

#[test]
fn test_adjustment_layer_visibility() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 10, 10);

    // Add base layer
    let mut base_layer = Layer::new_pixel("Base".to_string(), 10, 10);
    base_layer.fill(RgbaPixel::new(100, 100, 100, 255));
    document.add_layer(base_layer);

    // Add invisible adjustment layer
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 1.0); // Very bright
    let mut adj_layer =
        Layer::new_adjustment("Brightness".to_string(), "brightness".to_string(), params);
    adj_layer.visible = false; // Make invisible
    document.add_layer(adj_layer);

    // Render and check that adjustment wasn't applied
    let result = engine.render_document(&document).unwrap();
    let pixel = result.get_pixel(5, 5).unwrap();

    // Should be same as original since adjustment layer is invisible
    assert_eq!(pixel.r, 100);
    assert_eq!(pixel.g, 100);
    assert_eq!(pixel.b, 100);
}

#[test]
fn test_adjustment_layer_opacity_effect() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 10, 10);

    // Add base layer
    let mut base_layer = Layer::new_pixel("Base".to_string(), 10, 10);
    base_layer.fill(RgbaPixel::new(100, 100, 100, 255));
    document.add_layer(base_layer);

    // Add adjustment layer with reduced opacity
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 1.0); // 100% brighter
    let mut adj_layer =
        Layer::new_adjustment("Brightness".to_string(), "brightness".to_string(), params);
    adj_layer.opacity = 0.5; // 50% opacity
    document.add_layer(adj_layer);

    // Render and check that effect is reduced
    let result = engine.render_document(&document).unwrap();
    let pixel = result.get_pixel(5, 5).unwrap();

    // Should be brighter than original but not fully bright
    assert!(pixel.r > 100);
    assert!(pixel.r < 200); // Not fully bright due to opacity
}

#[test]
fn test_adjustment_layer_type_identification() {
    let mut params = HashMap::new();
    params.insert("hue".to_string(), 0.1);
    params.insert("saturation".to_string(), 0.2);
    params.insert("lightness".to_string(), 0.3);

    let layer = Layer::new_adjustment("HSL Adjustment".to_string(), "hsl".to_string(), params);

    // Test that we can identify the adjustment type
    match &layer.layer_type {
        LayerType::Adjustment {
            adjustment_type,
            parameters,
        } => {
            assert_eq!(adjustment_type, "hsl");
            assert_eq!(parameters.len(), 3);
            assert_eq!(parameters.get("hue"), Some(&0.1));
            assert_eq!(parameters.get("saturation"), Some(&0.2));
            assert_eq!(parameters.get("lightness"), Some(&0.3));
        }
        _ => panic!("Expected adjustment layer type"),
    }
}

#[test]
fn test_adjustment_layer_no_pixel_data() {
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 0.5);

    let layer = Layer::new_adjustment("Brightness".to_string(), "brightness".to_string(), params);

    // Adjustment layers should not have pixel data
    assert!(layer.pixel_data.is_none());
    assert!(!layer.has_pixel_data());
    assert_eq!(layer.dimensions(), None);
}

#[test]
fn test_adjustment_layer_bounds() {
    let mut params = HashMap::new();
    params.insert("brightness".to_string(), 0.5);

    let layer = Layer::new_adjustment("Brightness".to_string(), "brightness".to_string(), params);

    // Adjustment layers should have zero bounds
    assert_eq!(layer.bounds.width, 0.0);
    assert_eq!(layer.bounds.height, 0.0);
}
