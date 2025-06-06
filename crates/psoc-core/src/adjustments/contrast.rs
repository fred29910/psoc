//! Contrast adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Contrast adjustment
///
/// Adjusts the contrast of an image by scaling the difference between
/// each color channel and the midpoint (128).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastAdjustment {
    /// Contrast adjustment value (-1.0 to 1.0)
    /// -1.0 = no contrast (gray), 0.0 = no change, 1.0 = maximum contrast
    pub contrast: f32,
}

impl ContrastAdjustment {
    /// Create a new contrast adjustment
    pub fn new(contrast: f32) -> Self {
        Self {
            contrast: contrast.clamp(-1.0, 1.0),
        }
    }

    /// Create a contrast adjustment with no change
    pub fn identity() -> Self {
        Self::new(0.0)
    }

    /// Set the contrast value
    pub fn set_contrast(&mut self, contrast: f32) {
        self.contrast = contrast.clamp(-1.0, 1.0);
    }

    /// Get the contrast value
    pub fn get_contrast(&self) -> f32 {
        self.contrast
    }

    /// Check if this adjustment would have no effect
    pub fn is_identity(&self) -> bool {
        self.contrast == 0.0
    }

    /// Apply contrast adjustment to a single channel value
    fn adjust_channel(&self, value: u8) -> u8 {
        if self.is_identity() {
            return value;
        }

        let normalized = value as f32 / 255.0;
        let contrast_factor = 1.0 + self.contrast;

        // Apply contrast around the midpoint (0.5)
        let adjusted = ((normalized - 0.5) * contrast_factor + 0.5).clamp(0.0, 1.0);

        (adjusted * 255.0) as u8
    }
}

impl Default for ContrastAdjustment {
    fn default() -> Self {
        Self::identity()
    }
}

impl Adjustment for ContrastAdjustment {
    fn id(&self) -> &'static str {
        "contrast"
    }

    fn name(&self) -> &'static str {
        "Contrast"
    }

    fn description(&self) -> &'static str {
        "Adjusts the contrast of the image"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();

        for y in 0..height {
            for x in 0..width {
                let mut pixel = pixel_data
                    .get_pixel(x, y)
                    .ok_or_else(|| anyhow::anyhow!("Failed to get pixel at ({}, {})", x, y))?;

                // Apply contrast adjustment to RGB channels
                pixel.r = self.adjust_channel(pixel.r);
                pixel.g = self.adjust_channel(pixel.g);
                pixel.b = self.adjust_channel(pixel.b);
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

        Ok(RgbaPixel::new(
            self.adjust_channel(pixel.r),
            self.adjust_channel(pixel.g),
            self.adjust_channel(pixel.b),
            pixel.a, // Alpha unchanged
        ))
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "contrast": self.contrast
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(contrast) = parameters.get("contrast").and_then(|v| v.as_f64()) {
            self.set_contrast(contrast as f32);
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Invalid parameters for ContrastAdjustment: missing 'contrast' field"
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
    fn test_contrast_adjustment_creation() {
        let adj = ContrastAdjustment::new(0.5);
        assert_eq!(adj.get_contrast(), 0.5);
        assert!(!adj.is_identity());

        let identity = ContrastAdjustment::identity();
        assert_eq!(identity.get_contrast(), 0.0);
        assert!(identity.is_identity());
    }

    #[test]
    fn test_contrast_adjustment_clamping() {
        let adj = ContrastAdjustment::new(2.0);
        assert_eq!(adj.get_contrast(), 1.0);

        let adj = ContrastAdjustment::new(-2.0);
        assert_eq!(adj.get_contrast(), -1.0);
    }

    #[test]
    fn test_contrast_adjustment_pixel() {
        let adj = ContrastAdjustment::new(0.5); // Increase contrast
        let pixel = RgbaPixel::new(100, 200, 50, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Values below 128 should become darker, above 128 should become brighter
        assert!(result.r < 100); // 100 < 128, should be darker
        assert!(result.g > 200); // 200 > 128, should be brighter
        assert!(result.b < 50); // 50 < 128, should be darker
        assert_eq!(result.a, 255); // Alpha unchanged
    }

    #[test]
    fn test_contrast_adjustment_midpoint() {
        let adj = ContrastAdjustment::new(0.5);
        let pixel = RgbaPixel::new(128, 128, 128, 255); // Midpoint gray

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Midpoint should remain unchanged regardless of contrast
        assert_eq!(result.r, 128);
        assert_eq!(result.g, 128);
        assert_eq!(result.b, 128);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_contrast_adjustment_negative() {
        let adj = ContrastAdjustment::new(-0.5); // Decrease contrast
        let pixel = RgbaPixel::new(0, 255, 100, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // All values should move toward the midpoint (128)
        assert!(result.r > 0); // Black moves toward gray
        assert!(result.g < 255); // White moves toward gray
        assert!(result.b > 100 && result.b < 128); // Value moves toward gray
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_contrast_adjustment_apply() {
        let adj = ContrastAdjustment::new(0.2);
        let mut pixel_data = PixelData::new_rgba(2, 2);

        // Fill with different gray values
        pixel_data
            .set_pixel(0, 0, RgbaPixel::new(64, 64, 64, 255))
            .unwrap();
        pixel_data
            .set_pixel(1, 0, RgbaPixel::new(192, 192, 192, 255))
            .unwrap();
        pixel_data
            .set_pixel(0, 1, RgbaPixel::new(128, 128, 128, 255))
            .unwrap();
        pixel_data
            .set_pixel(1, 1, RgbaPixel::new(96, 96, 96, 255))
            .unwrap();

        adj.apply(&mut pixel_data).unwrap();

        // Check that contrast was applied
        let dark_pixel = pixel_data.get_pixel(0, 0).unwrap();
        let bright_pixel = pixel_data.get_pixel(1, 0).unwrap();
        let mid_pixel = pixel_data.get_pixel(0, 1).unwrap();

        assert!(dark_pixel.r < 64); // Dark pixel should be darker
        assert!(bright_pixel.r > 192); // Bright pixel should be brighter
        assert_eq!(mid_pixel.r, 128); // Mid pixel should be unchanged
    }

    #[test]
    fn test_contrast_adjustment_parameters() {
        let mut adj = ContrastAdjustment::new(0.0);

        // Test parameter serialization
        let params = adj.get_parameters();
        assert_eq!(params["contrast"], 0.0);

        // Test parameter deserialization
        let new_params = serde_json::json!({ "contrast": 0.3 });
        adj.set_parameters(new_params).unwrap();
        assert_eq!(adj.get_contrast(), 0.3);

        // Test invalid parameters
        let invalid_params = serde_json::json!({ "invalid": 0.5 });
        assert!(adj.set_parameters(invalid_params).is_err());
    }

    #[test]
    fn test_contrast_adjustment_identity() {
        let adj = ContrastAdjustment::identity();
        let pixel = RgbaPixel::new(100, 150, 200, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be unchanged
        assert_eq!(result, pixel);
        assert!(!adj.would_modify_pixel(pixel));
    }

    #[test]
    fn test_contrast_adjustment_metadata() {
        let adj = ContrastAdjustment::new(0.5);

        assert_eq!(adj.id(), "contrast");
        assert_eq!(adj.name(), "Contrast");
        assert!(!adj.description().is_empty());
    }

    #[test]
    fn test_contrast_adjustment_clone() {
        let adj = ContrastAdjustment::new(0.3);
        let cloned = adj.clone_adjustment();

        // Test that the clone has the same parameters
        let original_params = adj.get_parameters();
        let cloned_params = cloned.get_parameters();
        assert_eq!(original_params, cloned_params);
    }

    #[test]
    fn test_contrast_adjustment_extreme_values() {
        let adj = ContrastAdjustment::new(1.0); // Maximum contrast

        // Test with values near the extremes
        let black_pixel = RgbaPixel::new(10, 10, 10, 255);
        let white_pixel = RgbaPixel::new(245, 245, 245, 255);

        let black_result = adj.apply_to_pixel(black_pixel).unwrap();
        let white_result = adj.apply_to_pixel(white_pixel).unwrap();

        // Should push toward extremes
        assert!(black_result.r < black_pixel.r);
        assert!(white_result.r > white_pixel.r);
    }
}
