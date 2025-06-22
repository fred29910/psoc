//! Frosted glass and transparency effects
//! Provides modern glass-like visual effects for UI components

use iced::{Color, Border, Shadow, Vector};
use crate::ui::theme::{PsocTheme, ColorPalette};

/// Glass effect configuration
#[derive(Debug, Clone)]
pub struct GlassEffect {
    /// Background transparency (0.0 = transparent, 1.0 = opaque)
    pub transparency: f32,
    /// Blur intensity (simulated through color mixing)
    pub blur_intensity: f32,
    /// Tint color applied to the glass
    pub tint_color: Color,
    /// Border highlight color
    pub border_highlight: Color,
    /// Whether to add a subtle inner glow
    pub inner_glow: bool,
}

/// Frosted glass style variants
#[derive(Debug, Clone, Copy)]
pub enum FrostedGlassStyle {
    /// Light frosted glass
    Light,
    /// Medium frosted glass
    Medium,
    /// Heavy frosted glass
    Heavy,
    /// Colored frosted glass with tech blue tint
    TechBlue,
    /// Subtle frosted glass for panels
    Subtle,
}

impl GlassEffect {
    /// Create a new glass effect
    pub fn new(transparency: f32, blur_intensity: f32, tint_color: Color) -> Self {
        Self {
            transparency: transparency.clamp(0.0, 1.0),
            blur_intensity: blur_intensity.clamp(0.0, 1.0),
            tint_color,
            border_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            inner_glow: false,
        }
    }

    /// Create frosted glass effect based on style
    pub fn frosted(style: FrostedGlassStyle, theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        match style {
            FrostedGlassStyle::Light => Self {
                transparency: 0.9,
                blur_intensity: 0.3,
                tint_color: Color::from_rgba(
                    palette.surface.r,
                    palette.surface.g,
                    palette.surface.b,
                    0.9,
                ),
                border_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                inner_glow: true,
            },
            FrostedGlassStyle::Medium => Self {
                transparency: 0.8,
                blur_intensity: 0.5,
                tint_color: Color::from_rgba(
                    palette.surface.r,
                    palette.surface.g,
                    palette.surface.b,
                    0.8,
                ),
                border_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                inner_glow: true,
            },
            FrostedGlassStyle::Heavy => Self {
                transparency: 0.7,
                blur_intensity: 0.8,
                tint_color: Color::from_rgba(
                    palette.surface.r,
                    palette.surface.g,
                    palette.surface.b,
                    0.7,
                ),
                border_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.25),
                inner_glow: true,
            },
            FrostedGlassStyle::TechBlue => Self {
                transparency: 0.85,
                blur_intensity: 0.4,
                tint_color: Color::from_rgba(
                    palette.tech_blue.r * 0.1 + palette.surface.r * 0.9,
                    palette.tech_blue.g * 0.1 + palette.surface.g * 0.9,
                    palette.tech_blue.b * 0.1 + palette.surface.b * 0.9,
                    0.85,
                ),
                border_highlight: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.3,
                ),
                inner_glow: true,
            },
            FrostedGlassStyle::Subtle => Self {
                transparency: 0.95,
                blur_intensity: 0.2,
                tint_color: Color::from_rgba(
                    palette.surface.r,
                    palette.surface.g,
                    palette.surface.b,
                    0.95,
                ),
                border_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                inner_glow: false,
            },
        }
    }

    /// Convert to iced container style
    pub fn to_container_style(&self) -> iced::widget::container::Style {
        iced::widget::container::Style {
            text_color: None, // Let the container inherit text color
            background: Some(self.tint_color.into()),
            border: Border {
                color: self.border_highlight,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: if self.inner_glow {
                Shadow {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                }
            } else {
                Shadow::default()
            },
        }
    }

    /// Create a glass effect for dropdown menus
    pub fn dropdown_glass(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            transparency: 0.85,
            blur_intensity: 0.6,
            tint_color: palette.glass_bg,
            border_highlight: Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.2,
            ),
            inner_glow: true,
        }
    }

    /// Create a glass effect for panels
    pub fn panel_glass(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            transparency: 0.9,
            blur_intensity: 0.3,
            tint_color: Color::from_rgba(
                palette.dark_panel.r,
                palette.dark_panel.g,
                palette.dark_panel.b,
                0.9,
            ),
            border_highlight: Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.3,
            ),
            inner_glow: false,
        }
    }

    /// Create a glass effect for hover states
    pub fn hover_glass(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            transparency: 0.8,
            blur_intensity: 0.4,
            tint_color: Color::from_rgba(
                palette.tech_blue.r * 0.2 + palette.surface.r * 0.8,
                palette.tech_blue.g * 0.2 + palette.surface.g * 0.8,
                palette.tech_blue.b * 0.2 + palette.surface.b * 0.8,
                0.8,
            ),
            border_highlight: Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.4,
            ),
            inner_glow: true,
        }
    }

    /// Animate between two glass effects
    pub fn interpolate(&self, target: &GlassEffect, progress: f32) -> GlassEffect {
        let progress = progress.clamp(0.0, 1.0);
        
        GlassEffect {
            transparency: self.transparency + (target.transparency - self.transparency) * progress,
            blur_intensity: self.blur_intensity + (target.blur_intensity - self.blur_intensity) * progress,
            tint_color: Color {
                r: self.tint_color.r + (target.tint_color.r - self.tint_color.r) * progress,
                g: self.tint_color.g + (target.tint_color.g - self.tint_color.g) * progress,
                b: self.tint_color.b + (target.tint_color.b - self.tint_color.b) * progress,
                a: self.tint_color.a + (target.tint_color.a - self.tint_color.a) * progress,
            },
            border_highlight: Color {
                r: self.border_highlight.r + (target.border_highlight.r - self.border_highlight.r) * progress,
                g: self.border_highlight.g + (target.border_highlight.g - self.border_highlight.g) * progress,
                b: self.border_highlight.b + (target.border_highlight.b - self.border_highlight.b) * progress,
                a: self.border_highlight.a + (target.border_highlight.a - self.border_highlight.a) * progress,
            },
            inner_glow: if progress > 0.5 { target.inner_glow } else { self.inner_glow },
        }
    }
}

/// Helper function to create glass container with shadow
pub fn glass_container_with_shadow(
    glass_effect: &GlassEffect,
    shadow_color: Color,
    shadow_offset: Vector,
    shadow_blur: f32,
) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None, // Let the container inherit text color
        background: Some(glass_effect.tint_color.into()),
        border: Border {
            color: glass_effect.border_highlight,
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: shadow_color,
            offset: shadow_offset,
            blur_radius: shadow_blur,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glass_effect_creation() {
        let theme = PsocTheme::Dark;
        let glass = GlassEffect::frosted(FrostedGlassStyle::Medium, &theme);
        
        assert!(glass.transparency > 0.0 && glass.transparency <= 1.0);
        assert!(glass.blur_intensity >= 0.0 && glass.blur_intensity <= 1.0);
    }

    #[test]
    fn test_glass_interpolation() {
        let theme = PsocTheme::Dark;
        let glass1 = GlassEffect::frosted(FrostedGlassStyle::Light, &theme);
        let glass2 = GlassEffect::frosted(FrostedGlassStyle::Heavy, &theme);
        
        let interpolated = glass1.interpolate(&glass2, 0.5);
        
        // Should be between the two values
        assert!(interpolated.transparency >= glass2.transparency);
        assert!(interpolated.transparency <= glass1.transparency);
    }

    #[test]
    fn test_container_style_conversion() {
        let theme = PsocTheme::Dark;
        let glass = GlassEffect::dropdown_glass(&theme);
        let style = glass.to_container_style();
        
        assert!(style.background.is_some());
        assert!(style.border.width > 0.0);
    }
}
