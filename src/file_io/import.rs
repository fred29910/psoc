//! Image import functionality
//!
//! This module handles importing images from various file formats.

use std::path::Path;
use anyhow::{Context, Result};
use image::DynamicImage;
use tracing::{debug, error, info, instrument, warn};

/// Import an image from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub async fn import_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
    let path = path.as_ref();
    debug!("Starting image import from: {}", path.display());

    // Validate file exists
    if !path.exists() {
        return Err(anyhow::anyhow!("File does not exist: {}", path.display()));
    }

    // Validate file is readable
    if !path.is_file() {
        return Err(anyhow::anyhow!("Path is not a file: {}", path.display()));
    }

    // Get file size for logging
    let file_size = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    debug!(
        file_size = file_size,
        "File validation passed, loading image"
    );

    // Use tokio to run the blocking I/O operation
    let path_clone = path.to_path_buf();
    let image = tokio::task::spawn_blocking(move || {
        // Use the psoc-file-formats crate for actual loading
        psoc_file_formats::ImageIO::load_image(&path_clone)
    })
    .await
    .context("Failed to spawn image loading task")?
    .context("Failed to load image")?;

    info!(
        width = image.width(),
        height = image.height(),
        color_type = ?image.color(),
        file_size = file_size,
        "Image imported successfully"
    );

    Ok(image)
}

/// Import multiple images from file paths
#[instrument(skip_all)]
pub async fn import_images<P: AsRef<Path>>(paths: Vec<P>) -> Result<Vec<(String, DynamicImage)>> {
    debug!("Importing {} images", paths.len());

    let mut results = Vec::new();
    let mut errors = Vec::new();

    for path in paths {
        let path = path.as_ref();
        let path_str = path.display().to_string();

        match import_image(path).await {
            Ok(image) => {
                results.push((path_str, image));
            }
            Err(e) => {
                error!(
                    path = %path.display(),
                    error = %e,
                    "Failed to import image"
                );
                errors.push((path_str, e));
            }
        }
    }

    if !errors.is_empty() {
        warn!(
            successful = results.len(),
            failed = errors.len(),
            "Some images failed to import"
        );
    }

    if results.is_empty() {
        return Err(anyhow::anyhow!("No images were successfully imported"));
    }

    info!(
        imported = results.len(),
        "Batch image import completed"
    );

    Ok(results)
}

/// Check if a file extension is supported for import
pub fn is_supported_import_extension<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let supported_extensions = ["png", "jpg", "jpeg"];
        supported_extensions.iter().any(|&supported| {
            ext.to_lowercase() == supported
        })
    } else {
        false
    }
}

/// Validate if a file can be imported (checks both extension and file existence)
pub fn can_import<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    // Check if file exists and is readable
    if !path.exists() || !path.is_file() {
        return false;
    }

    // Check if extension is supported
    is_supported_import_extension(path)
}

/// Get image metadata without fully loading the image
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub async fn get_image_metadata<P: AsRef<Path>>(path: P) -> Result<ImageMetadata> {
    let path = path.as_ref();
    debug!("Getting image metadata from: {}", path.display());

    if !can_import(path) {
        return Err(anyhow::anyhow!("Cannot import file: {}", path.display()));
    }

    let path_clone = path.to_path_buf();
    let metadata = tokio::task::spawn_blocking(move || {
        // Use image crate to get basic info without fully decoding
        let reader = image::io::Reader::open(&path_clone)?;
        let reader = reader.with_guessed_format()?;
        
        let dimensions = reader.into_dimensions()?;
        let file_size = std::fs::metadata(&path_clone)?.len();

        Ok::<ImageMetadata, anyhow::Error>(ImageMetadata {
            width: dimensions.0,
            height: dimensions.1,
            file_size,
            format: psoc_file_formats::SupportedFormat::from_path(&path_clone),
        })
    })
    .await
    .context("Failed to spawn metadata reading task")?
    .context("Failed to read image metadata")?;

    debug!(
        width = metadata.width,
        height = metadata.height,
        file_size = metadata.file_size,
        format = ?metadata.format,
        "Image metadata retrieved"
    );

    Ok(metadata)
}

/// Image metadata information
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub file_size: u64,
    pub format: Option<psoc_file_formats::SupportedFormat>,
}

impl ImageMetadata {
    /// Get a human-readable description of the image
    pub fn description(&self) -> String {
        let format_str = self.format
            .map(|f| f.extension().to_uppercase())
            .unwrap_or_else(|| "Unknown".to_string());
        
        let size_str = if self.file_size < 1024 {
            format!("{} B", self.file_size)
        } else if self.file_size < 1024 * 1024 {
            format!("{:.1} KB", self.file_size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", self.file_size as f64 / (1024.0 * 1024.0))
        };

        format!(
            "{} × {} pixels, {} format, {}",
            self.width, self.height, format_str, size_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_import_image() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.png");

        // Create and save a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = DynamicImage::ImageRgb8(img);
        dynamic_img.save(&file_path)?;

        // Import the image
        let imported = import_image(&file_path).await?;

        assert_eq!(imported.width(), 100);
        assert_eq!(imported.height(), 100);

        Ok(())
    }

    #[test]
    fn test_is_supported_import_extension() {
        // Test with supported extensions
        assert!(is_supported_import_extension("test.png"));
        assert!(is_supported_import_extension("test.jpg"));
        assert!(is_supported_import_extension("test.jpeg"));
        assert!(is_supported_import_extension("test.PNG"));
        assert!(is_supported_import_extension("test.JPG"));

        // Test with unsupported extensions
        assert!(!is_supported_import_extension("test.txt"));
        assert!(!is_supported_import_extension("test.pdf"));
        assert!(!is_supported_import_extension("test"));
    }

    #[tokio::test]
    async fn test_get_image_metadata() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.png");

        // Create and save a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(200, 150);
        let dynamic_img = DynamicImage::ImageRgb8(img);
        dynamic_img.save(&file_path)?;

        // Get metadata
        let metadata = get_image_metadata(&file_path).await?;

        assert_eq!(metadata.width, 200);
        assert_eq!(metadata.height, 150);
        assert!(metadata.file_size > 0);
        assert!(metadata.format.is_some());

        Ok(())
    }
}
