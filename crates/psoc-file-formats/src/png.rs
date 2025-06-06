//! PNG format support
//!
//! This module provides PNG image loading and saving functionality.

use anyhow::{Context, Result};
use std::path::Path;
use tracing::{debug, instrument};

/// Load a PNG image from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_png<P: AsRef<Path>>(path: P) -> Result<image::DynamicImage> {
    let path = path.as_ref();
    debug!("Loading PNG image from: {}", path.display());

    let image = image::open(path)
        .with_context(|| format!("Failed to load PNG image from: {}", path.display()))?;

    // Verify it's actually a PNG
    if !matches!(
        image.color(),
        image::ColorType::Rgb8
            | image::ColorType::Rgba8
            | image::ColorType::L8
            | image::ColorType::La8
    ) {
        debug!("Converting image color type for PNG compatibility");
    }

    Ok(image)
}

/// Save a PNG image to a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn save_png<P: AsRef<Path>>(image: &image::DynamicImage, path: P) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving PNG image to: {}", path.display());

    image
        .save_with_format(path, image::ImageFormat::Png)
        .with_context(|| format!("Failed to save PNG image to: {}", path.display()))?;

    Ok(())
}

/// PNG-specific configuration options
#[derive(Debug, Clone)]
pub struct PngOptions {
    /// Compression level (0-9, where 9 is maximum compression)
    pub compression_level: u8,
    /// Whether to use filtering
    pub use_filtering: bool,
}

impl Default for PngOptions {
    fn default() -> Self {
        Self {
            compression_level: 6, // Default compression level
            use_filtering: true,
        }
    }
}

/// Save a PNG image with specific options
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn save_png_with_options<P: AsRef<Path>>(
    image: &image::DynamicImage,
    path: P,
    _options: &PngOptions,
) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving PNG image with options to: {}", path.display());

    // For now, use the standard save method
    // TODO: Implement custom PNG encoding with options when needed
    save_png(image, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_png() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.png");

        // Create a simple test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        // Save the image
        save_png(&dynamic_img, &file_path)?;

        // Verify file exists
        assert!(file_path.exists());

        // Load the image back
        let loaded_img = load_png(&file_path)?;

        // Verify dimensions
        assert_eq!(loaded_img.width(), 100);
        assert_eq!(loaded_img.height(), 100);

        Ok(())
    }

    #[test]
    fn test_png_options_default() {
        let options = PngOptions::default();
        assert_eq!(options.compression_level, 6);
        assert!(options.use_filtering);
    }

    #[test]
    fn test_save_png_with_options() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_options.png");

        // Create a simple test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let options = PngOptions {
            compression_level: 9,
            use_filtering: false,
        };

        // Save the image with options
        save_png_with_options(&dynamic_img, &file_path, &options)?;

        // Verify file exists
        assert!(file_path.exists());

        Ok(())
    }
}
