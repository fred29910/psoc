//! Test enhanced i18n system with lazy loading

use psoc::i18n::enhanced::{
    Language, EnhancedLocalizationManager, create_enhanced_localization_manager,
    init_thread_local_enhanced_localization, t_enhanced, t_enhanced_with_args
};
use fluent_bundle::FluentArgs;

#[test]
fn test_enhanced_language_features() {
    // Test basic language properties
    assert_eq!(Language::English.code(), "en");
    assert_eq!(Language::ChineseSimplified.code(), "zh-cn");
    assert_eq!(Language::Japanese.code(), "ja");
    assert_eq!(Language::Korean.code(), "ko");
    
    // Test language support
    assert!(Language::English.is_supported());
    assert!(Language::ChineseSimplified.is_supported());
    assert!(!Language::Japanese.is_supported()); // Not yet supported
    assert!(!Language::Korean.is_supported()); // Not yet supported
    
    // Test display names
    assert_eq!(Language::English.display_name(), "English");
    assert_eq!(Language::ChineseSimplified.display_name(), "简体中文");
    assert_eq!(Language::Japanese.display_name(), "日本語");
    assert_eq!(Language::Korean.display_name(), "한국어");
}

#[test]
fn test_enhanced_language_from_code() {
    // Test English variants
    assert_eq!(Language::from_code("en"), Some(Language::English));
    assert_eq!(Language::from_code("en-us"), Some(Language::English));
    assert_eq!(Language::from_code("en-gb"), Some(Language::English));
    assert_eq!(Language::from_code("english"), Some(Language::English));
    
    // Test Chinese variants
    assert_eq!(Language::from_code("zh"), Some(Language::ChineseSimplified));
    assert_eq!(Language::from_code("zh-cn"), Some(Language::ChineseSimplified));
    assert_eq!(Language::from_code("zh-hans"), Some(Language::ChineseSimplified));
    assert_eq!(Language::from_code("chinese"), Some(Language::ChineseSimplified));
    
    // Test Traditional Chinese
    assert_eq!(Language::from_code("zh-tw"), Some(Language::ChineseTraditional));
    assert_eq!(Language::from_code("zh-hant"), Some(Language::ChineseTraditional));
    
    // Test other languages
    assert_eq!(Language::from_code("ja"), Some(Language::Japanese));
    assert_eq!(Language::from_code("ko"), Some(Language::Korean));
    assert_eq!(Language::from_code("es"), Some(Language::Spanish));
    assert_eq!(Language::from_code("fr"), Some(Language::French));
    assert_eq!(Language::from_code("de"), Some(Language::German));
    assert_eq!(Language::from_code("ru"), Some(Language::Russian));
    
    // Test invalid codes
    assert_eq!(Language::from_code("invalid"), None);
    assert_eq!(Language::from_code(""), None);
}

#[test]
fn test_enhanced_localization_manager_creation() {
    let manager_result = create_enhanced_localization_manager();
    assert!(manager_result.is_ok());
    
    let manager = manager_result.unwrap();
    assert_eq!(manager.current_language(), Language::English);
    
    let available = manager.available_languages();
    assert!(!available.is_empty());
    assert!(available.contains(&Language::English));
    assert!(available.contains(&Language::ChineseSimplified));
}

#[test]
fn test_enhanced_localization_manager_language_switching() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // Test switching to English
    assert!(manager.set_language(Language::English).is_ok());
    assert_eq!(manager.current_language(), Language::English);
    
    // Test switching to Chinese
    if Language::ChineseSimplified.is_supported() {
        assert!(manager.set_language(Language::ChineseSimplified).is_ok());
        assert_eq!(manager.current_language(), Language::ChineseSimplified);
    }
    
    // Test switching to unsupported language
    assert!(manager.set_language(Language::Japanese).is_err());
}

#[test]
fn test_enhanced_translation_functionality() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // Test English translations
    manager.set_language(Language::English).unwrap();
    let file_en = manager.translate("menu-file");
    let edit_en = manager.translate("menu-edit");
    let view_en = manager.translate("menu-view");
    
    println!("English translations: file='{}', edit='{}', view='{}'", file_en, edit_en, view_en);
    assert!(!file_en.is_empty());
    assert!(!edit_en.is_empty());
    assert!(!view_en.is_empty());
    
    // Test Chinese translations
    if Language::ChineseSimplified.is_supported() {
        manager.set_language(Language::ChineseSimplified).unwrap();
        let file_zh = manager.translate("menu-file");
        let edit_zh = manager.translate("menu-edit");
        let view_zh = manager.translate("menu-view");
        
        println!("Chinese translations: file='{}', edit='{}', view='{}'", file_zh, edit_zh, view_zh);
        assert!(!file_zh.is_empty());
        assert!(!edit_zh.is_empty());
        assert!(!view_zh.is_empty());
    }
}

#[test]
fn test_enhanced_translation_caching() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // First translation should load and cache
    let translation1 = manager.translate("menu-file");
    
    // Second translation should use cache
    let translation2 = manager.translate("menu-file");
    
    assert_eq!(translation1, translation2);
    
    // Test cache clearing
    manager.clear_cache();
    let translation3 = manager.translate("menu-file");
    assert_eq!(translation1, translation3); // Should still be the same translation
}

#[test]
fn test_enhanced_translation_with_args() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // Test translation with arguments
    let mut args = FluentArgs::new();
    args.set("version", "1.0.0");
    
    let translation = manager.translate_with_args("about-version", Some(&args));
    println!("Translation with args: '{}'", translation);
    assert!(!translation.is_empty());
}

#[test]
fn test_enhanced_has_translation() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // Test existing keys
    assert!(manager.has_translation("menu-file"));
    assert!(manager.has_translation("menu-edit"));
    
    // Test non-existing keys
    assert!(!manager.has_translation("non-existent-key"));
    assert!(!manager.has_translation(""));
}

#[test]
fn test_enhanced_preload_language() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // Test preloading supported language
    if Language::ChineseSimplified.is_supported() {
        assert!(manager.preload_language(Language::ChineseSimplified).is_ok());
        
        let loaded = manager.loaded_languages();
        assert!(loaded.contains(&Language::ChineseSimplified));
    }
    
    // Test preloading unsupported language
    assert!(manager.preload_language(Language::Japanese).is_err());
}

#[test]
fn test_enhanced_thread_local_functions() {
    let _ = init_thread_local_enhanced_localization();
    
    // Test thread-local translation functions
    let translation = t_enhanced("menu-file");
    assert!(!translation.is_empty());
    println!("Thread-local translation: '{}'", translation);
    
    // Test with arguments
    let mut args = FluentArgs::new();
    args.set("test", "value");
    let translation_with_args = t_enhanced_with_args("menu-file", &args);
    assert!(!translation_with_args.is_empty());
}

#[test]
fn test_enhanced_language_font_recommendations() {
    // Test font recommendations for different languages
    let en_fonts = Language::English.recommended_fonts();
    assert!(en_fonts.contains(&"Segoe UI"));
    assert!(en_fonts.contains(&"Arial"));
    
    let zh_fonts = Language::ChineseSimplified.recommended_fonts();
    assert!(zh_fonts.contains(&"Noto Sans CJK SC"));
    assert!(zh_fonts.contains(&"Microsoft YaHei"));
    
    let ja_fonts = Language::Japanese.recommended_fonts();
    assert!(ja_fonts.contains(&"Noto Sans CJK JP"));
    assert!(ja_fonts.contains(&"Yu Gothic"));
    
    let ko_fonts = Language::Korean.recommended_fonts();
    assert!(ko_fonts.contains(&"Noto Sans CJK KR"));
    assert!(ko_fonts.contains(&"Malgun Gothic"));
}

#[test]
fn test_enhanced_translation_stats() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    let stats = manager.get_stats();
    assert_eq!(stats.current_language, Language::English);
    assert!(!stats.supported_languages.is_empty());
    assert!(stats.supported_languages.contains(&Language::English));
    assert!(stats.supported_languages.contains(&Language::ChineseSimplified));
    
    println!("Translation stats: {:?}", stats);
}

#[test]
fn test_enhanced_fallback_behavior() {
    let mut manager = create_enhanced_localization_manager().unwrap();
    
    // Test fallback for missing keys
    let missing_key = manager.translate("non-existent-key");
    assert_eq!(missing_key, "non-existent-key");
    
    // Test fallback for unsupported language (should not crash)
    // Note: We can't test this directly since set_language prevents unsupported languages
    // But the internal fallback mechanism is tested through the translation process
}

#[test]
fn test_enhanced_language_lists() {
    let supported = Language::supported();
    let all = Language::all();
    
    // Supported should be a subset of all
    assert!(supported.len() <= all.len());
    
    // All supported languages should be in the all list
    for lang in &supported {
        assert!(all.contains(lang));
    }
    
    // Test specific languages
    assert!(supported.contains(&Language::English));
    assert!(supported.contains(&Language::ChineseSimplified));
    assert!(all.contains(&Language::Japanese));
    assert!(all.contains(&Language::Korean));
    
    println!("Supported languages: {:?}", supported);
    println!("All languages: {:?}", all);
}
