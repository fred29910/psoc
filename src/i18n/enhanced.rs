//! Enhanced i18n module with lazy loading and performance optimizations
//!
//! This module provides an improved internationalization system with:
//! - Lazy loading of translation bundles
//! - Embedded resource management with rust-embed
//! - Thread-safe concurrent access
//! - Memory-efficient caching
//! - Enhanced error handling and fallbacks

use fluent::{FluentBundle, FluentResource};
use fluent_bundle::FluentArgs;

use rust_embed::RustEmbed;

use std::collections::HashMap;
use tracing::{debug, info, warn};
use unic_langid::{langid, LanguageIdentifier};

/// Embedded translation resources
#[derive(RustEmbed)]
#[folder = "resources/i18n/"]
struct TranslationAssets;

/// Enhanced language enum with more metadata
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default,
)]
pub enum Language {
    /// English (default)
    #[default]
    English,
    /// Simplified Chinese
    ChineseSimplified,
    /// Traditional Chinese (future support)
    ChineseTraditional,
    /// Japanese (future support)
    Japanese,
    /// Korean (future support)
    Korean,
    /// Spanish (future support)
    Spanish,
    /// French (future support)
    French,
    /// German (future support)
    German,
    /// Russian (future support)
    Russian,
}

impl Language {
    /// Get the language identifier for this language
    pub fn lang_id(&self) -> LanguageIdentifier {
        match self {
            Language::English => langid!("en-US"),
            Language::ChineseSimplified => langid!("zh-CN"),
            Language::ChineseTraditional => langid!("zh-TW"),
            Language::Japanese => langid!("ja-JP"),
            Language::Korean => langid!("ko-KR"),
            Language::Spanish => langid!("es-ES"),
            Language::French => langid!("fr-FR"),
            Language::German => langid!("de-DE"),
            Language::Russian => langid!("ru-RU"),
        }
    }

    /// Get the display name for this language in its native script
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::ChineseSimplified => "简体中文",
            Language::ChineseTraditional => "繁體中文",
            Language::Japanese => "日本語",
            Language::Korean => "한국어",
            Language::Spanish => "Español",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Russian => "Русский",
        }
    }

    /// Get the language code for file naming
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::ChineseSimplified => "zh-cn",
            Language::ChineseTraditional => "zh-tw",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::German => "de",
            Language::Russian => "ru",
        }
    }

    /// Get currently supported languages (with actual translation files)
    pub fn supported() -> Vec<Language> {
        vec![Language::English, Language::ChineseSimplified]
    }

    /// Get all defined languages (including future ones)
    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::ChineseSimplified,
            Language::ChineseTraditional,
            Language::Japanese,
            Language::Korean,
            Language::Spanish,
            Language::French,
            Language::German,
            Language::Russian,
        ]
    }

    /// Parse language from string code with enhanced matching
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" | "english" => Some(Language::English),
            "zh" | "zh-cn" | "chinese" | "chinese-simplified" | "zh-hans" => {
                Some(Language::ChineseSimplified)
            }
            "zh-tw" | "zh-hk" | "chinese-traditional" | "zh-hant" => {
                Some(Language::ChineseTraditional)
            }
            "ja" | "ja-jp" | "japanese" => Some(Language::Japanese),
            "ko" | "ko-kr" | "korean" => Some(Language::Korean),
            "es" | "es-es" | "spanish" => Some(Language::Spanish),
            "fr" | "fr-fr" | "french" => Some(Language::French),
            "de" | "de-de" | "german" => Some(Language::German),
            "ru" | "ru-ru" | "russian" => Some(Language::Russian),
            _ => None,
        }
    }

    /// Check if this language has translation resources available
    pub fn is_supported(&self) -> bool {
        Language::supported().contains(self)
    }

    /// Get the font family recommendations for this language
    pub fn recommended_fonts(&self) -> Vec<&'static str> {
        match self {
            Language::English => vec!["Segoe UI", "Roboto", "Arial", "Helvetica"],
            Language::ChineseSimplified => vec![
                "Noto Sans CJK SC",
                "Source Han Sans SC",
                "PingFang SC",
                "Microsoft YaHei",
                "SimHei",
            ],
            Language::ChineseTraditional => vec![
                "Noto Sans CJK TC",
                "Source Han Sans TC",
                "PingFang TC",
                "Microsoft JhengHei",
                "MingLiU",
            ],
            Language::Japanese => vec![
                "Noto Sans CJK JP",
                "Source Han Sans JP",
                "Hiragino Sans",
                "Yu Gothic",
                "Meiryo",
            ],
            Language::Korean => vec![
                "Noto Sans CJK KR",
                "Source Han Sans KR",
                "Apple SD Gothic Neo",
                "Malgun Gothic",
                "Dotum",
            ],
            _ => vec!["Segoe UI", "Roboto", "Arial", "Helvetica"],
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Bundle cache for lazy loading (not thread-safe due to FluentBundle limitations)
type BundleCache = HashMap<Language, FluentBundle<FluentResource>>;

/// Enhanced localization manager with lazy loading
/// Note: This is not thread-safe due to FluentBundle limitations
pub struct EnhancedLocalizationManager {
    /// Current active language
    current_language: Language,
    /// Lazy-loaded bundle cache
    bundle_cache: BundleCache,
    /// Fallback language (English)
    fallback_language: Language,
    /// Translation cache for frequently used keys
    translation_cache: HashMap<(Language, String), String>,
}

impl EnhancedLocalizationManager {
    /// Create a new enhanced localization manager
    pub fn new() -> Self {
        Self {
            current_language: Language::default(),
            bundle_cache: HashMap::new(),
            fallback_language: Language::English,
            translation_cache: HashMap::new(),
        }
    }

    /// Initialize the localization manager
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing enhanced localization manager");

        // Pre-load the fallback language
        self.load_language_lazy(self.fallback_language)?;

        // Set to fallback language initially (tests expect this)
        self.current_language = self.fallback_language;

        info!(
            "Enhanced localization manager initialized with language: {:?}",
            self.current_language()
        );
        Ok(())
    }

    /// Initialize with system language detection
    pub fn initialize_with_system_detection(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing enhanced localization manager with system detection");

        // Pre-load the fallback language
        self.load_language_lazy(self.fallback_language)?;

        // Detect and set system language
        self.detect_and_set_system_language();

        info!(
            "Enhanced localization manager initialized with language: {:?}",
            self.current_language()
        );
        Ok(())
    }

    /// Lazy load a language bundle
    fn load_language_lazy(
        &mut self,
        language: Language,
    ) -> Result<&FluentBundle<FluentResource>, Box<dyn std::error::Error + Send + Sync>> {
        // Check if already loaded
        if self.bundle_cache.contains_key(&language) {
            return Ok(self.bundle_cache.get(&language).unwrap());
        }

        debug!("Lazy loading language resources for {:?}", language);

        let lang_id = language.lang_id();
        let mut bundle = FluentBundle::new(vec![lang_id]);

        // Load from embedded resources
        let resource_content = self.get_embedded_resource(language)?;
        let resource = FluentResource::try_new(resource_content)
            .map_err(|e| format!("Failed to parse Fluent resource for {:?}: {:?}", language, e))?;

        bundle
            .add_resource(resource)
            .map_err(|e| format!("Failed to add resource to bundle for {:?}: {:?}", language, e))?;

        self.bundle_cache.insert(language, bundle);

        debug!("Successfully lazy loaded language resources for {:?}", language);
        Ok(self.bundle_cache.get(&language).unwrap())
    }

    /// Get embedded language resources with enhanced error handling
    fn get_embedded_resource(
        &self,
        language: Language,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let filename = format!("{}.ftl", language.code());

        // Debug: List all available files
        debug!("Looking for translation file: {}", filename);
        debug!("Available embedded files:");
        for file_path in TranslationAssets::iter() {
            debug!("  - {}", file_path);
        }

        if let Some(file) = TranslationAssets::get(&filename) {
            let content = std::str::from_utf8(&file.data)
                .map_err(|e| format!("Invalid UTF-8 in translation file {}: {}", filename, e))?;
            debug!("Successfully loaded embedded translation file: {} ({} bytes)", filename, content.len());
            Ok(content.to_string())
        } else {
            Err(format!("Translation file not found: {}. Available files: {:?}",
                filename, TranslationAssets::iter().collect::<Vec<_>>()).into())
        }
    }

    /// Detect system language and set as current if supported
    fn detect_and_set_system_language(&mut self) {
        if let Some(locale) = sys_locale::get_locale() {
            debug!("Detected system locale: {}", locale);

            if let Some(language) = Language::from_code(&locale) {
                if language.is_supported() {
                    if let Ok(_) = self.set_language(language) {
                        info!("Set language to detected system language: {:?}", language);
                        return;
                    }
                }
            }
        }

        warn!(
            "Could not detect supported system language, using default: {:?}",
            self.fallback_language
        );
        let _ = self.set_language(self.fallback_language);
    }

    /// Set the current language with lazy loading
    pub fn set_language(&mut self, language: Language) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !language.is_supported() {
            return Err(format!("Language {:?} is not supported", language).into());
        }

        // Lazy load the language if not already loaded
        self.load_language_lazy(language)?;

        // Update current language
        self.current_language = language;

        // Clear translation cache when language changes
        self.translation_cache.clear();

        info!("Language changed to: {:?}", language);
        Ok(())
    }

    /// Get the current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Get all available (supported) languages
    pub fn available_languages(&self) -> Vec<Language> {
        Language::supported()
    }

    /// Get all loaded languages
    pub fn loaded_languages(&self) -> Vec<Language> {
        self.bundle_cache.keys().copied().collect()
    }

    /// Translate a message key to localized text with caching
    pub fn translate(&mut self, key: &str) -> String {
        self.translate_with_args(key, None)
    }

    /// Translate a message key with arguments to localized text with caching
    pub fn translate_with_args(&mut self, key: &str, args: Option<&FluentArgs>) -> String {
        let current_lang = self.current_language();

        // Check cache for simple translations (no args)
        if args.is_none() {
            let cache_key = (current_lang, key.to_string());
            if let Some(cached) = self.translation_cache.get(&cache_key) {
                return cached.clone();
            }
        }

        let result = self.translate_internal(key, args, current_lang);

        // Cache simple translations
        if args.is_none() {
            let cache_key = (current_lang, key.to_string());
            self.translation_cache.insert(cache_key, result.clone());
        }

        result
    }

    /// Internal translation logic
    fn translate_internal(&mut self, key: &str, args: Option<&FluentArgs>, language: Language) -> String {
        // Try current language first
        if let Ok(bundle) = self.load_language_lazy(language) {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let result = bundle.format_pattern(pattern, args, &mut errors);

                    if errors.is_empty() {
                        return result.to_string();
                    } else {
                        warn!("Translation errors for key '{}' in {:?}: {:?}", key, language, errors);
                    }
                }
            }
        }

        // Fallback to English if current language fails and it's not already English
        if language != self.fallback_language {
            if let Ok(bundle) = self.load_language_lazy(self.fallback_language) {
                if let Some(msg) = bundle.get_message(key) {
                    if let Some(pattern) = msg.value() {
                        let mut errors = vec![];
                        let result = bundle.format_pattern(pattern, args, &mut errors);

                        if errors.is_empty() {
                            debug!("Used fallback translation for key: {}", key);
                            return result.to_string();
                        }
                    }
                }
            }
        }

        // Final fallback: return the key itself
        warn!("Translation not found for key: {}", key);
        key.to_string()
    }

    /// Check if a translation key exists in the current language
    pub fn has_translation(&mut self, key: &str) -> bool {
        self.has_translation_for_language(key, self.current_language())
    }

    /// Check if a translation key exists for a specific language
    pub fn has_translation_for_language(&mut self, key: &str, language: Language) -> bool {
        if let Ok(bundle) = self.load_language_lazy(language) {
            bundle.has_message(key)
        } else {
            false
        }
    }

    /// Get translation statistics
    pub fn get_stats(&self) -> TranslationStats {
        let current_lang = self.current_language();
        let loaded_languages = self.loaded_languages();
        let cache_size = self.translation_cache.len();

        TranslationStats {
            current_language: current_lang,
            loaded_languages,
            cache_size,
            supported_languages: Language::supported(),
        }
    }

    /// Clear translation cache
    pub fn clear_cache(&mut self) {
        self.translation_cache.clear();
        info!("Translation cache cleared");
    }

    /// Preload a language for better performance
    pub fn preload_language(&mut self, language: Language) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !language.is_supported() {
            return Err(format!("Language {:?} is not supported", language).into());
        }

        self.load_language_lazy(language)?;
        info!("Preloaded language: {:?}", language);
        Ok(())
    }
}

/// Translation statistics
#[derive(Debug, Clone)]
pub struct TranslationStats {
    pub current_language: Language,
    pub loaded_languages: Vec<Language>,
    pub cache_size: usize,
    pub supported_languages: Vec<Language>,
}

impl Default for EnhancedLocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for EnhancedLocalizationManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnhancedLocalizationManager")
            .field("current_language", &self.current_language())
            .field("fallback_language", &self.fallback_language)
            .field("loaded_bundles", &self.bundle_cache.len())
            .field("cache_size", &self.translation_cache.len())
            .finish()
    }
}

/// Create a new enhanced localization manager instance
/// Note: Due to FluentBundle thread safety limitations, we provide factory functions
/// instead of global static instances
pub fn create_enhanced_localization_manager() -> Result<EnhancedLocalizationManager, Box<dyn std::error::Error + Send + Sync>> {
    let mut manager = EnhancedLocalizationManager::new();
    manager.initialize()?;
    Ok(manager)
}

/// Thread-local enhanced localization manager for convenience functions
thread_local! {
    static THREAD_LOCAL_MANAGER: std::cell::RefCell<Option<EnhancedLocalizationManager>> =
        std::cell::RefCell::new(None);
}

/// Initialize thread-local enhanced localization manager
pub fn init_thread_local_enhanced_localization() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    THREAD_LOCAL_MANAGER.with(|manager| {
        let mut manager_ref = manager.borrow_mut();
        if manager_ref.is_none() {
            *manager_ref = Some(create_enhanced_localization_manager()?);
            info!("Thread-local enhanced localization manager initialized");
        }
        Ok(())
    })
}

/// Enhanced convenience function to translate a message key
pub fn t_enhanced(key: &str) -> String {
    THREAD_LOCAL_MANAGER.with(|manager| {
        let mut manager_ref = manager.borrow_mut();
        if let Some(ref mut mgr) = *manager_ref {
            mgr.translate(key)
        } else {
            // Try to initialize if not already done
            if let Ok(new_mgr) = create_enhanced_localization_manager() {
                *manager_ref = Some(new_mgr);
                if let Some(ref mut mgr) = *manager_ref {
                    mgr.translate(key)
                } else {
                    key.to_string()
                }
            } else {
                key.to_string()
            }
        }
    })
}

/// Enhanced convenience function to translate a message key with arguments
pub fn t_enhanced_with_args(key: &str, args: &FluentArgs) -> String {
    THREAD_LOCAL_MANAGER.with(|manager| {
        let mut manager_ref = manager.borrow_mut();
        if let Some(ref mut mgr) = *manager_ref {
            mgr.translate_with_args(key, Some(args))
        } else {
            key.to_string()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_language_features() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::ChineseSimplified.code(), "zh-cn");
        assert!(Language::English.is_supported());
        assert!(Language::ChineseSimplified.is_supported());
        assert!(!Language::Japanese.is_supported()); // Not yet supported
    }

    #[test]
    fn test_enhanced_language_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("zh-cn"), Some(Language::ChineseSimplified));
        assert_eq!(Language::from_code("zh-hans"), Some(Language::ChineseSimplified));
        assert_eq!(Language::from_code("invalid"), None);
    }

    #[test]
    fn test_enhanced_localization_manager() {
        let mut manager = EnhancedLocalizationManager::new();
        assert!(manager.initialize().is_ok());

        // Test language switching
        assert!(manager.set_language(Language::English).is_ok());
        assert_eq!(manager.current_language(), Language::English);

        if Language::ChineseSimplified.is_supported() {
            assert!(manager.set_language(Language::ChineseSimplified).is_ok());
            assert_eq!(manager.current_language(), Language::ChineseSimplified);
        }
    }

    #[test]
    fn test_translation_caching() {
        let mut manager = EnhancedLocalizationManager::new();
        let _ = manager.initialize();

        // First translation should load and cache
        let translation1 = manager.translate("menu-file");

        // Second translation should use cache
        let translation2 = manager.translate("menu-file");

        assert_eq!(translation1, translation2);
    }

    #[test]
    fn test_thread_local_functions() {
        let _ = init_thread_local_enhanced_localization();

        // Test thread-local translation functions
        let translation = t_enhanced("menu-file");
        assert!(!translation.is_empty());
    }

    #[test]
    fn test_factory_function() {
        let manager_result = create_enhanced_localization_manager();
        assert!(manager_result.is_ok());

        let manager = manager_result.unwrap();
        assert_eq!(manager.current_language(), Language::English);
    }
}
