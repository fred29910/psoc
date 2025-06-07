//! Integration tests for P2.3 Layer Blending and Rendering

use psoc::core::{BlendMode, Document, Layer, Point, RgbaPixel};
use psoc::rendering::{AppRenderer, RenderEngine};

#[test]
fn test_render_engine_creation() {
    let engine = RenderEngine::new();
    // Basic creation test
    assert!(true); // Engine created successfully
}

#[test]
fn test_document_rendering_empty() {
    let mut engine = RenderEngine::new();
    let document = Document::new("Test".to_string(), 100, 100);

    let result = engine.render_document(&document);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();
    let (width, height) = pixel_data.dimensions();
    assert_eq!(width, 100);
    assert_eq!(height, 100);
}

#[test]
fn test_document_rendering_with_layer() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add a red layer
    let mut layer = Layer::new_pixel("Red Layer".to_string(), 50, 50);
    layer.fill(RgbaPixel::new(255, 0, 0, 255));
    layer.offset = Point::new(25.0, 25.0);

    document.add_layer(layer);

    let result = engine.render_document(&document);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();
    let (width, height) = pixel_data.dimensions();
    assert_eq!(width, 100);
    assert_eq!(height, 100);

    // Check that the red pixel is rendered at the correct position
    let pixel = pixel_data.get_pixel(50, 50).unwrap();
    assert_eq!(pixel.r, 255);
    assert_eq!(pixel.g, 0);
    assert_eq!(pixel.b, 0);
}

#[test]
fn test_blend_modes_rendering() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add base layer (blue)
    let mut base_layer = Layer::new_pixel("Base".to_string(), 100, 100);
    base_layer.fill(RgbaPixel::new(0, 0, 255, 255));
    document.add_layer(base_layer);

    // Add overlay layer (red) with multiply blend mode
    let mut overlay_layer = Layer::new_pixel("Overlay".to_string(), 100, 100);
    overlay_layer.fill(RgbaPixel::new(255, 0, 0, 255));
    overlay_layer.blend_mode = BlendMode::Multiply;
    document.add_layer(overlay_layer);

    let result = engine.render_document(&document);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();

    // Check that multiply blending occurred
    let pixel = pixel_data.get_pixel(50, 50).unwrap();
    // Multiply of blue (0,0,255) and red (255,0,0) should be (0,0,0)
    // Allow for small rounding errors in the blending calculation
    assert!(
        pixel.r <= 1,
        "Red channel should be 0 or very close, got {}",
        pixel.r
    );
    assert!(
        pixel.g <= 1,
        "Green channel should be 0 or very close, got {}",
        pixel.g
    );
    assert!(
        pixel.b <= 1,
        "Blue channel should be 0 or very close, got {}",
        pixel.b
    );
}

#[test]
fn test_layer_opacity_rendering() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add base layer (white)
    let mut base_layer = Layer::new_pixel("Base".to_string(), 100, 100);
    base_layer.fill(RgbaPixel::new(255, 255, 255, 255));
    document.add_layer(base_layer);

    // Add overlay layer (black) with 50% opacity
    let mut overlay_layer = Layer::new_pixel("Overlay".to_string(), 100, 100);
    overlay_layer.fill(RgbaPixel::new(0, 0, 0, 255));
    overlay_layer.opacity = 0.5;
    document.add_layer(overlay_layer);

    let result = engine.render_document(&document);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();

    // Check that opacity blending occurred
    let pixel = pixel_data.get_pixel(50, 50).unwrap();
    // 50% blend of white and black should be gray
    assert!(pixel.r > 100 && pixel.r < 200);
    assert!(pixel.g > 100 && pixel.g < 200);
    assert!(pixel.b > 100 && pixel.b < 200);
}

#[test]
fn test_layer_visibility_rendering() {
    let mut engine = RenderEngine::new();
    let mut document = Document::new("Test".to_string(), 100, 100);

    // Add base layer (white)
    let mut base_layer = Layer::new_pixel("Base".to_string(), 100, 100);
    base_layer.fill(RgbaPixel::new(255, 255, 255, 255));
    document.add_layer(base_layer);

    // Add invisible overlay layer (red)
    let mut overlay_layer = Layer::new_pixel("Overlay".to_string(), 100, 100);
    overlay_layer.fill(RgbaPixel::new(255, 0, 0, 255));
    overlay_layer.visible = false;
    document.add_layer(overlay_layer);

    let result = engine.render_document(&document);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();

    // Check that invisible layer was not rendered
    let pixel = pixel_data.get_pixel(50, 50).unwrap();
    // Should still be white (base layer only)
    assert_eq!(pixel.r, 255);
    assert_eq!(pixel.g, 255);
    assert_eq!(pixel.b, 255);
}

#[test]
fn test_app_renderer() {
    let renderer = AppRenderer::new();
    let document = Document::new("Test".to_string(), 50, 50);

    let result = renderer.render_for_display(&document);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();
    let (width, height) = pixel_data.dimensions();
    assert_eq!(width, 50);
    assert_eq!(height, 50);
}

#[test]
fn test_viewport_rendering() {
    let renderer = AppRenderer::new();
    let mut document = Document::new("Test".to_string(), 200, 200);

    // Add a layer
    let mut layer = Layer::new_pixel("Test Layer".to_string(), 200, 200);
    layer.fill(RgbaPixel::new(100, 150, 200, 255));
    document.add_layer(layer);

    // Render a 50x50 region starting at (25, 25)
    let result = renderer.render_viewport(&document, 25, 25, 50, 50);
    assert!(result.is_ok());

    let pixel_data = result.unwrap();
    let (width, height) = pixel_data.dimensions();
    assert_eq!(width, 50);
    assert_eq!(height, 50);

    // Check that the pixel has the expected color
    let pixel = pixel_data.get_pixel(25, 25).unwrap();
    assert_eq!(pixel.r, 100);
    assert_eq!(pixel.g, 150);
    assert_eq!(pixel.b, 200);
}
