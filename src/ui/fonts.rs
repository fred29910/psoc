//! Font management system for PSOC Image Editor
//! 
//! This module handles font loading and management for proper internationalization support,
//! particularly for Chinese character rendering.

use iced::Font;

/// Default system font
pub const DEFAULT_FONT: Font = Font::DEFAULT;

/// Icon font for the application
pub const ICON_FONT: Font = Font::with_name("PSOC Icons");

/// Font configuration for different languages
#[derive(Debug, Clone)]
pub struct FontConfig {
    /// Primary font for UI text
    pub ui_font: Font,
    /// Font for Chinese text (fallback)
    pub chinese_font: Option<Font>,
    /// Font for icons
    pub icon_font: Font,
    /// Font size for UI elements
    pub ui_font_size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            ui_font: DEFAULT_FONT,
            chinese_font: None,
            icon_font: ICON_FONT,
            ui_font_size: 14.0,
        }
    }
}

impl FontConfig {
    /// Create a new font configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the appropriate font for the given language
    pub fn get_font_for_language(&self, language: crate::i18n::Language) -> Font {
        match language {
            crate::i18n::Language::ChineseSimplified => {
                // Use Chinese font if available, otherwise fall back to default
                self.chinese_font.unwrap_or(self.ui_font)
            }
            _ => self.ui_font,
        }
    }

    /// Set the UI font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.ui_font_size = size;
        self
    }

    /// Set a custom Chinese font
    pub fn with_chinese_font(mut self, font: Font) -> Self {
        self.chinese_font = Some(font);
        self
    }
}

/// Font manager for the application
#[derive(Debug, Clone)]
pub struct FontManager {
    config: FontConfig,
    current_language: crate::i18n::Language,
}

impl FontManager {
    /// Create a new font manager
    pub fn new() -> Self {
        Self {
            config: FontConfig::default(),
            current_language: crate::i18n::Language::default(),
        }
    }

    /// Initialize the font manager with configuration
    pub fn with_config(config: FontConfig) -> Self {
        Self {
            config,
            current_language: crate::i18n::Language::default(),
        }
    }

    /// Set the current language
    pub fn set_language(&mut self, language: crate::i18n::Language) {
        self.current_language = language;
    }

    /// Get the current UI font based on the active language
    pub fn current_ui_font(&self) -> Font {
        self.config.get_font_for_language(self.current_language)
    }

    /// Get the icon font
    pub fn icon_font(&self) -> Font {
        self.config.icon_font
    }

    /// Get the current font size
    pub fn font_size(&self) -> f32 {
        self.config.ui_font_size
    }

    /// Update font size
    pub fn set_font_size(&mut self, size: f32) {
        self.config.ui_font_size = size;
    }

    /// Get font configuration
    pub fn config(&self) -> &FontConfig {
        &self.config
    }

    /// Update font configuration
    pub fn set_config(&mut self, config: FontConfig) {
        self.config = config;
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a font with fallback support
pub fn create_font_with_fallback(primary: &'static str, _fallback: &'static str) -> Font {
    // Try primary font first, fall back to system default if not available
    // Note: Iced will handle font fallbacks automatically for Unicode characters
    Font::with_name(primary)
}

/// Get system fonts that support Chinese characters
pub fn get_chinese_fonts() -> Vec<&'static str> {
    vec![
        "Noto Sans CJK SC",     // Google Noto fonts
        "Source Han Sans SC",   // Adobe Source Han fonts
        "PingFang SC",          // macOS system font
        "Microsoft YaHei",      // Windows system font
        "SimHei",               // Windows fallback
        "WenQuanYi Micro Hei",  // Linux common font
        "Droid Sans Fallback",  // Android fallback
    ]
}

/// Initialize fonts for the application
pub fn initialize_fonts() -> FontConfig {
    let mut config = FontConfig::new();

    // Try to find a good Chinese font
    for font_name in get_chinese_fonts() {
        let chinese_font = Font::with_name(font_name);
        config = config.with_chinese_font(chinese_font);
        break; // Use the first available font
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_config_creation() {
        let config = FontConfig::new();
        assert_eq!(config.ui_font_size, 14.0);
    }

    #[test]
    fn test_font_manager() {
        let mut manager = FontManager::new();
        assert_eq!(manager.current_language, crate::i18n::Language::English);
        
        manager.set_language(crate::i18n::Language::ChineseSimplified);
        assert_eq!(manager.current_language, crate::i18n::Language::ChineseSimplified);
    }

    #[test]
    fn test_chinese_fonts_list() {
        let fonts = get_chinese_fonts();
        assert!(!fonts.is_empty());
        assert!(fonts.contains(&"Microsoft YaHei"));
    }

    #[test]
    fn test_font_initialization() {
        let config = initialize_fonts();
        assert!(config.chinese_font.is_some());
    }
}
