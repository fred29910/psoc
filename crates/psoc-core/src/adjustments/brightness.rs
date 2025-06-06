//! Brightness adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Brightness adjustment
///
/// Adjusts the brightness of an image by adding or subtracting a value
/// from each color channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessAdjustment {
    /// Brightness adjustment value (-1.0 to 1.0)
    /// -1.0 = completely black, 0.0 = no change, 1.0 = completely white
    pub brightness: f32,
}

impl BrightnessAdjustment {
    /// Create a new brightness adjustment
    pub fn new(brightness: f32) -> Self {
        Self {
            brightness: brightness.clamp(-1.0, 1.0),
        }
    }

    /// Create a brightness adjustment with no change
    pub fn identity() -> Self {
        Self::new(0.0)
    }

    /// Set the brightness value
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(-1.0, 1.0);
    }

    /// Get the brightness value
    pub fn get_brightness(&self) -> f32 {
        self.brightness
    }

    /// Check if this adjustment would have no effect
    pub fn is_identity(&self) -> bool {
        self.brightness == 0.0
    }
}

impl Default for BrightnessAdjustment {
    fn default() -> Self {
        Self::identity()
    }
}

impl Adjustment for BrightnessAdjustment {
    fn id(&self) -> &'static str {
        "brightness"
    }

    fn name(&self) -> &'static str {
        "Brightness"
    }

    fn description(&self) -> &'static str {
        "Adjusts the brightness of the image"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();
        let brightness_offset = (self.brightness * 255.0) as i32;

        for y in 0..height {
            for x in 0..width {
                let mut pixel = pixel_data
                    .get_pixel(x, y)
                    .ok_or_else(|| anyhow::anyhow!("Failed to get pixel at ({}, {})", x, y))?;

                // Apply brightness adjustment to RGB channels
                pixel.r = ((pixel.r as i32 + brightness_offset).clamp(0, 255)) as u8;
                pixel.g = ((pixel.g as i32 + brightness_offset).clamp(0, 255)) as u8;
                pixel.b = ((pixel.b as i32 + brightness_offset).clamp(0, 255)) as u8;
                // Alpha channel remains unchanged

                pixel_data.set_pixel(x, y, pixel)?;
            }
        }

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        if self.is_identity() {
            return Ok(pixel);
        }

        let brightness_offset = (self.brightness * 255.0) as i32;

        Ok(RgbaPixel::new(
            ((pixel.r as i32 + brightness_offset).clamp(0, 255)) as u8,
            ((pixel.g as i32 + brightness_offset).clamp(0, 255)) as u8,
            ((pixel.b as i32 + brightness_offset).clamp(0, 255)) as u8,
            pixel.a, // Alpha unchanged
        ))
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "brightness": self.brightness
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(brightness) = parameters.get("brightness").and_then(|v| v.as_f64()) {
            self.set_brightness(brightness as f32);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Invalid parameters for BrightnessAdjustment: missing 'brightness' field"
            ))
        }
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PixelData;

    #[test]
    fn test_brightness_adjustment_creation() {
        let adj = BrightnessAdjustment::new(0.5);
        assert_eq!(adj.get_brightness(), 0.5);
        assert!(!adj.is_identity());

        let identity = BrightnessAdjustment::identity();
        assert_eq!(identity.get_brightness(), 0.0);
        assert!(identity.is_identity());
    }

    #[test]
    fn test_brightness_adjustment_clamping() {
        let adj = BrightnessAdjustment::new(2.0);
        assert_eq!(adj.get_brightness(), 1.0);

        let adj = BrightnessAdjustment::new(-2.0);
        assert_eq!(adj.get_brightness(), -1.0);
    }

    #[test]
    fn test_brightness_adjustment_pixel() {
        let adj = BrightnessAdjustment::new(0.2); // 20% brighter
        let pixel = RgbaPixel::new(100, 100, 100, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be brighter (100 + 0.2 * 255 = 100 + 51 = 151)
        assert_eq!(result.r, 151);
        assert_eq!(result.g, 151);
        assert_eq!(result.b, 151);
        assert_eq!(result.a, 255); // Alpha unchanged
    }

    #[test]
    fn test_brightness_adjustment_pixel_clamping() {
        let adj = BrightnessAdjustment::new(1.0); // Maximum brightness
        let pixel = RgbaPixel::new(200, 200, 200, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be clamped to 255
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255);
        assert_eq!(result.b, 255);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_brightness_adjustment_negative() {
        let adj = BrightnessAdjustment::new(-0.2); // 20% darker
        let pixel = RgbaPixel::new(100, 100, 100, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be darker (100 - 0.2 * 255 = 100 - 51 = 49)
        assert_eq!(result.r, 49);
        assert_eq!(result.g, 49);
        assert_eq!(result.b, 49);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_brightness_adjustment_apply() {
        let adj = BrightnessAdjustment::new(0.1);
        let mut pixel_data = PixelData::new_rgba(2, 2);

        // Fill with gray pixels
        for y in 0..2 {
            for x in 0..2 {
                pixel_data
                    .set_pixel(x, y, RgbaPixel::new(128, 128, 128, 255))
                    .unwrap();
            }
        }

        adj.apply(&mut pixel_data).unwrap();

        // Check that all pixels were brightened
        for y in 0..2 {
            for x in 0..2 {
                let pixel = pixel_data.get_pixel(x, y).unwrap();
                assert!(pixel.r > 128);
                assert!(pixel.g > 128);
                assert!(pixel.b > 128);
                assert_eq!(pixel.a, 255);
            }
        }
    }

    #[test]
    fn test_brightness_adjustment_parameters() {
        let mut adj = BrightnessAdjustment::new(0.0);

        // Test parameter serialization
        let params = adj.get_parameters();
        assert_eq!(params["brightness"], 0.0);

        // Test parameter deserialization
        let new_params = serde_json::json!({ "brightness": 0.5 });
        adj.set_parameters(new_params).unwrap();
        assert_eq!(adj.get_brightness(), 0.5);

        // Test invalid parameters
        let invalid_params = serde_json::json!({ "invalid": 0.5 });
        assert!(adj.set_parameters(invalid_params).is_err());
    }

    #[test]
    fn test_brightness_adjustment_identity() {
        let adj = BrightnessAdjustment::identity();
        let pixel = RgbaPixel::new(100, 150, 200, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be unchanged
        assert_eq!(result, pixel);
        assert!(!adj.would_modify_pixel(pixel));
    }

    #[test]
    fn test_brightness_adjustment_metadata() {
        let adj = BrightnessAdjustment::new(0.5);

        assert_eq!(adj.id(), "brightness");
        assert_eq!(adj.name(), "Brightness");
        assert!(!adj.description().is_empty());
    }

    #[test]
    fn test_brightness_adjustment_clone() {
        let adj = BrightnessAdjustment::new(0.3);
        let cloned = adj.clone_adjustment();

        // Test that the clone has the same parameters
        let original_params = adj.get_parameters();
        let cloned_params = cloned.get_parameters();
        assert_eq!(original_params, cloned_params);
    }
}
