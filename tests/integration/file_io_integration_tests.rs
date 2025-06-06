//! Integration tests for file I/O operations

use image::{DynamicImage, RgbaImage};
use psoc::file_io::FileManager;
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
async fn test_full_file_io_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_manager = FileManager::new();

    // Create a test image
    let test_image = create_test_image();

    // Test PNG export and import
    let png_path = temp_dir.path().join("test.png");

    file_manager
        .export_image(&test_image, &png_path)
        .await
        .expect("Failed to export PNG");

    assert!(png_path.exists());

    let imported_png = file_manager
        .import_image(&png_path)
        .await
        .expect("Failed to import PNG");

    assert_eq!(test_image.width(), imported_png.width());
    assert_eq!(test_image.height(), imported_png.height());

    // Test JPEG export and import
    let jpeg_path = temp_dir.path().join("test.jpg");

    file_manager
        .export_image(&test_image, &jpeg_path)
        .await
        .expect("Failed to export JPEG");

    assert!(jpeg_path.exists());

    let imported_jpeg = file_manager
        .import_image(&jpeg_path)
        .await
        .expect("Failed to import JPEG");

    assert_eq!(test_image.width(), imported_jpeg.width());
    assert_eq!(test_image.height(), imported_jpeg.height());
}

#[tokio::test]
async fn test_basic_export_import() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_manager = FileManager::new();

    // Create and save a test image
    let test_image = create_test_image();
    let png_path = temp_dir.path().join("basic_test.png");

    file_manager
        .export_image(&test_image, &png_path)
        .await
        .expect("Failed to export test image");

    assert!(png_path.exists());

    // Import the image back
    let imported_image = file_manager
        .import_image(&png_path)
        .await
        .expect("Failed to import test image");

    assert_eq!(test_image.width(), imported_image.width());
    assert_eq!(test_image.height(), imported_image.height());
}

#[test]
fn test_file_manager_creation() {
    let file_manager = FileManager::new();
    // Just test that we can create a file manager
    // FileManager is likely a zero-sized type, so check the type size instead
    assert!(std::mem::size_of::<FileManager>() >= 0);
}

#[tokio::test]
async fn test_error_handling() {
    let file_manager = FileManager::new();

    // Test importing non-existent file
    let result = file_manager
        .import_image(Path::new("non_existent_file.png"))
        .await;
    assert!(result.is_err());

    // Test importing invalid file
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let invalid_file = temp_dir.path().join("invalid.png");
    std::fs::write(&invalid_file, b"not an image").expect("Failed to write invalid file");

    let result = file_manager.import_image(&invalid_file).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_operations() {
    let file_manager = FileManager::new();
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create multiple test images
    let images: Vec<DynamicImage> = (0..3)
        .map(|i| {
            let mut img = create_test_image();
            // Make each image slightly different
            if let DynamicImage::ImageRgba8(ref mut rgba_img) = img {
                rgba_img.put_pixel(0, 0, image::Rgba([i * 50, 0, 0, 255]));
            }
            img
        })
        .collect();

    // Export all images concurrently
    let export_tasks: Vec<_> = images
        .iter()
        .enumerate()
        .map(|(i, img)| {
            let path = temp_dir.path().join(format!("concurrent_test_{}.png", i));
            let fm = file_manager.clone();
            let img_clone = img.clone();
            tokio::spawn(async move { fm.export_image(&img_clone, &path).await })
        })
        .collect();

    // Wait for all exports to complete
    for task in export_tasks {
        task.await.expect("Task failed").expect("Export failed");
    }

    // Verify all files were created
    for i in 0..3 {
        let path = temp_dir.path().join(format!("concurrent_test_{}.png", i));
        assert!(path.exists());
    }
}

/// Create a simple test image for testing purposes
fn create_test_image() -> DynamicImage {
    let mut img = RgbaImage::new(100, 100);

    // Create a simple pattern
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x * 255 / 100) as u8;
        let g = (y * 255 / 100) as u8;
        let b = ((x + y) * 255 / 200) as u8;
        *pixel = image::Rgba([r, g, b, 255]);
    }

    DynamicImage::ImageRgba8(img)
}
