//! Modern theme system for PSOC Image Editor

use iced::{Color, Theme};

/// PSOC custom theme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
