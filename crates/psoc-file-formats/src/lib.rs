//! PSOC File Formats - File format support and I/O
//!
//! This crate provides comprehensive file format support for PSOC,
//! including loading and saving various image formats.

use std::path::Path;
use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use tracing::{debug, error, info, instrument};

pub mod png;
pub mod jpeg;

// Re-export commonly used types
pub use png::*;
pub use jpeg::*;

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    Png,
    Jpeg,
}

impl SupportedFormat {
    /// Get the format from a file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            _ => None,
        }
    }

    /// Get the format from a file path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::from_extension)
    }

    /// Get the image format for the image crate
    pub fn to_image_format(self) -> ImageFormat {
        match self {
            Self::Png => ImageFormat::Png,
            Self::Jpeg => ImageFormat::Jpeg,
        }
    }

    /// Get the file extension
    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
        }
    }

    /// Get the MIME type
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
        }
    }
}

/// Image loading and saving functionality
pub struct ImageIO;

impl ImageIO {
    /// Load an image from a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
        let path = path.as_ref();
        debug!("Loading image from: {}", path.display());

        let format = SupportedFormat::from_path(path)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", path.display()))?;

        let image = match format {
            SupportedFormat::Png => png::load_png(path)?,
            SupportedFormat::Jpeg => jpeg::load_jpeg(path)?,
        };

        info!(
            width = image.width(),
            height = image.height(),
            format = ?format,
            "Successfully loaded image"
        );

        Ok(image)
    }

    /// Save an image to a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn save_image<P: AsRef<Path>>(image: &DynamicImage, path: P) -> Result<()> {
        let path = path.as_ref();
        debug!("Saving image to: {}", path.display());

        let format = SupportedFormat::from_path(path)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", path.display()))?;

        match format {
            SupportedFormat::Png => png::save_png(image, path)?,
            SupportedFormat::Jpeg => jpeg::save_jpeg(image, path)?,
        }

        info!(
            width = image.width(),
            height = image.height(),
            format = ?format,
            "Successfully saved image"
        );

        Ok(())
    }

    /// Get supported file extensions
    pub fn supported_extensions() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg"]
    }

    /// Get file filter string for file dialogs
    pub fn file_filter() -> String {
        "Image Files (*.png, *.jpg, *.jpeg)|*.png;*.jpg;*.jpeg".to_string()
    }
}
