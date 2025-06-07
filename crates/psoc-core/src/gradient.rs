//! Gradient system for creating and managing color gradients
//!
//! This module provides comprehensive gradient functionality including:
//! - Linear and radial gradients
//! - Color stop management
//! - Gradient interpolation algorithms
//! - Gradient rendering and application

use crate::color::{HslColor, HsvColor};
use crate::geometry::{Point, Rect};
use crate::pixel::RgbaPixel;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Types of gradients supported
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradientType {
    /// Linear gradient from point A to point B
    Linear,
    /// Radial gradient from center point with radius
    Radial,
    /// Angular gradient around a center point
    Angular,
    /// Diamond gradient (square radial)
    Diamond,
}

impl Default for GradientType {
    fn default() -> Self {
        Self::Linear
    }
}

impl std::fmt::Display for GradientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GradientType::Linear => write!(f, "Linear"),
            GradientType::Radial => write!(f, "Radial"),
            GradientType::Angular => write!(f, "Angular"),
            GradientType::Diamond => write!(f, "Diamond"),
        }
    }
}

/// Color stop in a gradient
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorStop {
    /// Position along the gradient (0.0 to 1.0)
    pub position: f32,
    /// Color at this position
    pub color: RgbaPixel,
    /// Optional midpoint for color transition (0.0 to 1.0)
    pub midpoint: Option<f32>,
}

impl ColorStop {
    /// Create a new color stop
    pub fn new(position: f32, color: RgbaPixel) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
            midpoint: None,
        }
    }

    /// Create a color stop with midpoint
    pub fn with_midpoint(position: f32, color: RgbaPixel, midpoint: f32) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            color,
            midpoint: Some(midpoint.clamp(0.0, 1.0)),
        }
    }
}

/// Gradient interpolation methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationMethod {
    /// Linear RGB interpolation
    Linear,
    /// HSL color space interpolation
    Hsl,
    /// HSV color space interpolation
    Hsv,
    /// Smooth step interpolation
    Smooth,
}

impl Default for InterpolationMethod {
    fn default() -> Self {
        Self::Linear
    }
}

/// Gradient definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gradient {
    /// Type of gradient
    pub gradient_type: GradientType,
    /// Color stops sorted by position
    pub stops: BTreeMap<u32, ColorStop>, // Using u32 key for stable ordering
    /// Interpolation method
    pub interpolation: InterpolationMethod,
    /// Whether the gradient repeats
    pub repeat: bool,
    /// Gradient start point (for linear/angular)
    pub start_point: Point,
    /// Gradient end point (for linear) or radius (for radial)
    pub end_point: Point,
    /// Gradient name/identifier
    pub name: String,
}

impl Default for Gradient {
    fn default() -> Self {
        let mut stops = BTreeMap::new();
        stops.insert(0, ColorStop::new(0.0, RgbaPixel::new(0, 0, 0, 255)));
        stops.insert(1, ColorStop::new(1.0, RgbaPixel::new(255, 255, 255, 255)));

        Self {
            gradient_type: GradientType::Linear,
            stops,
            interpolation: InterpolationMethod::Linear,
            repeat: false,
            start_point: Point::new(0.0, 0.0),
            end_point: Point::new(100.0, 0.0),
            name: "Default Gradient".to_string(),
        }
    }
}

impl Gradient {
    /// Create a new gradient
    pub fn new(name: String, gradient_type: GradientType) -> Self {
        Self {
            name,
            gradient_type,
            ..Default::default()
        }
    }

    /// Create a simple two-color linear gradient
    pub fn linear_two_color(start_color: RgbaPixel, end_color: RgbaPixel) -> Self {
        let mut stops = BTreeMap::new();
        stops.insert(0, ColorStop::new(0.0, start_color));
        stops.insert(1, ColorStop::new(1.0, end_color));

        Self {
            gradient_type: GradientType::Linear,
            stops,
            interpolation: InterpolationMethod::Linear,
            repeat: false,
            start_point: Point::new(0.0, 0.0),
            end_point: Point::new(100.0, 0.0),
            name: "Two Color Linear".to_string(),
        }
    }

    /// Create a simple two-color radial gradient
    pub fn radial_two_color(center_color: RgbaPixel, edge_color: RgbaPixel) -> Self {
        let mut stops = BTreeMap::new();
        stops.insert(0, ColorStop::new(0.0, center_color));
        stops.insert(1, ColorStop::new(1.0, edge_color));

        Self {
            gradient_type: GradientType::Radial,
            stops,
            interpolation: InterpolationMethod::Linear,
            repeat: false,
            start_point: Point::new(50.0, 50.0), // Center
            end_point: Point::new(100.0, 50.0),  // Edge
            name: "Two Color Radial".to_string(),
        }
    }

    /// Add a color stop
    pub fn add_stop(&mut self, stop: ColorStop) -> u32 {
        let key = self.stops.len() as u32;
        self.stops.insert(key, stop);
        key
    }

    /// Remove a color stop
    pub fn remove_stop(&mut self, key: u32) -> Option<ColorStop> {
        if self.stops.len() <= 2 {
            return None; // Must have at least 2 stops
        }
        self.stops.remove(&key)
    }

    /// Update a color stop
    pub fn update_stop(&mut self, key: u32, stop: ColorStop) -> bool {
        if self.stops.contains_key(&key) {
            self.stops.insert(key, stop);
            true
        } else {
            false
        }
    }

    /// Get sorted color stops
    pub fn sorted_stops(&self) -> Vec<&ColorStop> {
        let mut stops: Vec<_> = self.stops.values().collect();
        stops.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
        stops
    }

    /// Set gradient geometry for linear gradient
    pub fn set_linear_geometry(&mut self, start: Point, end: Point) {
        self.gradient_type = GradientType::Linear;
        self.start_point = start;
        self.end_point = end;
    }

    /// Set gradient geometry for radial gradient
    pub fn set_radial_geometry(&mut self, center: Point, radius: f32) {
        self.gradient_type = GradientType::Radial;
        self.start_point = center;
        self.end_point = Point::new(center.x + radius, center.y);
    }

    /// Calculate color at a specific position (0.0 to 1.0)
    pub fn color_at(&self, position: f32) -> RgbaPixel {
        let position = if self.repeat {
            position.fract()
        } else {
            position.clamp(0.0, 1.0)
        };

        let stops = self.sorted_stops();
        if stops.is_empty() {
            return RgbaPixel::transparent();
        }

        if stops.len() == 1 {
            return stops[0].color;
        }

        // Find the two stops to interpolate between
        let mut before_stop = stops[0];
        let mut after_stop = stops[stops.len() - 1];

        for i in 0..stops.len() - 1 {
            if position >= stops[i].position && position <= stops[i + 1].position {
                before_stop = stops[i];
                after_stop = stops[i + 1];
                break;
            }
        }

        // Handle edge cases
        if position <= before_stop.position {
            return before_stop.color;
        }
        if position >= after_stop.position {
            return after_stop.color;
        }

        // Calculate interpolation factor
        let range = after_stop.position - before_stop.position;
        if range == 0.0 {
            return before_stop.color;
        }

        let t = (position - before_stop.position) / range;

        // Apply midpoint adjustment if present
        let adjusted_t = if let Some(midpoint) = before_stop.midpoint {
            self.apply_midpoint(t, midpoint)
        } else {
            t
        };

        // Interpolate colors based on method
        self.interpolate_colors(before_stop.color, after_stop.color, adjusted_t)
    }

    /// Apply midpoint adjustment to interpolation factor
    fn apply_midpoint(&self, t: f32, midpoint: f32) -> f32 {
        if t <= midpoint {
            0.5 * (t / midpoint)
        } else {
            0.5 + 0.5 * ((t - midpoint) / (1.0 - midpoint))
        }
    }

    /// Interpolate between two colors using the specified method
    fn interpolate_colors(&self, color1: RgbaPixel, color2: RgbaPixel, t: f32) -> RgbaPixel {
        match self.interpolation {
            InterpolationMethod::Linear => self.interpolate_linear(color1, color2, t),
            InterpolationMethod::Hsl => self.interpolate_hsl(color1, color2, t),
            InterpolationMethod::Hsv => self.interpolate_hsv(color1, color2, t),
            InterpolationMethod::Smooth => {
                let smooth_t = t * t * (3.0 - 2.0 * t); // Smoothstep
                self.interpolate_linear(color1, color2, smooth_t)
            }
        }
    }

    /// Linear RGB interpolation
    fn interpolate_linear(&self, color1: RgbaPixel, color2: RgbaPixel, t: f32) -> RgbaPixel {
        let r = (color1.r as f32 * (1.0 - t) + color2.r as f32 * t) as u8;
        let g = (color1.g as f32 * (1.0 - t) + color2.g as f32 * t) as u8;
        let b = (color1.b as f32 * (1.0 - t) + color2.b as f32 * t) as u8;
        let a = (color1.a as f32 * (1.0 - t) + color2.a as f32 * t) as u8;
        RgbaPixel::new(r, g, b, a)
    }

    /// HSL color space interpolation
    fn interpolate_hsl(&self, color1: RgbaPixel, color2: RgbaPixel, t: f32) -> RgbaPixel {
        let hsl1 = HslColor::from_rgba(color1);
        let hsl2 = HslColor::from_rgba(color2);

        let h = self.interpolate_hue(hsl1.h, hsl2.h, t);
        let s = hsl1.s * (1.0 - t) + hsl2.s * t;
        let l = hsl1.l * (1.0 - t) + hsl2.l * t;
        let a = hsl1.a * (1.0 - t) + hsl2.a * t;

        HslColor::new(h, s, l, a).to_rgba()
    }

    /// HSV color space interpolation
    fn interpolate_hsv(&self, color1: RgbaPixel, color2: RgbaPixel, t: f32) -> RgbaPixel {
        let hsv1 = HsvColor::from_rgba(color1);
        let hsv2 = HsvColor::from_rgba(color2);

        let h = self.interpolate_hue(hsv1.h, hsv2.h, t);
        let s = hsv1.s * (1.0 - t) + hsv2.s * t;
        let v = hsv1.v * (1.0 - t) + hsv2.v * t;
        let a = hsv1.a * (1.0 - t) + hsv2.a * t;

        HsvColor::new(h, s, v, a).to_rgba()
    }

    /// Interpolate hue values (handles wraparound)
    fn interpolate_hue(&self, h1: f32, h2: f32, t: f32) -> f32 {
        let diff = h2 - h1;
        if diff.abs() > 180.0 {
            // Take the shorter path around the color wheel
            if diff > 0.0 {
                (h1 + (diff - 360.0) * t).rem_euclid(360.0)
            } else {
                (h1 + (diff + 360.0) * t).rem_euclid(360.0)
            }
        } else {
            (h1 + diff * t).rem_euclid(360.0)
        }
    }

    /// Calculate gradient position for a point based on gradient type
    pub fn position_for_point(&self, point: Point) -> f32 {
        match self.gradient_type {
            GradientType::Linear => self.linear_position(point),
            GradientType::Radial => self.radial_position(point),
            GradientType::Angular => self.angular_position(point),
            GradientType::Diamond => self.diamond_position(point),
        }
    }

    /// Calculate position for linear gradient
    fn linear_position(&self, point: Point) -> f32 {
        let dx = self.end_point.x - self.start_point.x;
        let dy = self.end_point.y - self.start_point.y;
        let length_squared = dx * dx + dy * dy;

        if length_squared == 0.0 {
            return 0.0;
        }

        let px = point.x - self.start_point.x;
        let py = point.y - self.start_point.y;
        let dot_product = px * dx + py * dy;

        dot_product / length_squared
    }

    /// Calculate position for radial gradient
    fn radial_position(&self, point: Point) -> f32 {
        let dx = point.x - self.start_point.x;
        let dy = point.y - self.start_point.y;
        let distance = (dx * dx + dy * dy).sqrt();

        let radius = (self.end_point.x - self.start_point.x)
            .abs()
            .max((self.end_point.y - self.start_point.y).abs());

        if radius == 0.0 {
            0.0
        } else {
            distance / radius
        }
    }

    /// Calculate position for angular gradient
    fn angular_position(&self, point: Point) -> f32 {
        let dx = point.x - self.start_point.x;
        let dy = point.y - self.start_point.y;
        let angle = dy.atan2(dx);
        let normalized_angle = (angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        normalized_angle.rem_euclid(1.0)
    }

    /// Calculate position for diamond gradient
    fn diamond_position(&self, point: Point) -> f32 {
        let dx = (point.x - self.start_point.x).abs();
        let dy = (point.y - self.start_point.y).abs();
        let distance = dx.max(dy); // Manhattan distance (diamond shape)

        let radius = (self.end_point.x - self.start_point.x)
            .abs()
            .max((self.end_point.y - self.start_point.y).abs());

        if radius == 0.0 {
            0.0
        } else {
            distance / radius
        }
    }

    /// Render gradient to a rectangular region
    pub fn render_to_region(&self, region: Rect) -> Result<Vec<RgbaPixel>> {
        let width = region.width as usize;
        let height = region.height as usize;
        let mut pixels = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let point = Point::new(region.x + x as f32, region.y + y as f32);
                let position = self.position_for_point(point);
                let color = self.color_at(position);
                pixels.push(color);
            }
        }

        Ok(pixels)
    }

    /// Create a preview of the gradient as a horizontal strip
    pub fn create_preview(&self, width: u32, height: u32) -> Vec<RgbaPixel> {
        let mut pixels = Vec::with_capacity((width * height) as usize);

        for _y in 0..height {
            for x in 0..width {
                let position = x as f32 / (width - 1) as f32;
                let color = self.color_at(position);
                pixels.push(color);
            }
        }

        pixels
    }
}

/// Gradient manager for storing and managing gradients
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GradientManager {
    /// Collection of gradients by name
    gradients: BTreeMap<String, Gradient>,
    /// Currently selected gradient
    current_gradient: Option<String>,
}

impl GradientManager {
    /// Create a new gradient manager
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.add_default_gradients();
        manager
    }

    /// Add default gradients
    fn add_default_gradients(&mut self) {
        // Black to white linear
        let bw_linear = Gradient::linear_two_color(
            RgbaPixel::new(0, 0, 0, 255),
            RgbaPixel::new(255, 255, 255, 255),
        );
        self.gradients
            .insert("Black to White".to_string(), bw_linear);

        // Rainbow gradient
        let mut rainbow = Gradient::new("Rainbow".to_string(), GradientType::Linear);
        rainbow.add_stop(ColorStop::new(0.0, RgbaPixel::new(255, 0, 0, 255))); // Red
        rainbow.add_stop(ColorStop::new(0.17, RgbaPixel::new(255, 165, 0, 255))); // Orange
        rainbow.add_stop(ColorStop::new(0.33, RgbaPixel::new(255, 255, 0, 255))); // Yellow
        rainbow.add_stop(ColorStop::new(0.5, RgbaPixel::new(0, 255, 0, 255))); // Green
        rainbow.add_stop(ColorStop::new(0.67, RgbaPixel::new(0, 0, 255, 255))); // Blue
        rainbow.add_stop(ColorStop::new(0.83, RgbaPixel::new(75, 0, 130, 255))); // Indigo
        rainbow.add_stop(ColorStop::new(1.0, RgbaPixel::new(238, 130, 238, 255))); // Violet
        self.gradients.insert("Rainbow".to_string(), rainbow);

        // Transparent to opaque
        let transparent =
            Gradient::linear_two_color(RgbaPixel::new(0, 0, 0, 0), RgbaPixel::new(0, 0, 0, 255));
        self.gradients
            .insert("Transparent to Black".to_string(), transparent);

        // Set default current gradient
        self.current_gradient = Some("Black to White".to_string());
    }

    /// Add a gradient
    pub fn add_gradient(&mut self, gradient: Gradient) {
        let name = gradient.name.clone();
        self.gradients.insert(name.clone(), gradient);
        if self.current_gradient.is_none() {
            self.current_gradient = Some(name);
        }
    }

    /// Remove a gradient
    pub fn remove_gradient(&mut self, name: &str) -> Option<Gradient> {
        let removed = self.gradients.remove(name);
        if self.current_gradient.as_ref() == Some(&name.to_string()) {
            self.current_gradient = self.gradients.keys().next().cloned();
        }
        removed
    }

    /// Get a gradient by name
    pub fn get_gradient(&self, name: &str) -> Option<&Gradient> {
        self.gradients.get(name)
    }

    /// Get mutable gradient by name
    pub fn get_gradient_mut(&mut self, name: &str) -> Option<&mut Gradient> {
        self.gradients.get_mut(name)
    }

    /// Get current gradient
    pub fn current_gradient(&self) -> Option<&Gradient> {
        self.current_gradient
            .as_ref()
            .and_then(|name| self.gradients.get(name))
    }

    /// Set current gradient
    pub fn set_current_gradient(&mut self, name: &str) -> bool {
        if self.gradients.contains_key(name) {
            self.current_gradient = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// List all gradient names
    pub fn gradient_names(&self) -> Vec<String> {
        self.gradients.keys().cloned().collect()
    }

    /// Get number of gradients
    pub fn count(&self) -> usize {
        self.gradients.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_stop_creation() {
        let stop = ColorStop::new(0.5, RgbaPixel::new(255, 0, 0, 255));
        assert_eq!(stop.position, 0.5);
        assert_eq!(stop.color, RgbaPixel::new(255, 0, 0, 255));
        assert!(stop.midpoint.is_none());
    }

    #[test]
    fn test_color_stop_with_midpoint() {
        let stop = ColorStop::with_midpoint(0.5, RgbaPixel::new(255, 0, 0, 255), 0.3);
        assert_eq!(stop.position, 0.5);
        assert_eq!(stop.color, RgbaPixel::new(255, 0, 0, 255));
        assert_eq!(stop.midpoint, Some(0.3));
    }

    #[test]
    fn test_color_stop_position_clamping() {
        let stop = ColorStop::new(-0.5, RgbaPixel::new(255, 0, 0, 255));
        assert_eq!(stop.position, 0.0);

        let stop = ColorStop::new(1.5, RgbaPixel::new(255, 0, 0, 255));
        assert_eq!(stop.position, 1.0);
    }

    #[test]
    fn test_gradient_default() {
        let gradient = Gradient::default();
        assert_eq!(gradient.gradient_type, GradientType::Linear);
        assert_eq!(gradient.stops.len(), 2);
        assert_eq!(gradient.interpolation, InterpolationMethod::Linear);
        assert!(!gradient.repeat);
    }

    #[test]
    fn test_linear_two_color_gradient() {
        let start_color = RgbaPixel::new(255, 0, 0, 255);
        let end_color = RgbaPixel::new(0, 255, 0, 255);
        let gradient = Gradient::linear_two_color(start_color, end_color);

        assert_eq!(gradient.gradient_type, GradientType::Linear);
        assert_eq!(gradient.stops.len(), 2);

        let color_at_start = gradient.color_at(0.0);
        let color_at_end = gradient.color_at(1.0);
        let color_at_middle = gradient.color_at(0.5);

        assert_eq!(color_at_start, start_color);
        assert_eq!(color_at_end, end_color);
        // Middle should be a blend
        assert!(color_at_middle.r > 0 && color_at_middle.r < 255);
        assert!(color_at_middle.g > 0 && color_at_middle.g < 255);
    }

    #[test]
    fn test_radial_two_color_gradient() {
        let center_color = RgbaPixel::new(255, 255, 255, 255);
        let edge_color = RgbaPixel::new(0, 0, 0, 255);
        let gradient = Gradient::radial_two_color(center_color, edge_color);

        assert_eq!(gradient.gradient_type, GradientType::Radial);
        assert_eq!(gradient.stops.len(), 2);

        let color_at_center = gradient.color_at(0.0);
        let color_at_edge = gradient.color_at(1.0);

        assert_eq!(color_at_center, center_color);
        assert_eq!(color_at_edge, edge_color);
    }

    #[test]
    fn test_gradient_add_remove_stops() {
        let mut gradient = Gradient::default();
        let initial_count = gradient.stops.len();

        // Add a stop
        let red_stop = ColorStop::new(0.5, RgbaPixel::new(255, 0, 0, 255));
        let key = gradient.add_stop(red_stop);
        assert_eq!(gradient.stops.len(), initial_count + 1);

        // Remove the stop
        let removed = gradient.remove_stop(key);
        assert!(removed.is_some());
        assert_eq!(gradient.stops.len(), initial_count);

        // Cannot remove when only 2 stops remain
        let keys: Vec<_> = gradient.stops.keys().cloned().collect();
        let removed = gradient.remove_stop(keys[0]);
        assert!(removed.is_none());
        assert_eq!(gradient.stops.len(), 2);
    }

    #[test]
    fn test_gradient_color_interpolation() {
        let gradient = Gradient::linear_two_color(
            RgbaPixel::new(0, 0, 0, 255),
            RgbaPixel::new(255, 255, 255, 255),
        );

        // Test various positions
        let black = gradient.color_at(0.0);
        let white = gradient.color_at(1.0);
        let gray = gradient.color_at(0.5);

        assert_eq!(black, RgbaPixel::new(0, 0, 0, 255));
        assert_eq!(white, RgbaPixel::new(255, 255, 255, 255));
        assert_eq!(gray.r, 127); // Should be approximately middle gray
        assert_eq!(gray.g, 127);
        assert_eq!(gray.b, 127);
    }

    #[test]
    fn test_gradient_position_calculation() {
        let mut gradient = Gradient::default();
        gradient.set_linear_geometry(Point::new(0.0, 0.0), Point::new(100.0, 0.0));

        // Test linear position calculation
        let pos_start = gradient.position_for_point(Point::new(0.0, 0.0));
        let pos_middle = gradient.position_for_point(Point::new(50.0, 0.0));
        let pos_end = gradient.position_for_point(Point::new(100.0, 0.0));

        assert!((pos_start - 0.0).abs() < 0.001);
        assert!((pos_middle - 0.5).abs() < 0.001);
        assert!((pos_end - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_gradient_manager() {
        let manager = GradientManager::new();
        assert!(manager.count() > 0);
        assert!(manager.current_gradient().is_some());

        let gradient_names = manager.gradient_names();
        assert!(gradient_names.contains(&"Black to White".to_string()));
        assert!(gradient_names.contains(&"Rainbow".to_string()));
    }

    #[test]
    fn test_gradient_manager_operations() {
        let mut manager = GradientManager::new();
        let initial_count = manager.count();

        // Add a new gradient
        let custom_gradient = Gradient::linear_two_color(
            RgbaPixel::new(255, 0, 0, 255),
            RgbaPixel::new(0, 0, 255, 255),
        );
        manager.add_gradient(custom_gradient);
        assert_eq!(manager.count(), initial_count + 1);

        // Set current gradient
        assert!(manager.set_current_gradient("Red to Blue"));
        assert_eq!(manager.current_gradient().unwrap().name, "Red to Blue");

        // Remove gradient
        let removed = manager.remove_gradient("Red to Blue");
        assert!(removed.is_some());
        assert_eq!(manager.count(), initial_count);
    }

    #[test]
    fn test_gradient_preview() {
        let gradient = Gradient::linear_two_color(
            RgbaPixel::new(255, 0, 0, 255),
            RgbaPixel::new(0, 255, 0, 255),
        );

        let preview = gradient.create_preview(10, 1);
        assert_eq!(preview.len(), 10);

        // First pixel should be red
        assert_eq!(preview[0], RgbaPixel::new(255, 0, 0, 255));
        // Last pixel should be green
        assert_eq!(preview[9], RgbaPixel::new(0, 255, 0, 255));
    }

    #[test]
    fn test_hue_interpolation() {
        let gradient = Gradient::default();

        // Test normal case
        let result = gradient.interpolate_hue(0.0, 90.0, 0.5);
        assert!((result - 45.0).abs() < 0.001);

        // Test wraparound case (should take shorter path)
        let result = gradient.interpolate_hue(350.0, 10.0, 0.5);
        assert!((result - 0.0).abs() < 0.001 || (result - 360.0).abs() < 0.001);
    }
}
