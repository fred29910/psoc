//! PSOC File Formats - File format support and I/O
//!
//! This crate provides comprehensive file format support for PSOC,
//! including loading and saving various image formats.

use anyhow::Result;
use std::path::Path;
use tracing::{debug, info, instrument};

pub mod jpeg;
pub mod png;
pub mod project;

// Re-export commonly used types
pub use jpeg::*;
pub use png::*;
pub use project::*;

// Re-export image types for convenience
pub use image::{DynamicImage, ImageFormat};

// Re-export ICC profile types
pub use psoc_core::{ColorManager, IccProfile};

/// Image loading result with optional ICC profile
#[derive(Debug)]
pub struct ImageLoadResult {
    /// The loaded image
    pub image: image::DynamicImage,
    /// Embedded ICC profile, if any
    pub icc_profile: Option<IccProfile>,
}

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    Png,
    Jpeg,
}

/// All supported file formats (images and projects)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    /// PNG image format
    Png,
    /// JPEG image format
    Jpeg,
    /// PSOC project format
    Project,
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
    pub fn to_image_format(self) -> image::ImageFormat {
        match self {
            Self::Png => image::ImageFormat::Png,
            Self::Jpeg => image::ImageFormat::Jpeg,
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

impl FileFormat {
    /// Get the format from a file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "psoc" => Some(Self::Project),
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

    /// Get the file extension
    pub fn extension(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Project => "psoc",
        }
    }

    /// Check if this is an image format
    pub fn is_image(self) -> bool {
        matches!(self, Self::Png | Self::Jpeg)
    }

    /// Check if this is a project format
    pub fn is_project(self) -> bool {
        matches!(self, Self::Project)
    }

    /// Get the MIME type
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Project => "application/x-psoc-project",
        }
    }
}

/// Image loading and saving functionality
pub struct ImageIO;

impl ImageIO {
    /// Load an image from a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn load_image<P: AsRef<Path>>(path: P) -> Result<image::DynamicImage> {
        let result = Self::load_image_with_profile(path)?;
        Ok(result.image)
    }

    /// Load an image with ICC profile from a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn load_image_with_profile<P: AsRef<Path>>(path: P) -> Result<ImageLoadResult> {
        let path = path.as_ref();
        debug!("Loading image with profile from: {}", path.display());

        let format = SupportedFormat::from_path(path)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", path.display()))?;

        let result = match format {
            SupportedFormat::Png => {
                let png_result = png::load_png_with_profile(path)?;
                ImageLoadResult {
                    image: png_result.image,
                    icc_profile: png_result.icc_profile,
                }
            }
            SupportedFormat::Jpeg => {
                let jpeg_result = jpeg::load_jpeg_with_profile(path)?;
                ImageLoadResult {
                    image: jpeg_result.image,
                    icc_profile: jpeg_result.icc_profile,
                }
            }
        };

        info!(
            width = result.image.width(),
            height = result.image.height(),
            format = ?format,
            has_profile = result.icc_profile.is_some(),
            "Successfully loaded image"
        );

        Ok(result)
    }

    /// Save an image to a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn save_image<P: AsRef<Path>>(image: &image::DynamicImage, path: P) -> Result<()> {
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

    /// Get supported image file extensions
    pub fn supported_extensions() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg"]
    }

    /// Get supported project file extensions
    pub fn supported_project_extensions() -> Vec<&'static str> {
        vec!["psoc"]
    }

    /// Get all supported file extensions
    pub fn all_supported_extensions() -> Vec<&'static str> {
        let mut extensions = Self::supported_extensions();
        extensions.extend(Self::supported_project_extensions());
        extensions
    }

    /// Get file filter string for image file dialogs
    pub fn image_file_filter() -> String {
        "Image Files (*.png, *.jpg, *.jpeg)|*.png;*.jpg;*.jpeg".to_string()
    }

    /// Get file filter string for project file dialogs
    pub fn project_file_filter() -> String {
        "PSOC Project Files (*.psoc)|*.psoc".to_string()
    }

    /// Get file filter string for all supported files
    pub fn all_files_filter() -> String {
        "All Supported Files (*.png, *.jpg, *.jpeg, *.psoc)|*.png;*.jpg;*.jpeg;*.psoc|Image Files (*.png, *.jpg, *.jpeg)|*.png;*.jpg;*.jpeg|PSOC Project Files (*.psoc)|*.psoc".to_string()
    }
}

/// Unified file I/O for both images and projects
pub struct FileIO;

impl FileIO {
    /// Load a document from any supported file format
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn load_document<P: AsRef<Path>>(path: P) -> Result<psoc_core::Document> {
        use psoc_core::Document;

        let path = path.as_ref();
        debug!("Loading document from: {}", path.display());

        let format = FileFormat::from_path(path)
            .ok_or_else(|| anyhow::anyhow!("Unsupported file format: {}", path.display()))?;

        let document = match format {
            FileFormat::Project => project::load_project(path)?,
            FileFormat::Png | FileFormat::Jpeg => {
                // Load as image with ICC profile and convert to document
                let result = ImageIO::load_image_with_profile(path)?;
                let title = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Untitled")
                    .to_string();
                Document::from_image_with_profile(title, &result.image, result.icc_profile)?
            }
        };

        info!(
            format = ?format,
            layers = document.layers.len(),
            size = format!("{}x{}", document.size.width, document.size.height),
            "Successfully loaded document"
        );

        Ok(document)
    }

    /// Save a document to a project file
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn save_project<P: AsRef<Path>>(document: &psoc_core::Document, path: P) -> Result<()> {
        project::save_project(document, path)
    }

    /// Export a document as a flattened image
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub fn export_flattened<P: AsRef<Path>>(document: &psoc_core::Document, path: P) -> Result<()> {
        use psoc_core::RenderEngine;

        let path = path.as_ref();
        debug!("Exporting flattened document to: {}", path.display());

        // Render the document to a single image
        let render_engine = RenderEngine::new();
        let pixel_data = render_engine.render_document(document)?;

        // Convert to DynamicImage
        let image = pixel_data.to_image()?;

        // Save using ImageIO
        ImageIO::save_image(&image, path)?;

        info!(
            layers = document.layers.len(),
            size = format!("{}x{}", document.size.width, document.size.height),
            "Successfully exported flattened document"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_load_result_creation() {
        use image::{ImageBuffer, Rgb};

        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(40, 40);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let result = ImageLoadResult {
            image: dynamic_img,
            icc_profile: None,
        };

        assert_eq!(result.image.width(), 40);
        assert_eq!(result.image.height(), 40);
        assert!(result.icc_profile.is_none());
    }

    #[test]
    fn test_load_image_with_profile_compatibility() {
        use image::{ImageBuffer, Rgb};
        use tempfile::tempdir;

        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test_compat.png");

        // Create and save a test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(35, 35);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);
        png::save_png(&dynamic_img, &file_path).unwrap();

        // Test that both methods work
        let image1 = ImageIO::load_image(&file_path).unwrap();
        let result2 = ImageIO::load_image_with_profile(&file_path).unwrap();

        assert_eq!(image1.width(), result2.image.width());
        assert_eq!(image1.height(), result2.image.height());
        assert!(result2.icc_profile.is_none()); // No profile in test image
    }
}
