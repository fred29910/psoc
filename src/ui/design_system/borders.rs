//! Border system for consistent component styling
//! Provides unified border radius, width, and styling

use iced::{Border, Color};

/// Border system configuration
#[derive(Debug, Clone)]
pub struct BorderSystem {
    /// Base border radius (8px)
    pub base_radius: f32,
    /// Base border width (1px)
    pub base_width: f32,
    /// Border radius scale
    pub radius_scale: BorderRadius,
    /// Border width scale
    pub width_scale: BorderWidth,
}

/// Border radius scale with semantic names
#[derive(Debug, Clone)]
pub struct BorderRadius {
    /// No radius (0px)
    pub none: f32,
    /// Small radius (4px)
    pub sm: f32,
    /// Medium radius (8px)
    pub md: f32,
    /// Large radius (12px)
    pub lg: f32,
    /// Extra large radius (16px)
    pub xl: f32,
    /// Full radius (9999px - creates pill shape)
    pub full: f32,
}

/// Border width scale with semantic names
#[derive(Debug, Clone)]
pub struct BorderWidth {
    /// No border (0px)
    pub none: f32,
    /// Thin border (1px)
    pub thin: f32,
    /// Medium border (2px)
    pub medium: f32,
    /// Thick border (3px)
    pub thick: f32,
    /// Extra thick border (4px)
    pub extra_thick: f32,
}

impl Default for BorderSystem {
    fn default() -> Self {
        Self {
            base_radius: 8.0,
            base_width: 1.0,
            radius_scale: BorderRadius::default(),
            width_scale: BorderWidth::default(),
        }
    }
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        }
    }
}

impl Default for BorderWidth {
    fn default() -> Self {
        Self {
            none: 0.0,
            thin: 1.0,
            medium: 2.0,
            thick: 3.0,
            extra_thick: 4.0,
        }
    }
}

impl BorderSystem {
    /// Create a new border system with custom base values
    pub fn new(base_radius: f32, base_width: f32) -> Self {
        Self {
            base_radius,
            base_width,
            radius_scale: BorderRadius {
                none: 0.0,
                sm: base_radius * 0.5,
                md: base_radius,
                lg: base_radius * 1.5,
                xl: base_radius * 2.0,
                full: 9999.0,
            },
            width_scale: BorderWidth {
                none: 0.0,
                thin: base_width,
                medium: base_width * 2.0,
                thick: base_width * 3.0,
                extra_thick: base_width * 4.0,
            },
        }
    }

    /// Get border for component role
    pub fn border_for_role(&self, role: super::ComponentRole) -> Border {
        use super::ComponentRole;
        
        let (width, radius, color) = match role {
            ComponentRole::Primary => (
                self.width_scale.medium,
                self.radius_scale.md,
                Color::from_rgba(0.0, 0.75, 1.0, 0.8), // tech-blue
            ),
            ComponentRole::Secondary => (
                self.width_scale.thin,
                self.radius_scale.md,
                Color::from_rgba(0.5, 0.5, 0.5, 0.5), // neutral gray
            ),
            ComponentRole::Surface => (
                self.width_scale.thin,
                self.radius_scale.lg,
                Color::from_rgba(0.3, 0.3, 0.3, 0.3), // subtle border
            ),
            ComponentRole::Background => (
                self.width_scale.none,
                self.radius_scale.none,
                Color::TRANSPARENT,
            ),
            ComponentRole::Accent => (
                self.width_scale.medium,
                self.radius_scale.sm,
                Color::from_rgba(0.0, 0.75, 1.0, 1.0), // bright tech-blue
            ),
            ComponentRole::Neutral => (
                self.width_scale.thin,
                self.radius_scale.sm,
                Color::from_rgba(0.6, 0.6, 0.6, 0.4), // light gray
            ),
            ComponentRole::Success => (
                self.width_scale.thin,
                self.radius_scale.md,
                Color::from_rgba(0.0, 0.8, 0.0, 0.6), // green
            ),
            ComponentRole::Warning => (
                self.width_scale.thin,
                self.radius_scale.md,
                Color::from_rgba(1.0, 0.6, 0.0, 0.6), // orange
            ),
            ComponentRole::Error => (
                self.width_scale.medium,
                self.radius_scale.md,
                Color::from_rgba(1.0, 0.2, 0.2, 0.8), // red
            ),
            ComponentRole::Info => (
                self.width_scale.thin,
                self.radius_scale.md,
                Color::from_rgba(0.2, 0.6, 1.0, 0.6), // blue
            ),
        };

        Border {
            color,
            width,
            radius: radius.into(),
        }
    }

    /// Get border for component state
    pub fn border_for_state(&self, state: super::ComponentState, base_border: Border) -> Border {
        use super::ComponentState;
        
        match state {
            ComponentState::Default => base_border,
            ComponentState::Hovered => Border {
                color: self.brighten_color(base_border.color, 0.2),
                width: base_border.width,
                radius: base_border.radius,
            },
            ComponentState::Focused => Border {
                color: Color::from_rgba(0.0, 0.75, 1.0, 1.0), // tech-blue focus
                width: self.width_scale.medium,
                radius: base_border.radius,
            },
            ComponentState::Active => Border {
                color: self.darken_color(base_border.color, 0.2),
                width: base_border.width,
                radius: base_border.radius,
            },
            ComponentState::Disabled => Border {
                color: Color::from_rgba(0.5, 0.5, 0.5, 0.3),
                width: base_border.width,
                radius: base_border.radius,
            },
            ComponentState::Selected => Border {
                color: Color::from_rgba(0.0, 0.75, 1.0, 0.8),
                width: self.width_scale.medium,
                radius: base_border.radius,
            },
            ComponentState::Loading => Border {
                color: Color::from_rgba(0.0, 0.75, 1.0, 0.5),
                width: base_border.width,
                radius: base_border.radius,
            },
        }
    }

    /// Create a custom border
    pub fn custom_border(&self, width: f32, radius: f32, color: Color) -> Border {
        Border {
            color,
            width,
            radius: radius.into(),
        }
    }

    /// Get border for specific UI patterns
    pub fn pattern_border(&self, pattern: BorderPattern) -> Border {
        match pattern {
            BorderPattern::Button => Border {
                color: Color::from_rgba(0.0, 0.75, 1.0, 0.6),
                width: self.width_scale.thin,
                radius: self.radius_scale.md.into(),
            },
            BorderPattern::Input => Border {
                color: Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                width: self.width_scale.thin,
                radius: self.radius_scale.sm.into(),
            },
            BorderPattern::Card => Border {
                color: Color::from_rgba(0.3, 0.3, 0.3, 0.3),
                width: self.width_scale.thin,
                radius: self.radius_scale.lg.into(),
            },
            BorderPattern::Dialog => Border {
                color: Color::from_rgba(0.4, 0.4, 0.4, 0.4),
                width: self.width_scale.thin,
                radius: self.radius_scale.xl.into(),
            },
            BorderPattern::Panel => Border {
                color: Color::from_rgba(0.3, 0.3, 0.3, 0.2),
                width: self.width_scale.thin,
                radius: self.radius_scale.lg.into(),
            },
            BorderPattern::Toolbar => Border {
                color: Color::from_rgba(0.4, 0.4, 0.4, 0.3),
                width: self.width_scale.thin,
                radius: self.radius_scale.md.into(),
            },
            BorderPattern::Menu => Border {
                color: Color::from_rgba(0.3, 0.3, 0.3, 0.4),
                width: self.width_scale.thin,
                radius: self.radius_scale.md.into(),
            },
            BorderPattern::Tooltip => Border {
                color: Color::from_rgba(0.2, 0.2, 0.2, 0.6),
                width: self.width_scale.thin,
                radius: self.radius_scale.sm.into(),
            },
        }
    }

    /// Brighten a color by a factor
    fn brighten_color(&self, color: Color, factor: f32) -> Color {
        Color::from_rgba(
            (color.r + (1.0 - color.r) * factor).min(1.0),
            (color.g + (1.0 - color.g) * factor).min(1.0),
            (color.b + (1.0 - color.b) * factor).min(1.0),
            color.a,
        )
    }

    /// Darken a color by a factor
    fn darken_color(&self, color: Color, factor: f32) -> Color {
        Color::from_rgba(
            (color.r * (1.0 - factor)).max(0.0),
            (color.g * (1.0 - factor)).max(0.0),
            (color.b * (1.0 - factor)).max(0.0),
            color.a,
        )
    }

    /// Validate border system consistency
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check that base values are positive
        if self.base_radius < 0.0 {
            errors.push("Base radius must be non-negative".to_string());
        }

        if self.base_width < 0.0 {
            errors.push("Base width must be non-negative".to_string());
        }

        // Check that radius scale values are in ascending order (except full)
        let radius_values = [
            self.radius_scale.none,
            self.radius_scale.sm,
            self.radius_scale.md,
            self.radius_scale.lg,
            self.radius_scale.xl,
        ];

        for i in 1..radius_values.len() {
            if radius_values[i] <= radius_values[i - 1] {
                errors.push(format!("Border radius scale values must be in ascending order at index {}", i));
            }
        }

        // Check that width scale values are in ascending order
        let width_values = [
            self.width_scale.none,
            self.width_scale.thin,
            self.width_scale.medium,
            self.width_scale.thick,
            self.width_scale.extra_thick,
        ];

        for i in 1..width_values.len() {
            if width_values[i] <= width_values[i - 1] {
                errors.push(format!("Border width scale values must be in ascending order at index {}", i));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Common border patterns in UI design
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderPattern {
    /// Standard button border
    Button,
    /// Input field border
    Input,
    /// Card container border
    Card,
    /// Dialog border
    Dialog,
    /// Panel border
    Panel,
    /// Toolbar border
    Toolbar,
    /// Menu border
    Menu,
    /// Tooltip border
    Tooltip,
}

/// Global border system instance
pub const BORDERS: BorderSystem = BorderSystem {
    base_radius: 8.0,
    base_width: 1.0,
    radius_scale: BorderRadius {
        none: 0.0,
        sm: 4.0,
        md: 8.0,
        lg: 12.0,
        xl: 16.0,
        full: 9999.0,
    },
    width_scale: BorderWidth {
        none: 0.0,
        thin: 1.0,
        medium: 2.0,
        thick: 3.0,
        extra_thick: 4.0,
    },
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ComponentRole, ComponentState};

    #[test]
    fn test_border_system_creation() {
        let borders = BorderSystem::new(10.0, 2.0);
        
        assert_eq!(borders.base_radius, 10.0);
        assert_eq!(borders.base_width, 2.0);
        assert_eq!(borders.radius_scale.md, 10.0);
        assert_eq!(borders.width_scale.thin, 2.0);
    }

    #[test]
    fn test_border_validation() {
        let valid_borders = BorderSystem::default();
        assert!(valid_borders.validate().is_ok());

        let invalid_borders = BorderSystem {
            base_radius: -1.0,
            base_width: -1.0,
            radius_scale: BorderRadius::default(),
            width_scale: BorderWidth::default(),
        };
        assert!(invalid_borders.validate().is_err());
    }

    #[test]
    fn test_role_based_borders() {
        let borders = BorderSystem::default();
        
        let primary_border = borders.border_for_role(ComponentRole::Primary);
        let secondary_border = borders.border_for_role(ComponentRole::Secondary);
        
        // Primary should have thicker border
        assert!(primary_border.width > secondary_border.width);
    }

    #[test]
    fn test_state_based_borders() {
        let borders = BorderSystem::default();
        let base_border = borders.border_for_role(ComponentRole::Primary);

        let focused_border = borders.border_for_state(ComponentState::Focused, base_border);
        let disabled_border = borders.border_for_state(ComponentState::Disabled, base_border);
        
        // Focused should have tech-blue color
        assert_eq!(focused_border.color, Color::from_rgba(0.0, 0.75, 1.0, 1.0));
        
        // Disabled should have reduced opacity
        assert!(disabled_border.color.a < base_border.color.a);
    }

    #[test]
    fn test_pattern_borders() {
        let borders = BorderSystem::default();
        
        let button_border = borders.pattern_border(BorderPattern::Button);
        let card_border = borders.pattern_border(BorderPattern::Card);
        let dialog_border = borders.pattern_border(BorderPattern::Dialog);
        
        // Dialog should have larger radius than card, card larger than button
        assert!(dialog_border.radius.top_left > card_border.radius.top_left);
        assert!(card_border.radius.top_left > button_border.radius.top_left);
    }

    #[test]
    fn test_color_manipulation() {
        let borders = BorderSystem::default();
        let base_color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        
        let brightened = borders.brighten_color(base_color, 0.2);
        let darkened = borders.darken_color(base_color, 0.2);
        
        // Brightened should be lighter
        assert!(brightened.r > base_color.r);
        assert!(brightened.g > base_color.g);
        assert!(brightened.b > base_color.b);
        
        // Darkened should be darker
        assert!(darkened.r < base_color.r);
        assert!(darkened.g < base_color.g);
        assert!(darkened.b < base_color.b);
    }

    #[test]
    fn test_custom_border() {
        let borders = BorderSystem::default();
        let red_color = Color::from_rgba(1.0, 0.0, 0.0, 1.0);
        let custom = borders.custom_border(3.0, 15.0, red_color);

        assert_eq!(custom.width, 3.0);
        assert_eq!(custom.radius.top_left, 15.0);
        assert_eq!(custom.color, red_color);
    }
}
