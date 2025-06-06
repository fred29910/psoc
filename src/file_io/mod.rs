//! File input/output module
//!
//! This module provides high-level file I/O operations for the PSOC application,
//! including image import/export and project file management.

use anyhow::Result;
use psoc_core::Document;
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

    /// Load a document from any supported file format (images or projects)
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub async fn load_document<P: AsRef<Path>>(&self, path: P) -> Result<Document> {
        let path = path.as_ref();
        info!("Loading document from: {}", path.display());

        let path_clone = path.to_path_buf();
        let document = tokio::task::spawn_blocking(move || {
            psoc_file_formats::FileIO::load_document(&path_clone)
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn document loading task: {}", e))?
        .map_err(|e| anyhow::anyhow!("Failed to load document: {}", e))?;

        info!(
            layers = document.layers.len(),
            size = format!("{}x{}", document.size.width, document.size.height),
            "Document loaded successfully"
        );

        Ok(document)
    }

    /// Save a document as a project file
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub async fn save_project<P: AsRef<Path>>(&self, document: &Document, path: P) -> Result<()> {
        let path = path.as_ref();
        info!("Saving project to: {}", path.display());

        let document_clone = document.clone();
        let path_clone = path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            psoc_file_formats::FileIO::save_project(&document_clone, &path_clone)
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn project saving task: {}", e))?
        .map_err(|e| anyhow::anyhow!("Failed to save project: {}", e))?;

        debug!("Project saved successfully");
        Ok(())
    }

    /// Export a document as a flattened image
    #[instrument(skip_all, fields(path = %path.as_ref().display()))]
    pub async fn export_flattened<P: AsRef<Path>>(
        &self,
        document: &Document,
        path: P,
    ) -> Result<()> {
        let path = path.as_ref();
        info!("Exporting flattened document to: {}", path.display());

        let document_clone = document.clone();
        let path_clone = path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            psoc_file_formats::FileIO::export_flattened(&document_clone, &path_clone)
        })
        .await
        .map_err(|e| anyhow::anyhow!("Failed to spawn flattened export task: {}", e))?
        .map_err(|e| anyhow::anyhow!("Failed to export flattened document: {}", e))?;

        debug!("Flattened document exported successfully");
        Ok(())
    }

    /// Get supported import file extensions (images and projects)
    pub fn supported_import_extensions() -> Vec<&'static str> {
        psoc_file_formats::ImageIO::all_supported_extensions()
    }

    /// Get supported export file extensions (images only)
    pub fn supported_export_extensions() -> Vec<&'static str> {
        psoc_file_formats::ImageIO::supported_extensions()
    }

    /// Get supported project file extensions
    pub fn supported_project_extensions() -> Vec<&'static str> {
        psoc_file_formats::ImageIO::supported_project_extensions()
    }

    /// Get file filter for import dialogs (all supported files)
    pub fn import_file_filter() -> String {
        psoc_file_formats::ImageIO::all_files_filter()
    }

    /// Get file filter for export dialogs (images only)
    pub fn export_file_filter() -> String {
        psoc_file_formats::ImageIO::image_file_filter()
    }

    /// Get file filter for project file dialogs
    pub fn project_file_filter() -> String {
        psoc_file_formats::ImageIO::project_file_filter()
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}
