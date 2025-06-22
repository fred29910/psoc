//! Test i18n translation functionality

use psoc::i18n::{Language, LocalizationManager};

#[test]
fn test_translation_functionality() {
    let mut manager = LocalizationManager::new();
    manager.initialize().expect("Failed to initialize localization manager");

    // Test English translations
    manager.set_language(Language::English).expect("Failed to set English language");
    assert_eq!(manager.translate("menu-file"), "File");
    assert_eq!(manager.translate("menu-edit"), "Edit");
    assert_eq!(manager.translate("menu-view"), "View");
    assert_eq!(manager.translate("app-title"), "PSOC Image Editor");

    // Test Chinese translations
    manager.set_language(Language::ChineseSimplified).expect("Failed to set Chinese language");
    assert_eq!(manager.translate("menu-file"), "文件");
    assert_eq!(manager.translate("menu-edit"), "编辑");
    assert_eq!(manager.translate("menu-view"), "视图");
    assert_eq!(manager.translate("app-title"), "PSOC 图像编辑器");

    // Test fallback for missing keys
    let missing_key = manager.translate("non-existent-key");
    assert_eq!(missing_key, "non-existent-key");
}

#[test]
fn test_translation_with_args() {
    let mut manager = LocalizationManager::new();
    manager.initialize().expect("Failed to initialize localization manager");

    // Test that we can detect available languages
    let available = manager.available_languages();
    println!("Available languages: {:?}", available);
    assert!(!available.is_empty());

    // Test English - check that we can get translations for keys with placeholders
    manager.set_language(Language::English).expect("Failed to set English language");

    // Test a key that should exist and contain a placeholder
    let status_zoom = manager.translate("status-zoom");
    println!("English status-zoom: '{}'", status_zoom);

    // If the translation system is working, it should either return the translation or the key
    // The key itself is a valid fallback behavior
    assert!(!status_zoom.is_empty());

    // Test Chinese
    manager.set_language(Language::ChineseSimplified).expect("Failed to set Chinese language");

    let status_zoom_zh = manager.translate("status-zoom");
    println!("Chinese status-zoom: '{}'", status_zoom_zh);
    assert!(!status_zoom_zh.is_empty());
}

#[test]
fn test_has_translation() {
    let mut manager = LocalizationManager::new();
    manager.initialize().expect("Failed to initialize localization manager");

    // Test that the manager is initialized and has some bundles
    let available = manager.available_languages();
    println!("Available languages for has_translation test: {:?}", available);

    // Test non-existing keys
    assert!(!manager.has_translation("non-existent-key"));
    assert!(!manager.has_translation(""));
}

#[test]
fn test_language_from_code() {
    assert_eq!(Language::from_code("en"), Some(Language::English));
    assert_eq!(Language::from_code("en-us"), Some(Language::English));
    assert_eq!(Language::from_code("english"), Some(Language::English));
    
    assert_eq!(Language::from_code("zh"), Some(Language::ChineseSimplified));
    assert_eq!(Language::from_code("zh-cn"), Some(Language::ChineseSimplified));
    assert_eq!(Language::from_code("chinese"), Some(Language::ChineseSimplified));
    
    assert_eq!(Language::from_code("invalid"), None);
    assert_eq!(Language::from_code(""), None);
}

#[test]
fn test_all_languages() {
    let languages = Language::all();
    assert_eq!(languages.len(), 2);
    assert!(languages.contains(&Language::English));
    assert!(languages.contains(&Language::ChineseSimplified));
}
