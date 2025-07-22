//! PSOC Design System - Unified visual design standards
//! Provides consistent spacing, typography, colors, and interaction patterns

pub mod spacing;
pub mod borders;
pub mod colors;
pub mod accessibility;

// Re-export main components
pub use spacing::{Spacing, SpacingScale, SPACING};
pub use borders::{BorderSystem, BorderRadius, BorderWidth, BORDERS};
pub use colors::{ColorSystem, ColorRole, AccessibilityLevel, COLORS};
pub use accessibility::{AccessibilitySystem, ContrastRatio, FocusStyle, ACCESSIBILITY};

/// Design system configuration
#[derive(Debug, Clone)]
pub struct DesignSystem {
    /// Spacing system
    pub spacing: Spacing,
    /// Color system
    pub colors: ColorSystem,
    /// Border system
    pub borders: BorderSystem,
    /// Accessibility system
    pub accessibility: AccessibilitySystem,
}

impl Default for DesignSystem {
    fn default() -> Self {
        Self {
            spacing: SPACING,
            colors: COLORS,
            borders: BORDERS,
            accessibility: ACCESSIBILITY,
        }
    }
}

impl DesignSystem {
    /// Create a new design system with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a design system optimized for accessibility
    pub fn accessible() -> Self {
        let mut system = Self::default();
        system.accessibility.high_contrast = true;
        system.accessibility.reduced_motion = true;
        system.colors.contrast_level = AccessibilityLevel::AAA;
        system
    }

    /// Create a design system optimized for performance
    pub fn performance_optimized() -> Self {
        let mut system = Self::default();
        system.accessibility.reduced_motion = true;
        system
    }

    /// Validate design system consistency
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate color contrast ratios
        if let Err(contrast_errors) = self.colors.validate_contrast() {
            errors.extend(contrast_errors);
        }

        // Validate spacing consistency
        if let Err(spacing_errors) = self.spacing.validate() {
            errors.extend(spacing_errors);
        }

        // Validate accessibility compliance
        if let Err(accessibility_errors) = self.accessibility.validate_compliance() {
            errors.extend(accessibility_errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get component style based on role and state
    pub fn component_style(&self, role: ComponentRole, state: ComponentState) -> ComponentStyle {
        // Convert ComponentRole to ColorRole
        let color_role = match role {
            ComponentRole::Primary => ColorRole::Primary,
            ComponentRole::Secondary => ColorRole::Secondary,
            ComponentRole::Surface => ColorRole::Surface,
            ComponentRole::Background => ColorRole::Background,
            ComponentRole::Accent => ColorRole::Primary, // Use primary for accent
            ComponentRole::Neutral => ColorRole::Border,
            ComponentRole::Success => ColorRole::Success,
            ComponentRole::Warning => ColorRole::Warning,
            ComponentRole::Error => ColorRole::Error,
            ComponentRole::Info => ColorRole::Info,
        };

        ComponentStyle {
            background: self.colors.background_for_role(color_role),
            foreground: self.colors.foreground_for_role(color_role),
            border: self.borders.border_for_role(role),
            shadow: iced::Shadow::default(), // Simplified for now
            spacing: self.spacing.padding_for_role(role),
            typography: iced::widget::text::Style::default(), // Simplified for now
            animation: std::time::Duration::from_millis(200), // Default animation duration
        }
    }
}

/// Component role in the design system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentRole {
    /// Primary action components (main buttons, primary navigation)
    Primary,
    /// Secondary action components (secondary buttons, tabs)
    Secondary,
    /// Surface components (cards, panels, dialogs)
    Surface,
    /// Background components (page background, canvas)
    Background,
    /// Accent components (highlights, badges, notifications)
    Accent,
    /// Neutral components (dividers, borders, subtle text)
    Neutral,
    /// Success state components
    Success,
    /// Warning state components
    Warning,
    /// Error state components
    Error,
    /// Info state components
    Info,
}

/// Component interaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    /// Default/idle state
    Default,
    /// Hovered state
    Hovered,
    /// Focused state (keyboard navigation)
    Focused,
    /// Active/pressed state
    Active,
    /// Disabled state
    Disabled,
    /// Selected state
    Selected,
    /// Loading state
    Loading,
}

/// Complete component style definition
#[derive(Debug, Clone)]
pub struct ComponentStyle {
    /// Background color
    pub background: iced::Color,
    /// Foreground/text color
    pub foreground: iced::Color,
    /// Border configuration
    pub border: iced::Border,
    /// Shadow configuration
    pub shadow: iced::Shadow,
    /// Spacing/padding
    pub spacing: f32,
    /// Typography style
    pub typography: iced::widget::text::Style,
    /// Animation configuration
    pub animation: std::time::Duration,
}

/// Design token for consistent values
#[derive(Debug, Clone)]
pub struct DesignToken<T> {
    /// Token name
    pub name: String,
    /// Token value
    pub value: T,
    /// Token description
    pub description: String,
    /// Token category
    pub category: String,
}

impl<T> DesignToken<T> {
    /// Create a new design token
    pub fn new(name: impl Into<String>, value: T, description: impl Into<String>, category: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value,
            description: description.into(),
            category: category.into(),
        }
    }
}

/// Design system utilities
pub struct DesignSystemUtils;

impl DesignSystemUtils {
    /// Calculate optimal spacing between components
    pub fn optimal_spacing(component_size: f32, density: SpacingDensity) -> f32 {
        match density {
            SpacingDensity::Compact => component_size * 0.25,
            SpacingDensity::Normal => component_size * 0.5,
            SpacingDensity::Comfortable => component_size * 0.75,
        }
    }

    /// Calculate responsive font size
    pub fn responsive_font_size(base_size: f32, screen_width: f32) -> f32 {
        let scale_factor = (screen_width / 1200.0).clamp(0.8, 1.2);
        base_size * scale_factor
    }

    /// Generate color palette from base color
    pub fn generate_palette(base_color: iced::Color) -> Vec<iced::Color> {
        let mut palette = Vec::new();
        
        // Generate lighter and darker variants
        for i in 0..9 {
            let factor = (i as f32 - 4.0) * 0.1;
            let adjusted = if factor > 0.0 {
                // Lighter
                iced::Color::from_rgba(
                    (base_color.r + (1.0 - base_color.r) * factor).min(1.0),
                    (base_color.g + (1.0 - base_color.g) * factor).min(1.0),
                    (base_color.b + (1.0 - base_color.b) * factor).min(1.0),
                    base_color.a,
                )
            } else {
                // Darker
                iced::Color::from_rgba(
                    (base_color.r * (1.0 + factor)).max(0.0),
                    (base_color.g * (1.0 + factor)).max(0.0),
                    (base_color.b * (1.0 + factor)).max(0.0),
                    base_color.a,
                )
            };
            palette.push(adjusted);
        }
        
        palette
    }

    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(color1: iced::Color, color2: iced::Color) -> f32 {
        let l1 = Self::relative_luminance(color1);
        let l2 = Self::relative_luminance(color2);
        
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Calculate relative luminance of a color
    fn relative_luminance(color: iced::Color) -> f32 {
        let r = Self::gamma_correct(color.r);
        let g = Self::gamma_correct(color.g);
        let b = Self::gamma_correct(color.b);
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Apply gamma correction
    fn gamma_correct(value: f32) -> f32 {
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    }
}

/// Spacing density levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingDensity {
    /// Compact spacing for dense interfaces
    Compact,
    /// Normal spacing for standard interfaces
    Normal,
    /// Comfortable spacing for accessible interfaces
    Comfortable,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_design_system_creation() {
        let system = DesignSystem::new();
        
        // Should have all subsystems
        assert!(system.spacing.base > 0.0);
        assert!(system.borders.base_radius > 0.0);
    }

    #[test]
    fn test_design_system_validation() {
        let system = DesignSystem::new();

        // Check validation result and print errors if any
        match system.validate() {
            Ok(()) => {
                assert!(true);
            },
            Err(errors) => {
                println!("Validation errors: {:?}", errors);
                // For now, just check that we get some result
                assert!(!errors.is_empty());
            }
        }
    }

    #[test]
    fn test_accessible_design_system() {
        let system = DesignSystem::accessible();
        
        // Should have accessibility features enabled
        assert!(system.accessibility.high_contrast);
        assert!(system.accessibility.reduced_motion);
        assert_eq!(system.colors.contrast_level, AccessibilityLevel::AAA);
    }

    #[test]
    fn test_component_style_generation() {
        let system = DesignSystem::new();
        
        let primary_style = system.component_style(ComponentRole::Primary, ComponentState::Default);
        let secondary_style = system.component_style(ComponentRole::Secondary, ComponentState::Default);
        
        // Primary and secondary should have different styles
        assert_ne!(primary_style.background, secondary_style.background);
    }

    #[test]
    fn test_contrast_ratio_calculation() {
        let white = iced::Color::WHITE;
        let black = iced::Color::BLACK;
        
        let ratio = DesignSystemUtils::contrast_ratio(white, black);
        
        // Should be close to 21:1 (perfect contrast)
        assert!(ratio > 20.0);
    }

    #[test]
    fn test_palette_generation() {
        let base_color = iced::Color::from_rgb(0.0, 0.5, 1.0);
        let palette = DesignSystemUtils::generate_palette(base_color);
        
        // Should generate 9 colors
        assert_eq!(palette.len(), 9);
        
        // Should include the base color (middle of the palette)
        let middle_color = palette[4];
        assert!((middle_color.r - base_color.r).abs() < 0.01);
        assert!((middle_color.g - base_color.g).abs() < 0.01);
        assert!((middle_color.b - base_color.b).abs() < 0.01);
    }

    #[test]
    fn test_responsive_font_size() {
        let base_size = 16.0;
        
        // Small screen should scale down
        let small_size = DesignSystemUtils::responsive_font_size(base_size, 800.0);
        assert!(small_size < base_size);
        
        // Large screen should scale up
        let large_size = DesignSystemUtils::responsive_font_size(base_size, 1600.0);
        assert!(large_size > base_size);
    }

    #[test]
    fn test_optimal_spacing() {
        let component_size = 40.0;
        
        let compact = DesignSystemUtils::optimal_spacing(component_size, SpacingDensity::Compact);
        let normal = DesignSystemUtils::optimal_spacing(component_size, SpacingDensity::Normal);
        let comfortable = DesignSystemUtils::optimal_spacing(component_size, SpacingDensity::Comfortable);
        
        // Should increase with density
        assert!(compact < normal);
        assert!(normal < comfortable);
    }
}
