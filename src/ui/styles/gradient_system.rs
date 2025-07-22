//! Gradient system for modern UI effects
//! Provides gradient definitions and utilities for PSOC Image Editor

use iced::Color;
use crate::ui::theme::{PsocTheme, ColorPalette};

/// Gradient direction types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    /// Horizontal gradient (left to right)
    Horizontal,
    /// Vertical gradient (top to bottom)
    Vertical,
    /// Diagonal gradient (top-left to bottom-right)
    Diagonal,
    /// Radial gradient (center to edge)
    Radial,
}

/// Gradient stop point
#[derive(Debug, Clone)]
pub struct GradientStop {
    /// Color at this stop
    pub color: Color,
    /// Position along gradient (0.0 to 1.0)
    pub position: f32,
}

/// Gradient definition
#[derive(Debug, Clone)]
pub struct Gradient {
    /// Gradient stops
    pub stops: Vec<GradientStop>,
    /// Gradient direction
    pub direction: GradientDirection,
    /// Whether gradient repeats
    pub repeating: bool,
}

/// Predefined gradient types for PSOC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsocGradient {
    /// Orange to red gradient (primary brand gradient)
    OrangeRed,
    /// Tech blue gradient (various blue shades)
    TechBlue,
    /// Dark panel gradient (subtle dark variations)
    DarkPanel,
    /// Glass effect gradient (transparency variations)
    GlassEffect,
    /// Sunset gradient (warm colors)
    Sunset,
    /// Ocean gradient (cool blues)
    Ocean,
    /// Forest gradient (greens)
    Forest,
    /// Monochrome gradient (grays)
    Monochrome,
}

impl GradientStop {
    /// Create a new gradient stop
    pub fn new(color: Color, position: f32) -> Self {
        Self {
            color,
            position: position.clamp(0.0, 1.0),
        }
    }
}

impl Gradient {
    /// Create a new gradient
    pub fn new(stops: Vec<GradientStop>, direction: GradientDirection) -> Self {
        Self {
            stops,
            direction,
            repeating: false,
        }
    }

    /// Create a simple two-color gradient
    pub fn simple(start: Color, end: Color, direction: GradientDirection) -> Self {
        Self::new(
            vec![
                GradientStop::new(start, 0.0),
                GradientStop::new(end, 1.0),
            ],
            direction,
        )
    }

    /// Create a predefined PSOC gradient
    pub fn psoc_gradient(gradient_type: PsocGradient, theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        match gradient_type {
            PsocGradient::OrangeRed => {
                let (start, end) = palette.orange_red_gradient();
                Self::simple(start, end, GradientDirection::Diagonal)
            },
            PsocGradient::TechBlue => Self::new(
                vec![
                    GradientStop::new(palette.tech_blue_10, 0.0),
                    GradientStop::new(palette.tech_blue_50, 0.5),
                    GradientStop::new(palette.tech_blue_80, 1.0),
                ],
                GradientDirection::Vertical,
            ),
            PsocGradient::DarkPanel => Self::new(
                vec![
                    GradientStop::new(palette.dark_panel, 0.0),
                    GradientStop::new(palette.surface, 0.5),
                    GradientStop::new(palette.dark_bg, 1.0),
                ],
                GradientDirection::Vertical,
            ),
            PsocGradient::GlassEffect => Self::new(
                vec![
                    GradientStop::new(palette.glass_bg_light, 0.0),
                    GradientStop::new(palette.glass_bg_medium, 0.5),
                    GradientStop::new(palette.glass_bg_heavy, 1.0),
                ],
                GradientDirection::Radial,
            ),
            PsocGradient::Sunset => Self::new(
                vec![
                    GradientStop::new(Color::from_rgb(1.0, 0.8, 0.2), 0.0),  // Golden yellow
                    GradientStop::new(Color::from_rgb(1.0, 0.5, 0.1), 0.5),  // Orange
                    GradientStop::new(Color::from_rgb(0.9, 0.2, 0.4), 1.0),  // Deep red
                ],
                GradientDirection::Horizontal,
            ),
            PsocGradient::Ocean => Self::new(
                vec![
                    GradientStop::new(Color::from_rgb(0.0, 0.8, 1.0), 0.0),  // Light blue
                    GradientStop::new(Color::from_rgb(0.0, 0.5, 0.8), 0.5),  // Medium blue
                    GradientStop::new(Color::from_rgb(0.0, 0.2, 0.6), 1.0),  // Deep blue
                ],
                GradientDirection::Vertical,
            ),
            PsocGradient::Forest => Self::new(
                vec![
                    GradientStop::new(Color::from_rgb(0.6, 1.0, 0.4), 0.0),  // Light green
                    GradientStop::new(Color::from_rgb(0.2, 0.8, 0.2), 0.5),  // Medium green
                    GradientStop::new(Color::from_rgb(0.1, 0.4, 0.1), 1.0),  // Dark green
                ],
                GradientDirection::Diagonal,
            ),
            PsocGradient::Monochrome => Self::new(
                vec![
                    GradientStop::new(Color::from_rgb(0.9, 0.9, 0.9), 0.0),  // Light gray
                    GradientStop::new(Color::from_rgb(0.5, 0.5, 0.5), 0.5),  // Medium gray
                    GradientStop::new(Color::from_rgb(0.1, 0.1, 0.1), 1.0),  // Dark gray
                ],
                GradientDirection::Vertical,
            ),
        }
    }

    /// Get color at specific position (0.0 to 1.0)
    pub fn color_at(&self, position: f32) -> Color {
        let position = position.clamp(0.0, 1.0);
        
        if self.stops.is_empty() {
            return Color::TRANSPARENT;
        }
        
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }
        
        // Find the two stops to interpolate between
        let mut before_stop = &self.stops[0];
        let mut after_stop = &self.stops[self.stops.len() - 1];
        
        for i in 0..self.stops.len() - 1 {
            if position >= self.stops[i].position && position <= self.stops[i + 1].position {
                before_stop = &self.stops[i];
                after_stop = &self.stops[i + 1];
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
        
        // Interpolate between the two colors
        let range = after_stop.position - before_stop.position;
        let local_position = (position - before_stop.position) / range;
        
        self.interpolate_colors(before_stop.color, after_stop.color, local_position)
    }

    /// Interpolate between two colors
    fn interpolate_colors(&self, start: Color, end: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        
        Color::from_rgba(
            start.r + (end.r - start.r) * t,
            start.g + (end.g - start.g) * t,
            start.b + (end.b - start.b) * t,
            start.a + (end.a - start.a) * t,
        )
    }

    /// Get start color
    pub fn start_color(&self) -> Color {
        self.stops.first().map(|s| s.color).unwrap_or(Color::TRANSPARENT)
    }

    /// Get end color
    pub fn end_color(&self) -> Color {
        self.stops.last().map(|s| s.color).unwrap_or(Color::TRANSPARENT)
    }

    /// Get middle color (for three-stop gradients)
    pub fn middle_color(&self) -> Option<Color> {
        if self.stops.len() >= 3 {
            Some(self.stops[1].color)
        } else {
            None
        }
    }

    /// Create a reversed gradient
    pub fn reversed(&self) -> Self {
        let mut reversed_stops: Vec<GradientStop> = self.stops
            .iter()
            .map(|stop| GradientStop::new(stop.color, 1.0 - stop.position))
            .collect();
        
        reversed_stops.reverse();
        
        Self {
            stops: reversed_stops,
            direction: self.direction,
            repeating: self.repeating,
        }
    }

    /// Add a stop to the gradient
    pub fn add_stop(&mut self, color: Color, position: f32) {
        let stop = GradientStop::new(color, position);
        
        // Insert in the correct position to maintain order
        let insert_pos = self.stops
            .iter()
            .position(|s| s.position > position)
            .unwrap_or(self.stops.len());
        
        self.stops.insert(insert_pos, stop);
    }

    /// Remove a stop from the gradient
    pub fn remove_stop(&mut self, index: usize) {
        if index < self.stops.len() && self.stops.len() > 2 {
            self.stops.remove(index);
        }
    }

    /// Set repeating mode
    pub fn set_repeating(&mut self, repeating: bool) {
        self.repeating = repeating;
    }
}

/// Gradient utilities
pub struct GradientUtils;

impl GradientUtils {
    /// Create a gradient from theme colors
    pub fn from_theme_colors(theme: &PsocTheme, direction: GradientDirection) -> Gradient {
        let palette = theme.palette();
        
        Gradient::new(
            vec![
                GradientStop::new(palette.primary, 0.0),
                GradientStop::new(palette.secondary, 1.0),
            ],
            direction,
        )
    }

    /// Create a glass effect gradient
    pub fn glass_gradient(theme: &PsocTheme) -> Gradient {
        Gradient::psoc_gradient(PsocGradient::GlassEffect, theme)
    }

    /// Create a tech blue gradient
    pub fn tech_blue_gradient(theme: &PsocTheme) -> Gradient {
        Gradient::psoc_gradient(PsocGradient::TechBlue, theme)
    }

    /// Create an orange-red gradient
    pub fn orange_red_gradient(theme: &PsocTheme) -> Gradient {
        Gradient::psoc_gradient(PsocGradient::OrangeRed, theme)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_stop_creation() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let stop = GradientStop::new(red, 0.5);
        assert_eq!(stop.color, red);
        assert_eq!(stop.position, 0.5);

        // Test clamping
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let stop_clamped = GradientStop::new(blue, 1.5);
        assert_eq!(stop_clamped.position, 1.0);
    }

    #[test]
    fn test_simple_gradient_creation() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let gradient = Gradient::simple(red, blue, GradientDirection::Horizontal);
        assert_eq!(gradient.stops.len(), 2);
        assert_eq!(gradient.direction, GradientDirection::Horizontal);
        assert!(!gradient.repeating);
    }

    #[test]
    fn test_psoc_gradients() {
        let theme = PsocTheme::Dark;
        let gradients = [
            PsocGradient::OrangeRed,
            PsocGradient::TechBlue,
            PsocGradient::DarkPanel,
            PsocGradient::GlassEffect,
            PsocGradient::Sunset,
            PsocGradient::Ocean,
            PsocGradient::Forest,
            PsocGradient::Monochrome,
        ];

        for gradient_type in gradients {
            let gradient = Gradient::psoc_gradient(gradient_type, &theme);
            assert!(!gradient.stops.is_empty());
            assert!(gradient.stops.len() >= 2);
        }
    }

    #[test]
    fn test_gradient_color_interpolation() {
        let black = Color::from_rgb(0.0, 0.0, 0.0);
        let white = Color::from_rgb(1.0, 1.0, 1.0);
        let gradient = Gradient::simple(black, white, GradientDirection::Horizontal);

        let start_color = gradient.color_at(0.0);
        let middle_color = gradient.color_at(0.5);
        let end_color = gradient.color_at(1.0);

        assert_eq!(start_color, black);
        assert_eq!(end_color, white);

        // Middle should be gray
        assert!(middle_color.r > 0.4 && middle_color.r < 0.6);
        assert!(middle_color.g > 0.4 && middle_color.g < 0.6);
        assert!(middle_color.b > 0.4 && middle_color.b < 0.6);
    }

    #[test]
    fn test_gradient_start_end_colors() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let gradient = Gradient::simple(red, blue, GradientDirection::Vertical);
        assert_eq!(gradient.start_color(), red);
        assert_eq!(gradient.end_color(), blue);
    }

    #[test]
    fn test_gradient_reversal() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let original = Gradient::simple(red, blue, GradientDirection::Horizontal);
        let reversed = original.reversed();

        assert_eq!(original.start_color(), reversed.end_color());
        assert_eq!(original.end_color(), reversed.start_color());
    }

    #[test]
    fn test_gradient_add_remove_stops() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let green = Color::from_rgb(0.0, 1.0, 0.0);
        let mut gradient = Gradient::simple(red, blue, GradientDirection::Horizontal);
        assert_eq!(gradient.stops.len(), 2);

        gradient.add_stop(green, 0.5);
        assert_eq!(gradient.stops.len(), 3);
        assert_eq!(gradient.middle_color(), Some(green));

        gradient.remove_stop(1); // Remove middle stop
        assert_eq!(gradient.stops.len(), 2);
        assert_eq!(gradient.middle_color(), None);
    }

    #[test]
    fn test_gradient_direction_types() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let directions = [
            GradientDirection::Horizontal,
            GradientDirection::Vertical,
            GradientDirection::Diagonal,
            GradientDirection::Radial,
        ];

        for direction in directions {
            let gradient = Gradient::simple(red, blue, direction);
            assert_eq!(gradient.direction, direction);
        }
    }

    #[test]
    fn test_gradient_utils() {
        let theme = PsocTheme::Dark;

        let theme_gradient = GradientUtils::from_theme_colors(&theme, GradientDirection::Horizontal);
        assert_eq!(theme_gradient.stops.len(), 2);

        let glass_gradient = GradientUtils::glass_gradient(&theme);
        assert!(!glass_gradient.stops.is_empty());

        let tech_gradient = GradientUtils::tech_blue_gradient(&theme);
        assert!(!tech_gradient.stops.is_empty());

        let orange_red_gradient = GradientUtils::orange_red_gradient(&theme);
        assert!(!orange_red_gradient.stops.is_empty());
    }

    #[test]
    fn test_gradient_repeating_mode() {
        let red = Color::from_rgb(1.0, 0.0, 0.0);
        let blue = Color::from_rgb(0.0, 0.0, 1.0);
        let mut gradient = Gradient::simple(red, blue, GradientDirection::Horizontal);
        assert!(!gradient.repeating);

        gradient.set_repeating(true);
        assert!(gradient.repeating);
    }
}
