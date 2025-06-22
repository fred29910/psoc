//! Visual effects system for modern UI styling
//! Provides frosted glass, shadows, and other modern effects

use iced::{Color, Border, Shadow, Vector};
use crate::ui::theme::{PsocTheme, ColorPalette};

/// Visual effect configuration
#[derive(Debug, Clone)]
pub struct VisualEffectStyle {
    /// Background blur effect
    pub background_blur: Option<BlurEffect>,
    /// Drop shadow configuration
    pub drop_shadow: Option<DropShadowEffect>,
    /// Inner shadow/glow effect
    pub inner_shadow: Option<InnerShadowEffect>,
    /// Border configuration
    pub border_effect: Option<BorderEffect>,
    /// Gradient overlay
    pub gradient_overlay: Option<GradientEffect>,
}

/// Background blur effect configuration
#[derive(Debug, Clone)]
pub struct BlurEffect {
    /// Blur radius in pixels
    pub radius: f32,
    /// Background color with transparency
    pub background_color: Color,
    /// Saturation adjustment (1.0 = normal, 0.0 = grayscale)
    pub saturation: f32,
}

/// Drop shadow effect configuration
#[derive(Debug, Clone)]
pub struct DropShadowEffect {
    /// Shadow color
    pub color: Color,
    /// Shadow offset
    pub offset: Vector,
    /// Blur radius
    pub blur_radius: f32,
    /// Spread radius (expansion)
    pub spread: f32,
}

/// Inner shadow/glow effect
#[derive(Debug, Clone)]
pub struct InnerShadowEffect {
    /// Shadow color
    pub color: Color,
    /// Shadow offset (negative for inner)
    pub offset: Vector,
    /// Blur radius
    pub blur_radius: f32,
}

/// Border effect configuration
#[derive(Debug, Clone)]
pub struct BorderEffect {
    /// Border color
    pub color: Color,
    /// Border width
    pub width: f32,
    /// Border radius
    pub radius: f32,
    /// Gradient border colors (if any)
    pub gradient: Option<(Color, Color)>,
}

/// Gradient overlay effect
#[derive(Debug, Clone)]
pub struct GradientEffect {
    /// Start color
    pub start_color: Color,
    /// End color
    pub end_color: Color,
    /// Gradient direction (0.0 = horizontal, 90.0 = vertical)
    pub angle: f32,
    /// Opacity of the gradient
    pub opacity: f32,
}

impl Default for VisualEffectStyle {
    fn default() -> Self {
        Self {
            background_blur: None,
            drop_shadow: None,
            inner_shadow: None,
            border_effect: None,
            gradient_overlay: None,
        }
    }
}

impl VisualEffectStyle {
    /// Create a frosted glass effect
    pub fn frosted_glass(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            background_blur: Some(BlurEffect {
                radius: 20.0,
                background_color: Color::from_rgba(
                    palette.surface.r,
                    palette.surface.g,
                    palette.surface.b,
                    0.8, // Semi-transparent
                ),
                saturation: 1.1, // Slightly enhanced saturation
            }),
            drop_shadow: Some(DropShadowEffect {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
                spread: 0.0,
            }),
            border_effect: Some(BorderEffect {
                color: Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.3,
                ),
                width: 1.0,
                radius: 12.0,
                gradient: None,
            }),
            inner_shadow: None,
            gradient_overlay: None,
        }
    }

    /// Create a modern dropdown menu effect
    pub fn dropdown_menu(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            background_blur: Some(BlurEffect {
                radius: 16.0,
                background_color: palette.glass_bg,
                saturation: 1.0,
            }),
            drop_shadow: Some(DropShadowEffect {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
                spread: 0.0,
            }),
            border_effect: Some(BorderEffect {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.1,
                ),
                width: 1.0,
                radius: 8.0,
                gradient: None,
            }),
            inner_shadow: Some(InnerShadowEffect {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            }),
            gradient_overlay: None,
        }
    }

    /// Create a hover effect for menu items
    pub fn menu_item_hover(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            background_blur: None,
            drop_shadow: None,
            border_effect: None,
            inner_shadow: None,
            gradient_overlay: Some(GradientEffect {
                start_color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.1,
                ),
                end_color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.05,
                ),
                angle: 90.0, // Vertical gradient
                opacity: 1.0,
            }),
        }
    }

    /// Create a tech-blue accent effect
    pub fn tech_accent(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            background_blur: None,
            drop_shadow: Some(DropShadowEffect {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.3,
                ),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
                spread: 0.0,
            }),
            border_effect: Some(BorderEffect {
                color: palette.tech_blue,
                width: 1.0,
                radius: 4.0,
                gradient: Some((
                    palette.tech_blue,
                    Color::from_rgba(
                        palette.tech_blue.r,
                        palette.tech_blue.g,
                        palette.tech_blue.b,
                        0.5,
                    ),
                )),
            }),
            inner_shadow: None,
            gradient_overlay: None,
        }
    }

    /// Create a subtle panel effect
    pub fn panel_effect(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            background_blur: None,
            drop_shadow: Some(DropShadowEffect {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
                spread: 0.0,
            }),
            border_effect: Some(BorderEffect {
                color: Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.2,
                ),
                width: 1.0,
                radius: 6.0,
                gradient: None,
            }),
            inner_shadow: Some(InnerShadowEffect {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.02),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 1.0,
            }),
            gradient_overlay: None,
        }
    }
}

/// Apply visual effects to create an iced container style
pub fn apply_visual_effects(
    effects: &VisualEffectStyle,
    base_background: Option<Color>,
) -> iced::widget::container::Style {
    let mut style = iced::widget::container::Style::default();

    // Apply background color (with blur effect simulation)
    if let Some(blur) = &effects.background_blur {
        style.background = Some(blur.background_color.into());
    } else if let Some(bg) = base_background {
        style.background = Some(bg.into());
    }

    // Apply border effects
    if let Some(border) = &effects.border_effect {
        style.border = Border {
            color: border.color,
            width: border.width,
            radius: border.radius.into(),
        };
    }

    // Apply drop shadow
    if let Some(shadow) = &effects.drop_shadow {
        style.shadow = Shadow {
            color: shadow.color,
            offset: shadow.offset,
            blur_radius: shadow.blur_radius,
        };
    }

    // Set text color to None to inherit from parent
    style.text_color = None;

    style
}

/// Create a smooth transition between two visual effect styles
pub fn transition_effects(
    from: &VisualEffectStyle,
    to: &VisualEffectStyle,
    progress: f32, // 0.0 to 1.0
) -> VisualEffectStyle {
    let progress = progress.clamp(0.0, 1.0);
    
    // For now, return the target style when progress > 0.5, otherwise source
    // In a full implementation, we would interpolate all values
    if progress > 0.5 {
        to.clone()
    } else {
        from.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_effect_creation() {
        let theme = PsocTheme::Dark;
        let effect = VisualEffectStyle::frosted_glass(&theme);
        
        assert!(effect.background_blur.is_some());
        assert!(effect.drop_shadow.is_some());
        assert!(effect.border_effect.is_some());
    }

    #[test]
    fn test_apply_visual_effects() {
        let theme = PsocTheme::Dark;
        let effects = VisualEffectStyle::dropdown_menu(&theme);
        let style = apply_visual_effects(&effects, None);
        
        assert!(style.background.is_some());
        assert!(style.border.width > 0.0);
    }
}
