//! Unit tests for the menu system

use psoc::ui::components::{MenuCategory, MenuCategoryId, MenuItem, MenuSystem};

#[derive(Debug, Clone)]
enum TestMessage {
    Action1,
    Action2,
    Action3,
}

#[test]
fn test_menu_item_creation() {
    let item = MenuItem::new("test_id", "Test Item", TestMessage::Action1);
    
    assert_eq!(item.id, "test_id");
    assert_eq!(item.label, "Test Item");
    assert!(item.action.is_some());
    assert!(!item.is_separator);
    assert!(item.is_enabled);
}

#[test]
fn test_menu_item_separator() {
    let separator = MenuItem::<TestMessage>::separator();
    
    assert_eq!(separator.id, "separator");
    assert!(separator.label.is_empty());
    assert!(separator.action.is_none());
    assert!(separator.is_separator);
}

#[test]
fn test_menu_item_with_icon_and_shortcut() {
    use psoc::ui::icons::Icon;
    
    let item = MenuItem::new("test", "Test", TestMessage::Action1)
        .with_icon(Icon::New)
        .with_shortcut("Ctrl+N");
    
    assert!(item.icon.is_some());
    assert_eq!(item.shortcut, Some("Ctrl+N".to_string()));
}

#[test]
fn test_menu_category_creation() {
    let items = vec![
        MenuItem::new("item1", "Item 1", TestMessage::Action1),
        MenuItem::new("item2", "Item 2", TestMessage::Action2),
    ];
    
    let category = MenuCategory::new(MenuCategoryId::File, items);
    
    assert_eq!(category.id, MenuCategoryId::File);
    assert_eq!(category.items.len(), 2);
    assert!(!category.is_open);
}

#[test]
fn test_menu_system_creation() {
    let categories = vec![
        MenuCategory::new(MenuCategoryId::File, vec![
            MenuItem::new("new", "New", TestMessage::Action1),
            MenuItem::new("open", "Open", TestMessage::Action2),
        ]),
        MenuCategory::new(MenuCategoryId::Edit, vec![
            MenuItem::new("undo", "Undo", TestMessage::Action3),
        ]),
    ];
    
    let menu_system = MenuSystem::new(categories);
    
    assert_eq!(menu_system.categories.len(), 2);
    assert!(menu_system.active_menu.is_none());
    assert!(menu_system.hover_item.is_none());
    assert_eq!(menu_system.animation_states.len(), 2);
}

#[test]
fn test_menu_system_open_menu() {
    let categories = vec![
        MenuCategory::new(MenuCategoryId::File, vec![
            MenuItem::new("new", "New", TestMessage::Action1),
        ]),
        MenuCategory::new(MenuCategoryId::Edit, vec![
            MenuItem::new("undo", "Undo", TestMessage::Action2),
        ]),
    ];
    
    let mut menu_system = MenuSystem::new(categories);
    
    // Open File menu
    menu_system.open_menu(MenuCategoryId::File);
    
    assert_eq!(menu_system.active_menu, Some(MenuCategoryId::File));
    assert!(menu_system.is_menu_open(MenuCategoryId::File));
    assert!(!menu_system.is_menu_open(MenuCategoryId::Edit));
    
    // Check that the category is marked as open
    let file_category = menu_system.categories.iter().find(|c| c.id == MenuCategoryId::File).unwrap();
    assert!(file_category.is_open);
}

#[test]
fn test_menu_system_close_all() {
    let categories = vec![
        MenuCategory::new(MenuCategoryId::File, vec![
            MenuItem::new("new", "New", TestMessage::Action1),
        ]),
    ];
    
    let mut menu_system = MenuSystem::new(categories);
    
    // Open menu first
    menu_system.open_menu(MenuCategoryId::File);
    assert!(menu_system.is_menu_open(MenuCategoryId::File));
    
    // Close all menus
    menu_system.close_all();
    assert!(menu_system.active_menu.is_none());
    assert!(!menu_system.is_menu_open(MenuCategoryId::File));
    
    // Check that all categories are marked as closed
    for category in &menu_system.categories {
        assert!(!category.is_open);
    }
}

#[test]
fn test_menu_system_switch_menu() {
    let categories = vec![
        MenuCategory::new(MenuCategoryId::File, vec![
            MenuItem::new("new", "New", TestMessage::Action1),
        ]),
        MenuCategory::new(MenuCategoryId::Edit, vec![
            MenuItem::new("undo", "Undo", TestMessage::Action2),
        ]),
    ];
    
    let mut menu_system = MenuSystem::new(categories);
    
    // Open File menu
    menu_system.open_menu(MenuCategoryId::File);
    assert!(menu_system.is_menu_open(MenuCategoryId::File));
    
    // Switch to Edit menu
    menu_system.open_menu(MenuCategoryId::Edit);
    assert!(!menu_system.is_menu_open(MenuCategoryId::File));
    assert!(menu_system.is_menu_open(MenuCategoryId::Edit));
}

#[test]
fn test_menu_category_id_all() {
    let all_categories = MenuCategoryId::all();
    
    assert_eq!(all_categories.len(), 10);
    assert_eq!(all_categories[0], MenuCategoryId::File);
    assert_eq!(all_categories[1], MenuCategoryId::Edit);
    assert_eq!(all_categories[9], MenuCategoryId::Help);
}

#[test]
fn test_menu_category_id_title() {
    // Note: This test assumes the translation system returns the key if no translation is found
    assert!(!MenuCategoryId::File.title().is_empty());
    assert!(!MenuCategoryId::Edit.title().is_empty());
    assert!(!MenuCategoryId::Help.title().is_empty());
}

#[test]
fn test_menu_system_active_category() {
    let categories = vec![
        MenuCategory::new(MenuCategoryId::File, vec![
            MenuItem::new("new", "New", TestMessage::Action1),
        ]),
        MenuCategory::new(MenuCategoryId::Edit, vec![
            MenuItem::new("undo", "Undo", TestMessage::Action2),
        ]),
    ];
    
    let mut menu_system = MenuSystem::new(categories);
    
    // No active category initially
    assert!(menu_system.active_category().is_none());
    
    // Open File menu
    menu_system.open_menu(MenuCategoryId::File);
    let active = menu_system.active_category();
    assert!(active.is_some());
    assert_eq!(active.unwrap().id, MenuCategoryId::File);
}

#[test]
fn test_animation_state_default() {
    use psoc::ui::components::AnimationState;
    
    let state = AnimationState::default();
    assert!(matches!(state, AnimationState::Closed));
}

#[test]
fn test_menu_system_update_animations() {
    let categories = vec![
        MenuCategory::new(MenuCategoryId::File, vec![
            MenuItem::new("new", "New", TestMessage::Action1),
        ]),
    ];
    
    let mut menu_system = MenuSystem::new(categories);
    
    // Open menu to start animation
    menu_system.open_menu(MenuCategoryId::File);
    
    // Update animations
    menu_system.update_animations(0.1); // 100ms
    
    // Animation should be in progress or complete
    let animation_state = menu_system.animation_states.get(&MenuCategoryId::File);
    assert!(animation_state.is_some());
}
