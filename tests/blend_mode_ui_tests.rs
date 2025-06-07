//! P6.5 Blend Mode UI Integration Tests

use psoc::core::{BlendMode, Document, Layer, RgbaPixel};
use psoc::ui::LayerMessage;

#[test]
fn test_blend_mode_enum_completeness() {
    let all_modes = BlendMode::all();

    // Verify we have all the expected blend modes
    assert!(all_modes.contains(&BlendMode::Normal));
    assert!(all_modes.contains(&BlendMode::Multiply));
    assert!(all_modes.contains(&BlendMode::Screen));
    assert!(all_modes.contains(&BlendMode::Overlay));
    assert!(all_modes.contains(&BlendMode::SoftLight));
    assert!(all_modes.contains(&BlendMode::HardLight));
    assert!(all_modes.contains(&BlendMode::ColorDodge));
    assert!(all_modes.contains(&BlendMode::ColorBurn));
    assert!(all_modes.contains(&BlendMode::Darken));
    assert!(all_modes.contains(&BlendMode::Lighten));
    assert!(all_modes.contains(&BlendMode::Difference));
    assert!(all_modes.contains(&BlendMode::Exclusion));
    assert!(all_modes.contains(&BlendMode::Hue));
    assert!(all_modes.contains(&BlendMode::Saturation));
    assert!(all_modes.contains(&BlendMode::Color));
    assert!(all_modes.contains(&BlendMode::Luminosity));

    // Verify we have at least 16 blend modes (P6.5 requirement)
    assert!(all_modes.len() >= 16);
}

#[test]
fn test_blend_mode_names() {
    // Test that all blend modes have proper human-readable names
    let modes = BlendMode::all();

    for mode in modes {
        let name = mode.name();
        assert!(!name.is_empty(), "Blend mode {:?} should have a name", mode);
        assert!(
            name.len() > 2,
            "Blend mode name should be descriptive: {}",
            name
        );
    }

    // Test specific names
    assert_eq!(BlendMode::Normal.name(), "Normal");
    assert_eq!(BlendMode::Multiply.name(), "Multiply");
    assert_eq!(BlendMode::Screen.name(), "Screen");
    assert_eq!(BlendMode::SoftLight.name(), "Soft Light");
    assert_eq!(BlendMode::HardLight.name(), "Hard Light");
    assert_eq!(BlendMode::ColorDodge.name(), "Color Dodge");
    assert_eq!(BlendMode::ColorBurn.name(), "Color Burn");
}

#[test]
fn test_layer_blend_mode_assignment() {
    let mut layer = Layer::new_pixel("Test".to_string(), 100, 100);

    // Test default blend mode
    assert_eq!(layer.blend_mode, BlendMode::Normal);

    // Test setting different blend modes
    layer.blend_mode = BlendMode::Multiply;
    assert_eq!(layer.blend_mode, BlendMode::Multiply);

    layer.blend_mode = BlendMode::Screen;
    assert_eq!(layer.blend_mode, BlendMode::Screen);

    layer.blend_mode = BlendMode::Overlay;
    assert_eq!(layer.blend_mode, BlendMode::Overlay);
}

#[test]
fn test_document_layer_blend_modes() {
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add layers with different blend modes
    let mut layer1 = Layer::new_pixel("Layer 1".to_string(), 100, 100);
    layer1.blend_mode = BlendMode::Normal;
    document.add_layer(layer1);

    let mut layer2 = Layer::new_pixel("Layer 2".to_string(), 100, 100);
    layer2.blend_mode = BlendMode::Multiply;
    document.add_layer(layer2);

    let mut layer3 = Layer::new_pixel("Layer 3".to_string(), 100, 100);
    layer3.blend_mode = BlendMode::Screen;
    document.add_layer(layer3);

    // Verify blend modes are preserved
    assert_eq!(document.layers[0].blend_mode, BlendMode::Normal);
    assert_eq!(document.layers[1].blend_mode, BlendMode::Multiply);
    assert_eq!(document.layers[2].blend_mode, BlendMode::Screen);
}

#[test]
fn test_layer_message_blend_mode_change() {
    // Test that LayerMessage::ChangeLayerBlendMode exists and can be created
    let message = LayerMessage::ChangeLayerBlendMode(0, BlendMode::Multiply);

    match message {
        LayerMessage::ChangeLayerBlendMode(index, blend_mode) => {
            assert_eq!(index, 0);
            assert_eq!(blend_mode, BlendMode::Multiply);
        }
        _ => panic!("Expected ChangeLayerBlendMode message"),
    }
}

#[test]
fn test_blend_mode_cycling() {
    // Test the get_next_blend_mode function logic
    let modes = BlendMode::all();

    // Test cycling through all modes
    let mut current = BlendMode::Normal;
    let mut visited = Vec::new();

    for _ in 0..modes.len() {
        visited.push(current);
        // Simulate cycling (we can't call the private function directly)
        let current_index = modes.iter().position(|&mode| mode == current).unwrap_or(0);
        let next_index = (current_index + 1) % modes.len();
        current = modes[next_index];
    }

    // Should have visited all modes (check uniqueness)
    visited.sort_by_key(|mode| format!("{:?}", mode));
    visited.dedup();
    assert_eq!(visited.len(), modes.len());
}

#[test]
fn test_blend_mode_serialization() {
    // Test that blend modes can be serialized/deserialized (for project files)
    let original_mode = BlendMode::SoftLight;

    // Create a layer with the blend mode
    let mut layer = Layer::new_pixel("Test".to_string(), 10, 10);
    layer.blend_mode = original_mode;

    // Serialize to JSON
    let json = serde_json::to_string(&layer).expect("Should serialize");

    // Deserialize back
    let deserialized_layer: Layer = serde_json::from_str(&json).expect("Should deserialize");

    // Verify blend mode is preserved
    assert_eq!(deserialized_layer.blend_mode, original_mode);
}

#[test]
fn test_blend_mode_rendering_integration() {
    use psoc::rendering::RenderEngine;

    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Create base layer (red)
    let mut base_layer = Layer::new_pixel("Base".to_string(), 100, 100);
    base_layer.fill(RgbaPixel::new(255, 0, 0, 255));
    document.add_layer(base_layer);

    // Create overlay layer (blue) with multiply blend mode
    let mut overlay_layer = Layer::new_pixel("Overlay".to_string(), 100, 100);
    overlay_layer.fill(RgbaPixel::new(0, 0, 255, 255));
    overlay_layer.blend_mode = BlendMode::Multiply;
    document.add_layer(overlay_layer);

    // Render the document
    let result = engine.render_document(&document);
    assert!(result.is_ok(), "Should render successfully");

    let pixel_data = result.unwrap();
    let pixel = pixel_data.get_pixel(50, 50).unwrap();

    // Multiply of red (255,0,0) and blue (0,0,255) should be (0,0,0)
    assert!(pixel.r <= 1, "Red channel should be 0 or very close");
    assert!(pixel.g <= 1, "Green channel should be 0 or very close");
    assert!(pixel.b <= 1, "Blue channel should be 0 or very close");
}

#[test]
fn test_layer_opacity_and_blend_mode_interaction() {
    use psoc::rendering::RenderEngine;

    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Create base layer (white)
    let mut base_layer = Layer::new_pixel("Base".to_string(), 100, 100);
    base_layer.fill(RgbaPixel::new(255, 255, 255, 255));
    document.add_layer(base_layer);

    // Create overlay layer (black) with 50% opacity and multiply blend mode
    let mut overlay_layer = Layer::new_pixel("Overlay".to_string(), 100, 100);
    overlay_layer.fill(RgbaPixel::new(0, 0, 0, 255));
    overlay_layer.opacity = 0.5;
    overlay_layer.blend_mode = BlendMode::Multiply;
    document.add_layer(overlay_layer);

    // Render the document
    let result = engine.render_document(&document);
    assert!(result.is_ok(), "Should render successfully");

    let pixel_data = result.unwrap();
    let pixel = pixel_data.get_pixel(50, 50).unwrap();

    // With 50% opacity, the result should be between white and black
    assert!(pixel.r > 100 && pixel.r < 255, "Should be gray-ish");
    assert!(pixel.g > 100 && pixel.g < 255, "Should be gray-ish");
    assert!(pixel.b > 100 && pixel.b < 255, "Should be gray-ish");
}
