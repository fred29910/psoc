//! File input/output module
//!
//! This module provides high-level file I/O operations for the PSOC application,
//! including image import/export and project file management.

use anyhow::Result;
use psoc_file_formats::DynamicImage;
use std::path::Path;
use tracing::{debug, info, instrument};

pub mod export;
pub mod import;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use export::*;
pub use import::*;

/// File I/O manager for the application
#[derive(Debug, Clone)]
pub struct FileManager;

impl FileManager {
    /// Create a new file manager
    pub fn new() -> Self {
        Self
    }

    /// Import an image from a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub async fn import_image<P: AsRef<Path>>(&self, path: P) -> Result<DynamicImage> {
        let path = path.as_ref();
        info!("Importing image from: {}", path.display());

        let image = import::import_image(path).await?;

        debug!(
            width = image.width(),
            height = image.height(),
            "Image imported successfully"
        );

        Ok(image)
    }

    /// Export an image to a file path
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub async fn export_image<P: AsRef<Path>>(&self, image: &DynamicImage, path: P) -> Result<()> {
        let path = path.as_ref();
        info!("Exporting image to: {}", path.display());

        export::export_image(image, path).await?;

        debug!("Image exported successfully");
        Ok(())
    }

    /// Get supported import file extensions
    pub fn supported_import_extensions() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg"]
    }

    /// Get supported export file extensions
    pub fn supported_export_extensions() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg"]
    }

    /// Get file filter for import dialogs
    pub fn import_file_filter() -> String {
        "Image Files|*.png;*.jpg;*.jpeg|PNG Files|*.png|JPEG Files|*.jpg;*.jpeg|All Files|*.*"
            .to_string()
    }

    /// Get file filter for export dialogs
    pub fn export_file_filter() -> String {
        "PNG Files|*.png|JPEG Files|*.jpg;*.jpeg|All Files|*.*".to_string()
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}
