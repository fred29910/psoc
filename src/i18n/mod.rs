//! Internationalization (i18n) module for PSOC Image Editor
//!
//! This module provides multi-language support using the Fluent localization system.
//! It manages language resources, locale detection, and text translation throughout the application.

use fluent::{FluentBundle, FluentResource};
use fluent_bundle::FluentArgs;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{debug, error, info, warn};
use unic_langid::{langid, LanguageIdentifier};

/// Supported languages in PSOC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    /// English (default)
    English,
    /// Simplified Chinese
    ChineseSimplified,
}

impl Language {
    /// Get the language identifier for this language
    pub fn lang_id(&self) -> LanguageIdentifier {
        match self {
            Language::English => langid!("en-US"),
            Language::ChineseSimplified => langid!("zh-CN"),
        }
    }

    /// Get the display name for this language
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::ChineseSimplified => "简体中文",
        }
    }

    /// Get the language code for file naming
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::ChineseSimplified => "zh-cn",
        }
    }

    /// Get all supported languages
    pub fn all() -> Vec<Language> {
        vec![Language::English, Language::ChineseSimplified]
    }

    /// Parse language from string code
    pub fn from_code(code: &str) -> Option<Language> {
        match code.to_lowercase().as_str() {
            "en" | "en-us" | "english" => Some(Language::English),
            "zh" | "zh-cn" | "chinese" | "chinese-simplified" => Some(Language::ChineseSimplified),
            _ => None,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Localization manager for PSOC
pub struct LocalizationManager {
    /// Current active language
    current_language: Language,
    /// Fluent bundles for each language
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
    /// Fallback language (English)
    fallback_language: Language,
}

impl LocalizationManager {
    /// Create a new localization manager
    pub fn new() -> Self {
        Self {
            current_language: Language::default(),
            bundles: HashMap::new(),
            fallback_language: Language::English,
        }
    }

    /// Initialize the localization manager with language resources
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing localization manager");

        // Load all supported languages
        for language in Language::all() {
            if let Err(e) = self.load_language(language) {
                error!("Failed to load language {:?}: {}", language, e);
                // Continue loading other languages even if one fails
            }
        }

        // Detect system language
        self.detect_system_language();

        info!(
            "Localization manager initialized with language: {:?}",
            self.current_language
        );
        Ok(())
    }

    /// Load language resources for a specific language
    fn load_language(&mut self, language: Language) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Loading language resources for {:?}", language);

        let lang_id = language.lang_id();
        let mut bundle = FluentBundle::new(vec![lang_id]);

        // Load language files from resources directory
        let resource_path = format!("resources/i18n/{}.ftl", language.code());

        if Path::new(&resource_path).exists() {
            let content = fs::read_to_string(&resource_path)?;
            let resource = FluentResource::try_new(content)
                .map_err(|e| format!("Failed to parse Fluent resource: {:?}", e))?;

            bundle
                .add_resource(resource)
                .map_err(|e| format!("Failed to add resource to bundle: {:?}", e))?;

            debug!("Successfully loaded language file: {}", resource_path);
        } else {
            // Load embedded resources
            let content = self.get_embedded_resource(language)?;
            let resource = FluentResource::try_new(content)
                .map_err(|e| format!("Failed to parse embedded Fluent resource: {:?}", e))?;

            bundle
                .add_resource(resource)
                .map_err(|e| format!("Failed to add embedded resource to bundle: {:?}", e))?;

            debug!(
                "Successfully loaded embedded language resources for {:?}",
                language
            );
        }

        self.bundles.insert(language, bundle);
        Ok(())
    }

    /// Get embedded language resources
    fn get_embedded_resource(
        &self,
        language: Language,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match language {
            Language::English => Ok(include_str!("../../resources/i18n/en.ftl").to_string()),
            Language::ChineseSimplified => {
                Ok(include_str!("../../resources/i18n/zh-cn.ftl").to_string())
            }
        }
    }

    /// Detect system language and set as current if supported
    fn detect_system_language(&mut self) {
        if let Some(locale) = sys_locale::get_locale() {
            debug!("Detected system locale: {}", locale);

            if let Some(language) = Language::from_code(&locale) {
                if self.bundles.contains_key(&language) {
                    self.current_language = language;
                    info!("Set language to detected system language: {:?}", language);
                    return;
                }
            }
        }

        warn!(
            "Could not detect supported system language, using default: {:?}",
            self.fallback_language
        );
        self.current_language = self.fallback_language;
    }

    /// Set the current language
    pub fn set_language(&mut self, language: Language) -> Result<(), Box<dyn std::error::Error>> {
        if !self.bundles.contains_key(&language) {
            self.load_language(language)?;
        }

        self.current_language = language;
        info!("Language changed to: {:?}", language);
        Ok(())
    }

    /// Get the current language
    pub fn current_language(&self) -> Language {
        self.current_language
    }

    /// Get all available languages
    pub fn available_languages(&self) -> Vec<Language> {
        self.bundles.keys().copied().collect()
    }

    /// Translate a message key to localized text
    pub fn translate(&self, key: &str) -> String {
        self.translate_with_args(key, None)
    }

    /// Translate a message key with arguments to localized text
    pub fn translate_with_args(&self, key: &str, args: Option<&FluentArgs>) -> String {
        // Try current language first
        if let Some(bundle) = self.bundles.get(&self.current_language) {
            if let Some(msg) = bundle.get_message(key) {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let result = bundle.format_pattern(pattern, args, &mut errors);

                    if errors.is_empty() {
                        return result.to_string();
                    } else {
                        warn!("Translation errors for key '{}': {:?}", key, errors);
                    }
                }
            }
        }

        // Fallback to English if current language fails
        if self.current_language != self.fallback_language {
            if let Some(bundle) = self.bundles.get(&self.fallback_language) {
                if let Some(msg) = bundle.get_message(key) {
                    if let Some(pattern) = msg.value() {
                        let mut errors = vec![];
                        let result = bundle.format_pattern(pattern, args, &mut errors);

                        if errors.is_empty() {
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

    /// Check if a translation key exists
    pub fn has_translation(&self, key: &str) -> bool {
        if let Some(bundle) = self.bundles.get(&self.current_language) {
            bundle.has_message(key)
        } else {
            false
        }
    }
}

impl Default for LocalizationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for LocalizationManager {
    fn clone(&self) -> Self {
        let mut cloned = LocalizationManager::new();
        cloned.current_language = self.current_language;
        cloned.fallback_language = self.fallback_language;

        // Re-initialize the cloned manager to load language resources
        if let Err(e) = cloned.initialize() {
            error!("Failed to initialize cloned localization manager: {}", e);
        }

        // Set the language to match the original
        if let Err(e) = cloned.set_language(self.current_language) {
            error!(
                "Failed to set language in cloned localization manager: {}",
                e
            );
        }

        cloned
    }
}

impl std::fmt::Debug for LocalizationManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalizationManager")
            .field("current_language", &self.current_language)
            .field("fallback_language", &self.fallback_language)
            .field("bundles_count", &self.bundles.len())
            .finish()
    }
}

/// Global localization manager instance
static mut LOCALIZATION_MANAGER: Option<LocalizationManager> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the global localization manager
pub fn init_localization() -> Result<(), Box<dyn std::error::Error>> {
    INIT.call_once(|| {
        let mut manager = LocalizationManager::new();
        if let Err(e) = manager.initialize() {
            error!("Failed to initialize localization: {}", e);
        }
        unsafe {
            LOCALIZATION_MANAGER = Some(manager);
        }
    });
    Ok(())
}

/// Get a reference to the global localization manager
pub fn localization_manager() -> Option<&'static LocalizationManager> {
    unsafe { LOCALIZATION_MANAGER.as_ref() }
}

/// Get a mutable reference to the global localization manager
pub fn localization_manager_mut() -> Option<&'static mut LocalizationManager> {
    unsafe { LOCALIZATION_MANAGER.as_mut() }
}

/// Convenience function to translate a message key
pub fn t(key: &str) -> String {
    if let Some(manager) = localization_manager() {
        manager.translate(key)
    } else {
        key.to_string()
    }
}

/// Convenience function to translate a message key with arguments
pub fn t_with_args(key: &str, args: &FluentArgs) -> String {
    if let Some(manager) = localization_manager() {
        manager.translate_with_args(key, Some(args))
    } else {
        key.to_string()
    }
}
