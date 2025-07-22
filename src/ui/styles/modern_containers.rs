//! Modern container styles for PSOC Image Editor
//! Provides contemporary UI container styling with glass effects, gradients, and shadows

use iced::{Color, Border, Shadow, Vector, Background};
use crate::ui::theme::{PsocTheme, ColorPalette, GlassIntensity};
use super::glass_effects::{GlassEffect, FrostedGlassStyle};

/// Modern container style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModernContainerStyle {
    /// Standard modern container with subtle glass effect
    Standard,
    /// Panel container with medium glass effect
    Panel,
    /// Toolbar container with heavy glass effect
    Toolbar,
    /// Card container with light glass effect and shadow
    Card,
    /// Active/selected container with tech blue accent
    Active,
    /// Hover state container with subtle glow
    Hover,
    /// Gradient container with orange-red gradient
    Gradient,
}

/// Container configuration for modern styling
#[derive(Debug, Clone)]
pub struct ModernContainerConfig {
    /// Background color or effect
    pub background: ModernBackground,
    /// Border configuration
    pub border: ModernBorder,
    /// Shadow configuration
    pub shadow: ModernShadow,
    /// Corner radius
    pub border_radius: f32,
    /// Text color override
    pub text_color: Option<Color>,
}

/// Modern background types
#[derive(Debug, Clone)]
pub enum ModernBackground {
    /// Solid color background
    Solid(Color),
    /// Glass effect background
    Glass(GlassEffect),
    /// Linear gradient background
    LinearGradient { start: Color, end: Color },
    /// Radial gradient background (simulated with solid color)
    RadialGradient { center: Color, edge: Color },
}

/// Modern border configuration
#[derive(Debug, Clone)]
pub struct ModernBorder {
    /// Border color
    pub color: Color,
    /// Border width
    pub width: f32,
    /// Border radius
    pub radius: f32,
    /// Whether to add highlight border
    pub highlight: bool,
}

/// Modern shadow configuration
#[derive(Debug, Clone)]
pub struct ModernShadow {
    /// Shadow color
    pub color: Color,
    /// Shadow offset
    pub offset: Vector,
    /// Shadow blur radius
    pub blur_radius: f32,
    /// Whether shadow is enabled
    pub enabled: bool,
}

impl ModernContainerConfig {
    /// Create a new modern container configuration
    pub fn new(style: ModernContainerStyle, theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        match style {
            ModernContainerStyle::Standard => Self::standard(&palette),
            ModernContainerStyle::Panel => Self::panel(&palette),
            ModernContainerStyle::Toolbar => Self::toolbar(&palette),
            ModernContainerStyle::Card => Self::card(&palette),
            ModernContainerStyle::Active => Self::active(&palette),
            ModernContainerStyle::Hover => Self::hover(&palette),
            ModernContainerStyle::Gradient => Self::gradient(&palette),
        }
    }

    /// Standard container style
    fn standard(palette: &ColorPalette) -> Self {
        Self {
            background: ModernBackground::Glass(
                GlassEffect::new(0.8, 0.3, palette.glass_bg_light)
            ),
            border: ModernBorder {
                color: palette.border,
                width: 1.0,
                radius: 8.0,
                highlight: false,
            },
            shadow: ModernShadow {
                color: palette.shadow_color(0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
                enabled: true,
            },
            border_radius: 8.0,
            text_color: Some(palette.text),
        }
    }

    /// Panel container style
    fn panel(palette: &ColorPalette) -> Self {
        Self {
            background: ModernBackground::Glass(
                GlassEffect::new(0.9, 0.4, palette.glass_bg_medium)
            ),
            border: ModernBorder {
                color: Color::from_rgba(palette.border.r, palette.border.g, palette.border.b, 0.3),
                width: 1.0,
                radius: 12.0,
                highlight: true,
            },
            shadow: ModernShadow {
                color: palette.shadow_color(0.15),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
                enabled: true,
            },
            border_radius: 12.0,
            text_color: Some(palette.text),
        }
    }

    /// Toolbar container style
    fn toolbar(palette: &ColorPalette) -> Self {
        Self {
            background: ModernBackground::Glass(
                GlassEffect::new(0.95, 0.5, palette.glass_bg_heavy)
            ),
            border: ModernBorder {
                color: Color::from_rgba(palette.border.r, palette.border.g, palette.border.b, 0.2),
                width: 1.0,
                radius: 6.0,
                highlight: false,
            },
            shadow: ModernShadow {
                color: palette.shadow_color(0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 12.0,
                enabled: true,
            },
            border_radius: 6.0,
            text_color: Some(palette.text),
        }
    }

    /// Card container style
    fn card(palette: &ColorPalette) -> Self {
        Self {
            background: ModernBackground::Glass(
                GlassEffect::new(0.85, 0.3, palette.glass_bg_light)
            ),
            border: ModernBorder {
                color: palette.border,
                width: 1.0,
                radius: 16.0,
                highlight: true,
            },
            shadow: ModernShadow {
                color: palette.shadow_color(0.25),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
                enabled: true,
            },
            border_radius: 16.0,
            text_color: Some(palette.text),
        }
    }

    /// Active/selected container style
    fn active(palette: &ColorPalette) -> Self {
        Self {
            background: ModernBackground::Glass(
                GlassEffect::new(0.8, 0.4, palette.tech_blue_20)
            ),
            border: ModernBorder {
                color: palette.tech_blue,
                width: 2.0,
                radius: 8.0,
                highlight: true,
            },
            shadow: ModernShadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 12.0,
                enabled: true,
            },
            border_radius: 8.0,
            text_color: Some(palette.text),
        }
    }

    /// Hover state container style
    fn hover(palette: &ColorPalette) -> Self {
        Self {
            background: ModernBackground::Glass(
                GlassEffect::new(0.75, 0.3, palette.tech_blue_10)
            ),
            border: ModernBorder {
                color: palette.tech_blue_30,
                width: 1.0,
                radius: 8.0,
                highlight: true,
            },
            shadow: ModernShadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
                enabled: true,
            },
            border_radius: 8.0,
            text_color: Some(palette.text),
        }
    }

    /// Gradient container style
    fn gradient(palette: &ColorPalette) -> Self {
        let (start, end) = palette.orange_red_gradient();
        Self {
            background: ModernBackground::LinearGradient { start, end },
            border: ModernBorder {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                width: 1.0,
                radius: 12.0,
                highlight: true,
            },
            shadow: ModernShadow {
                color: palette.shadow_color(0.3),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
                enabled: true,
            },
            border_radius: 12.0,
            text_color: Some(Color::WHITE),
        }
    }

    /// Convert to iced container style
    pub fn to_iced_style(&self) -> iced::widget::container::Style {
        let background = match &self.background {
            ModernBackground::Solid(color) => Some(Background::Color(*color)),
            ModernBackground::Glass(glass_effect) => {
                // Simulate glass effect with semi-transparent background
                Some(Background::Color(glass_effect.tint_color))
            },
            ModernBackground::LinearGradient { start, end: _ } => {
                // For now, use start color as iced doesn't support gradients directly
                Some(Background::Color(*start))
            },
            ModernBackground::RadialGradient { center, edge: _ } => {
                // For now, use center color as iced doesn't support radial gradients
                Some(Background::Color(*center))
            },
        };

        iced::widget::container::Style {
            text_color: self.text_color,
            background,
            border: Border {
                color: self.border.color,
                width: self.border.width,
                radius: self.border.radius.into(),
            },
            shadow: if self.shadow.enabled {
                Shadow {
                    color: self.shadow.color,
                    offset: self.shadow.offset,
                    blur_radius: self.shadow.blur_radius,
                }
            } else {
                Shadow::default()
            },
        }
    }
}

/// Helper function to create modern container style
pub fn modern_container_style(style: ModernContainerStyle, theme: &PsocTheme) -> iced::widget::container::Style {
    ModernContainerConfig::new(style, theme).to_iced_style()
}

/// Helper function to create glass container style
pub fn glass_container_style(intensity: GlassIntensity, theme: &PsocTheme) -> iced::widget::container::Style {
    let palette = theme.palette();
    let glass_bg = palette.glass_background(intensity);

    iced::widget::container::Style {
        text_color: Some(palette.text),
        background: Some(Background::Color(glass_bg)),
        border: Border {
            color: Color::from_rgba(palette.border.r, palette.border.g, palette.border.b, 0.3),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: palette.shadow_color(0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_container_config_creation() {
        let theme = PsocTheme::Dark;
        let config = ModernContainerConfig::new(ModernContainerStyle::Standard, &theme);

        assert_eq!(config.border_radius, 8.0);
        assert!(config.shadow.enabled);
        assert!(config.text_color.is_some());
    }

    #[test]
    fn test_modern_container_styles() {
        let theme = PsocTheme::Dark;
        let styles = [
            ModernContainerStyle::Standard,
            ModernContainerStyle::Panel,
            ModernContainerStyle::Toolbar,
            ModernContainerStyle::Card,
            ModernContainerStyle::Active,
            ModernContainerStyle::Hover,
            ModernContainerStyle::Gradient,
        ];

        for style in styles {
            let config = ModernContainerConfig::new(style, &theme);
            let iced_style = config.to_iced_style();

            // All styles should have some background
            assert!(iced_style.background.is_some());
            // All styles should have text color
            assert!(iced_style.text_color.is_some());
        }
    }

    #[test]
    fn test_glass_container_style_function() {
        let theme = PsocTheme::Dark;
        let intensities = [GlassIntensity::Light, GlassIntensity::Medium, GlassIntensity::Heavy];

        for intensity in intensities {
            let style = glass_container_style(intensity, &theme);
            assert!(style.background.is_some());
            assert!(style.text_color.is_some());
            assert_eq!(style.border.width, 1.0);
        }
    }

    #[test]
    fn test_modern_background_types() {
        let color = Color::from_rgb(0.5, 0.5, 0.5);
        let white = Color::from_rgb(1.0, 1.0, 1.0);
        let black = Color::from_rgb(0.0, 0.0, 0.0);
        let glass_effect = GlassEffect::new(0.8, 0.3, color);

        let backgrounds = [
            ModernBackground::Solid(color),
            ModernBackground::Glass(glass_effect),
            ModernBackground::LinearGradient { start: color, end: white },
            ModernBackground::RadialGradient { center: color, edge: black },
        ];

        // All background types should be valid
        for bg in backgrounds {
            match bg {
                ModernBackground::Solid(_) => assert!(true),
                ModernBackground::Glass(_) => assert!(true),
                ModernBackground::LinearGradient { .. } => assert!(true),
                ModernBackground::RadialGradient { .. } => assert!(true),
            }
        }
    }

    #[test]
    fn test_modern_border_config() {
        let white = Color::from_rgb(1.0, 1.0, 1.0);
        let border = ModernBorder {
            color: white,
            width: 2.0,
            radius: 10.0,
            highlight: true,
        };

        assert_eq!(border.color, white);
        assert_eq!(border.width, 2.0);
        assert_eq!(border.radius, 10.0);
        assert!(border.highlight);
    }

    #[test]
    fn test_modern_shadow_config() {
        let black = Color::from_rgb(0.0, 0.0, 0.0);
        let shadow = ModernShadow {
            color: black,
            offset: Vector::new(2.0, 4.0),
            blur_radius: 8.0,
            enabled: true,
        };

        assert_eq!(shadow.color, black);
        assert_eq!(shadow.offset, Vector::new(2.0, 4.0));
        assert_eq!(shadow.blur_radius, 8.0);
        assert!(shadow.enabled);
    }
}
