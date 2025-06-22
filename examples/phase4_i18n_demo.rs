//! Phase 4: Internationalization Support Demo
//! Demonstrates dynamic language switching and menu system refresh

use psoc::i18n::{Language, LocalizationManager};
use psoc::ui::components::MenuFactory;

fn main() {
    println!("=== PSOC Phase 4: Internationalization Support Demo ===\n");

    // Initialize localization
    let mut manager = LocalizationManager::new();
    if let Err(e) = manager.initialize() {
        eprintln!("Failed to initialize localization: {}", e);
        return;
    }

    println!("✅ Localization system initialized successfully");

    // Demo 1: Dynamic Language Switching
    println!("\n🌍 Demo 1: Dynamic Language Switching");
    println!("=====================================");

    // Start with English
    manager.set_language(Language::English).unwrap();
    println!("Current language: {}", manager.current_language());
    println!("Menu translations:");
    println!("  - File: {}", manager.translate("menu-file"));
    println!("  - Edit: {}", manager.translate("menu-edit"));
    println!("  - View: {}", manager.translate("menu-view"));
    println!("  - Help: {}", manager.translate("menu-help"));

    // Switch to Chinese
    println!("\nSwitching to Chinese...");
    manager.set_language(Language::ChineseSimplified).unwrap();
    println!("Current language: {}", manager.current_language());
    println!("Menu translations:");
    println!("  - File: {}", manager.translate("menu-file"));
    println!("  - Edit: {}", manager.translate("menu-edit"));
    println!("  - View: {}", manager.translate("menu-view"));
    println!("  - Help: {}", manager.translate("menu-help"));

    // Demo 2: Complete Menu Item Translations
    println!("\n📋 Demo 2: Complete Menu Item Translations");
    println!("==========================================");

    println!("\nFile Menu Items (Chinese):");
    println!("  - New: {}", manager.translate("menu-file-new"));
    println!("  - Open: {}", manager.translate("menu-file-open"));
    println!("  - Save: {}", manager.translate("menu-file-save"));
    println!("  - Save As: {}", manager.translate("menu-file-save-as"));
    println!("  - Export: {}", manager.translate("menu-file-export"));
    println!("  - Import: {}", manager.translate("menu-file-import"));
    println!("  - Recent: {}", manager.translate("menu-file-recent"));
    println!("  - Exit: {}", manager.translate("menu-file-exit"));

    println!("\nEdit Menu Items (Chinese):");
    println!("  - Undo: {}", manager.translate("menu-edit-undo"));
    println!("  - Redo: {}", manager.translate("menu-edit-redo"));
    println!("  - Cut: {}", manager.translate("menu-edit-cut"));
    println!("  - Copy: {}", manager.translate("menu-edit-copy"));
    println!("  - Paste: {}", manager.translate("menu-edit-paste"));
    println!("  - Delete: {}", manager.translate("menu-edit-delete"));
    println!("  - Select All: {}", manager.translate("menu-edit-select-all"));
    println!("  - Deselect: {}", manager.translate("menu-edit-deselect"));
    println!("  - Preferences: {}", manager.translate("menu-edit-preferences"));

    // Switch back to English
    println!("\nSwitching back to English...");
    manager.set_language(Language::English).unwrap();

    println!("\nFile Menu Items (English):");
    println!("  - New: {}", manager.translate("menu-file-new"));
    println!("  - Open: {}", manager.translate("menu-file-open"));
    println!("  - Save: {}", manager.translate("menu-file-save"));
    println!("  - Save As: {}", manager.translate("menu-file-save-as"));
    println!("  - Export: {}", manager.translate("menu-file-export"));
    println!("  - Import: {}", manager.translate("menu-file-import"));
    println!("  - Recent: {}", manager.translate("menu-file-recent"));
    println!("  - Exit: {}", manager.translate("menu-file-exit"));

    // Demo 3: New Menu Categories
    println!("\n🆕 Demo 3: New Menu Categories");
    println!("==============================");

    println!("\nText Menu (English):");
    println!("  - Text: {}", manager.translate("menu-text"));
    println!("  - Text Tool: {}", manager.translate("menu-text-tool"));

    println!("\nSelect Menu (English):");
    println!("  - Select: {}", manager.translate("menu-select"));
    println!("  - Rectangle Selection: {}", manager.translate("menu-select-rectangle"));
    println!("  - Ellipse Selection: {}", manager.translate("menu-select-ellipse"));
    println!("  - Lasso Selection: {}", manager.translate("menu-select-lasso"));
    println!("  - Magic Wand: {}", manager.translate("menu-select-magic-wand"));
    println!("  - Select All: {}", manager.translate("menu-select-all"));
    println!("  - Deselect: {}", manager.translate("menu-select-deselect"));
    println!("  - Invert Selection: {}", manager.translate("menu-select-invert"));

    println!("\nWindow Menu (English):");
    println!("  - Window: {}", manager.translate("menu-window"));
    println!("  - Color Picker: {}", manager.translate("menu-window-color-picker"));
    println!("  - Color Palette: {}", manager.translate("menu-window-color-palette"));
    println!("  - Preferences: {}", manager.translate("menu-window-preferences"));

    println!("\nHelp Menu (English):");
    println!("  - Help: {}", manager.translate("menu-help"));
    println!("  - About PSOC: {}", manager.translate("menu-help-about"));
    println!("  - Help Documentation: {}", manager.translate("menu-help-help"));

    // Switch to Chinese for new categories
    manager.set_language(Language::ChineseSimplified).unwrap();

    println!("\nText Menu (Chinese):");
    println!("  - Text: {}", manager.translate("menu-text"));
    println!("  - Text Tool: {}", manager.translate("menu-text-tool"));

    println!("\nSelect Menu (Chinese):");
    println!("  - Select: {}", manager.translate("menu-select"));
    println!("  - Rectangle Selection: {}", manager.translate("menu-select-rectangle"));
    println!("  - Ellipse Selection: {}", manager.translate("menu-select-ellipse"));
    println!("  - Lasso Selection: {}", manager.translate("menu-select-lasso"));
    println!("  - Magic Wand: {}", manager.translate("menu-select-magic-wand"));
    println!("  - Select All: {}", manager.translate("menu-select-all"));
    println!("  - Deselect: {}", manager.translate("menu-select-deselect"));
    println!("  - Invert Selection: {}", manager.translate("menu-select-invert"));

    // Demo 4: Menu Factory Integration
    println!("\n🏭 Demo 4: Menu Factory Integration");
    println!("===================================");

    // Initialize global localization for menu factory
    if let Err(e) = psoc::i18n::init_localization() {
        eprintln!("Failed to initialize global localization: {}", e);
        return;
    }

    // Set global language to English
    if let Some(global_manager) = psoc::i18n::localization_manager_mut() {
        global_manager.set_language(Language::English).unwrap();
    }

    println!("\nCreating File Menu with English translations:");
    let file_menu = MenuFactory::create_file_menu();
    println!("Menu Category: {:?}", file_menu.id);
    println!("Menu Items:");
    for item in &file_menu.items {
        if item.is_separator {
            println!("  - [Separator]");
        } else {
            println!("  - {} ({})", item.label, item.id);
        }
    }

    // Set global language to Chinese
    if let Some(global_manager) = psoc::i18n::localization_manager_mut() {
        global_manager.set_language(Language::ChineseSimplified).unwrap();
    }

    println!("\nCreating File Menu with Chinese translations:");
    let file_menu_cn = MenuFactory::create_file_menu();
    println!("Menu Category: {:?}", file_menu_cn.id);
    println!("Menu Items:");
    for item in &file_menu_cn.items {
        if item.is_separator {
            println!("  - [分隔符]");
        } else {
            println!("  - {} ({})", item.label, item.id);
        }
    }

    // Demo 5: Language Preference Persistence
    println!("\n💾 Demo 5: Language Preference Persistence");
    println!("==========================================");

    use psoc::ui::dialogs::preferences::UserPreferences;

    let mut preferences = UserPreferences::default();
    println!("Default language: {:?}", preferences.interface.language);

    preferences.interface.language = Language::ChineseSimplified;
    println!("Changed language to: {:?}", preferences.interface.language);

    // Serialize to RON format
    let serialized = ron::ser::to_string_pretty(&preferences, ron::ser::PrettyConfig::default()).unwrap();
    println!("\nSerialized preferences (RON format):");
    println!("{}", serialized);

    // Deserialize back
    let deserialized: UserPreferences = ron::from_str(&serialized).unwrap();
    println!("Deserialized language: {:?}", deserialized.interface.language);

    println!("\n✅ Phase 4: Internationalization Support Demo Complete!");
    println!("\nKey Features Demonstrated:");
    println!("  ✓ Dynamic language switching");
    println!("  ✓ Complete menu item translations (English/Chinese)");
    println!("  ✓ New menu categories with translations");
    println!("  ✓ Menu factory integration with i18n");
    println!("  ✓ Language preference persistence");
    println!("  ✓ Runtime language change functionality");
    println!("\nThe menu system can now be refreshed dynamically when language changes,");
    println!("providing a seamless multilingual user experience!");
}
