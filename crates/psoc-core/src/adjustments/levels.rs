//! Levels adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Levels adjustment
///
/// Provides input/output level control and gamma correction for precise
/// tonal range adjustment. Similar to Photoshop's Levels adjustment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelsAdjustment {
    /// Input black point (0-255)
    pub input_black: u8,
    /// Input white point (0-255)
    pub input_white: u8,
    /// Gamma correction value (0.1-9.99, 1.0 = no change)
    pub gamma: f32,
    /// Output black point (0-255)
    pub output_black: u8,
    /// Output white point (0-255)
    pub output_white: u8,
    /// Whether to apply to individual channels or RGB composite
    pub per_channel: bool,
    /// Red channel levels (only used if per_channel is true)
    pub red_input_black: u8,
    pub red_input_white: u8,
    pub red_gamma: f32,
    pub red_output_black: u8,
    pub red_output_white: u8,
    /// Green channel levels (only used if per_channel is true)
    pub green_input_black: u8,
    pub green_input_white: u8,
    pub green_gamma: f32,
    pub green_output_black: u8,
    pub green_output_white: u8,
    /// Blue channel levels (only used if per_channel is true)
    pub blue_input_black: u8,
    pub blue_input_white: u8,
    pub blue_gamma: f32,
    pub blue_output_black: u8,
    pub blue_output_white: u8,
}

impl LevelsAdjustment {
    /// Create a new levels adjustment with identity settings
    pub fn new() -> Self {
        Self {
            input_black: 0,
            input_white: 255,
            gamma: 1.0,
            output_black: 0,
            output_white: 255,
            per_channel: false,
            red_input_black: 0,
            red_input_white: 255,
            red_gamma: 1.0,
            red_output_black: 0,
            red_output_white: 255,
            green_input_black: 0,
            green_input_white: 255,
            green_gamma: 1.0,
            green_output_black: 0,
            green_output_white: 255,
            blue_input_black: 0,
            blue_input_white: 255,
            blue_gamma: 1.0,
            blue_output_black: 0,
            blue_output_white: 255,
        }
    }

    /// Create an identity levels adjustment (no change)
    pub fn identity() -> Self {
        Self::new()
    }

    /// Check if this adjustment would make no changes
    pub fn is_identity(&self) -> bool {
        let rgb_identity = self.input_black == 0
            && self.input_white == 255
            && (self.gamma - 1.0).abs() < 1e-6
            && self.output_black == 0
            && self.output_white == 255;

        if !self.per_channel {
            return rgb_identity;
        }

        let red_identity = self.red_input_black == 0
            && self.red_input_white == 255
            && (self.red_gamma - 1.0).abs() < 1e-6
            && self.red_output_black == 0
            && self.red_output_white == 255;

        let green_identity = self.green_input_black == 0
            && self.green_input_white == 255
            && (self.green_gamma - 1.0).abs() < 1e-6
            && self.green_output_black == 0
            && self.green_output_white == 255;

        let blue_identity = self.blue_input_black == 0
            && self.blue_input_white == 255
            && (self.blue_gamma - 1.0).abs() < 1e-6
            && self.blue_output_black == 0
            && self.blue_output_white == 255;

        rgb_identity && red_identity && green_identity && blue_identity
    }

    /// Set RGB levels
    pub fn set_rgb_levels(&mut self, input_black: u8, input_white: u8, gamma: f32, output_black: u8, output_white: u8) {
        self.input_black = input_black.min(input_white);
        self.input_white = input_white.max(input_black);
        self.gamma = gamma.clamp(0.1, 9.99);
        self.output_black = output_black.min(output_white);
        self.output_white = output_white.max(output_black);
    }

    /// Set red channel levels
    pub fn set_red_levels(&mut self, input_black: u8, input_white: u8, gamma: f32, output_black: u8, output_white: u8) {
        self.red_input_black = input_black.min(input_white);
        self.red_input_white = input_white.max(input_black);
        self.red_gamma = gamma.clamp(0.1, 9.99);
        self.red_output_black = output_black.min(output_white);
        self.red_output_white = output_white.max(output_black);
        self.per_channel = true;
    }

    /// Set green channel levels
    pub fn set_green_levels(&mut self, input_black: u8, input_white: u8, gamma: f32, output_black: u8, output_white: u8) {
        self.green_input_black = input_black.min(input_white);
        self.green_input_white = input_white.max(input_black);
        self.green_gamma = gamma.clamp(0.1, 9.99);
        self.green_output_black = output_black.min(output_white);
        self.green_output_white = output_white.max(output_black);
        self.per_channel = true;
    }

    /// Set blue channel levels
    pub fn set_blue_levels(&mut self, input_black: u8, input_white: u8, gamma: f32, output_black: u8, output_white: u8) {
        self.blue_input_black = input_black.min(input_white);
        self.blue_input_white = input_white.max(input_black);
        self.blue_gamma = gamma.clamp(0.1, 9.99);
        self.blue_output_black = output_black.min(output_white);
        self.blue_output_white = output_white.max(output_black);
        self.per_channel = true;
    }

    /// Auto-adjust levels based on image histogram
    pub fn auto_levels(&mut self, pixel_data: &PixelData) {
        let (width, height) = pixel_data.dimensions();

        if width == 0 || height == 0 {
            return;
        }

        let mut min_r = 255u8;
        let mut max_r = 0u8;
        let mut min_g = 255u8;
        let mut max_g = 0u8;
        let mut min_b = 255u8;
        let mut max_b = 0u8;

        // Find min/max values for each channel
        for y in 0..height {
            for x in 0..width {
                if let Some(pixel) = pixel_data.get_pixel(x, y) {
                    min_r = min_r.min(pixel.r);
                    max_r = max_r.max(pixel.r);
                    min_g = min_g.min(pixel.g);
                    max_g = max_g.max(pixel.g);
                    min_b = min_b.min(pixel.b);
                    max_b = max_b.max(pixel.b);
                }
            }
        }

        // Set levels based on found values
        let overall_min = min_r.min(min_g).min(min_b);
        let overall_max = max_r.max(max_g).max(max_b);

        self.set_rgb_levels(overall_min, overall_max, 1.0, 0, 255);

        if self.per_channel {
            self.set_red_levels(min_r, max_r, 1.0, 0, 255);
            self.set_green_levels(min_g, max_g, 1.0, 0, 255);
            self.set_blue_levels(min_b, max_b, 1.0, 0, 255);
        }
    }

    /// Apply levels adjustment to a single channel value
    fn adjust_channel(&self, value: u8, input_black: u8, input_white: u8, gamma: f32, output_black: u8, output_white: u8) -> u8 {
        if input_white == input_black {
            return value;
        }

        // Normalize input to 0-1 range
        let normalized = if value <= input_black {
            0.0
        } else if value >= input_white {
            1.0
        } else {
            (value as f32 - input_black as f32) / (input_white as f32 - input_black as f32)
        };

        // Apply gamma correction
        let gamma_corrected = if (gamma - 1.0).abs() < 1e-6 {
            normalized
        } else {
            normalized.powf(1.0 / gamma)
        };

        // Map to output range
        let output_range = output_white as f32 - output_black as f32;
        let final_value = output_black as f32 + gamma_corrected * output_range;

        final_value.clamp(0.0, 255.0) as u8
    }

    /// Apply levels to a single pixel
    fn apply_to_pixel_internal(&self, pixel: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        let mut r = pixel.r;
        let mut g = pixel.g;
        let mut b = pixel.b;

        // Apply RGB composite levels
        r = self.adjust_channel(r, self.input_black, self.input_white, self.gamma, self.output_black, self.output_white);
        g = self.adjust_channel(g, self.input_black, self.input_white, self.gamma, self.output_black, self.output_white);
        b = self.adjust_channel(b, self.input_black, self.input_white, self.gamma, self.output_black, self.output_white);

        // Apply per-channel levels if enabled
        if self.per_channel {
            r = self.adjust_channel(r, self.red_input_black, self.red_input_white, self.red_gamma, self.red_output_black, self.red_output_white);
            g = self.adjust_channel(g, self.green_input_black, self.green_input_white, self.green_gamma, self.green_output_black, self.green_output_white);
            b = self.adjust_channel(b, self.blue_input_black, self.blue_input_white, self.blue_gamma, self.blue_output_black, self.blue_output_white);
        }

        RgbaPixel::new(r, g, b, pixel.a)
    }
}

impl Default for LevelsAdjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl Adjustment for LevelsAdjustment {
    fn id(&self) -> &'static str {
        "levels"
    }

    fn name(&self) -> &'static str {
        "Levels"
    }

    fn description(&self) -> &'static str {
        "Adjust input/output levels and gamma correction for precise tonal control"
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

                let adjusted_pixel = self.apply_to_pixel_internal(pixel);
                pixel_data.set_pixel(x, y, adjusted_pixel)?;
            }
        }

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        Ok(self.apply_to_pixel_internal(pixel))
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "input_black": self.input_black,
            "input_white": self.input_white,
            "gamma": self.gamma,
            "output_black": self.output_black,
            "output_white": self.output_white,
            "per_channel": self.per_channel,
            "red_input_black": self.red_input_black,
            "red_input_white": self.red_input_white,
            "red_gamma": self.red_gamma,
            "red_output_black": self.red_output_black,
            "red_output_white": self.red_output_white,
            "green_input_black": self.green_input_black,
            "green_input_white": self.green_input_white,
            "green_gamma": self.green_gamma,
            "green_output_black": self.green_output_black,
            "green_output_white": self.green_output_white,
            "blue_input_black": self.blue_input_black,
            "blue_input_white": self.blue_input_white,
            "blue_gamma": self.blue_gamma,
            "blue_output_black": self.blue_output_black,
            "blue_output_white": self.blue_output_white
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        // RGB composite parameters
        if let Some(value) = parameters.get("input_black").and_then(|v| v.as_u64()) {
            self.input_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("input_white").and_then(|v| v.as_u64()) {
            self.input_white = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("gamma").and_then(|v| v.as_f64()) {
            self.gamma = (value as f32).clamp(0.1, 9.99);
        }
        if let Some(value) = parameters.get("output_black").and_then(|v| v.as_u64()) {
            self.output_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("output_white").and_then(|v| v.as_u64()) {
            self.output_white = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("per_channel").and_then(|v| v.as_bool()) {
            self.per_channel = value;
        }

        // Red channel parameters
        if let Some(value) = parameters.get("red_input_black").and_then(|v| v.as_u64()) {
            self.red_input_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("red_input_white").and_then(|v| v.as_u64()) {
            self.red_input_white = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("red_gamma").and_then(|v| v.as_f64()) {
            self.red_gamma = (value as f32).clamp(0.1, 9.99);
        }
        if let Some(value) = parameters.get("red_output_black").and_then(|v| v.as_u64()) {
            self.red_output_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("red_output_white").and_then(|v| v.as_u64()) {
            self.red_output_white = (value as u8).min(255);
        }

        // Green channel parameters
        if let Some(value) = parameters.get("green_input_black").and_then(|v| v.as_u64()) {
            self.green_input_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("green_input_white").and_then(|v| v.as_u64()) {
            self.green_input_white = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("green_gamma").and_then(|v| v.as_f64()) {
            self.green_gamma = (value as f32).clamp(0.1, 9.99);
        }
        if let Some(value) = parameters.get("green_output_black").and_then(|v| v.as_u64()) {
            self.green_output_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("green_output_white").and_then(|v| v.as_u64()) {
            self.green_output_white = (value as u8).min(255);
        }

        // Blue channel parameters
        if let Some(value) = parameters.get("blue_input_black").and_then(|v| v.as_u64()) {
            self.blue_input_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("blue_input_white").and_then(|v| v.as_u64()) {
            self.blue_input_white = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("blue_gamma").and_then(|v| v.as_f64()) {
            self.blue_gamma = (value as f32).clamp(0.1, 9.99);
        }
        if let Some(value) = parameters.get("blue_output_black").and_then(|v| v.as_u64()) {
            self.blue_output_black = (value as u8).min(255);
        }
        if let Some(value) = parameters.get("blue_output_white").and_then(|v| v.as_u64()) {
            self.blue_output_white = (value as u8).min(255);
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
    fn test_levels_adjustment_creation() {
        let adjustment = LevelsAdjustment::new();
        assert!(adjustment.is_identity());
        assert!(!adjustment.per_channel);

        let identity = LevelsAdjustment::identity();
        assert!(identity.is_identity());
    }

    #[test]
    fn test_levels_adjustment_metadata() {
        let adjustment = LevelsAdjustment::new();
        assert_eq!(adjustment.id(), "levels");
        assert_eq!(adjustment.name(), "Levels");
        assert!(!adjustment.description().is_empty());
    }

    #[test]
    fn test_levels_adjustment_identity() {
        let adjustment = LevelsAdjustment::identity();
        assert!(adjustment.is_identity());

        let pixel = RgbaPixel::new(100, 150, 200, 255);
        let result = adjustment.apply_to_pixel(pixel).unwrap();
        assert_eq!(result, pixel);
    }

    #[test]
    fn test_levels_adjustment_set_rgb_levels() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_rgb_levels(50, 200, 1.5, 10, 240);

        assert_eq!(adjustment.input_black, 50);
        assert_eq!(adjustment.input_white, 200);
        assert_eq!(adjustment.gamma, 1.5);
        assert_eq!(adjustment.output_black, 10);
        assert_eq!(adjustment.output_white, 240);
        assert!(!adjustment.is_identity());
    }

    #[test]
    fn test_levels_adjustment_set_channel_levels() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_red_levels(30, 220, 0.8, 5, 250);

        assert!(adjustment.per_channel);
        assert_eq!(adjustment.red_input_black, 30);
        assert_eq!(adjustment.red_input_white, 220);
        assert_eq!(adjustment.red_gamma, 0.8);
        assert_eq!(adjustment.red_output_black, 5);
        assert_eq!(adjustment.red_output_white, 250);
    }

    #[test]
    fn test_levels_adjustment_clamping() {
        let mut adjustment = LevelsAdjustment::new();

        // Test gamma clamping
        adjustment.set_rgb_levels(0, 255, 15.0, 0, 255);
        assert_eq!(adjustment.gamma, 9.99);

        adjustment.set_rgb_levels(0, 255, -1.0, 0, 255);
        assert_eq!(adjustment.gamma, 0.1);

        // Test input/output order enforcement
        adjustment.set_rgb_levels(200, 50, 1.0, 240, 10);
        assert_eq!(adjustment.input_black, 50);
        assert_eq!(adjustment.input_white, 200);
        assert_eq!(adjustment.output_black, 10);
        assert_eq!(adjustment.output_white, 240);
    }

    #[test]
    fn test_levels_adjustment_apply() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_rgb_levels(50, 200, 1.0, 0, 255);

        // Test black point mapping
        let black_pixel = RgbaPixel::new(50, 50, 50, 255);
        let result = adjustment.apply_to_pixel(black_pixel).unwrap();
        assert_eq!(result.r, 0);
        assert_eq!(result.g, 0);
        assert_eq!(result.b, 0);
        assert_eq!(result.a, 255);

        // Test white point mapping
        let white_pixel = RgbaPixel::new(200, 200, 200, 255);
        let result = adjustment.apply_to_pixel(white_pixel).unwrap();
        assert_eq!(result.r, 255);
        assert_eq!(result.g, 255);
        assert_eq!(result.b, 255);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_levels_adjustment_gamma() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_rgb_levels(0, 255, 2.0, 0, 255);

        let mid_pixel = RgbaPixel::new(128, 128, 128, 255);
        let _result = adjustment.apply_to_pixel(mid_pixel).unwrap();

        // With gamma 2.0, midtones should be darker
        // 128/255 = 0.502, (0.502)^(1/2.0) = 0.708, 0.708 * 255 = 180.5 ≈ 181
        // So the result should be around 181, which is greater than 128
        // Let's test with gamma 0.5 instead to make midtones brighter
        adjustment.set_rgb_levels(0, 255, 0.5, 0, 255);
        let result = adjustment.apply_to_pixel(mid_pixel).unwrap();

        // With gamma 0.5, midtones should be darker
        println!("Gamma 0.5 result: r={}, g={}, b={}", result.r, result.g, result.b);
        // 128/255 = 0.502, (0.502)^(1/0.5) = (0.502)^2 = 0.252, 0.252 * 255 = 64.3 ≈ 64
        // So with gamma 0.5, the result should be darker
        assert!(result.r < 128);
        assert!(result.g < 128);
        assert!(result.b < 128);

        // Test with gamma > 1 to make midtones brighter
        adjustment.set_rgb_levels(0, 255, 3.0, 0, 255);
        let result = adjustment.apply_to_pixel(mid_pixel).unwrap();
        println!("Gamma 3.0 result: r={}, g={}, b={}", result.r, result.g, result.b);

        // With gamma 3.0, midtones should be brighter
        assert!(result.r > 128);
        assert!(result.g > 128);
        assert!(result.b > 128);
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_levels_adjustment_output_range() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_rgb_levels(0, 255, 1.0, 50, 200);

        let black_pixel = RgbaPixel::new(0, 0, 0, 255);
        let result = adjustment.apply_to_pixel(black_pixel).unwrap();
        assert_eq!(result.r, 50);
        assert_eq!(result.g, 50);
        assert_eq!(result.b, 50);

        let white_pixel = RgbaPixel::new(255, 255, 255, 255);
        let result = adjustment.apply_to_pixel(white_pixel).unwrap();
        assert_eq!(result.r, 200);
        assert_eq!(result.g, 200);
        assert_eq!(result.b, 200);
    }

    #[test]
    fn test_levels_adjustment_per_channel() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_red_levels(0, 255, 1.0, 0, 128);
        adjustment.set_green_levels(0, 255, 1.0, 0, 255);
        adjustment.set_blue_levels(0, 255, 1.0, 0, 64);

        let pixel = RgbaPixel::new(255, 255, 255, 255);
        let result = adjustment.apply_to_pixel(pixel).unwrap();

        assert_eq!(result.r, 128); // Red channel limited to 128
        assert_eq!(result.g, 255); // Green channel unchanged
        assert_eq!(result.b, 64);  // Blue channel limited to 64
        assert_eq!(result.a, 255);
    }

    #[test]
    fn test_levels_adjustment_parameters() {
        let mut adjustment = LevelsAdjustment::new();
        adjustment.set_rgb_levels(30, 220, 1.2, 10, 240);
        adjustment.per_channel = true;

        let params = adjustment.get_parameters();
        assert_eq!(params.get("input_black").unwrap().as_u64().unwrap(), 30);
        assert_eq!(params.get("input_white").unwrap().as_u64().unwrap(), 220);
        assert!((params.get("gamma").unwrap().as_f64().unwrap() - 1.2).abs() < 1e-6);
        assert_eq!(params.get("output_black").unwrap().as_u64().unwrap(), 10);
        assert_eq!(params.get("output_white").unwrap().as_u64().unwrap(), 240);
        assert!(params.get("per_channel").unwrap().as_bool().unwrap());

        // Test parameter setting
        let new_params = serde_json::json!({
            "input_black": 50,
            "input_white": 200,
            "gamma": 0.8,
            "output_black": 20,
            "output_white": 230,
            "per_channel": false
        });

        adjustment.set_parameters(new_params).unwrap();
        assert_eq!(adjustment.input_black, 50);
        assert_eq!(adjustment.input_white, 200);
        assert!((adjustment.gamma - 0.8).abs() < 1e-6);
        assert_eq!(adjustment.output_black, 20);
        assert_eq!(adjustment.output_white, 230);
        assert!(!adjustment.per_channel);
    }

    #[test]
    fn test_levels_adjustment_clone() {
        let adjustment = LevelsAdjustment::new();
        let cloned = adjustment.clone_adjustment();

        assert_eq!(cloned.id(), adjustment.id());
        assert_eq!(cloned.name(), adjustment.name());
    }

    #[test]
    fn test_levels_adjustment_auto_levels() {
        let mut pixel_data = PixelData::new_rgba(3, 3);

        // Fill all pixels with test data
        for y in 0..3 {
            for x in 0..3 {
                let pixel = if x == 0 && y == 0 {
                    RgbaPixel::new(50, 60, 70, 255)  // Min values
                } else if x == 1 && y == 1 {
                    RgbaPixel::new(200, 190, 180, 255)  // Max values
                } else {
                    RgbaPixel::new(100, 120, 140, 255)  // Mid values
                };
                pixel_data.set_pixel(x, y, pixel).unwrap();
            }
        }

        let mut adjustment = LevelsAdjustment::new();
        adjustment.auto_levels(&pixel_data);

        // Should set input levels based on min/max found
        assert_eq!(adjustment.input_black, 50);  // Overall minimum
        assert_eq!(adjustment.input_white, 200); // Overall maximum
        assert_eq!(adjustment.output_black, 0);
        assert_eq!(adjustment.output_white, 255);
    }
}
