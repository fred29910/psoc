//! Color balance adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Tonal range for color balance adjustments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TonalRange {
    /// Shadow tones (darker areas)
    Shadows,
    /// Midtone areas (middle brightness)
    Midtones,
    /// Highlight areas (brighter areas)
    Highlights,
}

impl Default for TonalRange {
    fn default() -> Self {
        Self::Midtones
    }
}

impl TonalRange {
    /// Get the display name for this tonal range
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Shadows => "Shadows",
            Self::Midtones => "Midtones",
            Self::Highlights => "Highlights",
        }
    }

    /// Calculate the weight for this tonal range based on pixel luminance
    fn calculate_weight(&self, luminance: f32) -> f32 {
        match self {
            Self::Shadows => {
                // Shadows: weight decreases as luminance increases
                if luminance < 0.33 {
                    1.0 - (luminance / 0.33)
                } else {
                    0.0
                }
            }
            Self::Midtones => {
                // Midtones: bell curve centered at 0.5
                if luminance < 0.5 {
                    luminance / 0.5
                } else {
                    2.0 - (luminance / 0.5)
                }
            }
            Self::Highlights => {
                // Highlights: weight increases as luminance increases
                if luminance > 0.67 {
                    (luminance - 0.67) / 0.33
                } else {
                    0.0
                }
            }
        }
    }
}

/// Color balance adjustment
///
/// Adjusts the color balance in shadows, midtones, and highlights independently.
/// Provides control over cyan-red, magenta-green, and yellow-blue color axes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorBalanceAdjustment {
    /// Cyan-Red balance for shadows (-1.0 = more cyan, 1.0 = more red)
    pub shadows_cyan_red: f32,
    /// Magenta-Green balance for shadows (-1.0 = more magenta, 1.0 = more green)
    pub shadows_magenta_green: f32,
    /// Yellow-Blue balance for shadows (-1.0 = more yellow, 1.0 = more blue)
    pub shadows_yellow_blue: f32,

    /// Cyan-Red balance for midtones
    pub midtones_cyan_red: f32,
    /// Magenta-Green balance for midtones
    pub midtones_magenta_green: f32,
    /// Yellow-Blue balance for midtones
    pub midtones_yellow_blue: f32,

    /// Cyan-Red balance for highlights
    pub highlights_cyan_red: f32,
    /// Magenta-Green balance for highlights
    pub highlights_magenta_green: f32,
    /// Yellow-Blue balance for highlights
    pub highlights_yellow_blue: f32,

    /// Whether to preserve luminosity during adjustment
    pub preserve_luminosity: bool,
}

impl ColorBalanceAdjustment {
    /// Create a new color balance adjustment with neutral settings
    pub fn new() -> Self {
        Self {
            shadows_cyan_red: 0.0,
            shadows_magenta_green: 0.0,
            shadows_yellow_blue: 0.0,
            midtones_cyan_red: 0.0,
            midtones_magenta_green: 0.0,
            midtones_yellow_blue: 0.0,
            highlights_cyan_red: 0.0,
            highlights_magenta_green: 0.0,
            highlights_yellow_blue: 0.0,
            preserve_luminosity: true,
        }
    }

    /// Set color balance for a specific tonal range
    pub fn set_balance_for_range(
        &mut self,
        range: TonalRange,
        cyan_red: f32,
        magenta_green: f32,
        yellow_blue: f32,
    ) {
        let cyan_red = cyan_red.clamp(-1.0, 1.0);
        let magenta_green = magenta_green.clamp(-1.0, 1.0);
        let yellow_blue = yellow_blue.clamp(-1.0, 1.0);

        match range {
            TonalRange::Shadows => {
                self.shadows_cyan_red = cyan_red;
                self.shadows_magenta_green = magenta_green;
                self.shadows_yellow_blue = yellow_blue;
            }
            TonalRange::Midtones => {
                self.midtones_cyan_red = cyan_red;
                self.midtones_magenta_green = magenta_green;
                self.midtones_yellow_blue = yellow_blue;
            }
            TonalRange::Highlights => {
                self.highlights_cyan_red = cyan_red;
                self.highlights_magenta_green = magenta_green;
                self.highlights_yellow_blue = yellow_blue;
            }
        }
    }

    /// Get color balance for a specific tonal range
    pub fn get_balance_for_range(&self, range: TonalRange) -> (f32, f32, f32) {
        match range {
            TonalRange::Shadows => (
                self.shadows_cyan_red,
                self.shadows_magenta_green,
                self.shadows_yellow_blue,
            ),
            TonalRange::Midtones => (
                self.midtones_cyan_red,
                self.midtones_magenta_green,
                self.midtones_yellow_blue,
            ),
            TonalRange::Highlights => (
                self.highlights_cyan_red,
                self.highlights_magenta_green,
                self.highlights_yellow_blue,
            ),
        }
    }

    /// Check if this adjustment would have no effect
    pub fn is_identity(&self) -> bool {
        self.shadows_cyan_red == 0.0
            && self.shadows_magenta_green == 0.0
            && self.shadows_yellow_blue == 0.0
            && self.midtones_cyan_red == 0.0
            && self.midtones_magenta_green == 0.0
            && self.midtones_yellow_blue == 0.0
            && self.highlights_cyan_red == 0.0
            && self.highlights_magenta_green == 0.0
            && self.highlights_yellow_blue == 0.0
    }

    /// Calculate pixel luminance
    fn calculate_luminance(&self, pixel: RgbaPixel) -> f32 {
        // ITU-R BT.709 luminance formula
        (pixel.r as f32 * 0.299 + pixel.g as f32 * 0.587 + pixel.b as f32 * 0.114) / 255.0
    }

    /// Apply color balance to a single pixel
    fn adjust_pixel(&self, pixel: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        let luminance = self.calculate_luminance(pixel);

        // Calculate weights for each tonal range
        let shadow_weight = TonalRange::Shadows.calculate_weight(luminance);
        let midtone_weight = TonalRange::Midtones.calculate_weight(luminance);
        let highlight_weight = TonalRange::Highlights.calculate_weight(luminance);

        // Calculate total color adjustments
        let total_cyan_red = self.shadows_cyan_red * shadow_weight
            + self.midtones_cyan_red * midtone_weight
            + self.highlights_cyan_red * highlight_weight;

        let total_magenta_green = self.shadows_magenta_green * shadow_weight
            + self.midtones_magenta_green * midtone_weight
            + self.highlights_magenta_green * highlight_weight;

        let total_yellow_blue = self.shadows_yellow_blue * shadow_weight
            + self.midtones_yellow_blue * midtone_weight
            + self.highlights_yellow_blue * highlight_weight;

        // Apply color balance adjustments
        let mut r = pixel.r as f32;
        let mut g = pixel.g as f32;
        let mut b = pixel.b as f32;

        // Cyan-Red adjustment
        if total_cyan_red > 0.0 {
            // More red
            r += total_cyan_red * 50.0;
        } else {
            // More cyan
            g -= total_cyan_red * 25.0;
            b -= total_cyan_red * 25.0;
        }

        // Magenta-Green adjustment
        if total_magenta_green > 0.0 {
            // More green
            g += total_magenta_green * 50.0;
        } else {
            // More magenta
            r -= total_magenta_green * 25.0;
            b -= total_magenta_green * 25.0;
        }

        // Yellow-Blue adjustment
        if total_yellow_blue > 0.0 {
            // More blue
            b += total_yellow_blue * 50.0;
        } else {
            // More yellow
            r -= total_yellow_blue * 25.0;
            g -= total_yellow_blue * 25.0;
        }

        // Clamp values
        r = r.clamp(0.0, 255.0);
        g = g.clamp(0.0, 255.0);
        b = b.clamp(0.0, 255.0);

        let mut result = RgbaPixel::new(r as u8, g as u8, b as u8, pixel.a);

        // Preserve luminosity if requested
        if self.preserve_luminosity {
            let original_luminance = luminance;
            let new_luminance = self.calculate_luminance(result);

            if new_luminance > 0.0 {
                let luminance_ratio = original_luminance / new_luminance;
                result.r = ((result.r as f32 * luminance_ratio).clamp(0.0, 255.0)) as u8;
                result.g = ((result.g as f32 * luminance_ratio).clamp(0.0, 255.0)) as u8;
                result.b = ((result.b as f32 * luminance_ratio).clamp(0.0, 255.0)) as u8;
            }
        }

        result
    }
}

impl Default for ColorBalanceAdjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl Adjustment for ColorBalanceAdjustment {
    fn id(&self) -> &'static str {
        "color_balance"
    }

    fn name(&self) -> &'static str {
        "Color Balance"
    }

    fn description(&self) -> &'static str {
        "Adjusts color balance in shadows, midtones, and highlights"
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
            "shadows_cyan_red": self.shadows_cyan_red,
            "shadows_magenta_green": self.shadows_magenta_green,
            "shadows_yellow_blue": self.shadows_yellow_blue,
            "midtones_cyan_red": self.midtones_cyan_red,
            "midtones_magenta_green": self.midtones_magenta_green,
            "midtones_yellow_blue": self.midtones_yellow_blue,
            "highlights_cyan_red": self.highlights_cyan_red,
            "highlights_magenta_green": self.highlights_magenta_green,
            "highlights_yellow_blue": self.highlights_yellow_blue,
            "preserve_luminosity": self.preserve_luminosity
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        macro_rules! set_param {
            ($field:ident) => {
                if let Some(value) = parameters.get(stringify!($field)).and_then(|v| v.as_f64()) {
                    self.$field = (value as f32).clamp(-1.0, 1.0);
                }
            };
        }

        set_param!(shadows_cyan_red);
        set_param!(shadows_magenta_green);
        set_param!(shadows_yellow_blue);
        set_param!(midtones_cyan_red);
        set_param!(midtones_magenta_green);
        set_param!(midtones_yellow_blue);
        set_param!(highlights_cyan_red);
        set_param!(highlights_magenta_green);
        set_param!(highlights_yellow_blue);

        if let Some(preserve) = parameters
            .get("preserve_luminosity")
            .and_then(|v| v.as_bool())
        {
            self.preserve_luminosity = preserve;
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
    fn test_color_balance_adjustment_creation() {
        let adj = ColorBalanceAdjustment::new();
        assert!(adj.is_identity());
        assert!(adj.preserve_luminosity);
    }

    #[test]
    fn test_tonal_range_display() {
        assert_eq!(TonalRange::Shadows.display_name(), "Shadows");
        assert_eq!(TonalRange::Midtones.display_name(), "Midtones");
        assert_eq!(TonalRange::Highlights.display_name(), "Highlights");
    }

    #[test]
    fn test_color_balance_set_get_range() {
        let mut adj = ColorBalanceAdjustment::new();
        adj.set_balance_for_range(TonalRange::Midtones, 0.5, -0.3, 0.8);

        let (cr, mg, yb) = adj.get_balance_for_range(TonalRange::Midtones);
        assert_eq!(cr, 0.5);
        assert_eq!(mg, -0.3);
        assert_eq!(yb, 0.8);
    }

    #[test]
    fn test_color_balance_clamping() {
        let mut adj = ColorBalanceAdjustment::new();
        adj.set_balance_for_range(TonalRange::Shadows, 2.0, -2.0, 1.5);

        let (cr, mg, yb) = adj.get_balance_for_range(TonalRange::Shadows);
        assert_eq!(cr, 1.0);
        assert_eq!(mg, -1.0);
        assert_eq!(yb, 1.0);
    }

    #[test]
    fn test_color_balance_metadata() {
        let adj = ColorBalanceAdjustment::new();
        assert_eq!(adj.id(), "color_balance");
        assert_eq!(adj.name(), "Color Balance");
        assert!(!adj.description().is_empty());
    }

    #[test]
    fn test_color_balance_luminance_calculation() {
        let adj = ColorBalanceAdjustment::new();
        let white_pixel = RgbaPixel::new(255, 255, 255, 255);
        let black_pixel = RgbaPixel::new(0, 0, 0, 255);
        let gray_pixel = RgbaPixel::new(128, 128, 128, 255);

        assert!((adj.calculate_luminance(white_pixel) - 1.0).abs() < 0.01);
        assert!(adj.calculate_luminance(black_pixel) < 0.01);
        assert!((adj.calculate_luminance(gray_pixel) - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_color_balance_identity() {
        let adj = ColorBalanceAdjustment::new();
        let pixel = RgbaPixel::new(128, 64, 192, 255);

        let result = adj.apply_to_pixel(pixel).unwrap();
        assert_eq!(result, pixel); // Should be unchanged
    }

    #[test]
    fn test_color_balance_midtone_adjustment() {
        let mut adj = ColorBalanceAdjustment::new();
        adj.set_balance_for_range(TonalRange::Midtones, 0.5, 0.0, 0.0); // More red in midtones

        let gray_pixel = RgbaPixel::new(128, 128, 128, 255); // Midtone pixel

        let result = adj.apply_to_pixel(gray_pixel).unwrap();

        // Should have more red
        assert!(result.r >= gray_pixel.r);
        assert_eq!(result.a, 255); // Alpha unchanged
    }

    #[test]
    fn test_color_balance_parameters() {
        let mut adj = ColorBalanceAdjustment::new();
        let params = serde_json::json!({
            "midtones_cyan_red": 0.3,
            "midtones_magenta_green": -0.2,
            "highlights_yellow_blue": 0.4,
            "preserve_luminosity": false
        });

        adj.set_parameters(params).unwrap();
        assert_eq!(adj.midtones_cyan_red, 0.3);
        assert_eq!(adj.midtones_magenta_green, -0.2);
        assert_eq!(adj.highlights_yellow_blue, 0.4);
        assert!(!adj.preserve_luminosity);

        let retrieved_params = adj.get_parameters();
        assert!((retrieved_params["midtones_cyan_red"].as_f64().unwrap() - 0.3).abs() < 0.001);
        assert_eq!(retrieved_params["preserve_luminosity"], false);
    }

    #[test]
    fn test_color_balance_apply() {
        let mut adj = ColorBalanceAdjustment::new();
        adj.set_balance_for_range(TonalRange::Midtones, 0.2, 0.0, 0.0);

        let mut pixel_data = PixelData::new_rgba(2, 1);
        pixel_data
            .set_pixel(0, 0, RgbaPixel::new(128, 128, 128, 255))
            .unwrap(); // Midtone
        pixel_data
            .set_pixel(1, 0, RgbaPixel::new(32, 32, 32, 255))
            .unwrap(); // Shadow

        let original_midtone = pixel_data.get_pixel(0, 0).unwrap();
        let original_shadow = pixel_data.get_pixel(1, 0).unwrap();

        adj.apply(&mut pixel_data).unwrap();

        let adjusted_midtone = pixel_data.get_pixel(0, 0).unwrap();
        let adjusted_shadow = pixel_data.get_pixel(1, 0).unwrap();

        // Midtone should be affected more than shadow
        assert_ne!(adjusted_midtone, original_midtone);
        // Shadow might be less affected or unchanged
        assert_eq!(adjusted_shadow.a, original_shadow.a); // Alpha preserved
    }

    #[test]
    fn test_tonal_range_weight_calculation() {
        // Test shadow weight
        let shadow_weight_dark = TonalRange::Shadows.calculate_weight(0.1);
        let shadow_weight_bright = TonalRange::Shadows.calculate_weight(0.8);
        assert!(shadow_weight_dark > shadow_weight_bright);

        // Test highlight weight
        let highlight_weight_dark = TonalRange::Highlights.calculate_weight(0.2);
        let highlight_weight_bright = TonalRange::Highlights.calculate_weight(0.9);
        assert!(highlight_weight_bright > highlight_weight_dark);

        // Test midtone weight
        let midtone_weight_mid = TonalRange::Midtones.calculate_weight(0.5);
        let midtone_weight_extreme = TonalRange::Midtones.calculate_weight(0.1);
        assert!(midtone_weight_mid > midtone_weight_extreme);
    }
}
