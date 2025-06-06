//! Mathematical utilities and functions
//!
//! This module provides mathematical utilities commonly used in image processing
//! and graphics operations.

use std::f32::consts::PI;

/// Mathematical constants
pub mod constants {
    /// Pi constant
    pub const PI: f32 = std::f32::consts::PI;

    /// 2 * Pi
    pub const TAU: f32 = 2.0 * PI;

    /// Pi / 2
    pub const HALF_PI: f32 = PI / 2.0;

    /// Pi / 4
    pub const QUARTER_PI: f32 = PI / 4.0;

    /// Degrees to radians conversion factor
    pub const DEG_TO_RAD: f32 = PI / 180.0;

    /// Radians to degrees conversion factor
    pub const RAD_TO_DEG: f32 = 180.0 / PI;

    /// Golden ratio
    pub const GOLDEN_RATIO: f32 = 1.618033988749;

    /// Square root of 2
    pub const SQRT_2: f32 = 1.4142135623731;
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Inverse linear interpolation - find t such that lerp(a, b, t) = value
pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if (b - a).abs() < f32::EPSILON {
        0.0
    } else {
        (value - a) / (b - a)
    }
}

/// Smoothstep interpolation (smooth cubic curve)
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smootherstep interpolation (even smoother quintic curve)
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Convert degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * constants::DEG_TO_RAD
}

/// Convert radians to degrees
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * constants::RAD_TO_DEG
}

/// Normalize angle to [0, 2π) range
pub fn normalize_angle_rad(angle: f32) -> f32 {
    angle.rem_euclid(constants::TAU)
}

/// Normalize angle to [0, 360) range
pub fn normalize_angle_deg(angle: f32) -> f32 {
    angle.rem_euclid(360.0)
}

/// Calculate the shortest angular distance between two angles (in radians)
pub fn angle_difference_rad(a: f32, b: f32) -> f32 {
    let diff = (b - a).rem_euclid(constants::TAU);
    if diff > PI {
        diff - constants::TAU
    } else {
        diff
    }
}

/// Calculate the shortest angular distance between two angles (in degrees)
pub fn angle_difference_deg(a: f32, b: f32) -> f32 {
    let diff = (b - a).rem_euclid(360.0);
    if diff > 180.0 {
        diff - 360.0
    } else {
        diff
    }
}

/// Map a value from one range to another
pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = inverse_lerp(from_min, from_max, value);
    lerp(to_min, to_max, t)
}

/// Check if two floating point numbers are approximately equal
pub fn approx_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

/// Check if a floating point number is approximately zero
pub fn approx_zero(value: f32, epsilon: f32) -> bool {
    value.abs() < epsilon
}

/// Round to nearest multiple of step
pub fn round_to_step(value: f32, step: f32) -> f32 {
    if step == 0.0 {
        value
    } else {
        (value / step).round() * step
    }
}

/// Snap value to grid
pub fn snap_to_grid(value: f32, grid_size: f32) -> f32 {
    round_to_step(value, grid_size)
}

/// Calculate distance between two 2D points
pub fn distance_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Calculate squared distance between two 2D points (faster than distance_2d)
pub fn distance_squared_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x1).powi(2) + (y2 - y1).powi(2)
}

/// Gaussian function
pub fn gaussian(x: f32, sigma: f32) -> f32 {
    let sigma_sq = sigma * sigma;
    let coefficient = 1.0 / (sigma * (2.0 * PI).sqrt());
    coefficient * (-x * x / (2.0 * sigma_sq)).exp()
}

/// 2D Gaussian function
pub fn gaussian_2d(x: f32, y: f32, sigma_x: f32, sigma_y: f32) -> f32 {
    let sigma_x_sq = sigma_x * sigma_x;
    let sigma_y_sq = sigma_y * sigma_y;
    let coefficient = 1.0 / (2.0 * PI * sigma_x * sigma_y);
    coefficient * (-(x * x / (2.0 * sigma_x_sq) + y * y / (2.0 * sigma_y_sq))).exp()
}

/// Ease-in cubic function
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease-out cubic function
pub fn ease_out_cubic(t: f32) -> f32 {
    let t1 = t - 1.0;
    1.0 + t1 * t1 * t1
}

/// Ease-in-out cubic function
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t1 = 2.0 * t - 2.0;
        1.0 + t1 * t1 * t1 / 2.0
    }
}

/// Calculate factorial (for small integers)
pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => (2..=n as u64).product(),
    }
}

/// Calculate binomial coefficient (n choose k)
pub fn binomial_coefficient(n: u32, k: u32) -> u64 {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        let k = k.min(n - k); // Take advantage of symmetry
        (1..=k).fold(1, |acc, i| acc * (n - i + 1) as u64 / i as u64)
    }
}

/// Generate weights for a 1D Gaussian kernel
pub fn gaussian_kernel_1d(size: usize, sigma: f32) -> Vec<f32> {
    let center = size as f32 / 2.0;
    let mut kernel = Vec::with_capacity(size);
    let mut sum = 0.0;

    for i in 0..size {
        let x = i as f32 - center;
        let weight = gaussian(x, sigma);
        kernel.push(weight);
        sum += weight;
    }

    // Normalize
    for weight in &mut kernel {
        *weight /= sum;
    }

    kernel
}

/// Generate weights for a 2D Gaussian kernel
pub fn gaussian_kernel_2d(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let center = size as f32 / 2.0;
    let mut kernel = vec![vec![0.0; size]; size];
    let mut sum = 0.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let weight = gaussian_2d(dx, dy, sigma, sigma);
            kernel[y][x] = weight;
            sum += weight;
        }
    }

    // Normalize
    for row in &mut kernel {
        for weight in row {
            *weight /= sum;
        }
    }

    kernel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
    }

    #[test]
    fn test_angle_conversion() {
        assert!((deg_to_rad(180.0) - PI).abs() < f32::EPSILON);
        assert!((rad_to_deg(PI) - 180.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_map_range() {
        let result = map_range(5.0, 0.0, 10.0, 0.0, 100.0);
        assert!((result - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_gaussian_kernel_1d() {
        let kernel = gaussian_kernel_1d(5, 1.0);
        assert_eq!(kernel.len(), 5);

        // Sum should be approximately 1.0
        let sum: f32 = kernel.iter().sum();
        assert!((sum - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_distance_2d() {
        let dist = distance_2d(0.0, 0.0, 3.0, 4.0);
        assert!((dist - 5.0).abs() < f32::EPSILON);
    }
}
