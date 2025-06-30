// tests/ui/menu_system_tests.rs

#[cfg(test)]
mod menu_system_tests {
    // Correctly import from the main lib crate `psoc`
    use psoc::ui::components::menu_system::{
        MenuSystem, MenuCategory, MenuItem, MenuCategoryId, MenuMessage, AnimationState
    };
    use psoc::ui::icons::Icon;
    use psoc::i18n::t; // For t! macro

    // Define TestAppMessage locally for these tests
    #[derive(Debug, Clone, PartialEq)]
    enum TestAppMessage {
        Action(String),
        // Add other variants if needed by specific menu item actions in tests
    }

    // Helper function to create a test message
    fn test_action(id: &str) -> Option<TestAppMessage> { // MenuItem::action is Option<M>
        Some(TestAppMessage::Action(id.to_string()))
    }

    fn create_test_menu_system() -> MenuSystem<TestAppMessage> {
        let file_items = vec![
            MenuItem::new("file-new", &t("menu-file-new"), test_action("file-new"))
                .with_icon(Icon::New)
                .with_shortcut("shortcut-new"),
            MenuItem::new("file-open", &t("menu-file-open"), test_action("file-open"))
                .with_icon(Icon::Open)
                .with_submenu(vec![
                    MenuItem::new("open-recent", &t("menu-open-recent"), test_action("open-recent")),
                    MenuItem::separator(),
                    MenuItem::new("open-from-cloud", &t("menu-open-from-cloud"), test_action("open-cloud")).enabled(false),
                ]),
            MenuItem::separator(),
            MenuItem::new("file-exit", &t("menu-file-exit"), test_action("file-exit")),
        ];

        let edit_items = vec![
            MenuItem::new("edit-undo", &t("menu-edit-undo"), test_action("edit-undo")).enabled(false), // Disabled
            MenuItem::new("edit-redo", &t("menu-edit-redo"), test_action("edit-redo")),
        ];

        let view_items = vec![
            MenuItem::new("view-zoom", &t("menu-view-zoom"), test_action("view-zoom"))
        ];

        let categories = vec![
            MenuCategory::new(MenuCategoryId::File, &t("menu-file"), file_items),
            MenuCategory::new(MenuCategoryId::Edit, &t("menu-edit"), edit_items),
            MenuCategory::new(MenuCategoryId::View, &t("menu-view"), view_items),
        ];
        MenuSystem::new(categories)
    }

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem::new("test_id", "Test Item", test_action("test_id"));
        assert_eq!(item.id, "test_id");
        assert_eq!(item.label_key, "Test Item"); // label_key stores the key
        assert!(item.action.is_some());
        assert!(!item.is_separator);
        assert!(item.is_enabled);
    }

    #[test]
    fn test_menu_item_separator() {
        let separator = MenuItem::<TestAppMessage>::separator();
        // ID of separator is random, so check its properties
        assert!(separator.label_key.is_empty());
        assert!(separator.action.is_none());
        assert!(separator.is_separator);
        assert!(!separator.is_enabled); // Separators are not enabled
    }

    #[test]
    fn test_menu_item_with_icon_and_shortcut() {
        let item = MenuItem::new("test", "Test", test_action("test"))
            .with_icon(Icon::New)
            .with_shortcut("shortcut-key-test"); // Use a key
        assert!(item.icon.is_some());
        assert_eq!(item.shortcut_key, Some("shortcut-key-test".to_string()));
    }

    #[test]
    fn test_menu_category_creation() {
        let items = vec![
            MenuItem::new("item1", "Item 1 Key", test_action("item1")),
            MenuItem::new("item2", "Item 2 Key", test_action("item2")),
        ];
        let category = MenuCategory::new(MenuCategoryId::File, "menu-file-key", items);
        assert_eq!(category.id, MenuCategoryId::File);
        assert_eq!(category.title_key, "menu-file-key");
        assert_eq!(category.items.len(), 2);
        assert!(!category.is_open);
    }

    #[test]
    fn test_menu_system_creation_and_initialization() {
        let menu_system = create_test_menu_system();
        assert_eq!(menu_system.categories.len(), 3);
        assert_eq!(menu_system.categories[0].id, MenuCategoryId::File);
        assert_eq!(menu_system.categories[0].items.len(), 4);
        assert_eq!(menu_system.categories[1].id, MenuCategoryId::Edit);
        assert_eq!(menu_system.categories[1].items.len(), 2);

        assert!(menu_system.active_menu_category_id.is_none());
        assert!(menu_system.focused_item_path.is_none());
        assert!(menu_system.focused_category_index.is_none());
        assert_eq!(menu_system.animation_states.len(), 3);
    }

    #[test]
    fn test_menu_state_management_open_close() {
        let mut menu_system = create_test_menu_system();

        menu_system.open_menu(MenuCategoryId::File, 0);
        assert_eq!(menu_system.active_menu_category_id, Some(MenuCategoryId::File));
        assert!(menu_system.categories[0].is_open);
        assert!(!menu_system.categories[1].is_open);
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 0]));

        menu_system.open_menu(MenuCategoryId::Edit, 1);
        assert_eq!(menu_system.active_menu_category_id, Some(MenuCategoryId::Edit));
        assert!(!menu_system.categories[0].is_open);
        assert!(menu_system.categories[1].is_open);
        assert_eq!(menu_system.focused_item_path, Some(vec![1, 0]));

        menu_system.focused_category_index = Some(1); // Simulate category bar focus on Edit
        menu_system.close_active_menu();
        assert!(menu_system.active_menu_category_id.is_none());
        assert_eq!(menu_system.focused_item_path, Some(vec![1])); // Focus back to Edit category in bar

        menu_system.toggle_menu(MenuCategoryId::File, 0);
        assert_eq!(menu_system.active_menu_category_id, Some(MenuCategoryId::File));
        assert_eq!(menu_system.focused_category_index, Some(0));

        menu_system.toggle_menu(MenuCategoryId::File, 0);
        assert!(menu_system.active_menu_category_id.is_none());
        assert_eq!(menu_system.focused_category_index, Some(0));
    }

    #[test]
    fn test_focus_menu_bar_and_categories() {
        let mut menu_system = create_test_menu_system();

        menu_system.focus_menu_bar(true);
        assert_eq!(menu_system.focused_category_index, Some(0));
        assert_eq!(menu_system.focused_item_path, Some(vec![0]));

        menu_system.focus_next_menu_category();
        assert_eq!(menu_system.focused_category_index, Some(1));
        assert_eq!(menu_system.focused_item_path, Some(vec![1]));

        menu_system.focus_next_menu_category();
        assert_eq!(menu_system.focused_category_index, Some(2));
        assert_eq!(menu_system.focused_item_path, Some(vec![2]));

        menu_system.focus_next_menu_category();
        assert_eq!(menu_system.focused_category_index, Some(0));
        assert_eq!(menu_system.focused_item_path, Some(vec![0]));

        menu_system.focus_prev_menu_category();
        assert_eq!(menu_system.focused_category_index, Some(2));
        assert_eq!(menu_system.focused_item_path, Some(vec![2]));

        menu_system.open_focused_menu_category_dropdown();
        assert_eq!(menu_system.active_menu_category_id, Some(MenuCategoryId::View));
        assert_eq!(menu_system.focused_item_path, Some(vec![2, 0]));
    }

    #[test]
    fn test_dropdown_item_navigation() {
        let mut menu_system = create_test_menu_system();
        menu_system.open_menu(MenuCategoryId::File, 0);
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 0]));

        menu_system.focus_next_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 1]));

        menu_system.focus_next_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 3]));

        menu_system.focus_next_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 0]));

        menu_system.focus_prev_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 3]));

        menu_system.focus_prev_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 1]));
    }
    
    #[test]
    fn test_disabled_items_are_skipped_in_navigation() {
        let mut menu_system = create_test_menu_system();
        menu_system.open_menu(MenuCategoryId::Edit, 1);

        // Initial focus from open_menu is on the first item [1,0] ("edit-undo", which is disabled)
        assert_eq!(menu_system.focused_item_path, Some(vec![1, 0]));

        // Now navigate, should skip to "edit-redo"
        menu_system.focus_next_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![1, 1]));
        assert_eq!(menu_system.get_focused_item().unwrap().id, "edit-redo");

        menu_system.focus_prev_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![1, 1]));
        assert_eq!(menu_system.get_focused_item().unwrap().id, "edit-redo");

        menu_system.focus_next_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![1, 1]));
        assert_eq!(menu_system.get_focused_item().unwrap().id, "edit-redo");
    }

    #[test]
    fn test_submenu_navigation_and_activation() {
        let mut menu_system = create_test_menu_system();
        menu_system.open_menu(MenuCategoryId::File, 0);
        menu_system.focused_item_path = Some(vec![0, 1]); // Manually focus "file-open"

        assert_eq!(menu_system.get_focused_item().unwrap().id, "file-open");

        menu_system.open_focused_submenu();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 1, 0]));
        assert_eq!(menu_system.get_focused_item().unwrap().id, "open-recent");

        menu_system.focus_next_item_in_dropdown();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 1, 0]));

        menu_system.close_submenu_or_active_menu();
        assert_eq!(menu_system.focused_item_path, Some(vec![0, 1]));

        menu_system.focused_category_index = Some(0);
        menu_system.close_submenu_or_active_menu();
        assert!(menu_system.active_menu_category_id.is_none());
        assert_eq!(menu_system.focused_item_path, Some(vec![0]));
    }
    
    #[test]
    fn test_get_focused_item_action_and_id() {
        let mut menu_system = create_test_menu_system();
        menu_system.open_menu(MenuCategoryId::File, 0);

        let item = menu_system.get_focused_item().expect("Should have focused item 'file-new'");
        assert_eq!(item.id, "file-new");
        assert_eq!(item.action, test_action("file-new"));

        menu_system.focused_item_path = Some(vec![0, 3]);
        let item_exit = menu_system.get_focused_item().expect("Should have focused item 'file-exit'");
        assert_eq!(item_exit.id, "file-exit");
        assert_eq!(item_exit.action, test_action("file-exit"));
    }

    #[test]
    fn test_menu_category_id_all() {
        let all_categories = MenuCategoryId::all();
        assert_eq!(all_categories.len(), 10);
        assert_eq!(all_categories[0], MenuCategoryId::File);
        assert_eq!(all_categories[9], MenuCategoryId::Help);
    }

    // MenuCategoryId::title() was removed in favor of MenuCategory.title_key
    // So this test is no longer directly applicable as is.
    // #[test]
    // fn test_menu_category_id_title() {
    //     assert_eq!(MenuCategoryId::File.title(), t("menu-file"));
    // }

    #[test]
    fn test_menu_system_get_active_category() {
        let mut menu_system = create_test_menu_system();
        assert!(menu_system.get_active_category().is_none());

        menu_system.open_menu(MenuCategoryId::File, 0);
        let active_cat = menu_system.get_active_category();
        assert!(active_cat.is_some());
        assert_eq!(active_cat.unwrap().id, MenuCategoryId::File);
    }

    #[test]
    fn test_animation_state_default() {
        let state = AnimationState::default();
        assert!(matches!(state, AnimationState::Closed));
    }

    #[test]
    fn test_menu_system_update_animations() {
        let mut menu_system = create_test_menu_system();
        menu_system.open_menu(MenuCategoryId::File, 0);

        let initial_state = menu_system.animation_states.get(&MenuCategoryId::File).unwrap();
        assert!(matches!(initial_state, AnimationState::Opening(val) if val == 0.0));

        menu_system.update_animations(0.1);
        let mid_state = menu_system.animation_states.get(&MenuCategoryId::File).unwrap();
        if let AnimationState::Opening(progress) = mid_state {
            assert!(*progress > 0.0 && *progress < 1.0);
        } else {
            panic!("Animation should be Opening, but is {:?}", mid_state);
        }

        menu_system.update_animations(0.2);
        let final_state = menu_system.animation_states.get(&MenuCategoryId::File).unwrap();
        assert!(matches!(final_state, AnimationState::Open));
    }
}
