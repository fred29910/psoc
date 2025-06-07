//! JPEG format support
//!
//! This module provides JPEG image loading and saving functionality with ICC profile support.

use anyhow::{Context, Result};
use psoc_core::{ColorManager, IccProfile};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use tracing::{debug, instrument, warn};

/// JPEG loading result with optional ICC profile
#[derive(Debug)]
pub struct JpegLoadResult {
    /// The loaded image
    pub image: image::DynamicImage,
    /// Embedded ICC profile, if any
    pub icc_profile: Option<IccProfile>,
}

/// Load a JPEG image from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_jpeg<P: AsRef<Path>>(path: P) -> Result<image::DynamicImage> {
    let result = load_jpeg_with_profile(path)?;
    Ok(result.image)
}

/// Load a JPEG image with ICC profile from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_jpeg_with_profile<P: AsRef<Path>>(path: P) -> Result<JpegLoadResult> {
    let path = path.as_ref();
    debug!("Loading JPEG image with profile from: {}", path.display());

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

    // Try to extract ICC profile from JPEG file
    let icc_profile = extract_jpeg_icc_profile(path)?;

    Ok(JpegLoadResult { image, icc_profile })
}

/// Extract ICC profile from JPEG file
fn extract_jpeg_icc_profile<P: AsRef<Path>>(path: P) -> Result<Option<IccProfile>> {
    let path = path.as_ref();

    let file = File::open(path)
        .with_context(|| format!("Failed to open JPEG file: {}", path.display()))?;
    let mut reader = BufReader::new(file);

    // Read JPEG SOI marker
    let mut marker = [0u8; 2];
    reader
        .read_exact(&mut marker)
        .context("Failed to read JPEG SOI marker")?;

    if marker != [0xFF, 0xD8] {
        return Ok(None); // Not a valid JPEG file
    }

    // Read segments to find APP2 segments with ICC profile
    let mut profile_chunks = Vec::new();
    let mut total_chunks = 0u8;
    let mut current_chunk = 0u8;

    loop {
        let mut segment_marker = [0u8; 2];
        if reader.read_exact(&mut segment_marker).is_err() {
            break; // End of file
        }

        if segment_marker[0] != 0xFF {
            break; // Invalid JPEG structure
        }

        let marker_type = segment_marker[1];

        // Check for SOS (Start of Scan) - image data begins
        if marker_type == 0xDA {
            break;
        }

        // Read segment length
        let mut length_bytes = [0u8; 2];
        reader
            .read_exact(&mut length_bytes)
            .context("Failed to read segment length")?;
        let length = u16::from_be_bytes(length_bytes) as usize;

        if length < 2 {
            break; // Invalid segment length
        }

        if marker_type == 0xE2 {
            // APP2 segment - might contain ICC profile
            let mut segment_data = vec![0u8; length - 2];
            reader
                .read_exact(&mut segment_data)
                .context("Failed to read APP2 segment data")?;

            // Check for ICC profile identifier
            if segment_data.len() >= 12 && &segment_data[0..12] == b"ICC_PROFILE\0" {
                if segment_data.len() >= 14 {
                    current_chunk = segment_data[12];
                    total_chunks = segment_data[13];

                    debug!("Found ICC profile chunk {}/{}", current_chunk, total_chunks);

                    let profile_data = segment_data[14..].to_vec();
                    profile_chunks.push((current_chunk, profile_data));
                }
            }
        } else {
            // Skip other segments
            reader
                .seek(SeekFrom::Current((length - 2) as i64))
                .context("Failed to skip segment")?;
        }
    }

    if profile_chunks.is_empty() {
        return Ok(None);
    }

    // Sort chunks by chunk number and concatenate
    profile_chunks.sort_by_key(|(chunk_num, _)| *chunk_num);

    let mut complete_profile = Vec::new();
    for (_, chunk_data) in profile_chunks {
        complete_profile.extend_from_slice(&chunk_data);
    }

    debug!(
        "Extracted ICC profile with {} bytes from {} chunks",
        complete_profile.len(),
        total_chunks
    );

    // Create ICC profile using ColorManager
    let mut color_manager = ColorManager::new().context("Failed to create color manager")?;

    let icc_profile = color_manager
        .load_profile_from_data(&complete_profile, "JPEG Embedded Profile".to_string())
        .context("Failed to load ICC profile from data")?;

    Ok(Some(icc_profile))
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
    /// ICC profile to embed (optional)
    pub icc_profile: Option<IccProfile>,
}

impl Default for JpegOptions {
    fn default() -> Self {
        Self {
            quality: 85,        // Good balance between quality and file size
            progressive: false, // Standard baseline JPEG
            optimize_huffman: true,
            icc_profile: None,
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
            icc_profile: None,
        }
    }

    /// Create low quality JPEG options for web use
    pub fn web_quality() -> Self {
        Self {
            quality: 75,
            progressive: true,
            optimize_huffman: true,
            icc_profile: None,
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

    if options.icc_profile.is_some() {
        // Use custom JPEG encoding with ICC profile
        save_jpeg_with_icc_profile(&image, path, options)
    } else {
        // Use standard save method
        image
            .save_with_format(path, image::ImageFormat::Jpeg)
            .with_context(|| format!("Failed to save JPEG image to: {}", path.display()))?;
        Ok(())
    }
}

/// Save a JPEG image with embedded ICC profile
fn save_jpeg_with_icc_profile<P: AsRef<Path>>(
    image: &image::DynamicImage,
    path: P,
    _options: &JpegOptions,
) -> Result<()> {
    let path = path.as_ref();

    // For now, save without ICC profile and log a warning
    // TODO: Implement custom JPEG encoding with ICC profile embedding
    warn!("ICC profile embedding in JPEG not yet implemented, saving without profile");

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

    #[test]
    fn test_jpeg_load_result_creation() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(15, 15);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let result = JpegLoadResult {
            image: dynamic_img,
            icc_profile: None,
        };

        assert_eq!(result.image.width(), 15);
        assert_eq!(result.image.height(), 15);
        assert!(result.icc_profile.is_none());
    }

    #[test]
    fn test_jpeg_options_with_profile() {
        let options = JpegOptions::default();
        assert!(options.icc_profile.is_none());

        let high_quality = JpegOptions::high_quality();
        assert!(high_quality.icc_profile.is_none());

        let web_quality = JpegOptions::web_quality();
        assert!(web_quality.icc_profile.is_none());
    }

    #[test]
    fn test_load_jpeg_with_profile_fallback() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_no_profile.jpg");

        // Create and save a simple test image without profile
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(25, 25);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);
        save_jpeg(&dynamic_img, &file_path)?;

        // Load with profile function - should work even without embedded profile
        let result = load_jpeg_with_profile(&file_path)?;

        assert_eq!(result.image.width(), 25);
        assert_eq!(result.image.height(), 25);
        assert!(result.icc_profile.is_none()); // No profile embedded

        Ok(())
    }

    #[test]
    fn test_extract_jpeg_icc_profile_invalid_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("not_a_jpeg.txt");

        // Create a non-JPEG file
        std::fs::write(&file_path, "This is not a JPEG file").unwrap();

        let result = extract_jpeg_icc_profile(&file_path);
        // Should not panic and should return Ok(None) for invalid JPEG
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_jpeg_options_quality_validation() {
        let options = JpegOptions::default().with_quality(150);
        assert_eq!(options.quality, 100); // Should be clamped to 100

        let options = JpegOptions::default().with_quality(0);
        assert_eq!(options.quality, 1); // Should be clamped to 1

        let options = JpegOptions::default().with_quality(85);
        assert_eq!(options.quality, 85); // Should remain unchanged
    }
}
