//! File I/O demonstration example
//!
//! This example demonstrates the file I/O capabilities of PSOC,
//! including loading and saving images in various formats.

use image::{DynamicImage, ImageBuffer, Rgb};
use psoc::file_io::{is_supported_import_extension, FileManager};
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    psoc::init_default_logging()?;

    println!("PSOC File I/O Demo");
    println!("==================");

    // Create a file manager
    let file_manager = FileManager::new();

    // Demonstrate supported formats
    println!("\n1. Supported Import Extensions:");
    let import_extensions = FileManager::supported_import_extensions();
    for ext in import_extensions {
        println!("   - {}", ext);
    }

    println!("\n2. Supported Export Extensions:");
    let export_extensions = FileManager::supported_export_extensions();
    for ext in export_extensions {
        println!("   - {}", ext);
    }

    // Test extension checking
    println!("\n3. Extension Support Check:");
    let test_files = ["test.png", "test.jpg", "test.jpeg", "test.txt", "test.pdf"];
    for file in test_files {
        let supported = is_supported_import_extension(file);
        println!(
            "   {} -> {}",
            file,
            if supported {
                "✓ Supported"
            } else {
                "✗ Not supported"
            }
        );
    }

    // Create a temporary directory for our demo
    let temp_dir = tempdir()?;
    println!("\n4. Creating Demo Images:");
    println!("   Working directory: {}", temp_dir.path().display());

    // Create a simple test image
    let width = 200;
    let height = 150;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Create a simple gradient pattern
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x as f32 / width as f32 * 255.0) as u8;
        let g = (y as f32 / height as f32 * 255.0) as u8;
        let b = 128;
        *pixel = Rgb([r, g, b]);
    }

    let dynamic_img = DynamicImage::ImageRgb8(img);

    // Save in different formats
    let png_path = temp_dir.path().join("demo.png");
    let jpg_path = temp_dir.path().join("demo.jpg");

    println!("   Creating PNG image...");
    file_manager.export_image(&dynamic_img, &png_path).await?;
    println!("   ✓ Saved: {}", png_path.display());

    println!("   Creating JPEG image...");
    file_manager.export_image(&dynamic_img, &jpg_path).await?;
    println!("   ✓ Saved: {}", jpg_path.display());

    // Load images back
    println!("\n5. Loading Images:");

    println!("   Loading PNG...");
    let loaded_png = file_manager.import_image(&png_path).await?;
    println!(
        "   ✓ PNG loaded: {}x{} pixels",
        loaded_png.width(),
        loaded_png.height()
    );

    println!("   Loading JPEG...");
    let loaded_jpg = file_manager.import_image(&jpg_path).await?;
    println!(
        "   ✓ JPEG loaded: {}x{} pixels",
        loaded_jpg.width(),
        loaded_jpg.height()
    );

    // Get metadata
    println!("\n6. Image Metadata:");
    let png_metadata = psoc::file_io::get_image_metadata(&png_path).await?;
    println!("   PNG: {}", png_metadata.description());

    let jpg_metadata = psoc::file_io::get_image_metadata(&jpg_path).await?;
    println!("   JPEG: {}", jpg_metadata.description());

    // Demonstrate export options
    println!("\n7. Export with Options:");

    // High quality JPEG
    let hq_jpg_path = temp_dir.path().join("demo_hq.jpg");
    let jpeg_options =
        psoc::file_io::ExportOptions::Jpeg(psoc_file_formats::JpegOptions::high_quality());
    psoc::file_io::export_image_with_options(&dynamic_img, &hq_jpg_path, jpeg_options).await?;
    println!("   ✓ High quality JPEG saved: {}", hq_jpg_path.display());

    // PNG with options
    let png_options_path = temp_dir.path().join("demo_options.png");
    let png_options = psoc::file_io::ExportOptions::Png(psoc_file_formats::PngOptions::default());
    psoc::file_io::export_image_with_options(&dynamic_img, &png_options_path, png_options).await?;
    println!(
        "   ✓ PNG with options saved: {}",
        png_options_path.display()
    );

    // File size estimation
    println!("\n8. File Size Estimation:");
    let png_estimate =
        psoc::file_io::estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Png);
    let jpeg_estimate =
        psoc::file_io::estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Jpeg);

    println!("   Estimated PNG size: {} bytes", png_estimate);
    println!("   Estimated JPEG size: {} bytes", jpeg_estimate);

    // Actual file sizes
    let png_actual = std::fs::metadata(&png_path)?.len();
    let jpg_actual = std::fs::metadata(&jpg_path)?.len();

    println!("   Actual PNG size: {} bytes", png_actual);
    println!("   Actual JPEG size: {} bytes", jpg_actual);

    println!("\n✓ File I/O demo completed successfully!");
    println!(
        "  All temporary files are in: {}",
        temp_dir.path().display()
    );

    Ok(())
}
