//! Accessibility system for inclusive design
//! Provides WCAG compliance and accessibility features

use iced::{Color, Border, Shadow, Vector};
use std::time::Duration;

/// Accessibility system configuration
#[derive(Debug, Clone)]
pub struct AccessibilitySystem {
    /// High contrast mode enabled
    pub high_contrast: bool,
    /// Reduced motion enabled
    pub reduced_motion: bool,
    /// Focus style configuration
    pub focus_style: FocusStyle,
    /// Minimum contrast ratios
    pub contrast_ratios: ContrastRatio,
    /// Touch target sizes
    pub touch_targets: TouchTargets,
    /// Screen reader support
    pub screen_reader: ScreenReaderSupport,
}

/// Focus style configuration
#[derive(Debug, Clone)]
pub struct FocusStyle {
    /// Focus ring color
    pub color: Color,
    /// Focus ring width
    pub width: f32,
    /// Focus ring offset
    pub offset: f32,
    /// Focus ring style
    pub style: FocusRingStyle,
    /// Focus animation duration
    pub animation_duration: Duration,
}

/// Focus ring style options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusRingStyle {
    /// Solid outline
    Solid,
    /// Dashed outline
    Dashed,
    /// Dotted outline
    Dotted,
    /// Shadow-based focus
    Shadow,
}

/// Contrast ratio requirements
#[derive(Debug, Clone)]
pub struct ContrastRatio {
    /// Normal text contrast ratio
    pub normal_text: f32,
    /// Large text contrast ratio
    pub large_text: f32,
    /// UI component contrast ratio
    pub ui_components: f32,
    /// Graphical elements contrast ratio
    pub graphics: f32,
}

/// Touch target size requirements
#[derive(Debug, Clone)]
pub struct TouchTargets {
    /// Minimum touch target size
    pub minimum_size: f32,
    /// Recommended touch target size
    pub recommended_size: f32,
    /// Minimum spacing between targets
    pub minimum_spacing: f32,
}

/// Screen reader support configuration
#[derive(Debug, Clone)]
pub struct ScreenReaderSupport {
    /// Provide alternative text for images
    pub alt_text: bool,
    /// Provide labels for form controls
    pub form_labels: bool,
    /// Provide landmarks for navigation
    pub landmarks: bool,
    /// Provide live region updates
    pub live_regions: bool,
}

impl Default for AccessibilitySystem {
    fn default() -> Self {
        Self {
            high_contrast: false,
            reduced_motion: false,
            focus_style: FocusStyle::default(),
            contrast_ratios: ContrastRatio::default(),
            touch_targets: TouchTargets::default(),
            screen_reader: ScreenReaderSupport::default(),
        }
    }
}

impl Default for FocusStyle {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(0.0, 0.75, 1.0, 1.0), // Tech blue
            width: 2.0,
            offset: 2.0,
            style: FocusRingStyle::Solid,
            animation_duration: Duration::from_millis(150),
        }
    }
}

impl Default for ContrastRatio {
    fn default() -> Self {
        Self {
            normal_text: 4.5,    // WCAG AA
            large_text: 3.0,     // WCAG AA for large text
            ui_components: 3.0,  // WCAG AA for UI components
            graphics: 3.0,       // WCAG AA for graphics
        }
    }
}

impl Default for TouchTargets {
    fn default() -> Self {
        Self {
            minimum_size: 44.0,      // iOS/Android minimum
            recommended_size: 48.0,   // Material Design recommendation
            minimum_spacing: 8.0,     // Minimum spacing between targets
        }
    }
}

impl Default for ScreenReaderSupport {
    fn default() -> Self {
        Self {
            alt_text: true,
            form_labels: true,
            landmarks: true,
            live_regions: true,
        }
    }
}

impl AccessibilitySystem {
    /// Create accessibility system with WCAG AAA compliance
    pub fn wcag_aaa() -> Self {
        Self {
            high_contrast: true,
            reduced_motion: false,
            focus_style: FocusStyle {
                width: 3.0, // Thicker focus ring for AAA
                ..FocusStyle::default()
            },
            contrast_ratios: ContrastRatio {
                normal_text: 7.0,    // WCAG AAA
                large_text: 4.5,     // WCAG AAA for large text
                ui_components: 4.5,  // Enhanced for AAA
                graphics: 4.5,       // Enhanced for AAA
            },
            touch_targets: TouchTargets {
                minimum_size: 48.0,      // Larger for AAA
                recommended_size: 56.0,   // Even larger recommendation
                minimum_spacing: 12.0,    // More spacing for AAA
            },
            screen_reader: ScreenReaderSupport::default(),
        }
    }

    /// Get focus border for accessibility
    pub fn focus_border(&self) -> Border {
        let color = if self.high_contrast {
            Color::from_rgba(1.0, 1.0, 0.0, 1.0) // High contrast yellow
        } else {
            self.focus_style.color
        };

        Border {
            color,
            width: self.focus_style.width,
            radius: 4.0.into(),
        }
    }

    /// Get focus shadow for accessibility
    pub fn focus_shadow(&self) -> Shadow {
        let color = if self.high_contrast {
            Color::from_rgba(1.0, 1.0, 0.0, 0.8) // High contrast yellow
        } else {
            Color::from_rgba(
                self.focus_style.color.r,
                self.focus_style.color.g,
                self.focus_style.color.b,
                0.5,
            )
        };

        Shadow {
            color,
            offset: Vector::new(0.0, 0.0),
            blur_radius: self.focus_style.width * 2.0,
        }
    }

    /// Check if color meets contrast requirements
    pub fn meets_contrast_requirement(&self, foreground: Color, background: Color, text_size: TextSize) -> bool {
        let ratio = super::DesignSystemUtils::contrast_ratio(foreground, background);
        let required_ratio = match text_size {
            TextSize::Normal => self.contrast_ratios.normal_text,
            TextSize::Large => self.contrast_ratios.large_text,
        };

        ratio >= required_ratio
    }

    /// Check if touch target meets size requirements
    pub fn meets_touch_target_requirement(&self, size: f32) -> bool {
        size >= self.touch_targets.minimum_size
    }

    /// Get accessible animation duration
    pub fn animation_duration(&self, base_duration: Duration) -> Duration {
        if self.reduced_motion {
            Duration::from_millis(0) // No animation
        } else {
            base_duration
        }
    }

    /// Get high contrast color variant
    pub fn high_contrast_color(&self, base_color: Color, background: Color) -> Color {
        if !self.high_contrast {
            return base_color;
        }

        // In high contrast mode, use pure black or white for maximum contrast
        let bg_luminance = super::DesignSystemUtils::relative_luminance(background);
        
        if bg_luminance > 0.5 {
            Color::BLACK // Dark text on light background
        } else {
            Color::WHITE // Light text on dark background
        }
    }

    /// Get accessible spacing for touch targets
    pub fn touch_target_spacing(&self, base_spacing: f32) -> f32 {
        base_spacing.max(self.touch_targets.minimum_spacing)
    }

    /// Validate accessibility compliance
    pub fn validate_compliance(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check contrast ratios
        if self.contrast_ratios.normal_text < 3.0 {
            errors.push("Normal text contrast ratio is below WCAG minimum (3:1)".to_string());
        }

        if self.contrast_ratios.ui_components < 3.0 {
            errors.push("UI component contrast ratio is below WCAG minimum (3:1)".to_string());
        }

        // Check touch targets
        if self.touch_targets.minimum_size < 24.0 {
            errors.push("Touch target size is below accessibility minimum (24px)".to_string());
        }

        // Check focus style
        if self.focus_style.width < 1.0 {
            errors.push("Focus ring width is too thin for accessibility".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get accessible text style
    pub fn accessible_text_style(&self, base_color: Color, background: Color) -> iced::widget::text::Style {
        let color = if self.high_contrast {
            self.high_contrast_color(base_color, background)
        } else {
            base_color
        };

        iced::widget::text::Style {
            color: Some(color),
        }
    }

    /// Create accessible button style
    pub fn accessible_button_style(&self, base_style: iced::widget::button::Style, is_focused: bool) -> iced::widget::button::Style {
        if is_focused {
            iced::widget::button::Style {
                border: self.focus_border(),
                shadow: self.focus_shadow(),
                ..base_style
            }
        } else {
            base_style
        }
    }
}

/// Text size categories for contrast requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize {
    /// Normal text (< 18pt or < 14pt bold)
    Normal,
    /// Large text (>= 18pt or >= 14pt bold)
    Large,
}

/// Accessibility utilities
pub struct AccessibilityUtils;

impl AccessibilityUtils {
    /// Calculate minimum font size for readability
    pub fn minimum_font_size(screen_density: f32) -> f32 {
        // Base minimum of 12px, scaled by screen density
        12.0 * screen_density.max(1.0)
    }

    /// Calculate optimal line height for readability
    pub fn optimal_line_height(font_size: f32) -> f32 {
        // 1.4-1.6 line height is optimal for readability
        font_size * 1.5
    }

    /// Check if element is keyboard accessible
    pub fn is_keyboard_accessible(element_type: ElementType) -> bool {
        match element_type {
            ElementType::Button | ElementType::Link | ElementType::Input | ElementType::Select => true,
            ElementType::Div | ElementType::Span | ElementType::Image => false,
        }
    }

    /// Get ARIA role for element type
    pub fn aria_role(element_type: ElementType) -> Option<&'static str> {
        match element_type {
            ElementType::Button => Some("button"),
            ElementType::Link => Some("link"),
            ElementType::Input => Some("textbox"),
            ElementType::Select => Some("combobox"),
            ElementType::Div => None,
            ElementType::Span => None,
            ElementType::Image => Some("img"),
        }
    }

    /// Generate accessible color palette
    pub fn accessible_palette(base_color: Color, background: Color) -> Vec<Color> {
        let mut palette = Vec::new();
        
        // Generate colors that meet WCAG AA contrast requirements
        for i in 0..5 {
            let factor = (i as f32) * 0.2;
            let adjusted_color = Color::from_rgba(
                base_color.r * (1.0 - factor) + factor,
                base_color.g * (1.0 - factor) + factor,
                base_color.b * (1.0 - factor) + factor,
                base_color.a,
            );
            
            // Check if it meets contrast requirements
            let contrast = super::DesignSystemUtils::contrast_ratio(adjusted_color, background);
            if contrast >= 4.5 {
                palette.push(adjusted_color);
            }
        }
        
        palette
    }
}

/// UI element types for accessibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Button,
    Link,
    Input,
    Select,
    Div,
    Span,
    Image,
}

/// Global accessibility system instance
pub const ACCESSIBILITY: AccessibilitySystem = AccessibilitySystem {
    high_contrast: false,
    reduced_motion: false,
    focus_style: FocusStyle {
        color: Color { r: 0.0, g: 0.75, b: 1.0, a: 1.0 },
        width: 2.0,
        offset: 2.0,
        style: FocusRingStyle::Solid,
        animation_duration: Duration::from_millis(150),
    },
    contrast_ratios: ContrastRatio {
        normal_text: 4.5,
        large_text: 3.0,
        ui_components: 3.0,
        graphics: 3.0,
    },
    touch_targets: TouchTargets {
        minimum_size: 44.0,
        recommended_size: 48.0,
        minimum_spacing: 8.0,
    },
    screen_reader: ScreenReaderSupport {
        alt_text: true,
        form_labels: true,
        landmarks: true,
        live_regions: true,
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessibility_system_creation() {
        let accessibility = AccessibilitySystem::default();
        
        // Should have reasonable defaults
        assert!(!accessibility.high_contrast);
        assert!(!accessibility.reduced_motion);
        assert_eq!(accessibility.contrast_ratios.normal_text, 4.5);
        assert_eq!(accessibility.touch_targets.minimum_size, 44.0);
    }

    #[test]
    fn test_wcag_aaa_compliance() {
        let accessibility = AccessibilitySystem::wcag_aaa();
        
        // Should have stricter requirements
        assert_eq!(accessibility.contrast_ratios.normal_text, 7.0);
        assert_eq!(accessibility.touch_targets.minimum_size, 48.0);
        assert_eq!(accessibility.focus_style.width, 3.0);
    }

    #[test]
    fn test_contrast_requirements() {
        let accessibility = AccessibilitySystem::default();
        
        let white = Color::WHITE;
        let black = Color::BLACK;
        let gray = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        
        // Black on white should meet requirements
        assert!(accessibility.meets_contrast_requirement(black, white, TextSize::Normal));
        
        // Gray on white might not meet requirements
        assert!(!accessibility.meets_contrast_requirement(gray, white, TextSize::Normal));
    }

    #[test]
    fn test_touch_target_requirements() {
        let accessibility = AccessibilitySystem::default();
        
        // 44px should meet minimum requirement
        assert!(accessibility.meets_touch_target_requirement(44.0));
        
        // 20px should not meet requirement
        assert!(!accessibility.meets_touch_target_requirement(20.0));
    }

    #[test]
    fn test_reduced_motion() {
        let mut accessibility = AccessibilitySystem::default();
        let base_duration = Duration::from_millis(300);
        
        // Normal motion should return base duration
        assert_eq!(accessibility.animation_duration(base_duration), base_duration);
        
        // Reduced motion should return zero duration
        accessibility.reduced_motion = true;
        assert_eq!(accessibility.animation_duration(base_duration), Duration::from_millis(0));
    }

    #[test]
    fn test_high_contrast_colors() {
        let mut accessibility = AccessibilitySystem::default();
        let base_color = Color::from_rgba(0.5, 0.5, 0.5, 1.0);
        let white_bg = Color::WHITE;
        let black_bg = Color::BLACK;
        
        // Normal mode should return base color
        assert_eq!(accessibility.high_contrast_color(base_color, white_bg), base_color);
        
        // High contrast mode should return black or white
        accessibility.high_contrast = true;
        assert_eq!(accessibility.high_contrast_color(base_color, white_bg), Color::BLACK);
        assert_eq!(accessibility.high_contrast_color(base_color, black_bg), Color::WHITE);
    }

    #[test]
    fn test_focus_styles() {
        let accessibility = AccessibilitySystem::default();
        
        let focus_border = accessibility.focus_border();
        let focus_shadow = accessibility.focus_shadow();
        
        // Should have tech blue color
        assert_eq!(focus_border.color, Color::from_rgba(0.0, 0.75, 1.0, 1.0));
        assert_eq!(focus_border.width, 2.0);
        
        // Shadow should have some blur
        assert!(focus_shadow.blur_radius > 0.0);
    }

    #[test]
    fn test_accessibility_validation() {
        let valid_accessibility = AccessibilitySystem::default();
        assert!(valid_accessibility.validate_compliance().is_ok());

        let invalid_accessibility = AccessibilitySystem {
            contrast_ratios: ContrastRatio {
                normal_text: 1.0, // Too low
                large_text: 1.0,  // Too low
                ui_components: 1.0, // Too low
                graphics: 1.0,    // Too low
            },
            touch_targets: TouchTargets {
                minimum_size: 10.0, // Too small
                recommended_size: 20.0,
                minimum_spacing: 2.0,
            },
            focus_style: FocusStyle {
                width: 0.5, // Too thin
                ..FocusStyle::default()
            },
            ..AccessibilitySystem::default()
        };
        
        assert!(invalid_accessibility.validate_compliance().is_err());
    }

    #[test]
    fn test_accessibility_utils() {
        // Test minimum font size
        let min_font = AccessibilityUtils::minimum_font_size(1.0);
        assert_eq!(min_font, 12.0);
        
        let scaled_font = AccessibilityUtils::minimum_font_size(2.0);
        assert_eq!(scaled_font, 24.0);
        
        // Test optimal line height
        let line_height = AccessibilityUtils::optimal_line_height(16.0);
        assert_eq!(line_height, 24.0);
        
        // Test keyboard accessibility
        assert!(AccessibilityUtils::is_keyboard_accessible(ElementType::Button));
        assert!(!AccessibilityUtils::is_keyboard_accessible(ElementType::Div));
        
        // Test ARIA roles
        assert_eq!(AccessibilityUtils::aria_role(ElementType::Button), Some("button"));
        assert_eq!(AccessibilityUtils::aria_role(ElementType::Div), None);
    }
}
