//! Sharpen filter implementations

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Unsharp mask filter
///
/// Applies unsharp masking for image sharpening. This is the standard
/// sharpening technique used in professional image editing software.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsharpMaskFilter {
    /// Amount of sharpening (0.0 to 5.0)
    pub amount: f32,
    /// Radius for the blur mask (0.1 to 10.0)
    pub radius: f32,
    /// Threshold to avoid sharpening noise (0 to 255)
    pub threshold: u8,
}

impl UnsharpMaskFilter {
    /// Create a new unsharp mask filter
    pub fn new(amount: f32, radius: f32, threshold: u8) -> Self {
        Self {
            amount: amount.clamp(0.0, 5.0),
            radius: radius.clamp(0.1, 10.0),
            threshold,
        }
    }

    /// Create an identity unsharp mask filter (no sharpening)
    pub fn identity() -> Self {
        Self::new(0.0, 1.0, 0)
    }

    /// Check if this filter would make no changes
    pub fn is_identity(&self) -> bool {
        self.amount < 0.01
    }

    /// Set the sharpening amount
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.clamp(0.0, 5.0);
    }

    /// Set the blur radius
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius.clamp(0.1, 10.0);
    }

    /// Set the threshold
    pub fn set_threshold(&mut self, threshold: u8) {
        self.threshold = threshold;
    }

    /// Apply unsharp mask to a single pixel
    fn apply_unsharp_mask(&self, original: RgbaPixel, blurred: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return original;
        }

        let apply_to_channel = |orig: u8, blur: u8| -> u8 {
            let diff = orig as i32 - blur as i32;
            
            // Apply threshold
            if diff.abs() < self.threshold as i32 {
                return orig;
            }
            
            // Apply sharpening
            let sharpened = orig as f32 + diff as f32 * self.amount;
            sharpened.clamp(0.0, 255.0) as u8
        };

        RgbaPixel::new(
            apply_to_channel(original.r, blurred.r),
            apply_to_channel(original.g, blurred.g),
            apply_to_channel(original.b, blurred.b),
            original.a, // Alpha unchanged
        )
    }

    /// Create a simple box blur for the mask
    fn create_blur_mask(&self, pixel_data: &PixelData) -> Result<PixelData> {
        let (width, height) = pixel_data.dimensions();
        let mut blurred = PixelData::new_rgba(width, height);
        
        let kernel_size = (self.radius * 2.0).ceil() as usize;
        let kernel_size = if kernel_size % 2 == 0 { kernel_size + 1 } else { kernel_size };
        let half_kernel = kernel_size / 2;

        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0u32;

                for ky in 0..kernel_size {
                    for kx in 0..kernel_size {
                        let sample_x = x as i32 + kx as i32 - half_kernel as i32;
                        let sample_y = y as i32 + ky as i32 - half_kernel as i32;
                        
                        if sample_x >= 0 && sample_x < width as i32 
                            && sample_y >= 0 && sample_y < height as i32 {
                            if let Some(pixel) = pixel_data.get_pixel(sample_x as u32, sample_y as u32) {
                                r_sum += pixel.r as u32;
                                g_sum += pixel.g as u32;
                                b_sum += pixel.b as u32;
                                a_sum += pixel.a as u32;
                                count += 1;
                            }
                        }
                    }
                }

                if count > 0 {
                    let blurred_pixel = RgbaPixel::new(
                        (r_sum / count) as u8,
                        (g_sum / count) as u8,
                        (b_sum / count) as u8,
                        (a_sum / count) as u8,
                    );
                    blurred.set_pixel(x, y, blurred_pixel)?;
                }
            }
        }

        Ok(blurred)
    }
}

impl Default for UnsharpMaskFilter {
    fn default() -> Self {
        Self::new(1.0, 1.0, 0)
    }
}

impl Adjustment for UnsharpMaskFilter {
    fn id(&self) -> &'static str {
        "unsharp_mask"
    }

    fn name(&self) -> &'static str {
        "Unsharp Mask"
    }

    fn description(&self) -> &'static str {
        "Sharpen image using unsharp masking technique"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();
        
        // Create blurred version
        let blurred = self.create_blur_mask(pixel_data)?;
        
        // Apply unsharp mask
        for y in 0..height {
            for x in 0..width {
                if let (Some(original), Some(blur)) = (
                    pixel_data.get_pixel(x, y),
                    blurred.get_pixel(x, y)
                ) {
                    let sharpened = self.apply_unsharp_mask(original, blur);
                    pixel_data.set_pixel(x, y, sharpened)?;
                }
            }
        }

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        // For single pixel, sharpening has no effect
        Ok(pixel)
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "amount": self.amount,
            "radius": self.radius,
            "threshold": self.threshold
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(amount) = parameters.get("amount").and_then(|v| v.as_f64()) {
            self.set_amount(amount as f32);
        }
        if let Some(radius) = parameters.get("radius").and_then(|v| v.as_f64()) {
            self.set_radius(radius as f32);
        }
        if let Some(threshold) = parameters.get("threshold").and_then(|v| v.as_u64()) {
            self.set_threshold(threshold as u8);
        }
        Ok(())
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}

/// Simple sharpen filter
///
/// Applies a basic 3x3 sharpening kernel for quick sharpening.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharpenFilter {
    /// Sharpening strength (0.0 to 2.0)
    pub strength: f32,
}

impl SharpenFilter {
    /// Create a new sharpen filter
    pub fn new(strength: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 2.0),
        }
    }

    /// Create an identity sharpen filter (no sharpening)
    pub fn identity() -> Self {
        Self::new(0.0)
    }

    /// Check if this filter would make no changes
    pub fn is_identity(&self) -> bool {
        self.strength < 0.01
    }

    /// Set the sharpening strength
    pub fn set_strength(&mut self, strength: f32) {
        self.strength = strength.clamp(0.0, 2.0);
    }
}

impl Default for SharpenFilter {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Adjustment for SharpenFilter {
    fn id(&self) -> &'static str {
        "sharpen"
    }

    fn name(&self) -> &'static str {
        "Sharpen"
    }

    fn description(&self) -> &'static str {
        "Apply basic sharpening filter"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();
        let mut result_data = PixelData::new_rgba(width, height);
        
        // 3x3 sharpening kernel
        let kernel = [
            [0.0, -self.strength, 0.0],
            [-self.strength, 1.0 + 4.0 * self.strength, -self.strength],
            [0.0, -self.strength, 0.0],
        ];

        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                let mut a_sum = 0.0f32;

                for ky in 0..3 {
                    for kx in 0..3 {
                        let sample_x = x as i32 + kx as i32 - 1;
                        let sample_y = y as i32 + ky as i32 - 1;
                        
                        let pixel = if sample_x >= 0 && sample_x < width as i32 
                            && sample_y >= 0 && sample_y < height as i32 {
                            pixel_data.get_pixel(sample_x as u32, sample_y as u32)
                                .unwrap_or_else(|| RgbaPixel::new(0, 0, 0, 0))
                        } else {
                            // Use edge pixel for out-of-bounds
                            pixel_data.get_pixel(x, y)
                                .unwrap_or_else(|| RgbaPixel::new(0, 0, 0, 0))
                        };
                        
                        let weight = kernel[ky][kx];
                        r_sum += pixel.r as f32 * weight;
                        g_sum += pixel.g as f32 * weight;
                        b_sum += pixel.b as f32 * weight;
                        a_sum += pixel.a as f32 * weight;
                    }
                }

                let result_pixel = RgbaPixel::new(
                    r_sum.clamp(0.0, 255.0) as u8,
                    g_sum.clamp(0.0, 255.0) as u8,
                    b_sum.clamp(0.0, 255.0) as u8,
                    a_sum.clamp(0.0, 255.0) as u8,
                );
                result_data.set_pixel(x, y, result_pixel)?;
            }
        }

        // Copy result back to original
        *pixel_data = result_data;
        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        // For single pixel, sharpening has no effect
        Ok(pixel)
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "strength": self.strength
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(strength) = parameters.get("strength").and_then(|v| v.as_f64()) {
            self.set_strength(strength as f32);
        }
        Ok(())
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}
