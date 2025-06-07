//! HSL (Hue, Saturation, Lightness) adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, color::HslColor, PixelData, RgbaPixel};

/// HSL adjustment
///
/// Adjusts the hue, saturation, and lightness of an image independently.
/// This provides professional-grade color correction capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HslAdjustment {
    /// Hue shift in degrees (-180.0 to 180.0)
    /// Positive values shift towards red, negative towards cyan
    pub hue: f32,
    /// Saturation adjustment (-1.0 to 1.0)
    /// -1.0 = completely desaturated, 0.0 = no change, 1.0 = double saturation
    pub saturation: f32,
    /// Lightness adjustment (-1.0 to 1.0)
    /// -1.0 = completely black, 0.0 = no change, 1.0 = completely white
    pub lightness: f32,
}

impl HslAdjustment {
    /// Create a new HSL adjustment
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue: hue.clamp(-180.0, 180.0),
            saturation: saturation.clamp(-1.0, 1.0),
            lightness: lightness.clamp(-1.0, 1.0),
        }
    }

    /// Create an HSL adjustment with no change
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Set the hue shift value
    pub fn set_hue(&mut self, hue: f32) {
        self.hue = hue.clamp(-180.0, 180.0);
    }

    /// Get the hue shift value
    pub fn get_hue(&self) -> f32 {
        self.hue
    }

    /// Set the saturation adjustment value
    pub fn set_saturation(&mut self, saturation: f32) {
        self.saturation = saturation.clamp(-1.0, 1.0);
    }

    /// Get the saturation adjustment value
    pub fn get_saturation(&self) -> f32 {
        self.saturation
    }

    /// Set the lightness adjustment value
    pub fn set_lightness(&mut self, lightness: f32) {
        self.lightness = lightness.clamp(-1.0, 1.0);
    }

    /// Get the lightness adjustment value
    pub fn get_lightness(&self) -> f32 {
        self.lightness
    }

    /// Check if this adjustment would have no effect
    pub fn is_identity(&self) -> bool {
        self.hue == 0.0 && self.saturation == 0.0 && self.lightness == 0.0
    }

    /// Apply HSL adjustment to a single pixel
    fn adjust_pixel(&self, pixel: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        let mut hsl = HslColor::from_rgba(pixel);

        // Apply hue shift
        if self.hue != 0.0 {
            hsl.h = (hsl.h + self.hue).rem_euclid(360.0);
        }

        // Apply saturation adjustment
        if self.saturation != 0.0 {
            if self.saturation > 0.0 {
                // Increase saturation
                hsl.s = (hsl.s + self.saturation * (1.0 - hsl.s)).clamp(0.0, 1.0);
            } else {
                // Decrease saturation
                hsl.s = (hsl.s * (1.0 + self.saturation)).clamp(0.0, 1.0);
            }
        }

        // Apply lightness adjustment
        if self.lightness != 0.0 {
            if self.lightness > 0.0 {
                // Increase lightness
                hsl.l = (hsl.l + self.lightness * (1.0 - hsl.l)).clamp(0.0, 1.0);
            } else {
                // Decrease lightness
                hsl.l = (hsl.l * (1.0 + self.lightness)).clamp(0.0, 1.0);
            }
        }

        hsl.to_rgba()
    }
}

impl Default for HslAdjustment {
    fn default() -> Self {
        Self::identity()
    }
}

impl Adjustment for HslAdjustment {
    fn id(&self) -> &'static str {
        "hsl"
    }

    fn name(&self) -> &'static str {
        "Hue/Saturation/Lightness"
    }

    fn description(&self) -> &'static str {
        "Adjusts the hue, saturation, and lightness of the image"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel = pixel_data
                    .get_pixel(x, y)
                    .ok_or_else(|| anyhow::anyhow!("Failed to get pixel at ({}, {})", x, y))?;

                let adjusted_pixel = self.adjust_pixel(pixel);
                pixel_data.set_pixel(x, y, adjusted_pixel)?;
            }
        }

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        Ok(self.adjust_pixel(pixel))
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "hue": self.hue,
            "saturation": self.saturation,
            "lightness": self.lightness
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(hue) = parameters.get("hue").and_then(|v| v.as_f64()) {
            self.set_hue(hue as f32);
        }
        if let Some(saturation) = parameters.get("saturation").and_then(|v| v.as_f64()) {
            self.set_saturation(saturation as f32);
        }
        if let Some(lightness) = parameters.get("lightness").and_then(|v| v.as_f64()) {
            self.set_lightness(lightness as f32);
        }
        Ok(())
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsl_adjustment_creation() {
        let adj = HslAdjustment::new(30.0, 0.5, -0.2);
        assert_eq!(adj.hue, 30.0);
        assert_eq!(adj.saturation, 0.5);
        assert_eq!(adj.lightness, -0.2);
    }

    #[test]
    fn test_hsl_adjustment_identity() {
        let adj = HslAdjustment::identity();
        assert!(adj.is_identity());
        assert_eq!(adj.hue, 0.0);
        assert_eq!(adj.saturation, 0.0);
        assert_eq!(adj.lightness, 0.0);
    }

    #[test]
    fn test_hsl_adjustment_clamping() {
        let adj = HslAdjustment::new(200.0, 2.0, -2.0);
        assert_eq!(adj.hue, 180.0);
        assert_eq!(adj.saturation, 1.0);
        assert_eq!(adj.lightness, -1.0);
    }

    #[test]
    fn test_hsl_adjustment_metadata() {
        let adj = HslAdjustment::identity();
        assert_eq!(adj.id(), "hsl");
        assert_eq!(adj.name(), "Hue/Saturation/Lightness");
        assert!(!adj.description().is_empty());
    }

    #[test]
    fn test_hsl_adjustment_parameters() {
        let mut adj = HslAdjustment::identity();
        let params = serde_json::json!({
            "hue": 45.0,
            "saturation": 0.3,
            "lightness": -0.1
        });

        adj.set_parameters(params).unwrap();
        assert_eq!(adj.hue, 45.0);
        assert_eq!(adj.saturation, 0.3);
        assert_eq!(adj.lightness, -0.1);

        let retrieved_params = adj.get_parameters();
        assert_eq!(retrieved_params["hue"], 45.0);
        assert!((retrieved_params["saturation"].as_f64().unwrap() - 0.3).abs() < 0.001);
        assert!((retrieved_params["lightness"].as_f64().unwrap() - (-0.1)).abs() < 0.001);
    }

    #[test]
    fn test_hsl_adjustment_clone() {
        let adj = HslAdjustment::new(60.0, 0.8, 0.2);
        let cloned = adj.clone();
        assert_eq!(adj.hue, cloned.hue);
        assert_eq!(adj.saturation, cloned.saturation);
        assert_eq!(adj.lightness, cloned.lightness);
    }

    #[test]
    fn test_hsl_adjustment_pixel() {
        let adj = HslAdjustment::new(0.0, 0.5, 0.0); // Increase saturation
        let pixel = RgbaPixel::new(128, 64, 192, 255); // Purple-ish color

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should have more saturated colors
        assert_ne!(result, pixel);
        assert_eq!(result.a, 255); // Alpha unchanged
    }

    #[test]
    fn test_hsl_adjustment_hue_shift() {
        let adj = HslAdjustment::new(180.0, 0.0, 0.0); // 180-degree hue shift
        let red_pixel = RgbaPixel::new(255, 0, 0, 255);

        let result = adj.apply_to_pixel(red_pixel).unwrap();

        // Red shifted by 180 degrees should become cyan-ish
        assert!(result.r < 128); // Less red
        assert!(result.g > 128); // More green
        assert!(result.b > 128); // More blue
        assert_eq!(result.a, 255); // Alpha unchanged
    }

    #[test]
    fn test_hsl_adjustment_lightness() {
        let adj = HslAdjustment::new(0.0, 0.0, 0.5); // Increase lightness
        let dark_pixel = RgbaPixel::new(64, 64, 64, 255);

        let result = adj.apply_to_pixel(dark_pixel).unwrap();

        // Should be lighter
        assert!(result.r > dark_pixel.r);
        assert!(result.g > dark_pixel.g);
        assert!(result.b > dark_pixel.b);
        assert_eq!(result.a, 255); // Alpha unchanged
    }

    #[test]
    fn test_hsl_adjustment_apply() {
        let adj = HslAdjustment::new(30.0, 0.2, -0.1);
        let mut pixel_data = PixelData::new_rgba(2, 2);

        // Fill with test colors
        pixel_data
            .set_pixel(0, 0, RgbaPixel::new(255, 0, 0, 255))
            .unwrap(); // Red
        pixel_data
            .set_pixel(1, 0, RgbaPixel::new(0, 255, 0, 255))
            .unwrap(); // Green
        pixel_data
            .set_pixel(0, 1, RgbaPixel::new(0, 0, 255, 255))
            .unwrap(); // Blue
        pixel_data
            .set_pixel(1, 1, RgbaPixel::new(128, 128, 128, 255))
            .unwrap(); // Gray

        let original_pixels = [
            pixel_data.get_pixel(0, 0).unwrap(),
            pixel_data.get_pixel(1, 0).unwrap(),
            pixel_data.get_pixel(0, 1).unwrap(),
            pixel_data.get_pixel(1, 1).unwrap(),
        ];

        adj.apply(&mut pixel_data).unwrap();

        // All pixels should be modified
        for i in 0..2 {
            for j in 0..2 {
                let new_pixel = pixel_data.get_pixel(i, j).unwrap();
                let original_pixel = original_pixels[i as usize + j as usize * 2];
                assert_ne!(new_pixel, original_pixel);
                assert_eq!(new_pixel.a, original_pixel.a); // Alpha unchanged
            }
        }
    }
}
