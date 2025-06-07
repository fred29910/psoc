//! Simple i18n functionality test

use psoc::i18n::{Language, LocalizationManager};

#[test]
fn test_basic_language_functionality() {
    // Test language codes
    assert_eq!(Language::English.code(), "en");
    assert_eq!(Language::ChineseSimplified.code(), "zh-cn");

    // Test display names
    assert_eq!(Language::English.display_name(), "English");
    assert_eq!(Language::ChineseSimplified.display_name(), "简体中文");

    // Test default
    assert_eq!(Language::default(), Language::English);
}

#[test]
fn test_localization_manager_basic() {
    let manager = LocalizationManager::new();
    assert_eq!(manager.current_language(), Language::English);
}

#[test]
fn test_language_switching() {
    let mut manager = LocalizationManager::new();
    let _ = manager.initialize();

    // Switch to Chinese
    let _ = manager.set_language(Language::ChineseSimplified);
    assert_eq!(manager.current_language(), Language::ChineseSimplified);

    // Switch back to English
    let _ = manager.set_language(Language::English);
    assert_eq!(manager.current_language(), Language::English);
}
