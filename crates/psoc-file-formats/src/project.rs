//! PSOC Project File Format Support
//!
//! This module provides support for loading and saving PSOC project files (.psoc)
//! which contain multi-layer documents with full layer information, metadata,
//! and document settings.

use anyhow::{Context, Result};
use psoc_core::Document;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info, instrument};

/// PSOC project file format version
pub const PROJECT_FORMAT_VERSION: &str = "1.0";

/// PSOC project file extension
pub const PROJECT_FILE_EXTENSION: &str = "psoc";

/// Project file container structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    /// Format version for compatibility checking
    pub version: String,
    /// Project metadata
    pub metadata: ProjectMetadata,
    /// The document data
    pub document: Document,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Application version that created this file
    pub created_with: String,
    /// Creation timestamp (ISO 8601 format)
    pub created_at: String,
    /// Last modification timestamp (ISO 8601 format)
    pub modified_at: String,
    /// Optional project description
    pub description: Option<String>,
    /// Optional project tags
    pub tags: Vec<String>,
}

impl ProjectFile {
    /// Create a new project file from a document
    pub fn new(document: Document) -> Self {
        let now = chrono::Utc::now().to_rfc3339();

        Self {
            version: PROJECT_FORMAT_VERSION.to_string(),
            metadata: ProjectMetadata {
                created_with: format!("PSOC v{}", env!("CARGO_PKG_VERSION")),
                created_at: now.clone(),
                modified_at: now,
                description: None,
                tags: Vec::new(),
            },
            document,
        }
    }

    /// Update the modification timestamp
    pub fn touch(&mut self) {
        self.metadata.modified_at = chrono::Utc::now().to_rfc3339();
    }

    /// Check if the project file version is compatible
    pub fn is_compatible(&self) -> bool {
        // For now, only support exact version match
        // In the future, we can implement version compatibility logic
        self.version == PROJECT_FORMAT_VERSION
    }
}

/// Load a PSOC project file
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_project<P: AsRef<Path>>(path: P) -> Result<Document> {
    let path = path.as_ref();
    debug!("Loading PSOC project from: {}", path.display());

    // Read the file content
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read project file: {}", path.display()))?;

    // Parse the RON format
    let project_file: ProjectFile = ron::from_str(&content)
        .with_context(|| format!("Failed to parse project file: {}", path.display()))?;

    // Check version compatibility
    if !project_file.is_compatible() {
        return Err(anyhow::anyhow!(
            "Incompatible project file version: {} (expected: {})",
            project_file.version,
            PROJECT_FORMAT_VERSION
        ));
    }

    info!(
        version = %project_file.version,
        created_with = %project_file.metadata.created_with,
        layers = project_file.document.layers.len(),
        "Successfully loaded PSOC project"
    );

    Ok(project_file.document)
}

/// Save a document as a PSOC project file
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn save_project<P: AsRef<Path>>(document: &Document, path: P) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving PSOC project to: {}", path.display());

    // Create project file structure
    let mut project_file = ProjectFile::new(document.clone());
    project_file.touch();

    // Serialize to RON format with pretty printing
    let content = ron::ser::to_string_pretty(&project_file, ron::ser::PrettyConfig::default())
        .context("Failed to serialize project file")?;

    // Ensure the target directory exists
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }

    // Write the file
    std::fs::write(path, content)
        .with_context(|| format!("Failed to write project file: {}", path.display()))?;

    info!(
        layers = document.layers.len(),
        size = format!("{}x{}", document.size.width, document.size.height),
        "Successfully saved PSOC project"
    );

    Ok(())
}

/// Check if a file path has the PSOC project extension
pub fn is_project_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case(PROJECT_FILE_EXTENSION))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use psoc_core::{Layer, RgbaPixel};
    use tempfile::NamedTempFile;

    #[test]
    fn test_project_file_creation() {
        let document = Document::new("Test Project".to_string(), 100, 100);
        let project_file = ProjectFile::new(document);

        assert_eq!(project_file.version, PROJECT_FORMAT_VERSION);
        assert_eq!(project_file.document.metadata.title, "Test Project");
        assert!(project_file.is_compatible());
    }

    #[test]
    fn test_project_file_extension_check() {
        assert!(is_project_file("test.psoc"));
        assert!(is_project_file("test.PSOC"));
        assert!(!is_project_file("test.png"));
        assert!(!is_project_file("test"));
    }

    #[test]
    fn test_save_and_load_project() -> Result<()> {
        // Create a test document with layers
        let mut document = Document::new("Test Project".to_string(), 200, 150);

        let mut layer1 = Layer::new_pixel("Background".to_string(), 200, 150);
        layer1.fill(RgbaPixel::new(255, 0, 0, 255)); // Red background

        let mut layer2 = Layer::new_pixel("Foreground".to_string(), 100, 75);
        layer2.fill(RgbaPixel::new(0, 255, 0, 128)); // Semi-transparent green
        layer2.opacity = 0.8;

        document.add_layer(layer1);
        document.add_layer(layer2);

        // Save to temporary file
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().with_extension("psoc");

        save_project(&document, &temp_path)?;

        // Load back and verify
        let loaded_document = load_project(&temp_path)?;

        assert_eq!(loaded_document.metadata.title, "Test Project");
        assert_eq!(loaded_document.layers.len(), 2);
        assert_eq!(loaded_document.layers[0].name, "Background");
        assert_eq!(loaded_document.layers[1].name, "Foreground");
        assert_eq!(loaded_document.layers[1].opacity, 0.8);

        Ok(())
    }

    #[test]
    fn test_project_file_roundtrip_with_complex_document() -> Result<()> {
        // Create a complex document
        let mut document = Document::new("Complex Project".to_string(), 300, 200);
        document.metadata.description = Some("A test project with multiple layers".to_string());

        // Add multiple layers with different properties
        for i in 0..5 {
            let mut layer = Layer::new_pixel(format!("Layer {}", i + 1), 100, 100);
            layer.opacity = 0.2 * (i + 1) as f32;
            layer.visible = i % 2 == 0; // Alternate visibility
            layer.fill(RgbaPixel::new(
                (i * 50) as u8,
                ((4 - i) * 50) as u8,
                128,
                255,
            ));
            document.add_layer(layer);
        }

        // Save and load
        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().with_extension("psoc");

        save_project(&document, &temp_path)?;
        let loaded_document = load_project(&temp_path)?;

        // Verify all properties are preserved
        assert_eq!(loaded_document.layers.len(), 5);
        for (i, layer) in loaded_document.layers.iter().enumerate() {
            assert_eq!(layer.name, format!("Layer {}", i + 1));
            assert_eq!(layer.opacity, 0.2 * (i + 1) as f32);
            assert_eq!(layer.visible, i % 2 == 0);
        }

        Ok(())
    }
}
