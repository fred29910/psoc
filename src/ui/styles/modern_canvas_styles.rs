//! Modern canvas styling system with enhanced visual effects
//! Provides professional canvas area, placeholder, and control styles

use iced::{
    widget::{container, button},
    Background, Color, Border, Shadow, Vector,
};

use crate::ui::theme::{PsocTheme, ColorPalette};
use super::glass_effects::GlassEffect;

/// Modern canvas area style with enhanced visual effects
pub fn modern_canvas_area_style(theme: &PsocTheme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            palette.dark_bg.r,
            palette.dark_bg.g,
            palette.dark_bg.b,
            1.0
        ))),
        border: Border {
            color: Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.3
            ),
            width: 1.0,
            radius: 12.0.into(),
        },
        shadow: Shadow {
            color: palette.shadow_color(0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        text_color: Some(palette.text),
    }
}

/// Modern canvas background style with subtle pattern
pub fn modern_canvas_background_style(theme: &PsocTheme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            0.15, 0.15, 0.18, 1.0 // Slightly lighter than dark_bg for contrast
        ))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: Some(palette.text),
    }
}

/// Enhanced placeholder style for empty canvas
pub fn modern_placeholder_style(theme: &PsocTheme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            palette.glass_bg_medium.r,
            palette.glass_bg_medium.g,
            palette.glass_bg_medium.b,
            0.6
        ))),
        border: Border {
            color: Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.3
            ),
            width: 2.0,
            radius: 16.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.1
            ),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        },
        text_color: Some(palette.text),
    }
}

/// Modern zoom control style
pub fn modern_zoom_control_style(
    palette: &ColorPalette,
    status: button::Status,
) -> button::Style {
    let (background, text_color, shadow) = match status {
        button::Status::Hovered => (
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
        button::Status::Active => (
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.3
            ))),
            Color::WHITE,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
        _ => (
            Some(Background::Color(Color::from_rgba(
                palette.glass_bg_medium.r,
                palette.glass_bg_medium.g,
                palette.glass_bg_medium.b,
                0.8
            ))),
            palette.text,
            Shadow {
                color: palette.shadow_color(0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
        ),
    };

    button::Style {
        background,
        text_color,
        border: Border {
            color: Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.3
            ),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow,
    }
}

/// Canvas document border style
pub fn canvas_document_border_style(theme: &PsocTheme, is_active: bool) -> Color {
    let palette = theme.palette();
    
    if is_active {
        Color::from_rgba(
            palette.tech_blue.r,
            palette.tech_blue.g,
            palette.tech_blue.b,
            0.8
        )
    } else {
        Color::from_rgba(
            palette.border.r,
            palette.border.g,
            palette.border.b,
            0.5
        )
    }
}

/// Canvas grid style
pub fn canvas_grid_style(theme: &PsocTheme, is_major: bool) -> Color {
    let palette = theme.palette();
    
    if is_major {
        Color::from_rgba(
            palette.border.r,
            palette.border.g,
            palette.border.b,
            0.3
        )
    } else {
        Color::from_rgba(
            palette.border.r,
            palette.border.g,
            palette.border.b,
            0.1
        )
    }
}

/// Canvas ruler style
pub fn canvas_ruler_style(theme: &PsocTheme) -> container::Style {
    let palette = theme.palette();
    
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            palette.glass_bg_light.r,
            palette.glass_bg_light.g,
            palette.glass_bg_light.b,
            0.9
        ))),
        border: Border {
            color: Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.2
            ),
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: palette.shadow_color(0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        text_color: Some(palette.text),
    }
}

/// Canvas selection style
pub fn canvas_selection_style(theme: &PsocTheme) -> Color {
    let palette = theme.palette();
    
    Color::from_rgba(
        palette.tech_blue.r,
        palette.tech_blue.g,
        palette.tech_blue.b,
        0.6
    )
}

/// Canvas guide style
pub fn canvas_guide_style(theme: &PsocTheme) -> Color {
    let palette = theme.palette();
    
    Color::from_rgba(
        palette.tech_blue.r,
        palette.tech_blue.g,
        palette.tech_blue.b,
        0.8
    )
}

/// Canvas configuration for modern styling
pub struct CanvasStyleConfig {
    /// Canvas area padding
    pub area_padding: f32,
    /// Document border width
    pub document_border_width: f32,
    /// Grid line width
    pub grid_line_width: f32,
    /// Selection border width
    pub selection_border_width: f32,
    /// Guide line width
    pub guide_line_width: f32,
    /// Ruler size
    pub ruler_size: f32,
    /// Corner radius for canvas elements
    pub corner_radius: f32,
}

impl Default for CanvasStyleConfig {
    fn default() -> Self {
        Self {
            area_padding: 16.0,
            document_border_width: 2.0,
            grid_line_width: 1.0,
            selection_border_width: 2.0,
            guide_line_width: 1.5,
            ruler_size: 24.0,
            corner_radius: 8.0,
        }
    }
}

impl CanvasStyleConfig {
    /// Create configuration optimized for different zoom levels
    pub fn for_zoom_level(zoom: f32) -> Self {
        let base = Self::default();
        let scale_factor = (zoom / 1.0).max(0.5).min(2.0); // Clamp scale factor
        
        Self {
            area_padding: base.area_padding,
            document_border_width: base.document_border_width * scale_factor,
            grid_line_width: base.grid_line_width * scale_factor,
            selection_border_width: base.selection_border_width * scale_factor,
            guide_line_width: base.guide_line_width * scale_factor,
            ruler_size: base.ruler_size,
            corner_radius: base.corner_radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_area_style() {
        let theme = PsocTheme::Dark;
        let style = modern_canvas_area_style(&theme);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should have border with radius
        assert!(style.border.width > 0.0);
        assert!(style.border.radius.top_left > 0.0);
        
        // Should have shadow
        assert!(style.shadow.blur_radius > 0.0);
    }

    #[test]
    fn test_canvas_background_style() {
        let theme = PsocTheme::Dark;
        let style = modern_canvas_background_style(&theme);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should have subtle shadow
        assert!(style.shadow.blur_radius > 0.0);
    }

    #[test]
    fn test_placeholder_style() {
        let theme = PsocTheme::Dark;
        let style = modern_placeholder_style(&theme);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should have tech-blue border
        assert!(style.border.width > 0.0);
        
        // Should have elevated shadow
        assert!(style.shadow.blur_radius > 10.0);
    }

    #[test]
    fn test_zoom_control_styles() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();
        
        let states = [
            button::Status::Disabled,
            button::Status::Hovered,
            button::Status::Active,
        ];

        for status in states {
            let style = modern_zoom_control_style(&palette, status);
            
            // Should have valid background
            assert!(style.background.is_some());
            
            // Should have border
            assert!(style.border.width > 0.0);
            
            // Hovered state should have white text
            if status == button::Status::Hovered {
                assert_eq!(style.text_color, Color::WHITE);
            }
        }
    }

    #[test]
    fn test_canvas_colors() {
        let theme = PsocTheme::Dark;
        
        // Test document border colors
        let active_border = canvas_document_border_style(&theme, true);
        let inactive_border = canvas_document_border_style(&theme, false);
        assert_ne!(active_border, inactive_border);
        
        // Test grid colors
        let major_grid = canvas_grid_style(&theme, true);
        let minor_grid = canvas_grid_style(&theme, false);
        assert!(major_grid.a > minor_grid.a); // Major grid should be more opaque
        
        // Test selection and guide colors
        let selection_color = canvas_selection_style(&theme);
        let guide_color = canvas_guide_style(&theme);
        assert!(selection_color.a > 0.0);
        assert!(guide_color.a > 0.0);
    }

    #[test]
    fn test_canvas_style_config() {
        let config = CanvasStyleConfig::default();
        
        // Should have reasonable default values
        assert!(config.area_padding > 0.0);
        assert!(config.document_border_width > 0.0);
        assert!(config.grid_line_width > 0.0);
        
        // Test zoom scaling
        let scaled_config = CanvasStyleConfig::for_zoom_level(2.0);
        assert!(scaled_config.document_border_width > config.document_border_width);
        
        // Test zoom clamping
        let clamped_config = CanvasStyleConfig::for_zoom_level(10.0);
        assert!(clamped_config.document_border_width <= config.document_border_width * 2.0);
    }

    #[test]
    fn test_ruler_style() {
        let theme = PsocTheme::Dark;
        let style = canvas_ruler_style(&theme);
        
        // Should have background
        assert!(style.background.is_some());
        
        // Should have border
        assert!(style.border.width > 0.0);
        
        // Should have text color
        assert!(style.text_color.is_some());
    }
}
