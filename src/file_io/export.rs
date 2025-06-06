//! Image export functionality
//!
//! This module handles exporting images to various file formats.

use std::path::Path;
use anyhow::{Context, Result};
use psoc_file_formats::DynamicImage;
use tracing::{debug, info, instrument, warn};

/// Export an image to a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub async fn export_image<P: AsRef<Path>>(image: &DynamicImage, path: P) -> Result<()> {
    let path = path.as_ref();
    debug!("Starting image export to: {}", path.display());

    // Validate the target directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
            debug!("Created directory: {}", parent.display());
        }
    }

    // Clone the image for the blocking operation
    let image_clone = image.clone();
    let path_clone = path.to_path_buf();

    // Use tokio to run the blocking I/O operation
    tokio::task::spawn_blocking(move || {
        // Use the psoc-file-formats crate for actual saving
        psoc_file_formats::ImageIO::save_image(&image_clone, &path_clone)
    })
    .await
    .context("Failed to spawn image saving task")?
    .context("Failed to save image")?;

    info!(
        width = image.width(),
        height = image.height(),
        color_type = ?image.color(),
        path = %path.display(),
        "Image exported successfully"
    );

    Ok(())
}

/// Export options for different formats
#[derive(Debug, Clone)]
pub enum ExportOptions {
    Png(psoc_file_formats::PngOptions),
    Jpeg(psoc_file_formats::JpegOptions),
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self::Png(psoc_file_formats::PngOptions::default())
    }
}

/// Export an image with specific format options
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub async fn export_image_with_options<P: AsRef<Path>>(
    image: &DynamicImage,
    path: P,
    options: ExportOptions,
) -> Result<()> {
    let path = path.as_ref();
    debug!("Starting image export with options to: {}", path.display());

    // Validate the target directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }

    // Clone the image for the blocking operation
    let image_clone = image.clone();
    let path_clone = path.to_path_buf();
    let options_debug = format!("{:?}", options);

    // Use tokio to run the blocking I/O operation
    tokio::task::spawn_blocking(move || {
        match options {
            ExportOptions::Png(png_options) => {
                psoc_file_formats::save_png_with_options(&image_clone, &path_clone, &png_options)
            }
            ExportOptions::Jpeg(jpeg_options) => {
                psoc_file_formats::save_jpeg_with_options(&image_clone, &path_clone, &jpeg_options)
            }
        }
    })
    .await
    .context("Failed to spawn image saving task")?
    .context("Failed to save image with options")?;

    info!(
        width = image.width(),
        height = image.height(),
        options = %options_debug,
        "Image exported with options successfully"
    );

    Ok(())
}

/// Validate if an image can be exported to the given path
pub fn can_export<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    // Check if extension is supported
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let supported_extensions = ["png", "jpg", "jpeg"];
        supported_extensions.iter().any(|&supported| {
            ext.to_lowercase() == supported
        })
    } else {
        false
    }
}

/// Get the recommended export options for a given file path
pub fn get_recommended_export_options<P: AsRef<Path>>(path: P) -> Option<ExportOptions> {
    let path = path.as_ref();
    
    if let Some(format) = psoc_file_formats::SupportedFormat::from_path(path) {
        match format {
            psoc_file_formats::SupportedFormat::Png => {
                Some(ExportOptions::Png(psoc_file_formats::PngOptions::default()))
            }
            psoc_file_formats::SupportedFormat::Jpeg => {
                Some(ExportOptions::Jpeg(psoc_file_formats::JpegOptions::default()))
            }
        }
    } else {
        None
    }
}

/// Export multiple images to different paths
#[instrument(skip_all)]
pub async fn export_images<P: AsRef<Path>>(
    images_and_paths: Vec<(&DynamicImage, P)>
) -> Result<Vec<String>> {
    debug!("Exporting {} images", images_and_paths.len());

    let mut successful_exports = Vec::new();
    let mut errors = Vec::new();

    for (image, path) in images_and_paths {
        let path = path.as_ref();
        let path_str = path.display().to_string();

        match export_image(image, path).await {
            Ok(()) => {
                successful_exports.push(path_str);
            }
            Err(e) => {
                warn!(
                    path = %path.display(),
                    error = %e,
                    "Failed to export image"
                );
                errors.push((path_str, e));
            }
        }
    }

    if !errors.is_empty() {
        warn!(
            successful = successful_exports.len(),
            failed = errors.len(),
            "Some images failed to export"
        );
    }

    if successful_exports.is_empty() {
        return Err(anyhow::anyhow!("No images were successfully exported"));
    }

    info!(
        exported = successful_exports.len(),
        "Batch image export completed"
    );

    Ok(successful_exports)
}

/// Estimate the file size for an export operation
pub fn estimate_export_size(image: &DynamicImage, format: psoc_file_formats::SupportedFormat) -> u64 {
    let pixel_count = (image.width() * image.height()) as u64;
    
    match format {
        psoc_file_formats::SupportedFormat::Png => {
            // PNG: roughly 3-4 bytes per pixel for RGB, 4-5 for RGBA (with compression)
            match image.color() {
                image::ColorType::Rgba8 => pixel_count * 4,
                _ => pixel_count * 3,
            }
        }
        psoc_file_formats::SupportedFormat::Jpeg => {
            // JPEG: roughly 0.5-2 bytes per pixel depending on quality
            pixel_count / 2 // Conservative estimate for default quality
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_export_image() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_export.png");

        // Create a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = DynamicImage::ImageRgb8(img);

        // Export the image
        export_image(&dynamic_img, &file_path).await?;

        // Verify file exists
        assert!(file_path.exists());

        Ok(())
    }

    #[test]
    fn test_can_export() {
        // Test with supported extensions
        assert!(can_export("test.png"));
        assert!(can_export("test.jpg"));
        assert!(can_export("test.jpeg"));
        assert!(can_export("test.PNG"));
        assert!(can_export("test.JPG"));

        // Test with unsupported extensions
        assert!(!can_export("test.txt"));
        assert!(!can_export("test.pdf"));
        assert!(!can_export("test"));
    }

    #[test]
    fn test_get_recommended_export_options() {
        let png_options = get_recommended_export_options("test.png");
        assert!(matches!(png_options, Some(ExportOptions::Png(_))));

        let jpeg_options = get_recommended_export_options("test.jpg");
        assert!(matches!(jpeg_options, Some(ExportOptions::Jpeg(_))));

        let unknown_options = get_recommended_export_options("test.txt");
        assert!(unknown_options.is_none());
    }

    #[test]
    fn test_estimate_export_size() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = DynamicImage::ImageRgb8(img);

        let png_size = estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Png);
        let jpeg_size = estimate_export_size(&dynamic_img, psoc_file_formats::SupportedFormat::Jpeg);

        assert!(png_size > 0);
        assert!(jpeg_size > 0);
        // JPEG should generally be smaller than PNG for photos
        assert!(jpeg_size < png_size);
    }

    #[tokio::test]
    async fn test_export_image_with_options() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_options.jpg");

        // Create a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = DynamicImage::ImageRgb8(img);

        let options = ExportOptions::Jpeg(psoc_file_formats::JpegOptions::high_quality());

        // Export the image with options
        export_image_with_options(&dynamic_img, &file_path, options).await?;

        // Verify file exists
        assert!(file_path.exists());

        Ok(())
    }
}
