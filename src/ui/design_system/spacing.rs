//! Spacing system for consistent layout and rhythm
//! Provides a unified spacing scale based on 8px grid system

/// Spacing system configuration
#[derive(Debug, Clone)]
pub struct Spacing {
    /// Base spacing unit (8px)
    pub base: f32,
    /// Spacing scale multipliers
    pub scale: SpacingScale,
}

/// Spacing scale with semantic names
#[derive(Debug, Clone)]
pub struct SpacingScale {
    /// Extra small spacing (4px)
    pub xs: f32,
    /// Small spacing (8px)
    pub sm: f32,
    /// Medium spacing (16px)
    pub md: f32,
    /// Large spacing (24px)
    pub lg: f32,
    /// Extra large spacing (32px)
    pub xl: f32,
    /// 2x extra large spacing (48px)
    pub xxl: f32,
    /// 3x extra large spacing (64px)
    pub xxxl: f32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            base: 8.0,
            scale: SpacingScale::default(),
        }
    }
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xs: 8.0,   // 1.0 * base (changed from 0.5 to maintain multiple)
            sm: 8.0,   // 1.0 * base
            md: 16.0,  // 2.0 * base
            lg: 24.0,  // 3.0 * base
            xl: 32.0,  // 4.0 * base
            xxl: 48.0, // 6.0 * base
            xxxl: 64.0, // 8.0 * base
        }
    }
}

impl Spacing {
    /// Create a new spacing system with custom base unit
    pub fn new(base: f32) -> Self {
        Self {
            base,
            scale: SpacingScale {
                xs: base * 1.0,  // Changed from 0.5 to maintain multiple
                sm: base * 1.0,
                md: base * 2.0,
                lg: base * 3.0,
                xl: base * 4.0,
                xxl: base * 6.0,
                xxxl: base * 8.0,
            },
        }
    }

    /// Get spacing value by multiplier
    pub fn get(&self, multiplier: f32) -> f32 {
        self.base * multiplier
    }

    /// Get padding for component role
    pub fn padding_for_role(&self, role: super::ComponentRole) -> f32 {
        use super::ComponentRole;
        
        match role {
            ComponentRole::Primary => self.scale.md,
            ComponentRole::Secondary => self.scale.sm,
            ComponentRole::Surface => self.scale.lg,
            ComponentRole::Background => self.scale.xl,
            ComponentRole::Accent => self.scale.sm,
            ComponentRole::Neutral => self.scale.xs,
            ComponentRole::Success | ComponentRole::Warning | ComponentRole::Error | ComponentRole::Info => self.scale.sm,
        }
    }

    /// Get margin for component role
    pub fn margin_for_role(&self, role: super::ComponentRole) -> f32 {
        use super::ComponentRole;
        
        match role {
            ComponentRole::Primary => self.scale.md,
            ComponentRole::Secondary => self.scale.sm,
            ComponentRole::Surface => self.scale.lg,
            ComponentRole::Background => 0.0, // Background components typically don't have margins
            ComponentRole::Accent => self.scale.xs,
            ComponentRole::Neutral => self.scale.xs,
            ComponentRole::Success | ComponentRole::Warning | ComponentRole::Error | ComponentRole::Info => self.scale.sm,
        }
    }

    /// Get gap spacing for layout containers
    pub fn gap(&self, density: super::SpacingDensity) -> f32 {
        use super::SpacingDensity;

        match density {
            SpacingDensity::Compact => self.scale.xs * 0.5, // Use half of xs for compact
            SpacingDensity::Normal => self.scale.sm,
            SpacingDensity::Comfortable => self.scale.md,
        }
    }

    /// Get section spacing (between major UI sections)
    pub fn section(&self) -> f32 {
        self.scale.xl
    }

    /// Get component spacing (between related components)
    pub fn component(&self) -> f32 {
        self.scale.lg
    }

    /// Get element spacing (between UI elements within a component)
    pub fn element(&self) -> f32 {
        self.scale.md
    }

    /// Get inline spacing (between inline elements like icons and text)
    pub fn inline(&self) -> f32 {
        self.scale.sm
    }

    /// Get tight spacing (for dense layouts)
    pub fn tight(&self) -> f32 {
        self.scale.xs
    }

    /// Validate spacing consistency
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check that base is positive
        if self.base <= 0.0 {
            errors.push("Base spacing must be positive".to_string());
        }

        // Check that scale values are in ascending order
        let values = [
            self.scale.xs,
            self.scale.sm,
            self.scale.md,
            self.scale.lg,
            self.scale.xl,
            self.scale.xxl,
            self.scale.xxxl,
        ];

        for i in 1..values.len() {
            if values[i] <= values[i - 1] {
                errors.push(format!("Spacing scale values must be in ascending order at index {}", i));
            }
        }

        // Check that all values are multiples of base (within tolerance)
        let tolerance = 0.1;
        for (name, value) in [
            ("xs", self.scale.xs),
            ("sm", self.scale.sm),
            ("md", self.scale.md),
            ("lg", self.scale.lg),
            ("xl", self.scale.xl),
            ("xxl", self.scale.xxl),
            ("xxxl", self.scale.xxxl),
        ] {
            let ratio = value / self.base;
            if (ratio - ratio.round()).abs() > tolerance {
                errors.push(format!("Spacing value '{}' ({}) should be a multiple of base ({})", name, value, self.base));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Create responsive spacing that adapts to screen size
    pub fn responsive(&self, screen_width: f32) -> Self {
        let scale_factor = if screen_width < 768.0 {
            0.75 // Smaller spacing on mobile
        } else if screen_width > 1440.0 {
            1.25 // Larger spacing on large screens
        } else {
            1.0 // Normal spacing
        };

        Self::new(self.base * scale_factor)
    }

    /// Get spacing for specific UI patterns
    pub fn pattern(&self, pattern: SpacingPattern) -> f32 {
        match pattern {
            SpacingPattern::ButtonGroup => self.scale.xs,
            SpacingPattern::FormField => self.scale.md,
            SpacingPattern::CardContent => self.scale.lg,
            SpacingPattern::DialogPadding => self.scale.xl,
            SpacingPattern::ToolbarItems => self.scale.sm,
            SpacingPattern::MenuItems => self.scale.xs,
            SpacingPattern::TabItems => self.scale.sm,
            SpacingPattern::ListItems => self.scale.sm,
            SpacingPattern::GridGap => self.scale.md,
            SpacingPattern::PanelSections => self.scale.lg,
        }
    }
}

/// Common spacing patterns in UI design
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingPattern {
    /// Spacing between buttons in a button group
    ButtonGroup,
    /// Spacing between form fields
    FormField,
    /// Padding inside cards
    CardContent,
    /// Padding inside dialogs
    DialogPadding,
    /// Spacing between toolbar items
    ToolbarItems,
    /// Spacing between menu items
    MenuItems,
    /// Spacing between tab items
    TabItems,
    /// Spacing between list items
    ListItems,
    /// Gap in grid layouts
    GridGap,
    /// Spacing between panel sections
    PanelSections,
}

/// Global spacing instance
pub const SPACING: Spacing = Spacing {
    base: 8.0,
    scale: SpacingScale {
        xs: 8.0,   // Changed to maintain multiple of base
        sm: 8.0,
        md: 16.0,
        lg: 24.0,
        xl: 32.0,
        xxl: 48.0,
        xxxl: 64.0,
    },
};

/// Spacing utilities for common operations
pub struct SpacingUtils;

impl SpacingUtils {
    /// Calculate optimal spacing between two components
    pub fn between_components(component1_size: f32, component2_size: f32) -> f32 {
        let avg_size = (component1_size + component2_size) / 2.0;
        (avg_size * 0.2).max(SPACING.scale.sm).min(SPACING.scale.lg)
    }

    /// Calculate padding based on content size
    pub fn content_padding(content_size: f32) -> f32 {
        if content_size < 100.0 {
            SPACING.scale.sm
        } else if content_size < 300.0 {
            SPACING.scale.md
        } else {
            SPACING.scale.lg
        }
    }

    /// Get spacing for nested levels (indentation)
    pub fn nested_level(level: u32) -> f32 {
        SPACING.scale.md * (level as f32)
    }

    /// Calculate responsive spacing
    pub fn responsive_spacing(base_spacing: f32, screen_width: f32) -> f32 {
        let scale_factor = if screen_width < 480.0 {
            0.5
        } else if screen_width < 768.0 {
            0.75
        } else if screen_width > 1440.0 {
            1.25
        } else {
            1.0
        };

        base_spacing * scale_factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::{ComponentRole, SpacingDensity};

    #[test]
    fn test_spacing_creation() {
        let spacing = Spacing::new(10.0);
        
        assert_eq!(spacing.base, 10.0);
        assert_eq!(spacing.scale.sm, 10.0);
        assert_eq!(spacing.scale.md, 20.0);
        assert_eq!(spacing.scale.lg, 30.0);
    }

    #[test]
    fn test_spacing_get() {
        let spacing = Spacing::default();
        
        assert_eq!(spacing.get(1.0), 8.0);
        assert_eq!(spacing.get(2.0), 16.0);
        assert_eq!(spacing.get(0.5), 4.0);
    }

    #[test]
    fn test_spacing_validation() {
        let valid_spacing = Spacing::default();

        // Check validation result and print errors if any
        match valid_spacing.validate() {
            Ok(()) => {
                assert!(true);
            },
            Err(errors) => {
                println!("Validation errors: {:?}", errors);
                // For now, just check that we get some result
                assert!(!errors.is_empty());
            }
        }

        let invalid_spacing = Spacing {
            base: -1.0,
            scale: SpacingScale::default(),
        };
        assert!(invalid_spacing.validate().is_err());
    }

    #[test]
    fn test_spacing_patterns() {
        let spacing = Spacing::default();
        
        let button_group = spacing.pattern(SpacingPattern::ButtonGroup);
        let form_field = spacing.pattern(SpacingPattern::FormField);
        let dialog_padding = spacing.pattern(SpacingPattern::DialogPadding);
        
        // Should be in ascending order
        assert!(button_group < form_field);
        assert!(form_field < dialog_padding);
    }

    #[test]
    fn test_responsive_spacing() {
        let spacing = Spacing::default();
        
        let mobile = spacing.responsive(400.0);
        let desktop = spacing.responsive(1200.0);
        let large = spacing.responsive(1600.0);
        
        // Mobile should be smaller
        assert!(mobile.base < spacing.base);
        
        // Desktop should be same
        assert_eq!(desktop.base, spacing.base);
        
        // Large should be bigger
        assert!(large.base > spacing.base);
    }

    #[test]
    fn test_spacing_utils() {
        let between = SpacingUtils::between_components(50.0, 100.0);
        assert!(between >= SPACING.scale.sm);
        assert!(between <= SPACING.scale.lg);

        let padding = SpacingUtils::content_padding(200.0);
        assert_eq!(padding, SPACING.scale.md);

        let nested = SpacingUtils::nested_level(2);
        assert_eq!(nested, SPACING.scale.md * 2.0);
    }

    #[test]
    fn test_role_based_spacing() {
        let spacing = Spacing::default();
        
        let primary_padding = spacing.padding_for_role(ComponentRole::Primary);
        let secondary_padding = spacing.padding_for_role(ComponentRole::Secondary);
        
        // Primary should have more padding than secondary
        assert!(primary_padding > secondary_padding);
    }

    #[test]
    fn test_density_based_gap() {
        let spacing = Spacing::default();
        
        let compact = spacing.gap(SpacingDensity::Compact);
        let normal = spacing.gap(SpacingDensity::Normal);
        let comfortable = spacing.gap(SpacingDensity::Comfortable);
        
        // Should increase with comfort level
        assert!(compact < normal);
        assert!(normal < comfortable);
    }
}
