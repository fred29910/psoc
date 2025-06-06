//! Rendering module
//!
//! This module provides high-level rendering interfaces and re-exports
//! the core rendering functionality from psoc-core.

// Re-export the core rendering engine
pub use psoc_core::rendering::*;

use crate::core::{Document, PixelData};
use crate::utils::Result;
use tracing::{debug, instrument};

/// High-level rendering utilities for the PSOC application
#[derive(Debug)]
pub struct AppRenderer {
    engine: psoc_core::rendering::RenderEngine,
}

impl Default for AppRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl AppRenderer {
    /// Create a new application renderer
    pub fn new() -> Self {
        Self {
            engine: psoc_core::rendering::RenderEngine::new(),
        }
    }

    /// Create renderer with custom settings
    pub fn with_settings(parallel_enabled: bool, tile_size: u32) -> Self {
        Self {
            engine: psoc_core::rendering::RenderEngine::with_settings(parallel_enabled, tile_size),
        }
    }

    /// Render document for display in the UI
    #[instrument(skip(self, document))]
    pub fn render_for_display(&self, document: &Document) -> Result<PixelData> {
        debug!("Rendering document for display");
        self.engine.render_document(document).map_err(Into::into)
    }

    /// Render document region for viewport display
    #[instrument(skip(self, document))]
    pub fn render_viewport(
        &self,
        document: &Document,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<PixelData> {
        debug!(
            "Rendering viewport region: ({}, {}) {}x{}",
            x, y, width, height
        );
        self.engine
            .render_region(document, x, y, width, height)
            .map_err(Into::into)
    }

    /// Get the underlying render engine
    pub fn engine(&self) -> &psoc_core::rendering::RenderEngine {
        &self.engine
    }
}
