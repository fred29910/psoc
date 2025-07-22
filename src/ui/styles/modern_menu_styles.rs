//! Modern menu styling system with enhanced visual effects
//! Provides professional menu bar, dropdown, and menu item styles

use iced::{
    widget::{container, button},
    Background, Color, Border, Shadow, Vector,
};

use crate::ui::theme::{PsocTheme, ColorPalette, GlassIntensity};
use super::glass_effects::GlassEffect;

/// Modern menu bar style with glass effect and professional appearance
pub fn modern_menu_bar_style(theme: &PsocTheme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            palette.glass_bg_light.r,
            palette.glass_bg_light.g,
            palette.glass_bg_light.b,
            0.95, // High opacity for menu bar
        ))),
        border: Border {
            color: Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.2
            ),
            width: 0.0,
            radius: 0.0.into(), // Menu bars typically have no radius
        },
        shadow: Shadow {
            color: palette.shadow_color(0.15),
            offset: Vector::new(0.0, 2.0), // Subtle downward shadow
            blur_radius: 12.0,
        },
        text_color: Some(palette.text),
    }
}

/// Enhanced dropdown menu style with modern glass effect
pub fn modern_dropdown_style(theme: &PsocTheme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            palette.glass_bg_medium.r,
            palette.glass_bg_medium.g,
            palette.glass_bg_medium.b,
            0.92, // Slightly transparent for glass effect
        ))),
        border: Border {
            color: Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.4
            ),
            width: 1.0,
            radius: 12.0.into(), // Modern rounded corners
        },
        shadow: Shadow {
            color: palette.shadow_color(0.25),
            offset: Vector::new(0.0, 8.0), // Elevated shadow for dropdown
            blur_radius: 24.0,
        },
        text_color: Some(palette.text),
    }
}

/// Modern menu button style with hover and active states
pub fn modern_menu_button_style(
    palette: &ColorPalette,
    status: button::Status,
    is_active: bool,
) -> button::Style {
    let (background, text_color, shadow) = match (status, is_active) {
        // Active menu button (menu is open)
        (_, true) => (
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.2
            ))),
            Color::WHITE,
            Shadow {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.3
                ),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        ),
        // Hovered state
        (button::Status::Hovered, false) => (
            Some(Background::Color(Color::from_rgba(
                palette.glass_bg_light.r,
                palette.glass_bg_light.g,
                palette.glass_bg_light.b,
                0.8
            ))),
            palette.tech_blue,
            Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
        ),
        // Pressed state
        (button::Status::Active, false) => (
            Some(Background::Color(Color::from_rgba(
                palette.glass_bg_medium.r,
                palette.glass_bg_medium.g,
                palette.glass_bg_medium.b,
                0.6
            ))),
            palette.text,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
        // Default state
        _ => (
            Some(Background::Color(Color::TRANSPARENT)),
            palette.text,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
    };

    button::Style {
        background,
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(), // Subtle rounded corners
        },
        shadow,
    }
}

/// Modern menu item style with enhanced hover effects
pub fn modern_menu_item_style(
    palette: &ColorPalette,
    status: button::Status,
) -> button::Style {
    let (background, text_color, shadow) = match status {
        button::Status::Hovered => (
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.15
            ))),
            Color::WHITE,
            Shadow {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.1
                ),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
        ),
        button::Status::Active => (
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.25
            ))),
            Color::WHITE,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
        _ => (
            Some(Background::Color(Color::TRANSPARENT)),
            palette.text,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
    };

    button::Style {
        background,
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow,
    }
}

/// Menu separator style
pub fn menu_separator_style(palette: &ColorPalette) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            palette.border.r,
            palette.border.g,
            palette.border.b,
            0.3
        ))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.5.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
        text_color: None,
    }
}

/// Underline indicator style for active menu
pub fn menu_underline_style(palette: &ColorPalette, is_active: bool) -> container::Style {
    let (background, shadow) = if is_active {
        (
            Some(Background::Color(palette.tech_blue)),
            Shadow {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.4
                ),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
        )
    } else {
        (
            Some(Background::Color(Color::TRANSPARENT)),
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        )
    };

    container::Style {
        background,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 1.0.into(),
        },
        shadow,
        text_color: None,
    }
}

/// Enhanced menu typography settings
pub struct MenuTypography {
    /// Menu bar text size
    pub menu_bar_text_size: f32,
    /// Menu item text size
    pub menu_item_text_size: f32,
    /// Shortcut text size
    pub shortcut_text_size: f32,
    /// Menu bar padding
    pub menu_bar_padding: [f32; 2],
    /// Menu item padding
    pub menu_item_padding: [f32; 2],
    /// Menu spacing
    pub menu_spacing: f32,
}

impl Default for MenuTypography {
    fn default() -> Self {
        Self {
            menu_bar_text_size: 14.0,
            menu_item_text_size: 13.0,
            shortcut_text_size: 11.0,
            menu_bar_padding: [12.0, 20.0],
            menu_item_padding: [8.0, 16.0],
            menu_spacing: 2.0,
        }
    }
}

impl MenuTypography {
    /// Create typography settings optimized for different screen densities
    pub fn for_scale_factor(scale_factor: f32) -> Self {
        let base = Self::default();
        Self {
            menu_bar_text_size: base.menu_bar_text_size * scale_factor,
            menu_item_text_size: base.menu_item_text_size * scale_factor,
            shortcut_text_size: base.shortcut_text_size * scale_factor,
            menu_bar_padding: [
                base.menu_bar_padding[0] * scale_factor,
                base.menu_bar_padding[1] * scale_factor,
            ],
            menu_item_padding: [
                base.menu_item_padding[0] * scale_factor,
                base.menu_item_padding[1] * scale_factor,
            ],
            menu_spacing: base.menu_spacing * scale_factor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_bar_style() {
        let theme = PsocTheme::Dark;
        let style = modern_menu_bar_style(&theme);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should have shadow
        assert!(style.shadow.blur_radius > 0.0);
        
        // Should have text color
        assert!(style.text_color.is_some());
    }

    #[test]
    fn test_dropdown_style() {
        let theme = PsocTheme::Dark;
        let style = modern_dropdown_style(&theme);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should have border
        assert!(style.border.width > 0.0);
        assert!(style.border.radius.top_left > 0.0);
        
        // Should have elevated shadow
        assert!(style.shadow.blur_radius > 10.0);
    }

    #[test]
    fn test_menu_button_styles() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();
        
        let states = [
            (button::Status::Disabled, false),
            (button::Status::Hovered, false),
            (button::Status::Active, false),
            (button::Status::Disabled, true),
            (button::Status::Hovered, true),
            (button::Status::Active, true),
        ];

        for (status, is_active) in states {
            let style = modern_menu_button_style(&palette, status, is_active);
            
            // Should have valid background
            assert!(style.background.is_some());
            
            // Active buttons should have different styling
            if is_active {
                assert_eq!(style.text_color, Color::WHITE);
            }
        }
    }

    #[test]
    fn test_menu_item_styles() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();
        
        let states = [
            button::Status::Disabled,
            button::Status::Hovered,
            button::Status::Active,
        ];

        for status in states {
            let style = modern_menu_item_style(&palette, status);
            
            // Should have valid background
            assert!(style.background.is_some());
            
            // Hovered state should have white text
            if status == button::Status::Hovered {
                assert_eq!(style.text_color, Color::WHITE);
            }
        }
    }

    #[test]
    fn test_menu_typography() {
        let typography = MenuTypography::default();
        
        // Should have reasonable default values
        assert!(typography.menu_bar_text_size > 10.0);
        assert!(typography.menu_item_text_size > 10.0);
        assert!(typography.shortcut_text_size > 8.0);
        
        // Test scale factor
        let scaled = MenuTypography::for_scale_factor(1.5);
        assert!(scaled.menu_bar_text_size > typography.menu_bar_text_size);
    }

    #[test]
    fn test_separator_style() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();
        let style = menu_separator_style(&palette);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should be transparent border
        assert_eq!(style.border.color, Color::TRANSPARENT);
    }

    #[test]
    fn test_underline_style() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();
        
        // Test active underline
        let active_style = menu_underline_style(&palette, true);
        if let Some(Background::Color(color)) = active_style.background {
            assert_eq!(color, palette.tech_blue);
        }
        
        // Test inactive underline
        let inactive_style = menu_underline_style(&palette, false);
        if let Some(Background::Color(color)) = inactive_style.background {
            assert_eq!(color, Color::TRANSPARENT);
        }
    }
}
