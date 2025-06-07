//! Noise filter implementations

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Noise type for noise generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseType {
    /// Uniform random noise
    Uniform,
    /// Gaussian (normal distribution) noise
    Gaussian,
    /// Salt and pepper noise
    SaltPepper,
}

impl Default for NoiseType {
    fn default() -> Self {
        Self::Uniform
    }
}

impl std::fmt::Display for NoiseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoiseType::Uniform => write!(f, "Uniform"),
            NoiseType::Gaussian => write!(f, "Gaussian"),
            NoiseType::SaltPepper => write!(f, "Salt & Pepper"),
        }
    }
}

/// Add noise filter
///
/// Adds various types of noise to the image for artistic effects
/// or to simulate camera sensor noise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddNoiseFilter {
    /// Type of noise to add
    pub noise_type: NoiseType,
    /// Amount of noise (0.0 to 1.0)
    pub amount: f32,
    /// Whether to apply noise to color channels
    pub monochromatic: bool,
    /// Random seed for reproducible results
    pub seed: u32,
}

impl AddNoiseFilter {
    /// Create a new add noise filter
    pub fn new(noise_type: NoiseType, amount: f32) -> Self {
        Self {
            noise_type,
            amount: amount.clamp(0.0, 1.0),
            monochromatic: false,
            seed: 12345, // Default seed
        }
    }

    /// Create an identity noise filter (no noise)
    pub fn identity() -> Self {
        Self::new(NoiseType::Uniform, 0.0)
    }

    /// Check if this filter would make no changes
    pub fn is_identity(&self) -> bool {
        self.amount < 0.001
    }

    /// Set the noise amount
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.clamp(0.0, 1.0);
    }

    /// Set the noise type
    pub fn set_noise_type(&mut self, noise_type: NoiseType) {
        self.noise_type = noise_type;
    }

    /// Set monochromatic mode
    pub fn set_monochromatic(&mut self, monochromatic: bool) {
        self.monochromatic = monochromatic;
    }

    /// Set random seed
    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }

    /// Simple linear congruential generator for reproducible random numbers
    fn next_random(&self, state: &mut u32) -> f32 {
        *state = state.wrapping_mul(1103515245).wrapping_add(12345);
        (*state as f32) / (u32::MAX as f32)
    }

    /// Generate Gaussian random number using Box-Muller transform
    fn gaussian_random(&self, state: &mut u32) -> f32 {
        static mut SPARE: Option<f32> = None;
        static mut HAS_SPARE: bool = false;

        unsafe {
            if HAS_SPARE {
                HAS_SPARE = false;
                return SPARE.unwrap_or(0.0);
            }
        }

        let u1 = self.next_random(state);
        let u2 = self.next_random(state);

        let mag = self.amount * (-2.0 * u1.ln()).sqrt();
        let z0 = mag * (2.0 * std::f32::consts::PI * u2).cos();
        let z1 = mag * (2.0 * std::f32::consts::PI * u2).sin();

        unsafe {
            SPARE = Some(z1);
            HAS_SPARE = true;
        }

        z0
    }

    /// Apply noise to a single pixel
    fn apply_noise_to_pixel(&self, pixel: RgbaPixel, x: u32, y: u32) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        // Create unique seed for this pixel
        let mut rng_state = self.seed.wrapping_add(x).wrapping_mul(31).wrapping_add(y);

        match self.noise_type {
            NoiseType::Uniform => {
                let noise_range = self.amount * 255.0;

                if self.monochromatic {
                    let noise = (self.next_random(&mut rng_state) - 0.5) * noise_range;
                    RgbaPixel::new(
                        (pixel.r as f32 + noise).clamp(0.0, 255.0) as u8,
                        (pixel.g as f32 + noise).clamp(0.0, 255.0) as u8,
                        (pixel.b as f32 + noise).clamp(0.0, 255.0) as u8,
                        pixel.a,
                    )
                } else {
                    let noise_r = (self.next_random(&mut rng_state) - 0.5) * noise_range;
                    let noise_g = (self.next_random(&mut rng_state) - 0.5) * noise_range;
                    let noise_b = (self.next_random(&mut rng_state) - 0.5) * noise_range;

                    RgbaPixel::new(
                        (pixel.r as f32 + noise_r).clamp(0.0, 255.0) as u8,
                        (pixel.g as f32 + noise_g).clamp(0.0, 255.0) as u8,
                        (pixel.b as f32 + noise_b).clamp(0.0, 255.0) as u8,
                        pixel.a,
                    )
                }
            }
            NoiseType::Gaussian => {
                let noise_scale = self.amount * 64.0; // Scale for visible effect

                if self.monochromatic {
                    let noise = self.gaussian_random(&mut rng_state) * noise_scale;
                    RgbaPixel::new(
                        (pixel.r as f32 + noise).clamp(0.0, 255.0) as u8,
                        (pixel.g as f32 + noise).clamp(0.0, 255.0) as u8,
                        (pixel.b as f32 + noise).clamp(0.0, 255.0) as u8,
                        pixel.a,
                    )
                } else {
                    let noise_r = self.gaussian_random(&mut rng_state) * noise_scale;
                    let noise_g = self.gaussian_random(&mut rng_state) * noise_scale;
                    let noise_b = self.gaussian_random(&mut rng_state) * noise_scale;

                    RgbaPixel::new(
                        (pixel.r as f32 + noise_r).clamp(0.0, 255.0) as u8,
                        (pixel.g as f32 + noise_g).clamp(0.0, 255.0) as u8,
                        (pixel.b as f32 + noise_b).clamp(0.0, 255.0) as u8,
                        pixel.a,
                    )
                }
            }
            NoiseType::SaltPepper => {
                let threshold = self.amount;
                let rand_val = self.next_random(&mut rng_state);

                if rand_val < threshold / 2.0 {
                    // Salt (white)
                    RgbaPixel::new(255, 255, 255, pixel.a)
                } else if rand_val < threshold {
                    // Pepper (black)
                    RgbaPixel::new(0, 0, 0, pixel.a)
                } else {
                    // No change
                    pixel
                }
            }
        }
    }
}

impl Default for AddNoiseFilter {
    fn default() -> Self {
        Self::new(NoiseType::Uniform, 0.1)
    }
}

impl Adjustment for AddNoiseFilter {
    fn id(&self) -> &'static str {
        "add_noise"
    }

    fn name(&self) -> &'static str {
        "Add Noise"
    }

    fn description(&self) -> &'static str {
        "Add various types of noise to the image"
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

                let noisy_pixel = self.apply_noise_to_pixel(pixel, x, y);
                pixel_data.set_pixel(x, y, noisy_pixel)?;
            }
        }

        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        // For single pixel, use coordinates (0, 0)
        Ok(self.apply_noise_to_pixel(pixel, 0, 0))
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "noise_type": match self.noise_type {
                NoiseType::Uniform => "Uniform",
                NoiseType::Gaussian => "Gaussian",
                NoiseType::SaltPepper => "SaltPepper",
            },
            "amount": self.amount,
            "monochromatic": self.monochromatic,
            "seed": self.seed
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(noise_type_str) = parameters.get("noise_type").and_then(|v| v.as_str()) {
            let noise_type = match noise_type_str {
                "Uniform" => NoiseType::Uniform,
                "Gaussian" => NoiseType::Gaussian,
                "SaltPepper" => NoiseType::SaltPepper,
                _ => return Err(anyhow::anyhow!("Invalid noise type: {}", noise_type_str)),
            };
            self.set_noise_type(noise_type);
        }

        if let Some(amount) = parameters.get("amount").and_then(|v| v.as_f64()) {
            self.set_amount(amount as f32);
        }

        if let Some(monochromatic) = parameters.get("monochromatic").and_then(|v| v.as_bool()) {
            self.set_monochromatic(monochromatic);
        }

        if let Some(seed) = parameters.get("seed").and_then(|v| v.as_u64()) {
            self.set_seed(seed as u32);
        }

        Ok(())
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}

/// Reduce noise filter
///
/// Applies noise reduction using a simple median filter approach.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReduceNoiseFilter {
    /// Filter strength (1 to 5)
    pub strength: u8,
    /// Preserve details (0.0 to 1.0)
    pub preserve_details: f32,
}

impl ReduceNoiseFilter {
    /// Create a new reduce noise filter
    pub fn new(strength: u8, preserve_details: f32) -> Self {
        Self {
            strength: strength.clamp(1, 5),
            preserve_details: preserve_details.clamp(0.0, 1.0),
        }
    }

    /// Create an identity reduce noise filter (no filtering)
    pub fn identity() -> Self {
        Self::new(1, 1.0)
    }

    /// Check if this filter would make no changes
    pub fn is_identity(&self) -> bool {
        self.strength <= 1 && self.preserve_details >= 0.99
    }

    /// Set the filter strength
    pub fn set_strength(&mut self, strength: u8) {
        self.strength = strength.clamp(1, 5);
    }

    /// Set detail preservation
    pub fn set_preserve_details(&mut self, preserve_details: f32) {
        self.preserve_details = preserve_details.clamp(0.0, 1.0);
    }
}

impl Default for ReduceNoiseFilter {
    fn default() -> Self {
        Self::new(2, 0.5)
    }
}

impl Adjustment for ReduceNoiseFilter {
    fn id(&self) -> &'static str {
        "reduce_noise"
    }

    fn name(&self) -> &'static str {
        "Reduce Noise"
    }

    fn description(&self) -> &'static str {
        "Reduce image noise while preserving details"
    }

    fn apply(&self, pixel_data: &mut PixelData) -> Result<()> {
        if self.is_identity() {
            return Ok(());
        }

        let (width, height) = pixel_data.dimensions();
        let mut result_data = PixelData::new_rgba(width, height);

        let kernel_size = (self.strength as usize * 2 + 1).min(9);
        let half_kernel = kernel_size / 2;

        for y in 0..height {
            for x in 0..width {
                let mut r_values = Vec::new();
                let mut g_values = Vec::new();
                let mut b_values = Vec::new();
                let mut a_values = Vec::new();

                // Collect neighborhood values
                for ky in 0..kernel_size {
                    for kx in 0..kernel_size {
                        let sample_x = x as i32 + kx as i32 - half_kernel as i32;
                        let sample_y = y as i32 + ky as i32 - half_kernel as i32;

                        if sample_x >= 0
                            && sample_x < width as i32
                            && sample_y >= 0
                            && sample_y < height as i32
                        {
                            if let Some(pixel) =
                                pixel_data.get_pixel(sample_x as u32, sample_y as u32)
                            {
                                r_values.push(pixel.r);
                                g_values.push(pixel.g);
                                b_values.push(pixel.b);
                                a_values.push(pixel.a);
                            }
                        }
                    }
                }

                if !r_values.is_empty() {
                    // Sort for median calculation
                    r_values.sort_unstable();
                    g_values.sort_unstable();
                    b_values.sort_unstable();
                    a_values.sort_unstable();

                    let median_idx = r_values.len() / 2;
                    let median_r = r_values[median_idx];
                    let median_g = g_values[median_idx];
                    let median_b = b_values[median_idx];
                    let median_a = a_values[median_idx];

                    // Blend with original based on preserve_details
                    if let Some(original) = pixel_data.get_pixel(x, y) {
                        let blend_factor = 1.0 - self.preserve_details;
                        let result_pixel = RgbaPixel::new(
                            (original.r as f32 * self.preserve_details
                                + median_r as f32 * blend_factor) as u8,
                            (original.g as f32 * self.preserve_details
                                + median_g as f32 * blend_factor) as u8,
                            (original.b as f32 * self.preserve_details
                                + median_b as f32 * blend_factor) as u8,
                            (original.a as f32 * self.preserve_details
                                + median_a as f32 * blend_factor) as u8,
                        );
                        result_data.set_pixel(x, y, result_pixel)?;
                    }
                }
            }
        }

        // Copy result back to original
        *pixel_data = result_data;
        Ok(())
    }

    fn apply_to_pixel(&self, pixel: RgbaPixel) -> Result<RgbaPixel> {
        // For single pixel, noise reduction has no effect
        Ok(pixel)
    }

    fn would_modify_pixel(&self, _pixel: RgbaPixel) -> bool {
        !self.is_identity()
    }

    fn get_parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "strength": self.strength,
            "preserve_details": self.preserve_details
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(strength) = parameters.get("strength").and_then(|v| v.as_u64()) {
            self.set_strength(strength as u8);
        }
        if let Some(preserve_details) = parameters.get("preserve_details").and_then(|v| v.as_f64())
        {
            self.set_preserve_details(preserve_details as f32);
        }
        Ok(())
    }

    fn clone_adjustment(&self) -> Box<dyn Adjustment> {
        Box::new(self.clone())
    }
}
