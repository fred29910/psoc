//! Advanced shadow system for depth and visual hierarchy
//! Provides various shadow effects for modern UI design

use iced::{Color, Shadow, Vector};
use crate::ui::theme::{PsocTheme, ColorPalette};

/// Shadow configuration levels
#[derive(Debug, Clone, Copy)]
pub enum ShadowLevel {
    /// No shadow
    None,
    /// Subtle shadow for slight elevation
    Subtle,
    /// Low shadow for cards and panels
    Low,
    /// Medium shadow for dropdowns and modals
    Medium,
    /// High shadow for floating elements
    High,
    /// Very high shadow for tooltips and overlays
    VeryHigh,
}

/// Drop shadow configuration
#[derive(Debug, Clone)]
pub struct DropShadow {
    /// Shadow color
    pub color: Color,
    /// Horizontal and vertical offset
    pub offset: Vector,
    /// Blur radius
    pub blur_radius: f32,
    /// Spread radius (expansion of shadow)
    pub spread: f32,
}

/// Inner shadow configuration
#[derive(Debug, Clone)]
pub struct InnerShadow {
    /// Shadow color
    pub color: Color,
    /// Horizontal and vertical offset
    pub offset: Vector,
    /// Blur radius
    pub blur_radius: f32,
}

/// Complete shadow configuration
#[derive(Debug, Clone)]
pub struct ShadowConfig {
    /// Primary drop shadow
    pub drop_shadow: Option<DropShadow>,
    /// Secondary drop shadow for layered effect
    pub secondary_shadow: Option<DropShadow>,
    /// Inner shadow for depth
    pub inner_shadow: Option<InnerShadow>,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            drop_shadow: None,
            secondary_shadow: None,
            inner_shadow: None,
        }
    }
}

impl DropShadow {
    /// Create a new drop shadow
    pub fn new(color: Color, offset: Vector, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
            spread: 0.0,
        }
    }

    /// Create a drop shadow with spread
    pub fn with_spread(color: Color, offset: Vector, blur_radius: f32, spread: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
            spread,
        }
    }

    /// Create shadow based on elevation level
    pub fn for_level(level: ShadowLevel, theme: &PsocTheme) -> Option<Self> {
        let palette = theme.palette();
        let base_shadow_color = match theme {
            PsocTheme::Dark => Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            PsocTheme::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            PsocTheme::HighContrast => Color::from_rgba(0.0, 0.0, 0.0, 0.8),
        };

        match level {
            ShadowLevel::None => None,
            ShadowLevel::Subtle => Some(Self::new(
                Color::from_rgba(base_shadow_color.r, base_shadow_color.g, base_shadow_color.b, 0.1),
                Vector::new(0.0, 1.0),
                2.0,
            )),
            ShadowLevel::Low => Some(Self::new(
                Color::from_rgba(base_shadow_color.r, base_shadow_color.g, base_shadow_color.b, 0.15),
                Vector::new(0.0, 2.0),
                4.0,
            )),
            ShadowLevel::Medium => Some(Self::new(
                Color::from_rgba(base_shadow_color.r, base_shadow_color.g, base_shadow_color.b, 0.2),
                Vector::new(0.0, 4.0),
                8.0,
            )),
            ShadowLevel::High => Some(Self::new(
                Color::from_rgba(base_shadow_color.r, base_shadow_color.g, base_shadow_color.b, 0.25),
                Vector::new(0.0, 8.0),
                16.0,
            )),
            ShadowLevel::VeryHigh => Some(Self::new(
                Color::from_rgba(base_shadow_color.r, base_shadow_color.g, base_shadow_color.b, 0.3),
                Vector::new(0.0, 12.0),
                24.0,
            )),
        }
    }

    /// Convert to iced Shadow
    pub fn to_iced_shadow(&self) -> Shadow {
        Shadow {
            color: self.color,
            offset: self.offset,
            blur_radius: self.blur_radius,
        }
    }
}

impl InnerShadow {
    /// Create a new inner shadow
    pub fn new(color: Color, offset: Vector, blur_radius: f32) -> Self {
        Self {
            color,
            offset,
            blur_radius,
        }
    }

    /// Create inner glow effect
    pub fn inner_glow(color: Color, intensity: f32) -> Self {
        Self {
            color: Color::from_rgba(color.r, color.g, color.b, intensity),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 4.0,
        }
    }

    /// Create tech blue inner glow
    pub fn tech_glow(theme: &PsocTheme, intensity: f32) -> Self {
        let palette = theme.palette();
        Self::inner_glow(palette.tech_blue, intensity)
    }
}

impl ShadowConfig {
    /// Create shadow configuration for dropdown menus
    pub fn dropdown_menu(theme: &PsocTheme) -> Self {
        Self {
            drop_shadow: DropShadow::for_level(ShadowLevel::Medium, theme),
            secondary_shadow: Some(DropShadow::new(
                Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                Vector::new(0.0, 2.0),
                4.0,
            )),
            inner_shadow: Some(InnerShadow::new(
                Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                Vector::new(0.0, 1.0),
                1.0,
            )),
        }
    }

    /// Create shadow configuration for panels
    pub fn panel(theme: &PsocTheme) -> Self {
        Self {
            drop_shadow: DropShadow::for_level(ShadowLevel::Low, theme),
            secondary_shadow: None,
            inner_shadow: Some(InnerShadow::new(
                Color::from_rgba(1.0, 1.0, 1.0, 0.02),
                Vector::new(0.0, 1.0),
                1.0,
            )),
        }
    }

    /// Create shadow configuration for floating elements
    pub fn floating(theme: &PsocTheme) -> Self {
        Self {
            drop_shadow: DropShadow::for_level(ShadowLevel::High, theme),
            secondary_shadow: Some(DropShadow::new(
                Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                Vector::new(0.0, 4.0),
                8.0,
            )),
            inner_shadow: None,
        }
    }

    /// Create shadow configuration for buttons
    pub fn button(theme: &PsocTheme, is_pressed: bool) -> Self {
        if is_pressed {
            Self {
                drop_shadow: DropShadow::for_level(ShadowLevel::Subtle, theme),
                secondary_shadow: None,
                inner_shadow: Some(InnerShadow::new(
                    Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    Vector::new(0.0, 1.0),
                    2.0,
                )),
            }
        } else {
            Self {
                drop_shadow: DropShadow::for_level(ShadowLevel::Low, theme),
                secondary_shadow: None,
                inner_shadow: None,
            }
        }
    }

    /// Create tech-blue accent shadow
    pub fn tech_accent(theme: &PsocTheme) -> Self {
        let palette = theme.palette();
        
        Self {
            drop_shadow: Some(DropShadow::new(
                Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.3,
                ),
                Vector::new(0.0, 2.0),
                8.0,
            )),
            secondary_shadow: None,
            inner_shadow: Some(InnerShadow::tech_glow(theme, 0.2)),
        }
    }

    /// Get the primary shadow for iced
    pub fn primary_iced_shadow(&self) -> Shadow {
        if let Some(ref shadow) = self.drop_shadow {
            shadow.to_iced_shadow()
        } else {
            Shadow::default()
        }
    }

    /// Animate between two shadow configurations
    pub fn interpolate(&self, target: &ShadowConfig, progress: f32) -> ShadowConfig {
        let progress = progress.clamp(0.0, 1.0);
        
        // For simplicity, we'll just transition the primary drop shadow
        let drop_shadow = match (&self.drop_shadow, &target.drop_shadow) {
            (Some(from), Some(to)) => Some(DropShadow {
                color: Color {
                    r: from.color.r + (to.color.r - from.color.r) * progress,
                    g: from.color.g + (to.color.g - from.color.g) * progress,
                    b: from.color.b + (to.color.b - from.color.b) * progress,
                    a: from.color.a + (to.color.a - from.color.a) * progress,
                },
                offset: Vector::new(
                    from.offset.x + (to.offset.x - from.offset.x) * progress,
                    from.offset.y + (to.offset.y - from.offset.y) * progress,
                ),
                blur_radius: from.blur_radius + (to.blur_radius - from.blur_radius) * progress,
                spread: from.spread + (to.spread - from.spread) * progress,
            }),
            (None, Some(to)) => Some(DropShadow {
                color: Color::from_rgba(to.color.r, to.color.g, to.color.b, to.color.a * progress),
                offset: Vector::new(to.offset.x * progress, to.offset.y * progress),
                blur_radius: to.blur_radius * progress,
                spread: to.spread * progress,
            }),
            (Some(from), None) => Some(DropShadow {
                color: Color::from_rgba(from.color.r, from.color.g, from.color.b, from.color.a * (1.0 - progress)),
                offset: Vector::new(from.offset.x * (1.0 - progress), from.offset.y * (1.0 - progress)),
                blur_radius: from.blur_radius * (1.0 - progress),
                spread: from.spread * (1.0 - progress),
            }),
            (None, None) => None,
        };

        ShadowConfig {
            drop_shadow,
            secondary_shadow: if progress > 0.5 { target.secondary_shadow.clone() } else { self.secondary_shadow.clone() },
            inner_shadow: if progress > 0.5 { target.inner_shadow.clone() } else { self.inner_shadow.clone() },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_level_creation() {
        let theme = PsocTheme::Dark;
        let shadow = DropShadow::for_level(ShadowLevel::Medium, &theme);
        
        assert!(shadow.is_some());
        let shadow = shadow.unwrap();
        assert!(shadow.blur_radius > 0.0);
    }

    #[test]
    fn test_shadow_config_dropdown() {
        let theme = PsocTheme::Dark;
        let config = ShadowConfig::dropdown_menu(&theme);
        
        assert!(config.drop_shadow.is_some());
        assert!(config.secondary_shadow.is_some());
    }

    #[test]
    fn test_shadow_interpolation() {
        let theme = PsocTheme::Dark;
        let config1 = ShadowConfig::panel(&theme);
        let config2 = ShadowConfig::floating(&theme);
        
        let interpolated = config1.interpolate(&config2, 0.5);
        assert!(interpolated.drop_shadow.is_some());
    }
}
