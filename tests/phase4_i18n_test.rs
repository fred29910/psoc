//! Phase 4: Internationalization Support Test
//! Tests for dynamic language switching, menu translations, and runtime language changes

use psoc::i18n::{Language, LocalizationManager};
use psoc::ui::components::MenuFactory;

#[test]
fn test_dynamic_language_switching() {
    // Initialize localization manager
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test English (default) - but the manager might start with a different language
    // So let's explicitly set it to English first
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.current_language(), Language::English);
    assert_eq!(manager.translate("menu-file"), "File");
    assert_eq!(manager.translate("menu-edit"), "Edit");
    assert_eq!(manager.translate("menu-view"), "View");

    // Switch to Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    assert_eq!(manager.current_language(), Language::ChineseSimplified);
    assert_eq!(manager.translate("menu-file"), "文件");
    assert_eq!(manager.translate("menu-edit"), "编辑");
    assert_eq!(manager.translate("menu-view"), "视图");

    // Switch back to English
    manager.set_language(Language::English).unwrap();
    assert_eq!(manager.current_language(), Language::English);
    assert_eq!(manager.translate("menu-file"), "File");
}

#[test]
fn test_complete_menu_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test all menu categories in English
    manager.set_language(Language::English).unwrap();
    
    // File menu
    assert_eq!(manager.translate("menu-file-new"), "New");
    assert_eq!(manager.translate("menu-file-open"), "Open");
    assert_eq!(manager.translate("menu-file-save"), "Save");
    assert_eq!(manager.translate("menu-file-save-as"), "Save As");
    assert_eq!(manager.translate("menu-file-export"), "Export");
    assert_eq!(manager.translate("menu-file-import"), "Import");
    assert_eq!(manager.translate("menu-file-recent"), "Recent Files");
    assert_eq!(manager.translate("menu-file-exit"), "Exit");

    // Edit menu
    assert_eq!(manager.translate("menu-edit-undo"), "Undo");
    assert_eq!(manager.translate("menu-edit-redo"), "Redo");
    assert_eq!(manager.translate("menu-edit-cut"), "Cut");
    assert_eq!(manager.translate("menu-edit-copy"), "Copy");
    assert_eq!(manager.translate("menu-edit-paste"), "Paste");
    assert_eq!(manager.translate("menu-edit-delete"), "Delete");
    assert_eq!(manager.translate("menu-edit-select-all"), "Select All");
    assert_eq!(manager.translate("menu-edit-deselect"), "Deselect");
    assert_eq!(manager.translate("menu-edit-preferences"), "Preferences");

    // Test all menu categories in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    
    // File menu
    assert_eq!(manager.translate("menu-file-new"), "新建");
    assert_eq!(manager.translate("menu-file-open"), "打开");
    assert_eq!(manager.translate("menu-file-save"), "保存");
    assert_eq!(manager.translate("menu-file-save-as"), "另存为");
    assert_eq!(manager.translate("menu-file-export"), "导出");
    assert_eq!(manager.translate("menu-file-import"), "导入");
    assert_eq!(manager.translate("menu-file-recent"), "最近文件");
    assert_eq!(manager.translate("menu-file-exit"), "退出");

    // Edit menu
    assert_eq!(manager.translate("menu-edit-undo"), "撤销");
    assert_eq!(manager.translate("menu-edit-redo"), "重做");
    assert_eq!(manager.translate("menu-edit-cut"), "剪切");
    assert_eq!(manager.translate("menu-edit-copy"), "复制");
    assert_eq!(manager.translate("menu-edit-paste"), "粘贴");
    assert_eq!(manager.translate("menu-edit-delete"), "删除");
    assert_eq!(manager.translate("menu-edit-select-all"), "全选");
    assert_eq!(manager.translate("menu-edit-deselect"), "取消选择");
    assert_eq!(manager.translate("menu-edit-preferences"), "首选项");
}

#[test]
fn test_menu_factory_with_translations() {
    // Initialize global localization for menu factory
    if let Err(e) = psoc::i18n::init_localization() {
        eprintln!("Failed to initialize localization: {}", e);
    }

    // Test menu creation with English
    if let Some(manager) = psoc::i18n::localization_manager_mut() {
        manager.set_language(Language::English).unwrap();
    }

    let file_menu = MenuFactory::create_file_menu();
    assert_eq!(file_menu.id, psoc::ui::components::MenuCategoryId::File);
    
    // Check that menu items have proper localization keys
    // The MenuFactory stores the key from t!("...") directly into label_key
    let new_item = file_menu.items.iter().find(|item| item.id == "file-new").unwrap();
    assert_eq!(new_item.label_key, psoc::i18n::t("menu-file-new")); // t() here is for test comparison clarity
    
    let open_item = file_menu.items.iter().find(|item| item.id == "file-open").unwrap();
    assert_eq!(open_item.label_key, psoc::i18n::t("menu-file-open"));

    // Test menu creation with Chinese
    // The factory will still use the keys; the `t` function itself will provide Chinese.
    // So, the label_key field will contain the same key.
    if let Some(manager) = psoc::i18n::localization_manager_mut() {
        manager.set_language(Language::ChineseSimplified).unwrap();
    }
    
    // Re-create the menu with Chinese active to check if `t()` inside factory picks it up.
    // The `label_key` field in MenuItem will still be the key, e.g., "menu-file-new".
    // The MenuCategory's `title` field, however, is resolved by `id.title()` using the current language.
    let file_menu_cn = MenuFactory::create_file_menu();
    assert_eq!(file_menu_cn.title_key, psoc::i18n::t("menu-file")); // Category title_key is the key itself
                                                                 // and its .title field would be "文件"
                                                                 // (not directly tested here but implied by id.title())

    let new_item_cn = file_menu_cn.items.iter().find(|item| item.id == "file-new").unwrap();
    // label_key remains the key, the actual display string is what changes.
    assert_eq!(new_item_cn.label_key, psoc::i18n::t("menu-file-new"));
    // To check the actual Chinese string, you'd need to call t() on new_item_cn.label_key
    // within a context where Chinese is the active language.
    // For this test, we are verifying that MenuFactory correctly stores the keys.
    // The test `test_dynamic_language_switching` already checks if `t()` gives correct translations.

    let open_item_cn = file_menu_cn.items.iter().find(|item| item.id == "file-open").unwrap();
    assert_eq!(open_item_cn.label_key, psoc::i18n::t("menu-file-open"));
}

#[test]
fn test_new_menu_categories_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test new menu categories in English
    manager.set_language(Language::English).unwrap();
    
    // Text menu
    assert_eq!(manager.translate("menu-text"), "Text");
    assert_eq!(manager.translate("menu-text-tool"), "Text Tool");

    // Select menu
    assert_eq!(manager.translate("menu-select"), "Select");
    assert_eq!(manager.translate("menu-select-rectangle"), "Rectangle Selection");
    assert_eq!(manager.translate("menu-select-ellipse"), "Ellipse Selection");
    assert_eq!(manager.translate("menu-select-lasso"), "Lasso Selection");
    assert_eq!(manager.translate("menu-select-magic-wand"), "Magic Wand");
    assert_eq!(manager.translate("menu-select-all"), "Select All");
    assert_eq!(manager.translate("menu-select-deselect"), "Deselect");
    assert_eq!(manager.translate("menu-select-invert"), "Invert Selection");

    // Window menu
    assert_eq!(manager.translate("menu-window"), "Window");
    assert_eq!(manager.translate("menu-window-color-picker"), "Color Picker");
    assert_eq!(manager.translate("menu-window-color-palette"), "Color Palette");
    assert_eq!(manager.translate("menu-window-preferences"), "Preferences");

    // Help menu
    assert_eq!(manager.translate("menu-help"), "Help");
    assert_eq!(manager.translate("menu-help-about"), "About PSOC");
    assert_eq!(manager.translate("menu-help-help"), "Help Documentation");

    // Test new menu categories in Chinese
    manager.set_language(Language::ChineseSimplified).unwrap();
    
    // Text menu
    assert_eq!(manager.translate("menu-text"), "文字");
    assert_eq!(manager.translate("menu-text-tool"), "文字工具");

    // Select menu
    assert_eq!(manager.translate("menu-select"), "选择");
    assert_eq!(manager.translate("menu-select-rectangle"), "矩形选择");
    assert_eq!(manager.translate("menu-select-ellipse"), "椭圆选择");
    assert_eq!(manager.translate("menu-select-lasso"), "套索选择");
    assert_eq!(manager.translate("menu-select-magic-wand"), "魔术棒");
    assert_eq!(manager.translate("menu-select-all"), "全选");
    assert_eq!(manager.translate("menu-select-deselect"), "取消选择");
    assert_eq!(manager.translate("menu-select-invert"), "反选");

    // Window menu
    assert_eq!(manager.translate("menu-window"), "窗口");
    assert_eq!(manager.translate("menu-window-color-picker"), "颜色选择器");
    assert_eq!(manager.translate("menu-window-color-palette"), "调色板");
    assert_eq!(manager.translate("menu-window-preferences"), "首选项");

    // Help menu
    assert_eq!(manager.translate("menu-help"), "帮助");
    assert_eq!(manager.translate("menu-help-about"), "关于 PSOC");
    assert_eq!(manager.translate("menu-help-help"), "帮助文档");
}

#[test]
fn test_language_persistence() {
    // Test that language preferences can be saved and loaded
    use psoc::ui::dialogs::preferences::UserPreferences;
    
    let mut preferences = UserPreferences::default();
    assert_eq!(preferences.interface.language, Language::English);
    
    // Change language
    preferences.interface.language = Language::ChineseSimplified;
    assert_eq!(preferences.interface.language, Language::ChineseSimplified);
    
    // Test serialization/deserialization
    let serialized = ron::ser::to_string(&preferences).unwrap();
    let deserialized: UserPreferences = ron::from_str(&serialized).unwrap();
    assert_eq!(deserialized.interface.language, Language::ChineseSimplified);
}

#[test]
fn test_fallback_translations() {
    let mut manager = LocalizationManager::new();
    manager.initialize().unwrap();

    // Test fallback to English for missing keys
    manager.set_language(Language::ChineseSimplified).unwrap();
    
    // Test with a key that might not exist in Chinese
    let result = manager.translate("non-existent-key");
    // Should return the key itself as fallback
    assert_eq!(result, "non-existent-key");
    
    // Test with existing key
    let result = manager.translate("menu-file");
    assert_eq!(result, "文件");
}
