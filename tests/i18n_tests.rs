//! Tests for internationalization (i18n) functionality

use psoc::i18n::{Language, LocalizationManager};

#[test]
fn test_language_enum() {
    // Test language codes
    assert_eq!(Language::English.code(), "en");
    assert_eq!(Language::ChineseSimplified.code(), "zh-cn");

    // Test display names
    assert_eq!(Language::English.display_name(), "English");
    assert_eq!(Language::ChineseSimplified.display_name(), "简体中文");

    // Test language parsing
    assert_eq!(Language::from_code("en"), Some(Language::English));
    assert_eq!(Language::from_code("en-us"), Some(Language::English));
    assert_eq!(
        Language::from_code("zh-cn"),
        Some(Language::ChineseSimplified)
    );
    assert_eq!(Language::from_code("invalid"), None);

    // Test default
    assert_eq!(Language::default(), Language::English);
}

#[test]
fn test_language_display() {
    assert_eq!(format!("{}", Language::English), "English");
    assert_eq!(format!("{}", Language::ChineseSimplified), "简体中文");
}

#[test]
fn test_localization_manager_creation() {
    let manager = LocalizationManager::new();
    assert_eq!(manager.current_language(), Language::English);
}

#[test]
fn test_localization_manager_initialization() {
    let mut manager = LocalizationManager::new();
    assert!(manager.initialize().is_ok());

    // Should have at least English available
    let available = manager.available_languages();
    assert!(available.contains(&Language::English));
}

#[test]
fn test_language_switching() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Switch to Chinese
    assert!(manager.set_language(Language::ChineseSimplified).is_ok());
    assert_eq!(manager.current_language(), Language::ChineseSimplified);

    // Switch back to English
    assert!(manager.set_language(Language::English).is_ok());
    assert_eq!(manager.current_language(), Language::English);
}

#[test]
fn test_basic_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test English translations
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("app-title"), "PSOC Image Editor");
    assert_eq!(manager.translate("menu-file"), "File");
    assert_eq!(manager.translate("menu-edit"), "Edit");

    // Test Chinese translations
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("app-title"), "PSOC 图像编辑器");
    assert_eq!(manager.translate("menu-file"), "文件");
    assert_eq!(manager.translate("menu-edit"), "编辑");
}

#[test]
fn test_missing_translation_fallback() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test with non-existent key
    let result = manager.translate("non-existent-key");
    assert_eq!(result, "non-existent-key"); // Should return the key itself
}

#[test]
fn test_translation_key_existence() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test key existence
    assert!(manager.has_translation("app-title"));
    assert!(manager.has_translation("menu-file"));
    assert!(!manager.has_translation("non-existent-key"));
}

#[test]
fn test_menu_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test menu translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("menu-file-new"), "New");
    assert_eq!(manager.translate("menu-file-open"), "Open");
    assert_eq!(manager.translate("menu-file-save"), "Save");
    assert_eq!(manager.translate("menu-edit-undo"), "Undo");
    assert_eq!(manager.translate("menu-edit-redo"), "Redo");

    // Test menu translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("menu-file-new"), "新建");
    assert_eq!(manager.translate("menu-file-open"), "打开");
    assert_eq!(manager.translate("menu-file-save"), "保存");
    assert_eq!(manager.translate("menu-edit-undo"), "撤销");
    assert_eq!(manager.translate("menu-edit-redo"), "重做");
}

#[test]
fn test_tool_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test tool translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("tool-select"), "Select");
    assert_eq!(manager.translate("tool-brush"), "Brush");
    assert_eq!(manager.translate("tool-eraser"), "Eraser");

    // Test tool translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("tool-select"), "选择");
    assert_eq!(manager.translate("tool-brush"), "画笔");
    assert_eq!(manager.translate("tool-eraser"), "橡皮擦");
}

#[test]
fn test_dialog_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test dialog translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("dialog-ok"), "OK");
    assert_eq!(manager.translate("dialog-cancel"), "Cancel");
    assert_eq!(manager.translate("dialog-apply"), "Apply");

    // Test dialog translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("dialog-ok"), "确定");
    assert_eq!(manager.translate("dialog-cancel"), "取消");
    assert_eq!(manager.translate("dialog-apply"), "应用");
}

#[test]
fn test_status_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test status translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("status-no-document"), "No document");
    assert_eq!(manager.translate("status-document-saved"), "Saved");
    assert_eq!(manager.translate("status-document-unsaved"), "Unsaved");

    // Test status translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("status-no-document"), "无文档");
    assert_eq!(manager.translate("status-document-saved"), "已保存");
    assert_eq!(manager.translate("status-document-unsaved"), "未保存");
}

#[test]
fn test_canvas_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test canvas translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("canvas-no-document"), "No Document Open");
    assert_eq!(
        manager.translate("canvas-click-open"),
        "Click 'Open' to load an image"
    );

    // Test canvas translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("canvas-no-document"), "无文档打开");
    assert_eq!(
        manager.translate("canvas-click-open"),
        "点击\"打开\"加载图像"
    );
}

#[test]
fn test_global_localization_functions() {
    // Initialize global localization
    psoc::i18n::init_localization().unwrap();

    // Test global translation functions
    let english_title = psoc::i18n::t("app-title");
    assert!(!english_title.is_empty());

    // The global functions should work
    let file_menu = psoc::i18n::t("menu-file");
    assert!(!file_menu.is_empty());
}

#[test]
fn test_language_selector_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test language selector translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("language-selector-title"), "Language");
    assert_eq!(
        manager.translate("language-selector-placeholder"),
        "Select Language"
    );

    // Test language selector translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("language-selector-title"), "语言");
    assert_eq!(
        manager.translate("language-selector-placeholder"),
        "选择语言"
    );
}

#[test]
fn test_blend_mode_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test blend mode translations in English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.translate("blend-mode-normal"), "Normal");
    assert_eq!(manager.translate("blend-mode-multiply"), "Multiply");
    assert_eq!(manager.translate("blend-mode-screen"), "Screen");

    // Test blend mode translations in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.translate("blend-mode-normal"), "正常");
    assert_eq!(manager.translate("blend-mode-multiply"), "正片叠底");
    assert_eq!(manager.translate("blend-mode-screen"), "滤色");
}

#[test]
fn test_all_languages_available() {
    let languages = Language::all();
    assert_eq!(languages.len(), 2);
    assert!(languages.contains(&Language::English));
    assert!(languages.contains(&Language::ChineseSimplified));
}
