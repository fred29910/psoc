//! Pixel data representation and manipulation
//!
//! This module provides efficient pixel data structures and operations for image editing.
//! It supports multiple color formats and provides conversion between different representations.

use anyhow::{Context, Result};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use ndarray::Array3;
use serde::{Deserialize, Serialize};

/// Color channel type - 8-bit unsigned integer
pub type Channel = u8;

/// Maximum value for a color channel
pub const CHANNEL_MAX: Channel = 255;

/// Minimum value for a color channel  
pub const CHANNEL_MIN: Channel = 0;

/// RGBA pixel representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbaPixel {
    pub r: Channel,
    pub g: Channel,
    pub b: Channel,
    pub a: Channel,
}

impl RgbaPixel {
    /// Create a new RGBA pixel
    pub fn new(r: Channel, g: Channel, b: Channel, a: Channel) -> Self {
        Self { r, g, b, a }
    }

    /// Create a fully opaque RGB pixel
    pub fn rgb(r: Channel, g: Channel, b: Channel) -> Self {
        Self::new(r, g, b, CHANNEL_MAX)
    }

    /// Create a transparent pixel
    pub fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Create a white pixel
    pub fn white() -> Self {
        Self::rgb(CHANNEL_MAX, CHANNEL_MAX, CHANNEL_MAX)
    }

    /// Create a black pixel
    pub fn black() -> Self {
        Self::rgb(CHANNEL_MIN, CHANNEL_MIN, CHANNEL_MIN)
    }

    /// Check if pixel is fully transparent
    pub fn is_transparent(&self) -> bool {
        self.a == 0
    }

    /// Check if pixel is fully opaque
    pub fn is_opaque(&self) -> bool {
        self.a == CHANNEL_MAX
    }

    /// Convert to array [r, g, b, a]
    pub fn to_array(self) -> [Channel; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Create from array [r, g, b, a]
    pub fn from_array(arr: [Channel; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    /// Premultiply alpha
    pub fn premultiply_alpha(&self) -> Self {
        if self.a == 0 {
            return Self::transparent();
        }
        if self.a == CHANNEL_MAX {
            return *self;
        }

        let alpha_f = self.a as f32 / CHANNEL_MAX as f32;
        Self::new(
            (self.r as f32 * alpha_f) as Channel,
            (self.g as f32 * alpha_f) as Channel,
            (self.b as f32 * alpha_f) as Channel,
            self.a,
        )
    }

    /// Unpremultiply alpha
    pub fn unpremultiply_alpha(&self) -> Self {
        if self.a == 0 {
            return Self::transparent();
        }
        if self.a == CHANNEL_MAX {
            return *self;
        }

        let alpha_f = self.a as f32 / CHANNEL_MAX as f32;
        Self::new(
            (self.r as f32 / alpha_f).min(CHANNEL_MAX as f32) as Channel,
            (self.g as f32 / alpha_f).min(CHANNEL_MAX as f32) as Channel,
            (self.b as f32 / alpha_f).min(CHANNEL_MAX as f32) as Channel,
            self.a,
        )
    }

    /// Linear interpolation between two pixels
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;

        Self {
            r: (self.r as f32 * inv_t + other.r as f32 * t) as u8,
            g: (self.g as f32 * inv_t + other.g as f32 * t) as u8,
            b: (self.b as f32 * inv_t + other.b as f32 * t) as u8,
            a: (self.a as f32 * inv_t + other.a as f32 * t) as u8,
        }
    }
}

impl From<Rgba<u8>> for RgbaPixel {
    fn from(rgba: Rgba<u8>) -> Self {
        Self::new(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

impl From<RgbaPixel> for Rgba<u8> {
    fn from(pixel: RgbaPixel) -> Self {
        Rgba([pixel.r, pixel.g, pixel.b, pixel.a])
    }
}

/// Pixel data storage format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PixelData {
    /// RGBA data stored as a 3D array (height, width, channels)
    Rgba(Array3<Channel>),
    /// Raw byte data with format information
    Raw {
        data: Vec<Channel>,
        width: u32,
        height: u32,
        channels: u8,
    },
}

impl PixelData {
    /// Create new RGBA pixel data with given dimensions
    pub fn new_rgba(width: u32, height: u32) -> Self {
        let array = Array3::zeros((height as usize, width as usize, 4));
        Self::Rgba(array)
    }

    /// Create pixel data from image
    pub fn from_image(image: &DynamicImage) -> Result<Self> {
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();

        let mut array = Array3::zeros((height as usize, width as usize, 4));

        for (x, y, pixel) in rgba_image.enumerate_pixels() {
            let rgba = RgbaPixel::from(*pixel);
            array[[y as usize, x as usize, 0]] = rgba.r;
            array[[y as usize, x as usize, 1]] = rgba.g;
            array[[y as usize, x as usize, 2]] = rgba.b;
            array[[y as usize, x as usize, 3]] = rgba.a;
        }

        Ok(Self::Rgba(array))
    }

    /// Convert to image
    pub fn to_image(&self) -> Result<DynamicImage> {
        match self {
            Self::Rgba(array) => {
                let (height, width, channels) = array.dim();
                if channels != 4 {
                    return Err(anyhow::anyhow!(
                        "Expected 4 channels for RGBA data, got {}",
                        channels
                    ));
                }

                let mut img_buffer = RgbaImage::new(width as u32, height as u32);

                for y in 0..height {
                    for x in 0..width {
                        let pixel = RgbaPixel::new(
                            array[[y, x, 0]],
                            array[[y, x, 1]],
                            array[[y, x, 2]],
                            array[[y, x, 3]],
                        );
                        img_buffer.put_pixel(x as u32, y as u32, pixel.into());
                    }
                }

                Ok(DynamicImage::ImageRgba8(img_buffer))
            }
            Self::Raw {
                data,
                width,
                height,
                channels,
            } => {
                if *channels != 4 {
                    return Err(anyhow::anyhow!(
                        "Only RGBA format supported for raw data conversion"
                    ));
                }

                let img_buffer = ImageBuffer::from_raw(*width, *height, data.clone())
                    .context("Failed to create image buffer from raw data")?;

                Ok(DynamicImage::ImageRgba8(img_buffer))
            }
        }
    }

    /// Get dimensions (width, height)
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Rgba(array) => {
                let (height, width, _) = array.dim();
                (width as u32, height as u32)
            }
            Self::Raw { width, height, .. } => (*width, *height),
        }
    }

    /// Get pixel at coordinates
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        match self {
            Self::Rgba(array) => {
                let (height, width, _) = array.dim();
                if x >= width as u32 || y >= height as u32 {
                    return None;
                }

                Some(RgbaPixel::new(
                    array[[y as usize, x as usize, 0]],
                    array[[y as usize, x as usize, 1]],
                    array[[y as usize, x as usize, 2]],
                    array[[y as usize, x as usize, 3]],
                ))
            }
            Self::Raw {
                data,
                width,
                height,
                channels,
            } => {
                if x >= *width || y >= *height || *channels != 4 {
                    return None;
                }

                let index = ((y * width + x) * *channels as u32) as usize;
                if index + 3 >= data.len() {
                    return None;
                }

                Some(RgbaPixel::new(
                    data[index],
                    data[index + 1],
                    data[index + 2],
                    data[index + 3],
                ))
            }
        }
    }

    /// Set pixel at coordinates
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: RgbaPixel) -> Result<()> {
        match self {
            Self::Rgba(array) => {
                let (height, width, _) = array.dim();
                if x >= width as u32 || y >= height as u32 {
                    return Err(anyhow::anyhow!("Pixel coordinates out of bounds"));
                }

                array[[y as usize, x as usize, 0]] = pixel.r;
                array[[y as usize, x as usize, 1]] = pixel.g;
                array[[y as usize, x as usize, 2]] = pixel.b;
                array[[y as usize, x as usize, 3]] = pixel.a;

                Ok(())
            }
            Self::Raw {
                data,
                width,
                height,
                channels,
            } => {
                if x >= *width || y >= *height || *channels != 4 {
                    return Err(anyhow::anyhow!("Invalid pixel coordinates or format"));
                }

                let index = ((y * *width + x) * *channels as u32) as usize;
                if index + 3 >= data.len() {
                    return Err(anyhow::anyhow!("Pixel index out of bounds"));
                }

                data[index] = pixel.r;
                data[index + 1] = pixel.g;
                data[index + 2] = pixel.b;
                data[index + 3] = pixel.a;

                Ok(())
            }
        }
    }

    /// Fill entire pixel data with a single color
    pub fn fill(&mut self, pixel: RgbaPixel) {
        match self {
            Self::Rgba(array) => {
                let (height, width, _) = array.dim();
                for y in 0..height {
                    for x in 0..width {
                        array[[y, x, 0]] = pixel.r;
                        array[[y, x, 1]] = pixel.g;
                        array[[y, x, 2]] = pixel.b;
                        array[[y, x, 3]] = pixel.a;
                    }
                }
            }
            Self::Raw { data, channels, .. } => {
                if *channels == 4 {
                    for chunk in data.chunks_exact_mut(4) {
                        chunk[0] = pixel.r;
                        chunk[1] = pixel.g;
                        chunk[2] = pixel.b;
                        chunk[3] = pixel.a;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgba_pixel_creation() {
        let pixel = RgbaPixel::new(255, 128, 64, 200);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 128);
        assert_eq!(pixel.b, 64);
        assert_eq!(pixel.a, 200);
    }

    #[test]
    fn test_rgba_pixel_presets() {
        let white = RgbaPixel::white();
        assert_eq!(white, RgbaPixel::new(255, 255, 255, 255));

        let black = RgbaPixel::black();
        assert_eq!(black, RgbaPixel::new(0, 0, 0, 255));

        let transparent = RgbaPixel::transparent();
        assert_eq!(transparent, RgbaPixel::new(0, 0, 0, 0));
        assert!(transparent.is_transparent());
        assert!(!transparent.is_opaque());
    }

    #[test]
    fn test_pixel_data_creation() {
        let pixel_data = PixelData::new_rgba(100, 50);
        let (width, height) = pixel_data.dimensions();
        assert_eq!(width, 100);
        assert_eq!(height, 50);
    }

    #[test]
    fn test_pixel_data_get_set() {
        let mut pixel_data = PixelData::new_rgba(10, 10);
        let test_pixel = RgbaPixel::new(255, 128, 64, 200);

        pixel_data.set_pixel(5, 5, test_pixel).unwrap();
        let retrieved = pixel_data.get_pixel(5, 5).unwrap();

        assert_eq!(retrieved, test_pixel);
    }

    #[test]
    fn test_pixel_data_fill() {
        let mut pixel_data = PixelData::new_rgba(5, 5);
        let fill_color = RgbaPixel::new(100, 150, 200, 255);

        pixel_data.fill(fill_color);

        // Check a few pixels
        assert_eq!(pixel_data.get_pixel(0, 0).unwrap(), fill_color);
        assert_eq!(pixel_data.get_pixel(2, 3).unwrap(), fill_color);
        assert_eq!(pixel_data.get_pixel(4, 4).unwrap(), fill_color);
    }

    #[test]
    fn test_premultiply_alpha() {
        let pixel = RgbaPixel::new(200, 100, 50, 128);
        let premultiplied = pixel.premultiply_alpha();

        // With alpha = 128 (50%), colors should be roughly halved
        assert!(premultiplied.r <= pixel.r);
        assert!(premultiplied.g <= pixel.g);
        assert!(premultiplied.b <= pixel.b);
        assert_eq!(premultiplied.a, pixel.a);
    }
}
