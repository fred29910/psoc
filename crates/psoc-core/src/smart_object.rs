//! Smart object management and processing
//!
//! This module provides functionality for managing smart objects, including
//! loading embedded content, applying non-destructive transformations, and
//! caching rendered results for performance.

use crate::{
    geometry::Size,
    layer::{InterpolationQuality, SmartObjectContentType, SmartTransform},
    pixel::PixelData,
};
use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
// use serde::{Deserialize, Serialize}; // For future serialization support
use std::collections::HashMap;
use std::io::Cursor;
// use std::path::PathBuf; // For future file path handling
// use std::time::SystemTime; // For future timestamp tracking
// use uuid::Uuid; // For future unique ID generation

/// Smart object manager for handling embedded content and transformations
#[derive(Debug)]
pub struct SmartObjectManager {
    /// Cache of loaded content by content hash
    content_cache: HashMap<String, DynamicImage>,
    /// Cache of rendered results by parameters hash
    render_cache: HashMap<String, PixelData>,
    /// Maximum cache size (number of entries)
    max_cache_size: usize,
}

impl Default for SmartObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartObjectManager {
    /// Create a new smart object manager
    pub fn new() -> Self {
        Self {
            content_cache: HashMap::new(),
            render_cache: HashMap::new(),
            max_cache_size: 100, // Reasonable default
        }
    }

    /// Create a new smart object manager with custom cache size
    pub fn with_cache_size(max_cache_size: usize) -> Self {
        Self {
            content_cache: HashMap::new(),
            render_cache: HashMap::new(),
            max_cache_size,
        }
    }

    /// Load content from smart object content type
    pub fn load_content(&mut self, content_type: &SmartObjectContentType) -> Result<DynamicImage> {
        match content_type {
            SmartObjectContentType::EmbeddedImage {
                image_data, format, ..
            } => {
                let content_hash = self.calculate_content_hash(image_data);

                // Check cache first
                if let Some(cached_image) = self.content_cache.get(&content_hash) {
                    return Ok(cached_image.clone());
                }

                // Load image from embedded data
                let cursor = Cursor::new(image_data);
                let image_format = self.parse_image_format(format)?;
                let image = image::load(cursor, image_format).with_context(|| {
                    format!("Failed to load embedded image with format: {}", format)
                })?;

                // Cache the loaded image
                self.cache_content(content_hash, image.clone());
                Ok(image)
            }
            SmartObjectContentType::LinkedImage { file_path, .. } => {
                let content_hash = format!("linked:{}", file_path.display());

                // Check cache first
                if let Some(cached_image) = self.content_cache.get(&content_hash) {
                    return Ok(cached_image.clone());
                }

                // Load image from file
                let image = image::open(file_path).with_context(|| {
                    format!("Failed to load linked image: {}", file_path.display())
                })?;

                // Cache the loaded image
                self.cache_content(content_hash, image.clone());
                Ok(image)
            }
            SmartObjectContentType::EmbeddedDocument { document_data } => {
                // For now, we'll treat embedded documents as a placeholder
                // In a full implementation, this would deserialize and render the document
                let content_hash = self.calculate_content_hash(document_data);

                if let Some(cached_image) = self.content_cache.get(&content_hash) {
                    return Ok(cached_image.clone());
                }

                // Create a placeholder image for embedded documents
                let placeholder = self.create_document_placeholder()?;
                self.cache_content(content_hash, placeholder.clone());
                Ok(placeholder)
            }
        }
    }

    /// Render smart object with transformations applied
    pub fn render_smart_object(
        &mut self,
        content_type: &SmartObjectContentType,
        original_size: Size,
        smart_transform: &SmartTransform,
        target_size: Option<Size>,
    ) -> Result<PixelData> {
        // Generate cache key based on parameters
        let cache_key = self.generate_render_cache_key(
            content_type,
            original_size,
            smart_transform,
            target_size,
        );

        // Check render cache first
        if let Some(cached_result) = self.render_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        // Load the original content
        let original_image = self.load_content(content_type)?;

        // Apply smart transformations
        let transformed_image = self.apply_smart_transform(
            original_image,
            original_size,
            smart_transform,
            target_size,
        )?;

        // Convert to PixelData
        let pixel_data = PixelData::from_image(&transformed_image)?;

        // Cache the result
        self.cache_render_result(cache_key, pixel_data.clone());

        Ok(pixel_data)
    }

    /// Apply smart transformations to an image
    fn apply_smart_transform(
        &self,
        mut image: DynamicImage,
        original_size: Size,
        smart_transform: &SmartTransform,
        target_size: Option<Size>,
    ) -> Result<DynamicImage> {
        // Calculate final dimensions
        let (scale_x, scale_y) = if smart_transform.maintain_aspect_ratio {
            let scale = smart_transform.scale.0.min(smart_transform.scale.1);
            (scale, scale)
        } else {
            smart_transform.scale
        };

        let final_width = (original_size.width * scale_x) as u32;
        let final_height = (original_size.height * scale_y) as u32;

        // Apply scaling if needed
        if scale_x != 1.0 || scale_y != 1.0 {
            let filter = match smart_transform.interpolation_quality {
                InterpolationQuality::Nearest => image::imageops::FilterType::Nearest,
                InterpolationQuality::Linear => image::imageops::FilterType::Triangle,
                InterpolationQuality::High => image::imageops::FilterType::Lanczos3,
            };

            image = image.resize(final_width, final_height, filter);
        }

        // Apply rotation if needed
        if smart_transform.rotation != 0.0 {
            // Convert radians to degrees for image crate
            let degrees = smart_transform.rotation.to_degrees();

            // For simplicity, we'll only support 90-degree rotations for now
            // A full implementation would support arbitrary rotations
            let normalized_degrees = (degrees % 360.0 + 360.0) % 360.0;

            if (normalized_degrees - 90.0).abs() < 1.0 {
                image = image.rotate90();
            } else if (normalized_degrees - 180.0).abs() < 1.0 {
                image = image.rotate180();
            } else if (normalized_degrees - 270.0).abs() < 1.0 {
                image = image.rotate270();
            }
            // For other angles, we'd need more complex transformation logic
        }

        // Apply target size constraint if specified
        if let Some(target) = target_size {
            if image.width() != target.width as u32 || image.height() != target.height as u32 {
                let filter = match smart_transform.interpolation_quality {
                    InterpolationQuality::Nearest => image::imageops::FilterType::Nearest,
                    InterpolationQuality::Linear => image::imageops::FilterType::Triangle,
                    InterpolationQuality::High => image::imageops::FilterType::Lanczos3,
                };
                image = image.resize(target.width as u32, target.height as u32, filter);
            }
        }

        Ok(image)
    }

    /// Check if linked file has been modified
    pub fn check_linked_file_update(&self, content_type: &SmartObjectContentType) -> Result<bool> {
        if let SmartObjectContentType::LinkedImage {
            file_path,
            last_modified,
        } = content_type
        {
            if let Some(stored_time) = last_modified {
                let metadata = std::fs::metadata(file_path).with_context(|| {
                    format!("Failed to get metadata for: {}", file_path.display())
                })?;
                let current_time = metadata
                    .modified()
                    .with_context(|| "Failed to get file modification time")?;

                Ok(current_time > *stored_time)
            } else {
                // No stored time means we should check
                Ok(true)
            }
        } else {
            // Non-linked content doesn't need update checking
            Ok(false)
        }
    }

    /// Update linked file modification time
    pub fn update_linked_file_time(&self, content_type: &mut SmartObjectContentType) -> Result<()> {
        if let SmartObjectContentType::LinkedImage {
            file_path,
            last_modified,
        } = content_type
        {
            let metadata = std::fs::metadata(&*file_path)
                .with_context(|| format!("Failed to get metadata for: {}", file_path.display()))?;
            let current_time = metadata
                .modified()
                .with_context(|| "Failed to get file modification time")?;

            *last_modified = Some(current_time);
        }
        Ok(())
    }

    /// Clear all caches
    pub fn clear_caches(&mut self) {
        self.content_cache.clear();
        self.render_cache.clear();
    }

    /// Clear content cache
    pub fn clear_content_cache(&mut self) {
        self.content_cache.clear();
    }

    /// Clear render cache
    pub fn clear_render_cache(&mut self) {
        self.render_cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.content_cache.len(), self.render_cache.len())
    }

    // Private helper methods

    fn calculate_content_hash(&self, data: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn parse_image_format(&self, format_str: &str) -> Result<ImageFormat> {
        match format_str.to_lowercase().as_str() {
            "png" => Ok(ImageFormat::Png),
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "gif" => Ok(ImageFormat::Gif),
            "bmp" => Ok(ImageFormat::Bmp),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            "webp" => Ok(ImageFormat::WebP),
            _ => Err(anyhow::anyhow!("Unsupported image format: {}", format_str)),
        }
    }

    fn create_document_placeholder(&self) -> Result<DynamicImage> {
        // Create a simple placeholder image for embedded documents
        let width = 200;
        let height = 150;
        let mut img = image::RgbImage::new(width, height);

        // Fill with a light gray background
        for pixel in img.pixels_mut() {
            *pixel = image::Rgb([240, 240, 240]);
        }

        Ok(DynamicImage::ImageRgb8(img))
    }

    fn generate_render_cache_key(
        &self,
        content_type: &SmartObjectContentType,
        original_size: Size,
        smart_transform: &SmartTransform,
        target_size: Option<Size>,
    ) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Hash content type identifier
        match content_type {
            SmartObjectContentType::EmbeddedImage { image_data, .. } => {
                "embedded".hash(&mut hasher);
                self.calculate_content_hash(image_data).hash(&mut hasher);
            }
            SmartObjectContentType::LinkedImage { file_path, .. } => {
                "linked".hash(&mut hasher);
                file_path.hash(&mut hasher);
            }
            SmartObjectContentType::EmbeddedDocument { document_data } => {
                "document".hash(&mut hasher);
                self.calculate_content_hash(document_data).hash(&mut hasher);
            }
        }

        // Hash transformation parameters
        original_size.width.to_bits().hash(&mut hasher);
        original_size.height.to_bits().hash(&mut hasher);
        smart_transform.scale.0.to_bits().hash(&mut hasher);
        smart_transform.scale.1.to_bits().hash(&mut hasher);
        smart_transform.rotation.to_bits().hash(&mut hasher);
        smart_transform.translation.x.to_bits().hash(&mut hasher);
        smart_transform.translation.y.to_bits().hash(&mut hasher);
        smart_transform.maintain_aspect_ratio.hash(&mut hasher);

        if let Some(target) = target_size {
            target.width.to_bits().hash(&mut hasher);
            target.height.to_bits().hash(&mut hasher);
        }

        format!("{:x}", hasher.finish())
    }

    fn cache_content(&mut self, key: String, image: DynamicImage) {
        if self.content_cache.len() >= self.max_cache_size {
            // Simple LRU: remove first entry (in a real implementation, we'd use a proper LRU)
            if let Some(first_key) = self.content_cache.keys().next().cloned() {
                self.content_cache.remove(&first_key);
            }
        }
        self.content_cache.insert(key, image);
    }

    fn cache_render_result(&mut self, key: String, result: PixelData) {
        if self.render_cache.len() >= self.max_cache_size {
            // Simple LRU: remove first entry
            if let Some(first_key) = self.render_cache.keys().next().cloned() {
                self.render_cache.remove(&first_key);
            }
        }
        self.render_cache.insert(key, result);
    }
}
