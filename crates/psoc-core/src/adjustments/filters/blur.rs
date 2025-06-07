//! Blur filter implementations

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, math::gaussian_kernel_1d, PixelData, RgbaPixel};

/// Gaussian blur filter
///
/// Applies a Gaussian blur effect to the image using separable convolution
/// for optimal performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaussianBlurFilter {
    /// Blur radius in pixels (0.0 to 100.0)
    pub radius: f32,
    /// Quality factor for kernel size (1.0 to 3.0)
    /// Higher values create smoother blur but are slower
    pub quality: f32,
}

impl GaussianBlurFilter {
    /// Create a new Gaussian blur filter
    pub fn new(radius: f32) -> Self {
        Self {
            radius: radius.clamp(0.0, 100.0),
            quality: 2.0,
        }
    }

    /// Create a new Gaussian blur filter with custom quality
    pub fn with_quality(radius: f32, quality: f32) -> Self {
        Self {
            radius: radius.clamp(0.0, 100.0),
            quality: quality.clamp(1.0, 3.0),
        }
    }

    /// Create an identity blur filter (no blur)
    pub fn identity() -> Self {
        Self::new(0.0)
    }

    /// Check if this filter would make no changes
    pub fn is_identity(&self) -> bool {
        self.radius < 0.01
    }

    /// Set the blur radius
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius.clamp(0.0, 100.0);
    }

    /// Set the quality factor
    pub fn set_quality(&mut self, quality: f32) {
        self.quality = quality.clamp(1.0, 3.0);
    }

    /// Calculate the kernel size based on radius and quality
    fn calculate_kernel_size(&self) -> usize {
        if self.radius < 0.01 {
            return 1;
        }

        let size = (self.radius * self.quality * 2.0).ceil() as usize;
        // Ensure odd size and minimum of 3
        let size = if size % 2 == 0 { size + 1 } else { size };
        size.clamp(3, 201) // Cap at reasonable maximum
    }

    /// Apply horizontal blur pass
    fn blur_horizontal(&self, input: &PixelData, output: &mut PixelData) -> Result<()> {
        let (width, height) = input.dimensions();
        let kernel_size = self.calculate_kernel_size();
        let kernel = gaussian_kernel_1d(kernel_size, self.radius);
        let half_kernel = kernel_size / 2;

        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                let mut a_sum = 0.0f32;
                let mut weight_sum = 0.0f32;

                for (i, &weight) in kernel.iter().enumerate().take(kernel_size) {
                    let sample_x = x as i32 + i as i32 - half_kernel as i32;

                    if sample_x >= 0 && sample_x < width as i32 {
                        if let Some(pixel) = input.get_pixel(sample_x as u32, y) {
                            r_sum += pixel.r as f32 * weight;
                            g_sum += pixel.g as f32 * weight;
                            b_sum += pixel.b as f32 * weight;
                            a_sum += pixel.a as f32 * weight;
                            weight_sum += weight;
                        }
                    }
                }

                if weight_sum > 0.0 {
                    let result_pixel = RgbaPixel::new(
                        (r_sum / weight_sum).clamp(0.0, 255.0) as u8,
                        (g_sum / weight_sum).clamp(0.0, 255.0) as u8,
                        (b_sum / weight_sum).clamp(0.0, 255.0) as u8,
                        (a_sum / weight_sum).clamp(0.0, 255.0) as u8,
                    );
                    output.set_pixel(x, y, result_pixel)?;
                }
            }
        }

        Ok(())
    }

    /// Apply vertical blur pass
    fn blur_vertical(&self, input: &PixelData, output: &mut PixelData) -> Result<()> {
        let (width, height) = input.dimensions();
        let kernel_size = self.calculate_kernel_size();
        let kernel = gaussian_kernel_1d(kernel_size, self.radius);
        let half_kernel = kernel_size / 2;

        for x in 0..width {
            for y in 0..height {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                let mut a_sum = 0.0f32;
                let mut weight_sum = 0.0f32;

                for (i, &weight) in kernel.iter().enumerate().take(kernel_size) {
                    let sample_y = y as i32 + i as i32 - half_kernel as i32;

                    if sample_y >= 0 && sample_y < height as i32 {
                        if let Some(pixel) = input.get_pixel(x, sample_y as u32) {
                            r_sum += pixel.r as f32 * weight;
                            g_sum += pixel.g as f32 * weight;
                            b_sum += pixel.b as f32 * weight;
                            a_sum += pixel.a as f32 * weight;
                            weight_sum += weight;
                        }
                    }
                }

                if weight_sum > 0.0 {
                    let result_pixel = RgbaPixel::new(
                        (r_sum / weight_sum).clamp(0.0, 255.0) as u8,
                        (g_sum / weight_sum).clamp(0.0, 255.0) as u8,
                        (b_sum / weight_sum).clamp(0.0, 255.0) as u8,
                        (a_sum / weight_sum).clamp(0.0, 255.0) as u8,
                    );
                    output.set_pixel(x, y, result_pixel)?;
                }
            }
        }

        Ok(())
    }
}

impl Default for GaussianBlurFilter {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Adjustment for GaussianBlurFilter {
    fn id(&self) -> &'static str {
        "gaussian_blur"
    }

    fn name(&self) -> &'static str {
        "Gaussian Blur"
    }

    fn description(&self) -> &'static str {
        "Apply Gaussian blur effect to soften the image"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();

        // Create temporary buffer for horizontal pass
        let mut temp_data = PixelData::new_rgba(width, height);

        // Apply horizontal blur
        self.blur_horizontal(pixel_data, &mut temp_data)?;

        // Apply vertical blur (back to original)
        self.blur_vertical(&temp_data, pixel_data)?;

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        // For single pixel, blur has no effect
        Ok(pixel)
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "radius": self.radius,
            "quality": self.quality
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(radius) = parameters.get("radius").and_then(|v| v.as_f64()) {
            self.set_radius(radius as f32);
        }
        if let Some(quality) = parameters.get("quality").and_then(|v| v.as_f64()) {
            self.set_quality(quality as f32);
        }
        Ok(())
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}

/// Motion blur filter
///
/// Applies directional motion blur effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionBlurFilter {
    /// Blur distance in pixels
    pub distance: f32,
    /// Blur angle in degrees (0-360)
    pub angle: f32,
}

impl MotionBlurFilter {
    /// Create a new motion blur filter
    pub fn new(distance: f32, angle: f32) -> Self {
        Self {
            distance: distance.clamp(0.0, 100.0),
            angle: angle % 360.0,
        }
    }

    /// Create an identity motion blur filter (no blur)
    pub fn identity() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Check if this filter would make no changes
    pub fn is_identity(&self) -> bool {
        self.distance < 0.01
    }

    /// Set the blur distance
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(0.0, 100.0);
    }

    /// Set the blur angle
    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle % 360.0;
    }
}

impl Default for MotionBlurFilter {
    fn default() -> Self {
        Self::new(5.0, 0.0)
    }
}

impl Adjustment for MotionBlurFilter {
    fn id(&self) -> &'static str {
        "motion_blur"
    }

    fn name(&self) -> &'static str {
        "Motion Blur"
    }

    fn description(&self) -> &'static str {
        "Apply directional motion blur effect"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();
        let mut result_data = PixelData::new_rgba(width, height);

        // Convert angle to radians
        let angle_rad = self.angle.to_radians();
        let dx = angle_rad.cos() * self.distance;
        let dy = angle_rad.sin() * self.distance;

        // Number of samples along the motion path
        let samples = (self.distance.ceil() as usize).clamp(1, 50);

        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0f32;
                let mut g_sum = 0.0f32;
                let mut b_sum = 0.0f32;
                let mut a_sum = 0.0f32;
                let mut count = 0;

                for i in 0..samples {
                    let t = i as f32 / samples as f32 - 0.5;
                    let sample_x = x as f32 + dx * t;
                    let sample_y = y as f32 + dy * t;

                    let sample_x_int = sample_x.round() as i32;
                    let sample_y_int = sample_y.round() as i32;

                    if sample_x_int >= 0
                        && sample_x_int < width as i32
                        && sample_y_int >= 0
                        && sample_y_int < height as i32
                    {
                        if let Some(pixel) =
                            pixel_data.get_pixel(sample_x_int as u32, sample_y_int as u32)
                        {
                            r_sum += pixel.r as f32;
                            g_sum += pixel.g as f32;
                            b_sum += pixel.b as f32;
                            a_sum += pixel.a as f32;
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    let result_pixel = RgbaPixel::new(
                        (r_sum / count as f32).clamp(0.0, 255.0) as u8,
                        (g_sum / count as f32).clamp(0.0, 255.0) as u8,
                        (b_sum / count as f32).clamp(0.0, 255.0) as u8,
                        (a_sum / count as f32).clamp(0.0, 255.0) as u8,
                    );
                    result_data.set_pixel(x, y, result_pixel)?;
                } else if let Some(original_pixel) = pixel_data.get_pixel(x, y) {
                    result_data.set_pixel(x, y, original_pixel)?;
                }
            }
        }

        // Copy result back to original
        *pixel_data = result_data;
        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        // For single pixel, motion blur has no effect
        Ok(pixel)
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "distance": self.distance,
            "angle": self.angle
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(distance) = parameters.get("distance").and_then(|v| v.as_f64()) {
            self.set_distance(distance as f32);
        }
        if let Some(angle) = parameters.get("angle").and_then(|v| v.as_f64()) {
            self.set_angle(angle as f32);
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
    use crate::PixelData;

    #[test]
    fn test_gaussian_blur_filter_creation() {
        let filter = GaussianBlurFilter::new(2.0);
        assert_eq!(filter.radius, 2.0);
        assert_eq!(filter.quality, 2.0);
        assert!(!filter.is_identity());

        let identity = GaussianBlurFilter::identity();
        assert!(identity.is_identity());
    }

    #[test]
    fn test_gaussian_blur_filter_metadata() {
        let filter = GaussianBlurFilter::new(1.0);
        assert_eq!(filter.id(), "gaussian_blur");
        assert_eq!(filter.name(), "Gaussian Blur");
        assert!(!filter.description().is_empty());
    }

    #[test]
    fn test_gaussian_blur_filter_parameters() {
        let mut filter = GaussianBlurFilter::new(1.0);

        let params = filter.get_parameters();
        assert_eq!(params.get("radius").unwrap().as_f64().unwrap(), 1.0);
        assert_eq!(params.get("quality").unwrap().as_f64().unwrap(), 2.0);

        let new_params = serde_json::json!({
            "radius": 3.0,
            "quality": 1.5
        });

        filter.set_parameters(new_params).unwrap();
        assert_eq!(filter.radius, 3.0);
        assert_eq!(filter.quality, 1.5);
    }

    #[test]
    fn test_motion_blur_filter_creation() {
        let filter = MotionBlurFilter::new(5.0, 45.0);
        assert_eq!(filter.distance, 5.0);
        assert_eq!(filter.angle, 45.0);
        assert!(!filter.is_identity());

        let identity = MotionBlurFilter::identity();
        assert!(identity.is_identity());
    }

    #[test]
    fn test_motion_blur_filter_metadata() {
        let filter = MotionBlurFilter::new(5.0, 0.0);
        assert_eq!(filter.id(), "motion_blur");
        assert_eq!(filter.name(), "Motion Blur");
        assert!(!filter.description().is_empty());
    }

    #[test]
    fn test_motion_blur_filter_parameters() {
        let mut filter = MotionBlurFilter::new(5.0, 0.0);

        let params = filter.get_parameters();
        assert_eq!(params.get("distance").unwrap().as_f64().unwrap(), 5.0);
        assert_eq!(params.get("angle").unwrap().as_f64().unwrap(), 0.0);

        let new_params = serde_json::json!({
            "distance": 10.0,
            "angle": 90.0
        });

        filter.set_parameters(new_params).unwrap();
        assert_eq!(filter.distance, 10.0);
        assert_eq!(filter.angle, 90.0);
    }
}
