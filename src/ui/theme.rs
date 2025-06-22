//! Modern theme system for PSOC Image Editor

use iced::{Color, Theme, Border, Shadow, Vector};

/// PSOC custom theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PsocTheme {
    /// Dark theme (default)
    Dark,
    /// Light theme
    Light,
    /// High contrast theme
    HighContrast,
}

impl Default for PsocTheme {
    fn default() -> Self {
        Self::Dark
    }
}

impl std::fmt::Display for PsocTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PsocTheme::Dark => write!(f, "Dark"),
            PsocTheme::Light => write!(f, "Light"),
            PsocTheme::HighContrast => write!(f, "High Contrast"),
        }
    }
}

/// Color palette for the application
#[derive(Debug, Clone)]
pub struct ColorPalette {
    /// Primary background color
    pub background: Color,
    /// Secondary background color (panels, cards)
    pub surface: Color,
    /// Primary text color
    pub text: Color,
    /// Secondary text color
    pub text_secondary: Color,
    /// Primary accent color
    pub primary: Color,
    /// Secondary accent color
    pub secondary: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error color
    pub error: Color,
    /// Border color
    pub border: Color,
    /// Shadow color
    pub shadow: Color,
    /// Tech blue accent color (#00BFFF)
    pub tech_blue: Color,
    /// Dark background color (#222222)
    pub dark_bg: Color,
    /// Dark panel color (#2a2a2e)
    pub dark_panel: Color,
    /// Menu hover color
    pub menu_hover: Color,
    /// Menu active color
    pub menu_active: Color,
    /// Glass background for frosted glass effect
    pub glass_bg: Color,
}

impl ColorPalette {
    /// Get menu background color
    pub fn menu_background(&self) -> Color {
        self.glass_bg
    }

    /// Get menu hover color
    pub fn menu_hover_color(&self) -> Color {
        self.menu_hover
    }

    /// Get menu active color
    pub fn menu_active_color(&self) -> Color {
        self.menu_active
    }

    /// Get menu separator color
    pub fn menu_separator(&self) -> Color {
        Color::from_rgba(self.border.r, self.border.g, self.border.b, 0.3)
    }

    /// Get tech blue with custom alpha
    pub fn tech_blue_alpha(&self, alpha: f32) -> Color {
        Color::from_rgba(self.tech_blue.r, self.tech_blue.g, self.tech_blue.b, alpha)
    }

    /// Get surface color with custom alpha
    pub fn surface_alpha(&self, alpha: f32) -> Color {
        Color::from_rgba(self.surface.r, self.surface.g, self.surface.b, alpha)
    }

    /// Get shadow color for the theme
    pub fn shadow_color(&self, intensity: f32) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, intensity)
    }

    /// Get highlight color for borders and accents
    pub fn highlight_color(&self) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 0.1)
    }
}

impl PsocTheme {
    /// Get the color palette for this theme
    pub fn palette(self) -> ColorPalette {
        match self {
            PsocTheme::Dark => ColorPalette {
                background: Color::from_rgb(0.12, 0.12, 0.12), // #1e1e1e
                surface: Color::from_rgb(0.16, 0.16, 0.16),    // #282828
                text: Color::from_rgb(0.9, 0.9, 0.9),          // #e6e6e6
                text_secondary: Color::from_rgb(0.7, 0.7, 0.7), // #b3b3b3
                primary: Color::from_rgb(0.0, 0.48, 1.0),      // #007acc
                secondary: Color::from_rgb(0.4, 0.4, 0.4),     // #666666
                success: Color::from_rgb(0.0, 0.8, 0.4),       // #00cc66
                warning: Color::from_rgb(1.0, 0.6, 0.0),       // #ff9900
                error: Color::from_rgb(0.9, 0.2, 0.2),         // #e63333
                border: Color::from_rgb(0.3, 0.3, 0.3),        // #4d4d4d
                shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3),  // rgba(0,0,0,0.3)
                tech_blue: Color::from_rgb(0.0, 0.75, 1.0),    // #00BFFF
                dark_bg: Color::from_rgb(0.13, 0.13, 0.13),    // #222222
                dark_panel: Color::from_rgb(0.16, 0.16, 0.18), // #2a2a2e
                menu_hover: Color::from_rgba(0.0, 0.75, 1.0, 0.1), // tech_blue with alpha
                menu_active: Color::from_rgba(0.0, 0.75, 1.0, 0.2), // tech_blue with alpha
                glass_bg: Color::from_rgba(0.16, 0.16, 0.16, 0.8), // surface with alpha
            },
            PsocTheme::Light => ColorPalette {
                background: Color::from_rgb(0.98, 0.98, 0.98), // #fafafa
                surface: Color::WHITE,                         // #ffffff
                text: Color::from_rgb(0.1, 0.1, 0.1),          // #1a1a1a
                text_secondary: Color::from_rgb(0.4, 0.4, 0.4), // #666666
                primary: Color::from_rgb(0.0, 0.48, 1.0),      // #007acc
                secondary: Color::from_rgb(0.6, 0.6, 0.6),     // #999999
                success: Color::from_rgb(0.0, 0.6, 0.3),       // #009933
                warning: Color::from_rgb(0.9, 0.5, 0.0),       // #e68000
                error: Color::from_rgb(0.8, 0.1, 0.1),         // #cc1a1a
                border: Color::from_rgb(0.8, 0.8, 0.8),        // #cccccc
                shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.1),  // rgba(0,0,0,0.1)
                tech_blue: Color::from_rgb(0.0, 0.75, 1.0),    // #00BFFF
                dark_bg: Color::from_rgb(0.95, 0.95, 0.95),    // Light equivalent
                dark_panel: Color::from_rgb(0.92, 0.92, 0.92), // Light equivalent
                menu_hover: Color::from_rgba(0.0, 0.75, 1.0, 0.1), // tech_blue with alpha
                menu_active: Color::from_rgba(0.0, 0.75, 1.0, 0.2), // tech_blue with alpha
                glass_bg: Color::from_rgba(1.0, 1.0, 1.0, 0.8), // white with alpha
            },
            PsocTheme::HighContrast => ColorPalette {
                background: Color::BLACK,                       // #000000
                surface: Color::from_rgb(0.1, 0.1, 0.1),        // #1a1a1a
                text: Color::WHITE,                             // #ffffff
                text_secondary: Color::from_rgb(0.8, 0.8, 0.8), // #cccccc
                primary: Color::from_rgb(0.0, 0.8, 1.0),        // #00ccff
                secondary: Color::from_rgb(0.5, 0.5, 0.5),      // #808080
                success: Color::from_rgb(0.0, 1.0, 0.0),        // #00ff00
                warning: Color::from_rgb(1.0, 1.0, 0.0),        // #ffff00
                error: Color::from_rgb(1.0, 0.0, 0.0),          // #ff0000
                border: Color::WHITE,                           // #ffffff
                shadow: Color::from_rgba(1.0, 1.0, 1.0, 0.2),   // rgba(255,255,255,0.2)
                tech_blue: Color::from_rgb(0.0, 1.0, 1.0),      // Bright cyan for high contrast
                dark_bg: Color::BLACK,                          // #000000
                dark_panel: Color::from_rgb(0.1, 0.1, 0.1),     // #1a1a1a
                menu_hover: Color::from_rgba(0.0, 1.0, 1.0, 0.2), // tech_blue with alpha
                menu_active: Color::from_rgba(0.0, 1.0, 1.0, 0.3), // tech_blue with alpha
                glass_bg: Color::from_rgba(0.1, 0.1, 0.1, 0.9), // surface with alpha
            },
        }
    }

    /// Convert to iced Theme
    pub fn to_iced_theme(self) -> Theme {
        match self {
            PsocTheme::Dark => Theme::Dark,
            PsocTheme::Light => Theme::Light,
            PsocTheme::HighContrast => Theme::Dark, // Use dark as base for high contrast
        }
    }

    /// Get enhanced container style with visual effects
    pub fn enhanced_container_style(self, style: VisualStyle) -> iced::widget::container::Style {
        let palette = self.palette();

        match style {
            VisualStyle::None => iced::widget::container::Style::default(),
            VisualStyle::FrostedGlass => iced::widget::container::Style {
                text_color: Some(palette.text),
                background: Some(palette.glass_bg.into()),
                border: Border {
                    color: Color::from_rgba(palette.border.r, palette.border.g, palette.border.b, 0.3),
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    offset: Vector::new(0.0, 8.0),
                    blur_radius: 24.0,
                },
            },
            VisualStyle::TechAccent => iced::widget::container::Style {
                text_color: Some(palette.text),
                background: Some(palette.surface.into()),
                border: Border {
                    color: palette.tech_blue,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(palette.tech_blue.r, palette.tech_blue.g, palette.tech_blue.b, 0.3),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            },
            VisualStyle::Hover => iced::widget::container::Style {
                text_color: Some(palette.text),
                background: Some(palette.menu_hover.into()),
                border: Border {
                    color: Color::from_rgba(palette.tech_blue.r, palette.tech_blue.g, palette.tech_blue.b, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
            },
            VisualStyle::Active => iced::widget::container::Style {
                text_color: Some(palette.text),
                background: Some(palette.menu_active.into()),
                border: Border {
                    color: Color::from_rgba(palette.tech_blue.r, palette.tech_blue.g, palette.tech_blue.b, 0.4),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: Shadow::default(),
            },
            VisualStyle::Floating => iced::widget::container::Style {
                text_color: Some(palette.text),
                background: Some(palette.surface.into()),
                border: Border {
                    color: palette.border,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                    offset: Vector::new(0.0, 8.0),
                    blur_radius: 16.0,
                },
            },
        }
    }
}

/// Custom button styles (simplified)
#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Ghost,
    Icon,
    Tool,
}

/// Custom container styles (simplified)
#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerStyle {
    #[default]
    Default,
    Panel,
    Card,
    Toolbar,
    StatusBar,
    MenuBar,
    MenuDropdown,
}

/// Menu-specific styles
#[derive(Debug, Clone, Copy, Default)]
pub enum MenuStyle {
    #[default]
    TopBar,
    Dropdown,
    MenuItem,
    Separator,
    Submenu,
}

/// Visual effect styles for modern UI
#[derive(Debug, Clone, Copy, Default)]
pub enum VisualStyle {
    #[default]
    None,
    FrostedGlass,
    TechAccent,
    Hover,
    Active,
    Floating,
}

/// Spacing constants for consistent layout
pub mod spacing {
    /// Extra small spacing (2px)
    pub const XS: f32 = 2.0;
    /// Small spacing (4px)
    pub const SM: f32 = 4.0;
    /// Medium spacing (8px)
    pub const MD: f32 = 8.0;
    /// Large spacing (16px)
    pub const LG: f32 = 16.0;
    /// Extra large spacing (24px)
    pub const XL: f32 = 24.0;
    /// Extra extra large spacing (32px)
    pub const XXL: f32 = 32.0;
}

/// Font sizes for consistent typography
pub mod typography {
    /// Small text size
    pub const SMALL: f32 = 12.0;
    /// Normal text size
    pub const NORMAL: f32 = 14.0;
    /// Medium text size
    pub const MEDIUM: f32 = 16.0;
    /// Large text size
    pub const LARGE: f32 = 18.0;
    /// Extra large text size
    pub const XLARGE: f32 = 24.0;
    /// Heading text size
    pub const HEADING: f32 = 32.0;
}
