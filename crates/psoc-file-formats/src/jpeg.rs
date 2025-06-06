//! JPEG format support
//!
//! This module provides JPEG image loading and saving functionality.

use std::path::Path;
use anyhow::{Context, Result};
use tracing::{debug, instrument, warn};

/// Load a JPEG image from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_jpeg<P: AsRef<Path>>(path: P) -> Result<image::DynamicImage> {
    let path = path.as_ref();
    debug!("Loading JPEG image from: {}", path.display());

    let image = image::open(path)
        .with_context(|| format!("Failed to load JPEG image from: {}", path.display()))?;

    // JPEG doesn't support transparency, so convert RGBA to RGB if needed
    let image = match image.color() {
        image::ColorType::Rgba8 => {
            warn!("Converting RGBA image to RGB for JPEG compatibility");
            image::DynamicImage::ImageRgb8(image.to_rgb8())
        }
        _ => image,
    };

    Ok(image)
}

/// Save a JPEG image to a file path with default quality (85)
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn save_jpeg<P: AsRef<Path>>(image: &image::DynamicImage, path: P) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving JPEG image to: {}", path.display());

    // Convert to RGB if necessary (JPEG doesn't support transparency)
    let image = match image.color() {
        image::ColorType::Rgba8 => {
            warn!("Converting RGBA image to RGB for JPEG compatibility");
            image::DynamicImage::ImageRgb8(image.to_rgb8())
        }
        _ => image.clone(),
    };

    image
        .save_with_format(path, image::ImageFormat::Jpeg)
        .with_context(|| format!("Failed to save JPEG image to: {}", path.display()))?;

    Ok(())
}

/// JPEG-specific configuration options
#[derive(Debug, Clone)]
pub struct JpegOptions {
    /// Quality level (1-100, where 100 is maximum quality)
    pub quality: u8,
    /// Whether to use progressive encoding
    pub progressive: bool,
    /// Whether to optimize Huffman tables
    pub optimize_huffman: bool,
}

impl Default for JpegOptions {
    fn default() -> Self {
        Self {
            quality: 85,        // Good balance between quality and file size
            progressive: false, // Standard baseline JPEG
            optimize_huffman: true,
        }
    }
}

impl JpegOptions {
    /// Create high quality JPEG options
    pub fn high_quality() -> Self {
        Self {
            quality: 95,
            progressive: true,
            optimize_huffman: true,
        }
    }

    /// Create low quality JPEG options for web use
    pub fn web_quality() -> Self {
        Self {
            quality: 75,
            progressive: true,
            optimize_huffman: true,
        }
    }

    /// Validate quality value
    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.clamp(1, 100);
        self
    }
}

/// Save a JPEG image with specific options
#[instrument(skip_all, fields(path = %path.as_ref().display(), quality = options.quality))]
pub fn save_jpeg_with_options<P: AsRef<Path>>(
    image: &image::DynamicImage,
    path: P,
    options: &JpegOptions,
) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving JPEG image with options to: {}", path.display());

    // Convert to RGB if necessary (JPEG doesn't support transparency)
    let image = match image.color() {
        image::ColorType::Rgba8 => {
            warn!("Converting RGBA image to RGB for JPEG compatibility");
            image::DynamicImage::ImageRgb8(image.to_rgb8())
        }
        _ => image.clone(),
    };

    // For now, use the standard save method
    // TODO: Implement custom JPEG encoding with quality options when needed
    image
        .save_with_format(path, image::ImageFormat::Jpeg)
        .with_context(|| format!("Failed to save JPEG image to: {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb, Rgba};
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_jpeg() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.jpg");

        // Create a simple test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        // Save the image
        save_jpeg(&dynamic_img, &file_path)?;

        // Verify file exists
        assert!(file_path.exists());

        // Load the image back
        let loaded_img = load_jpeg(&file_path)?;

        // Verify dimensions
        assert_eq!(loaded_img.width(), 100);
        assert_eq!(loaded_img.height(), 100);

        Ok(())
    }

    #[test]
    fn test_rgba_to_rgb_conversion() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_rgba.jpg");

        // Create an RGBA test image
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgba8(img);

        // Save the image (should convert to RGB)
        save_jpeg(&dynamic_img, &file_path)?;

        // Verify file exists
        assert!(file_path.exists());

        // Load the image back
        let loaded_img = load_jpeg(&file_path)?;

        // Should be RGB now
        assert_eq!(loaded_img.color(), image::ColorType::Rgb8);

        Ok(())
    }

    #[test]
    fn test_jpeg_options_default() {
        let options = JpegOptions::default();
        assert_eq!(options.quality, 85);
        assert!(!options.progressive);
        assert!(options.optimize_huffman);
    }

    #[test]
    fn test_jpeg_options_presets() {
        let high_quality = JpegOptions::high_quality();
        assert_eq!(high_quality.quality, 95);
        assert!(high_quality.progressive);

        let web_quality = JpegOptions::web_quality();
        assert_eq!(web_quality.quality, 75);
        assert!(web_quality.progressive);
    }

    #[test]
    fn test_quality_clamping() {
        let options = JpegOptions::default().with_quality(150);
        assert_eq!(options.quality, 100);

        let options = JpegOptions::default().with_quality(0);
        assert_eq!(options.quality, 1);
    }

    #[test]
    fn test_save_jpeg_with_options() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_options.jpg");

        // Create a simple test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let options = JpegOptions::high_quality();

        // Save the image with options
        save_jpeg_with_options(&dynamic_img, &file_path, &options)?;

        // Verify file exists
        assert!(file_path.exists());

        Ok(())
    }
}
