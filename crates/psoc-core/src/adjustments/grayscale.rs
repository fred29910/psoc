//! Grayscale adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Grayscale conversion methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrayscaleMethod {
    /// Simple average of RGB channels
    Average,
    /// Luminance-based conversion (ITU-R BT.709 standard)
    Luminance,
    /// Lightness-based conversion (average of min and max)
    Lightness,
    /// Custom weighted average
    Custom,
}

impl Default for GrayscaleMethod {
    fn default() -> Self {
        Self::Luminance
    }
}

impl GrayscaleMethod {
    /// Get the display name for this method
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Average => "Average",
            Self::Luminance => "Luminance",
            Self::Lightness => "Lightness",
            Self::Custom => "Custom",
        }
    }

    /// Get the description for this method
    pub fn description(&self) -> &'static str {
        match self {
            Self::Average => "Simple average of RGB channels",
            Self::Luminance => "Perceptual luminance (ITU-R BT.709)",
            Self::Lightness => "Average of minimum and maximum RGB values",
            Self::Custom => "Custom weighted average",
        }
    }
}

/// Grayscale adjustment
///
/// Converts color images to grayscale using various methods.
/// Supports different conversion algorithms and custom weights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrayscaleAdjustment {
    /// Conversion method to use
    pub method: GrayscaleMethod,
    /// Custom weights for RGB channels (only used with Custom method)
    /// Should sum to 1.0 for proper results
    pub red_weight: f32,
    pub green_weight: f32,
    pub blue_weight: f32,
    /// Opacity of the grayscale effect (0.0 = no effect, 1.0 = full grayscale)
    pub opacity: f32,
}

impl GrayscaleAdjustment {
    /// Create a new grayscale adjustment with luminance method
    pub fn new() -> Self {
        Self {
            method: GrayscaleMethod::Luminance,
            red_weight: 0.299, // ITU-R BT.709 standard
            green_weight: 0.587,
            blue_weight: 0.114,
            opacity: 1.0,
        }
    }

    /// Create a grayscale adjustment with custom weights
    pub fn with_custom_weights(red: f32, green: f32, blue: f32) -> Self {
        Self {
            method: GrayscaleMethod::Custom,
            red_weight: red.clamp(0.0, 1.0),
            green_weight: green.clamp(0.0, 1.0),
            blue_weight: blue.clamp(0.0, 1.0),
            opacity: 1.0,
        }
    }

    /// Create a grayscale adjustment with specific method
    pub fn with_method(method: GrayscaleMethod) -> Self {
        let mut adj = Self::new();
        adj.method = method;
        adj
    }

    /// Set the conversion method
    pub fn set_method(&mut self, method: GrayscaleMethod) {
        self.method = method;
        // Update weights based on method
        match method {
            GrayscaleMethod::Average => {
                self.red_weight = 1.0 / 3.0;
                self.green_weight = 1.0 / 3.0;
                self.blue_weight = 1.0 / 3.0;
            }
            GrayscaleMethod::Luminance => {
                self.red_weight = 0.299;
                self.green_weight = 0.587;
                self.blue_weight = 0.114;
            }
            GrayscaleMethod::Lightness => {
                // Weights don't apply to lightness method
                self.red_weight = 0.0;
                self.green_weight = 0.0;
                self.blue_weight = 0.0;
            }
            GrayscaleMethod::Custom => {
                // Keep existing custom weights
            }
        }
    }

    /// Set custom weights (automatically sets method to Custom)
    pub fn set_custom_weights(&mut self, red: f32, green: f32, blue: f32) {
        self.method = GrayscaleMethod::Custom;
        self.red_weight = red.clamp(0.0, 1.0);
        self.green_weight = green.clamp(0.0, 1.0);
        self.blue_weight = blue.clamp(0.0, 1.0);
    }

    /// Set the opacity of the grayscale effect
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Get the opacity of the grayscale effect
    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }

    /// Check if this adjustment would have no effect
    pub fn is_identity(&self) -> bool {
        self.opacity == 0.0
    }

    /// Convert a pixel to grayscale
    fn convert_pixel(&self, pixel: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        let gray_value = match self.method {
            GrayscaleMethod::Average => {
                ((pixel.r as u32 + pixel.g as u32 + pixel.b as u32) / 3) as u8
            }
            GrayscaleMethod::Luminance => {
                (pixel.r as f32 * 0.299 + pixel.g as f32 * 0.587 + pixel.b as f32 * 0.114) as u8
            }
            GrayscaleMethod::Lightness => {
                let min = pixel.r.min(pixel.g.min(pixel.b));
                let max = pixel.r.max(pixel.g.max(pixel.b));
                ((min as u32 + max as u32) / 2) as u8
            }
            GrayscaleMethod::Custom => {
                (pixel.r as f32 * self.red_weight
                    + pixel.g as f32 * self.green_weight
                    + pixel.b as f32 * self.blue_weight) as u8
            }
        };

        if self.opacity >= 1.0 {
            // Full grayscale
            RgbaPixel::new(gray_value, gray_value, gray_value, pixel.a)
        } else {
            // Blend between original and grayscale
            let inv_opacity = 1.0 - self.opacity;
            RgbaPixel::new(
                (pixel.r as f32 * inv_opacity + gray_value as f32 * self.opacity) as u8,
                (pixel.g as f32 * inv_opacity + gray_value as f32 * self.opacity) as u8,
                (pixel.b as f32 * inv_opacity + gray_value as f32 * self.opacity) as u8,
                pixel.a,
            )
        }
    }
}

impl Default for GrayscaleAdjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl Adjustment for GrayscaleAdjustment {
    fn id(&self) -> &'static str {
        "grayscale"
    }

    fn name(&self) -> &'static str {
        "Grayscale"
    }

    fn description(&self) -> &'static str {
        "Converts the image to grayscale using various methods"
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

                let gray_pixel = self.convert_pixel(pixel);
                pixel_data.set_pixel(x, y, gray_pixel)?;
            }
        }

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        Ok(self.convert_pixel(pixel))
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "method": self.method,
            "red_weight": self.red_weight,
            "green_weight": self.green_weight,
            "blue_weight": self.blue_weight,
            "opacity": self.opacity
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(method_str) = parameters.get("method").and_then(|v| v.as_str()) {
            let method = match method_str {
                "Average" => GrayscaleMethod::Average,
                "Luminance" => GrayscaleMethod::Luminance,
                "Lightness" => GrayscaleMethod::Lightness,
                "Custom" => GrayscaleMethod::Custom,
                _ => return Err(anyhow::anyhow!("Invalid grayscale method: {}", method_str)),
            };
            self.set_method(method);
        }

        if let Some(red) = parameters.get("red_weight").and_then(|v| v.as_f64()) {
            self.red_weight = red as f32;
        }
        if let Some(green) = parameters.get("green_weight").and_then(|v| v.as_f64()) {
            self.green_weight = green as f32;
        }
        if let Some(blue) = parameters.get("blue_weight").and_then(|v| v.as_f64()) {
            self.blue_weight = blue as f32;
        }
        if let Some(opacity) = parameters.get("opacity").and_then(|v| v.as_f64()) {
            self.set_opacity(opacity as f32);
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
    fn test_grayscale_adjustment_creation() {
        let adj = GrayscaleAdjustment::new();
        assert_eq!(adj.method, GrayscaleMethod::Luminance);
        assert_eq!(adj.opacity, 1.0);
    }

    #[test]
    fn test_grayscale_method_display() {
        assert_eq!(GrayscaleMethod::Average.display_name(), "Average");
        assert_eq!(GrayscaleMethod::Luminance.display_name(), "Luminance");
        assert_eq!(GrayscaleMethod::Lightness.display_name(), "Lightness");
        assert_eq!(GrayscaleMethod::Custom.display_name(), "Custom");
    }

    #[test]
    fn test_grayscale_custom_weights() {
        let adj = GrayscaleAdjustment::with_custom_weights(0.5, 0.3, 0.2);
        assert_eq!(adj.method, GrayscaleMethod::Custom);
        assert_eq!(adj.red_weight, 0.5);
        assert_eq!(adj.green_weight, 0.3);
        assert_eq!(adj.blue_weight, 0.2);
    }

    #[test]
    fn test_grayscale_identity() {
        let mut adj = GrayscaleAdjustment::new();
        adj.set_opacity(0.0);
        assert!(adj.is_identity());
    }

    #[test]
    fn test_grayscale_metadata() {
        let adj = GrayscaleAdjustment::new();
        assert_eq!(adj.id(), "grayscale");
        assert_eq!(adj.name(), "Grayscale");
        assert!(!adj.description().is_empty());
    }

    #[test]
    fn test_grayscale_luminance_conversion() {
        let adj = GrayscaleAdjustment::with_method(GrayscaleMethod::Luminance);
        let red_pixel = RgbaPixel::new(255, 0, 0, 255);

        let result = adj.apply_to_pixel(red_pixel).unwrap();

        // Red should convert to a specific gray value based on luminance
        let expected_gray = (255.0 * 0.299) as u8;
        assert_eq!(result.r, expected_gray);
        assert_eq!(result.g, expected_gray);
        assert_eq!(result.b, expected_gray);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_grayscale_average_conversion() {
        let adj = GrayscaleAdjustment::with_method(GrayscaleMethod::Average);
        let pixel = RgbaPixel::new(120, 180, 60, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be average of RGB values
        let expected_gray = ((120u32 + 180u32 + 60u32) / 3) as u8;
        assert_eq!(result.r, expected_gray);
        assert_eq!(result.g, expected_gray);
        assert_eq!(result.b, expected_gray);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_grayscale_partial_opacity() {
        let mut adj = GrayscaleAdjustment::new();
        adj.set_opacity(0.5);
        let pixel = RgbaPixel::new(200, 100, 50, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();

        // Should be blend between original and grayscale
        assert!(result.r > 50 && result.r < 200);
        assert!(result.g > 50 && result.g < 200);
        assert!(result.b > 50 && result.b < 200);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_grayscale_parameters() {
        let mut adj = GrayscaleAdjustment::new();
        let params = serde_json::json!({
            "method": "Custom",
            "red_weight": 0.4,
            "green_weight": 0.4,
            "blue_weight": 0.2,
            "opacity": 0.8
        });

        adj.set_parameters(params).unwrap();
        assert_eq!(adj.method, GrayscaleMethod::Custom);
        assert_eq!(adj.red_weight, 0.4);
        assert_eq!(adj.green_weight, 0.4);
        assert_eq!(adj.blue_weight, 0.2);
        assert_eq!(adj.opacity, 0.8);
    }

    #[test]
    fn test_grayscale_apply() {
        let adj = GrayscaleAdjustment::new();
        let mut pixel_data = PixelData::new_rgba(2, 1);

        pixel_data
            .set_pixel(0, 0, RgbaPixel::new(255, 0, 0, 255))
            .unwrap(); // Red
        pixel_data
            .set_pixel(1, 0, RgbaPixel::new(0, 255, 0, 255))
            .unwrap(); // Green

        adj.apply(&mut pixel_data).unwrap();

        // Both pixels should be grayscale now
        let pixel1 = pixel_data.get_pixel(0, 0).unwrap();
        let pixel2 = pixel_data.get_pixel(1, 0).unwrap();

        assert_eq!(pixel1.r, pixel1.g);
        assert_eq!(pixel1.g, pixel1.b);
        assert_eq!(pixel2.r, pixel2.g);
        assert_eq!(pixel2.g, pixel2.b);
    }
}
