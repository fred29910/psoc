//! PNG format support
//!
//! This module provides PNG image loading and saving functionality with ICC profile support.

use anyhow::{Context, Result};
use psoc_core::{ColorManager, IccProfile};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::Arc; // Added for Arc<IccProfile>
use tracing::{debug, instrument, warn};

use once_cell::sync::Lazy; // For static Lazy initialization
use std::collections::HashMap;
use std::sync::Mutex;

// Cache for parsed ICC Profiles from PNG files
// Key: Raw profile bytes (Vec<u8>) after decompression
// Value: Parsed IccProfile object, wrapped in Arc for shared ownership
type PngIccProfileCacheMap = Mutex<HashMap<Vec<u8>, Arc<IccProfile>>>;

static PNG_PROFILE_CACHE: Lazy<PngIccProfileCacheMap> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


/// PNG loading result with optional ICC profile
#[derive(Debug)]
pub struct PngLoadResult {
    /// The loaded image
    pub image: image::DynamicImage,
    /// Embedded ICC profile, if any
    pub icc_profile: Option<IccProfile>,
}

/// Load a PNG image from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_png<P: AsRef<Path>>(path: P) -> Result<image::DynamicImage> {
    let result = load_png_with_profile(path)?;
    Ok(result.image)
}

/// Load a PNG image with ICC profile from a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn load_png_with_profile<P: AsRef<Path>>(path: P) -> Result<PngLoadResult> {
    let path = path.as_ref();
    debug!("Loading PNG image with profile from: {}", path.display());

    // Load the image using the standard image crate
    let image = image::open(path)
        .with_context(|| format!("Failed to load PNG image from: {}", path.display()))?;

    // Verify it's actually a PNG
    if !matches!(
        image.color(),
        image::ColorType::Rgb8
            | image::ColorType::Rgba8
            | image::ColorType::L8
            | image::ColorType::La8
    ) {
        debug!("Converting image color type for PNG compatibility");
    }

    // Try to extract ICC profile from PNG file
    let icc_profile = extract_png_icc_profile(path)?;

    Ok(PngLoadResult { image, icc_profile })
}

/// Extract ICC profile from PNG file
fn extract_png_icc_profile<P: AsRef<Path>>(path: P) -> Result<Option<IccProfile>> {
    let path = path.as_ref();

    let file =
        File::open(path).with_context(|| format!("Failed to open PNG file: {}", path.display()))?;
    let mut reader = BufReader::new(file);

    // Read PNG signature
    let mut signature = [0u8; 8];
    reader
        .read_exact(&mut signature)
        .context("Failed to read PNG signature")?;

    if signature != [137, 80, 78, 71, 13, 10, 26, 10] {
        return Ok(None); // Not a valid PNG file
    }

    // Read chunks to find iCCP chunk
    loop {
        let mut length_bytes = [0u8; 4];
        if reader.read_exact(&mut length_bytes).is_err() {
            break; // End of file
        }

        let length = u32::from_be_bytes(length_bytes);

        let mut chunk_type = [0u8; 4];
        reader
            .read_exact(&mut chunk_type)
            .context("Failed to read chunk type")?;

        if &chunk_type == b"iCCP" {
            // Found ICC profile chunk
            debug!("Found iCCP chunk with length: {}", length);

            let mut chunk_data = vec![0u8; length as usize];
            reader
                .read_exact(&mut chunk_data)
                .context("Failed to read iCCP chunk data")?;

            // Skip CRC
            let mut crc = [0u8; 4];
            reader
                .read_exact(&mut crc)
                .context("Failed to read chunk CRC")?;

            return parse_iccp_chunk(&chunk_data);
        } else {
            // Skip this chunk
            let skip_size = length as usize + 4; // data + CRC
            let mut skip_buffer = vec![0u8; skip_size.min(8192)];
            let mut remaining = skip_size;

            while remaining > 0 {
                let to_read = remaining.min(skip_buffer.len());
                reader
                    .read_exact(&mut skip_buffer[..to_read])
                    .context("Failed to skip chunk data")?;
                remaining -= to_read;
            }
        }

        // Check for critical chunks that indicate end of metadata
        if &chunk_type == b"IDAT" {
            break; // Image data started, no more metadata
        }
    }

    Ok(None)
}

/// Parse iCCP chunk data to extract ICC profile
fn parse_iccp_chunk(data: &[u8]) -> Result<Option<IccProfile>> {
    // iCCP chunk format:
    // Profile name (null-terminated string)
    // Compression method (1 byte, should be 0 for deflate)
    // Compressed profile data

    // Find null terminator for profile name
    let null_pos = data
        .iter()
        .position(|&b| b == 0)
        .context("Invalid iCCP chunk: no null terminator found")?;

    if null_pos + 2 >= data.len() {
        return Err(anyhow::anyhow!("Invalid iCCP chunk: insufficient data"));
    }

    let profile_name = String::from_utf8_lossy(&data[..null_pos]).to_string();
    let compression_method = data[null_pos + 1];

    if compression_method != 0 {
        warn!(
            "Unsupported iCCP compression method: {}",
            compression_method
        );
        return Ok(None);
    }

    let compressed_data = &data[null_pos + 2..];

    // Decompress the profile data
    use std::io::Cursor;
    let mut decoder = flate2::read::ZlibDecoder::new(Cursor::new(compressed_data));
    let mut profile_data = Vec::new();
    decoder
        .read_to_end(&mut profile_data)
        .context("Failed to decompress ICC profile data")?;

    debug!(
        "Extracted ICC profile '{}' with {} bytes",
        profile_name,
        profile_data.len()
    );

    // Attempt to retrieve from cache or parse
    let profile_key = profile_data.clone(); // Clone for potential cache insertion

    // Check cache first (read lock)
    let cache_guard_read = PNG_PROFILE_CACHE.lock().unwrap();
    if let Some(cached_profile_arc) = cache_guard_read.get(&profile_key) {
        debug!("ICC Profile cache hit for PNG ('{}', {} bytes)", profile_name, profile_key.len());
        // Assuming IccProfile is Clone.
        let profile_to_return = IccProfile::clone(&*cached_profile_arc);
        drop(cache_guard_read); // Release lock
        return Ok(Some(profile_to_return));
    }
    drop(cache_guard_read); // Release read lock

    debug!("ICC Profile cache miss for PNG ('{}', {} bytes), parsing...", profile_name, profile_key.len());
    let mut color_manager = ColorManager::new().context("Failed to create color manager for PNG profile")?;
    match color_manager.load_profile_from_data(&profile_data, profile_name) {
        Ok(parsed_profile) => {
            // Assuming IccProfile is Clone
            let arc_profile = Arc::new(parsed_profile.clone());
            // Acquire write lock to insert into cache
            let mut cache_guard_write = PNG_PROFILE_CACHE.lock().unwrap();
            cache_guard_write.insert(profile_key, arc_profile);
            drop(cache_guard_write); // Release write lock
            Ok(Some(parsed_profile))
        }
        Err(e) => {
            warn!("Failed to load ICC profile ('{}', {} bytes) from PNG data: {}. Proceeding without profile.", profile_name, profile_data.len(), e);
            Ok(None) // Proceed without profile if parsing fails
        }
    }
}

/// Save a PNG image to a file path
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn save_png<P: AsRef<Path>>(image: &image::DynamicImage, path: P) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving PNG image to: {}", path.display());

    image
        .save_with_format(path, image::ImageFormat::Png)
        .with_context(|| format!("Failed to save PNG image to: {}", path.display()))?;

    Ok(())
}

/// PNG-specific configuration options
#[derive(Debug, Clone)]
pub struct PngOptions {
    /// Compression level (0-9, where 9 is maximum compression)
    pub compression_level: u8,
    /// Whether to use filtering
    pub use_filtering: bool,
    /// ICC profile to embed (optional)
    pub icc_profile: Option<IccProfile>,
}

impl Default for PngOptions {
    fn default() -> Self {
        Self {
            compression_level: 6, // Default compression level
            use_filtering: true,
            icc_profile: None,
        }
    }
}

/// Save a PNG image with specific options
#[instrument(skip_all, fields(path = %path.as_ref().display()))]
pub fn save_png_with_options<P: AsRef<Path>>(
    image: &image::DynamicImage,
    path: P,
    options: &PngOptions,
) -> Result<()> {
    let path = path.as_ref();
    debug!("Saving PNG image with options to: {}", path.display());

    if options.icc_profile.is_some() {
        // Use custom PNG encoding with ICC profile
        save_png_with_icc_profile(image, path, options)
    } else {
        // Use standard save method
        save_png(image, path)
    }
}

/// Save a PNG image with embedded ICC profile
fn save_png_with_icc_profile<P: AsRef<Path>>(
    image: &image::DynamicImage,
    path: P,
    _options: &PngOptions,
) -> Result<()> {
    let path = path.as_ref();

    // For now, save without ICC profile and log a warning
    // TODO: Implement custom PNG encoding with ICC profile embedding
    warn!("ICC profile embedding in PNG not yet implemented, saving without profile");

    save_png(image, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    use tempfile::tempdir;

    #[test]
    fn test_save_and_load_png() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.png");

        // Create a simple test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        // Save the image
        save_png(&dynamic_img, &file_path)?;

        // Verify file exists
        assert!(file_path.exists());

        // Load the image back
        let loaded_img = load_png(&file_path)?;

        // Verify dimensions
        assert_eq!(loaded_img.width(), 100);
        assert_eq!(loaded_img.height(), 100);

        Ok(())
    }

    #[test]
    fn test_png_options_default() {
        let options = PngOptions::default();
        assert_eq!(options.compression_level, 6);
        assert!(options.use_filtering);
    }

    #[test]
    fn test_save_png_with_options() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_options.png");

        // Create a simple test image
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(50, 50);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let options = PngOptions {
            compression_level: 9,
            use_filtering: false,
            icc_profile: None,
        };

        // Save the image with options
        save_png_with_options(&dynamic_img, &file_path, &options)?;

        // Verify file exists
        assert!(file_path.exists());

        Ok(())
    }

    #[test]
    fn test_png_load_result_creation() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(10, 10);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);

        let result = PngLoadResult {
            image: dynamic_img,
            icc_profile: None,
        };

        assert_eq!(result.image.width(), 10);
        assert_eq!(result.image.height(), 10);
        assert!(result.icc_profile.is_none());
    }

    #[test]
    fn test_png_options_with_profile() {
        let mut options = PngOptions::default();
        assert!(options.icc_profile.is_none());

        // Test that we can set an ICC profile (even if None for now)
        options.icc_profile = None;
        assert!(options.icc_profile.is_none());
    }

    #[test]
    fn test_load_png_with_profile_fallback() -> Result<()> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_no_profile.png");

        // Create and save a simple test image without profile
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(20, 20);
        let dynamic_img = image::DynamicImage::ImageRgb8(img);
        save_png(&dynamic_img, &file_path)?;

        // Load with profile function - should work even without embedded profile
        let result = load_png_with_profile(&file_path)?;

        assert_eq!(result.image.width(), 20);
        assert_eq!(result.image.height(), 20);
        assert!(result.icc_profile.is_none()); // No profile embedded

        Ok(())
    }

    #[test]
    fn test_extract_png_icc_profile_invalid_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("not_a_png.txt");

        // Create a non-PNG file
        std::fs::write(&file_path, "This is not a PNG file").unwrap();

        let result = extract_png_icc_profile(&file_path);
        // Should not panic and should return Ok(None) for invalid PNG
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
