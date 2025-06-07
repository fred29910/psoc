//! Curves adjustment implementation

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{adjustment::Adjustment, PixelData, RgbaPixel};

/// Curve channel types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurveChannel {
    /// RGB composite curve (affects all channels equally)
    Rgb,
    /// Red channel only
    Red,
    /// Green channel only
    Green,
    /// Blue channel only
    Blue,
}

impl Default for CurveChannel {
    fn default() -> Self {
        Self::Rgb
    }
}

impl std::fmt::Display for CurveChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurveChannel::Rgb => write!(f, "RGB"),
            CurveChannel::Red => write!(f, "Red"),
            CurveChannel::Green => write!(f, "Green"),
            CurveChannel::Blue => write!(f, "Blue"),
        }
    }
}

/// A single point on a curve
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurvePoint {
    /// Input value (0.0 to 1.0)
    pub input: f32,
    /// Output value (0.0 to 1.0)
    pub output: f32,
}

impl CurvePoint {
    /// Create a new curve point
    pub fn new(input: f32, output: f32) -> Self {
        Self {
            input: input.clamp(0.0, 1.0),
            output: output.clamp(0.0, 1.0),
        }
    }
}

/// A tone curve for a single channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneCurve {
    /// Control points defining the curve
    points: Vec<CurvePoint>,
    /// Lookup table for fast evaluation (256 values for 8-bit)
    lookup_table: Vec<u8>,
}

impl ToneCurve {
    /// Create a new linear tone curve (identity)
    pub fn linear() -> Self {
        let points = vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(1.0, 1.0),
        ];
        let mut curve = Self {
            points,
            lookup_table: Vec::new(),
        };
        curve.rebuild_lookup_table();
        curve
    }

    /// Create a tone curve from control points
    pub fn from_points(mut points: Vec<CurvePoint>) -> Self {
        // Sort points by input value
        points.sort_by(|a, b| a.input.partial_cmp(&b.input).unwrap());
        
        // Ensure we have start and end points
        if points.is_empty() || points[0].input > 0.0 {
            points.insert(0, CurvePoint::new(0.0, 0.0));
        }
        if points.is_empty() || points[points.len() - 1].input < 1.0 {
            points.push(CurvePoint::new(1.0, 1.0));
        }

        let mut curve = Self {
            points,
            lookup_table: Vec::new(),
        };
        curve.rebuild_lookup_table();
        curve
    }

    /// Add a control point to the curve
    pub fn add_point(&mut self, point: CurvePoint) {
        self.points.push(point);
        self.points.sort_by(|a, b| a.input.partial_cmp(&b.input).unwrap());
        self.rebuild_lookup_table();
    }

    /// Remove a control point (except start and end points)
    pub fn remove_point(&mut self, index: usize) {
        if index > 0 && index < self.points.len() - 1 {
            self.points.remove(index);
            self.rebuild_lookup_table();
        }
    }

    /// Get control points
    pub fn points(&self) -> &[CurvePoint] {
        &self.points
    }

    /// Evaluate the curve at a given input value
    pub fn evaluate(&self, input: f32) -> f32 {
        if self.lookup_table.is_empty() {
            return self.interpolate(input);
        }

        let input_clamped = input.clamp(0.0, 1.0);
        let index = (input_clamped * 255.0) as usize;
        (self.lookup_table[index.min(255)] as f32) / 255.0
    }

    /// Apply the curve to a u8 value
    pub fn apply_u8(&self, value: u8) -> u8 {
        self.lookup_table[value as usize]
    }

    /// Check if this is an identity curve (no change)
    pub fn is_identity(&self) -> bool {
        self.points.len() == 2 
            && (self.points[0].input - 0.0).abs() < 1e-6
            && (self.points[0].output - 0.0).abs() < 1e-6
            && (self.points[1].input - 1.0).abs() < 1e-6
            && (self.points[1].output - 1.0).abs() < 1e-6
    }

    /// Rebuild the lookup table for fast evaluation
    fn rebuild_lookup_table(&mut self) {
        self.lookup_table.clear();
        self.lookup_table.reserve(256);

        for i in 0..256 {
            let input = i as f32 / 255.0;
            let output = self.interpolate(input);
            self.lookup_table.push((output * 255.0).clamp(0.0, 255.0) as u8);
        }
    }

    /// Interpolate between control points using linear interpolation
    fn interpolate(&self, input: f32) -> f32 {
        if self.points.is_empty() {
            return input;
        }

        if self.points.len() == 1 {
            return self.points[0].output;
        }

        // Handle edge cases
        if input <= self.points[0].input {
            return self.points[0].output;
        }
        if input >= self.points[self.points.len() - 1].input {
            return self.points[self.points.len() - 1].output;
        }

        // Find the two points to interpolate between
        for i in 0..self.points.len() - 1 {
            let left = &self.points[i];
            let right = &self.points[i + 1];

            if input >= left.input && input <= right.input {
                if (right.input - left.input).abs() < 1e-6 {
                    return left.output;
                }

                let t = (input - left.input) / (right.input - left.input);
                return left.output + t * (right.output - left.output);
            }
        }

        // Fallback (should not reach here)
        input
    }
}

/// Curves adjustment
///
/// Provides tone curve adjustments for RGB composite and individual channels.
/// Allows precise control over tonal mapping using control points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvesAdjustment {
    /// RGB composite curve
    pub rgb_curve: ToneCurve,
    /// Red channel curve
    pub red_curve: ToneCurve,
    /// Green channel curve
    pub green_curve: ToneCurve,
    /// Blue channel curve
    pub blue_curve: ToneCurve,
    /// Whether to apply individual channel curves
    pub use_individual_curves: bool,
}

impl CurvesAdjustment {
    /// Create a new curves adjustment with linear curves
    pub fn new() -> Self {
        Self {
            rgb_curve: ToneCurve::linear(),
            red_curve: ToneCurve::linear(),
            green_curve: ToneCurve::linear(),
            blue_curve: ToneCurve::linear(),
            use_individual_curves: false,
        }
    }

    /// Create an identity curves adjustment (no change)
    pub fn identity() -> Self {
        Self::new()
    }

    /// Check if this adjustment would make no changes
    pub fn is_identity(&self) -> bool {
        self.rgb_curve.is_identity() 
            && (!self.use_individual_curves 
                || (self.red_curve.is_identity() 
                    && self.green_curve.is_identity() 
                    && self.blue_curve.is_identity()))
    }

    /// Set a curve for a specific channel
    pub fn set_curve(&mut self, channel: CurveChannel, curve: ToneCurve) {
        match channel {
            CurveChannel::Rgb => self.rgb_curve = curve,
            CurveChannel::Red => {
                self.red_curve = curve;
                self.use_individual_curves = true;
            }
            CurveChannel::Green => {
                self.green_curve = curve;
                self.use_individual_curves = true;
            }
            CurveChannel::Blue => {
                self.blue_curve = curve;
                self.use_individual_curves = true;
            }
        }
    }

    /// Get a curve for a specific channel
    pub fn get_curve(&self, channel: CurveChannel) -> &ToneCurve {
        match channel {
            CurveChannel::Rgb => &self.rgb_curve,
            CurveChannel::Red => &self.red_curve,
            CurveChannel::Green => &self.green_curve,
            CurveChannel::Blue => &self.blue_curve,
        }
    }

    /// Apply curves to a single pixel
    fn apply_to_pixel_internal(&self, pixel: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        let mut r = pixel.r;
        let mut g = pixel.g;
        let mut b = pixel.b;

        // Apply RGB composite curve first
        if !self.rgb_curve.is_identity() {
            r = self.rgb_curve.apply_u8(r);
            g = self.rgb_curve.apply_u8(g);
            b = self.rgb_curve.apply_u8(b);
        }

        // Apply individual channel curves if enabled
        if self.use_individual_curves {
            if !self.red_curve.is_identity() {
                r = self.red_curve.apply_u8(r);
            }
            if !self.green_curve.is_identity() {
                g = self.green_curve.apply_u8(g);
            }
            if !self.blue_curve.is_identity() {
                b = self.blue_curve.apply_u8(b);
            }
        }

        RgbaPixel::new(r, g, b, pixel.a)
    }
}

impl Default for CurvesAdjustment {
    fn default() -> Self {
        Self::new()
    }
}

impl Adjustment for CurvesAdjustment {
    fn id(&self) -> &'static str {
        "curves"
    }

    fn name(&self) -> &'static str {
        "Curves"
    }

    fn description(&self) -> &'static str {
        "Adjust tones using curves for precise control over highlights, midtones, and shadows"
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
            "rgb_curve": {
                "points": self.rgb_curve.points()
            },
            "red_curve": {
                "points": self.red_curve.points()
            },
            "green_curve": {
                "points": self.green_curve.points()
            },
            "blue_curve": {
                "points": self.blue_curve.points()
            },
            "use_individual_curves": self.use_individual_curves
        })
    }

    fn set_parameters(&mut self, parameters: serde_json::Value) -> Result<()> {
        if let Some(use_individual) = parameters.get("use_individual_curves").and_then(|v| v.as_bool()) {
            self.use_individual_curves = use_individual;
        }

        // Helper function to parse curve points
        let parse_curve_points = |curve_data: &serde_json::Value| -> Result<Vec<CurvePoint>> {
            if let Some(points_array) = curve_data.get("points").and_then(|v| v.as_array()) {
                let mut points = Vec::new();
                for point_value in points_array {
                    if let (Some(input), Some(output)) = (
                        point_value.get("input").and_then(|v| v.as_f64()),
                        point_value.get("output").and_then(|v| v.as_f64()),
                    ) {
                        points.push(CurvePoint::new(input as f32, output as f32));
                    }
                }
                Ok(points)
            } else {
                Ok(vec![CurvePoint::new(0.0, 0.0), CurvePoint::new(1.0, 1.0)])
            }
        };

        // Parse RGB curve
        if let Some(rgb_curve_data) = parameters.get("rgb_curve") {
            let points = parse_curve_points(rgb_curve_data)?;
            self.rgb_curve = ToneCurve::from_points(points);
        }

        // Parse individual channel curves
        if let Some(red_curve_data) = parameters.get("red_curve") {
            let points = parse_curve_points(red_curve_data)?;
            self.red_curve = ToneCurve::from_points(points);
        }

        if let Some(green_curve_data) = parameters.get("green_curve") {
            let points = parse_curve_points(green_curve_data)?;
            self.green_curve = ToneCurve::from_points(points);
        }

        if let Some(blue_curve_data) = parameters.get("blue_curve") {
            let points = parse_curve_points(blue_curve_data)?;
            self.blue_curve = ToneCurve::from_points(points);
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
    fn test_curve_point_creation() {
        let point = CurvePoint::new(0.5, 0.7);
        assert_eq!(point.input, 0.5);
        assert_eq!(point.output, 0.7);

        // Test clamping
        let point = CurvePoint::new(-0.1, 1.5);
        assert_eq!(point.input, 0.0);
        assert_eq!(point.output, 1.0);
    }

    #[test]
    fn test_tone_curve_linear() {
        let curve = ToneCurve::linear();
        assert!(curve.is_identity());

        // Test evaluation
        let val_0 = curve.evaluate(0.0);
        let val_05 = curve.evaluate(0.5);
        let val_1 = curve.evaluate(1.0);

        println!("curve.evaluate(0.0) = {}", val_0);
        println!("curve.evaluate(0.5) = {}", val_05);
        println!("curve.evaluate(1.0) = {}", val_1);

        assert!((val_0 - 0.0).abs() < 1e-3, "Expected ~0.0, got {}", val_0);
        assert!((val_05 - 0.5).abs() < 0.01, "Expected ~0.5, got {}", val_05); // Allow for lookup table quantization
        assert!((val_1 - 1.0).abs() < 1e-3, "Expected ~1.0, got {}", val_1);

        // Test u8 application
        assert_eq!(curve.apply_u8(0), 0);
        assert_eq!(curve.apply_u8(128), 128);
        assert_eq!(curve.apply_u8(255), 255);
    }

    #[test]
    fn test_tone_curve_from_points() {
        let points = vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(0.5, 0.8),
            CurvePoint::new(1.0, 1.0),
        ];
        let curve = ToneCurve::from_points(points);

        assert!(!curve.is_identity());
        assert_eq!(curve.points().len(), 3);

        // Test that midpoint is brighter
        let mid_output = curve.evaluate(0.5);
        assert!(mid_output > 0.5);
    }

    #[test]
    fn test_tone_curve_add_remove_points() {
        let mut curve = ToneCurve::linear();
        assert_eq!(curve.points().len(), 2);

        curve.add_point(CurvePoint::new(0.5, 0.3));
        assert_eq!(curve.points().len(), 3);

        curve.remove_point(1); // Remove middle point
        assert_eq!(curve.points().len(), 2);

        // Can't remove start or end points
        curve.remove_point(0);
        assert_eq!(curve.points().len(), 2);
    }

    #[test]
    fn test_curves_adjustment_creation() {
        let adjustment = CurvesAdjustment::new();
        assert!(adjustment.is_identity());
        assert!(!adjustment.use_individual_curves);

        let identity = CurvesAdjustment::identity();
        assert!(identity.is_identity());
    }

    #[test]
    fn test_curves_adjustment_set_get_curve() {
        let mut adjustment = CurvesAdjustment::new();

        let points = vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(0.5, 0.3),
            CurvePoint::new(1.0, 1.0),
        ];
        let curve = ToneCurve::from_points(points);

        adjustment.set_curve(CurveChannel::Red, curve);
        assert!(adjustment.use_individual_curves);

        let red_curve = adjustment.get_curve(CurveChannel::Red);
        assert!(!red_curve.is_identity());
    }

    #[test]
    fn test_curves_adjustment_metadata() {
        let adjustment = CurvesAdjustment::new();
        assert_eq!(adjustment.id(), "curves");
        assert_eq!(adjustment.name(), "Curves");
        assert!(!adjustment.description().is_empty());
    }

    #[test]
    fn test_curves_adjustment_identity() {
        let adjustment = CurvesAdjustment::identity();
        assert!(adjustment.is_identity());

        let pixel = RgbaPixel::new(100, 150, 200, 255);
        let result = adjustment.apply_to_pixel(pixel).unwrap();
        assert_eq!(result, pixel);
    }

    #[test]
    fn test_curves_adjustment_apply() {
        let mut adjustment = CurvesAdjustment::new();

        // Create a curve that darkens midtones
        let points = vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(0.5, 0.3),
            CurvePoint::new(1.0, 1.0),
        ];
        adjustment.rgb_curve = ToneCurve::from_points(points);

        let pixel = RgbaPixel::new(128, 128, 128, 255);
        let result = adjustment.apply_to_pixel(pixel).unwrap();

        // Midtones should be darker
        assert!(result.r < pixel.r);
        assert!(result.g < pixel.g);
        assert!(result.b < pixel.b);
        assert_eq!(result.a, pixel.a); // Alpha unchanged
    }

    #[test]
    fn test_curves_adjustment_parameters() {
        let mut adjustment = CurvesAdjustment::new();
        adjustment.use_individual_curves = true;

        let params = adjustment.get_parameters();
        assert!(params.get("use_individual_curves").unwrap().as_bool().unwrap());

        // Test parameter setting
        let new_params = serde_json::json!({
            "use_individual_curves": false,
            "rgb_curve": {
                "points": [
                    {"input": 0.0, "output": 0.0},
                    {"input": 0.5, "output": 0.8},
                    {"input": 1.0, "output": 1.0}
                ]
            }
        });

        adjustment.set_parameters(new_params).unwrap();
        assert!(!adjustment.use_individual_curves);
        assert!(!adjustment.rgb_curve.is_identity());
    }

    #[test]
    fn test_curves_adjustment_clone() {
        let adjustment = CurvesAdjustment::new();
        let cloned = adjustment.clone_adjustment();

        assert_eq!(cloned.id(), adjustment.id());
        assert_eq!(cloned.name(), adjustment.name());
    }

    #[test]
    fn test_curve_channel_display() {
        assert_eq!(format!("{}", CurveChannel::Rgb), "RGB");
        assert_eq!(format!("{}", CurveChannel::Red), "Red");
        assert_eq!(format!("{}", CurveChannel::Green), "Green");
        assert_eq!(format!("{}", CurveChannel::Blue), "Blue");
    }
}
