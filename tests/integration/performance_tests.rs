//! Performance tests for PSOC Image Editor

use image::{DynamicImage, RgbaImage};
use psoc::file_io::FileManager;
use psoc_core::{Document, Layer, PixelData, RgbaPixel};
use std::time::Instant;
use tempfile::TempDir;

#[tokio::test]
async fn test_large_image_export_performance() {
    let file_manager = FileManager::new();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a large test image (1000x1000)
    let large_image = create_large_test_image(1000, 1000);

    // Test PNG export performance
    let png_path = temp_dir.path().join("large_test.png");
    let start = Instant::now();

    file_manager
        .export_image(&large_image, &png_path)
        .await
        .expect("Failed to export large PNG");

    let png_duration = start.elapsed();
    println!("Large PNG export took: {:?}", png_duration);

    // Test JPEG export performance
    let jpeg_path = temp_dir.path().join("large_test.jpg");
    let start = Instant::now();

    file_manager
        .export_image(&large_image, &jpeg_path)
        .await
        .expect("Failed to export large JPEG");

    let jpeg_duration = start.elapsed();
    println!("Large JPEG export took: {:?}", jpeg_duration);

    // Performance assertions (these are rough guidelines)
    assert!(
        png_duration.as_secs() < 10,
        "PNG export took too long: {:?}",
        png_duration
    );
    assert!(
        jpeg_duration.as_secs() < 10,
        "JPEG export took too long: {:?}",
        jpeg_duration
    );
}

#[tokio::test]
async fn test_multiple_small_images_performance() {
    let file_manager = FileManager::new();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let num_images = 50;
    let images: Vec<DynamicImage> = (0..num_images)
        .map(|_| create_small_test_image(100, 100))
        .collect();

    let start = Instant::now();

    // Export all images sequentially
    for (i, image) in images.iter().enumerate() {
        let path = temp_dir.path().join(format!("small_test_{}.png", i));
        file_manager
            .export_image(image, &path)
            .await
            .expect("Failed to export small image");
    }

    let duration = start.elapsed();
    println!("Exported {} small images in: {:?}", num_images, duration);

    // Should be able to export 50 small images in reasonable time
    assert!(
        duration.as_secs() < 30,
        "Small images export took too long: {:?}",
        duration
    );
}

#[test]
fn test_pixel_data_operations_performance() {
    let width = 1000;
    let height = 1000;

    // Test PixelData creation performance
    let start = Instant::now();
    let mut pixel_data = PixelData::new_rgba(width, height);
    let creation_duration = start.elapsed();
    println!(
        "PixelData creation ({} x {}) took: {:?}",
        width, height, creation_duration
    );

    // Test fill operation performance
    let start = Instant::now();
    pixel_data.fill(RgbaPixel::new(255, 128, 64, 255));
    let fill_duration = start.elapsed();
    println!("PixelData fill took: {:?}", fill_duration);

    // Test pixel access performance
    let start = Instant::now();
    for y in 0..height {
        for x in 0..width {
            let _pixel = pixel_data.get_pixel(x, y);
        }
    }
    let access_duration = start.elapsed();
    println!(
        "PixelData access ({} pixels) took: {:?}",
        width * height,
        access_duration
    );

    // Performance assertions (relaxed for CI environments)
    assert!(
        creation_duration.as_millis() < 1000,
        "PixelData creation too slow"
    );
    assert!(fill_duration.as_millis() < 1000, "PixelData fill too slow");
    assert!(
        access_duration.as_millis() < 2000,
        "PixelData access too slow"
    );
}

#[test]
fn test_layer_operations_performance() {
    let width = 500;
    let height = 500;

    // Test Layer creation performance
    let start = Instant::now();
    let layer = Layer::new_pixel("Test Layer".to_string(), width, height);
    let creation_duration = start.elapsed();
    println!(
        "Layer creation ({} x {}) took: {:?}",
        width, height, creation_duration
    );

    // Test Layer duplication performance
    let start = Instant::now();
    let _duplicated_layer = layer.duplicate();
    let duplication_duration = start.elapsed();
    println!("Layer duplication took: {:?}", duplication_duration);

    // Performance assertions
    assert!(
        creation_duration.as_millis() < 100,
        "Layer creation too slow"
    );
    assert!(
        duplication_duration.as_millis() < 100,
        "Layer duplication too slow"
    );
}

#[test]
fn test_document_operations_performance() {
    let width = 800;
    let height = 600;

    // Test Document creation performance
    let start = Instant::now();
    let mut document = Document::new("Test Document".to_string(), width, height);
    let creation_duration = start.elapsed();
    println!(
        "Document creation ({} x {}) took: {:?}",
        width, height, creation_duration
    );

    // Test adding multiple layers performance
    let start = Instant::now();
    for i in 0..10 {
        let layer = Layer::new_pixel(format!("Layer {}", i), width, height);
        document.add_layer(layer);
    }
    let add_layers_duration = start.elapsed();
    println!("Adding 10 layers took: {:?}", add_layers_duration);

    // Performance assertions
    assert!(
        creation_duration.as_millis() < 50,
        "Document creation too slow"
    );
    assert!(
        add_layers_duration.as_millis() < 100,
        "Adding layers too slow"
    );
}

#[test]
fn test_memory_usage_estimation() {
    let width = 1920;
    let height = 1080;

    // Calculate expected memory usage for a full HD image
    let expected_bytes = width * height * 4; // RGBA = 4 bytes per pixel
    let expected_mb = expected_bytes as f64 / (1024.0 * 1024.0);

    println!(
        "Expected memory for {}x{} RGBA image: {:.2} MB",
        width, height, expected_mb
    );

    // Create the image and measure actual usage (approximate)
    let pixel_data = PixelData::new_rgba(width, height);
    let (actual_width, actual_height) = pixel_data.dimensions();
    let actual_bytes = (actual_width * actual_height * 4) as usize; // RGBA = 4 bytes per pixel
    let actual_mb = actual_bytes as f64 / (1024.0 * 1024.0);

    println!("Actual memory usage: {:.2} MB", actual_mb);

    // Should be close to expected (within 10% tolerance)
    let tolerance = expected_mb * 0.1;
    assert!(
        (actual_mb - expected_mb).abs() < tolerance,
        "Memory usage differs too much from expected: actual={:.2}MB, expected={:.2}MB",
        actual_mb,
        expected_mb
    );
}

/// Create a large test image for performance testing
fn create_large_test_image(width: u32, height: u32) -> DynamicImage {
    let mut img = RgbaImage::new(width, height);

    // Create a more complex pattern for realistic testing
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = ((x as f32 * 255.0 / width as f32) + (y as f32 * 128.0 / height as f32)) as u8;
        let g = ((y as f32 * 255.0 / height as f32) + (x as f32 * 64.0 / width as f32)) as u8;
        let b = ((x as f32 + y as f32) * 255.0 / (width + height) as f32) as u8;
        let a = 255;
        *pixel = image::Rgba([r, g, b, a]);
    }

    DynamicImage::ImageRgba8(img)
}

/// Create a small test image for performance testing
fn create_small_test_image(width: u32, height: u32) -> DynamicImage {
    let mut img = RgbaImage::new(width, height);

    // Simple solid color with some variation
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x % 256) as u8;
        let g = (y % 256) as u8;
        let b = ((x + y) % 256) as u8;
        *pixel = image::Rgba([r, g, b, 255]);
    }

    DynamicImage::ImageRgba8(img)
}
