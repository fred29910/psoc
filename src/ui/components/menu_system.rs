//! Core menu system for PSOC Image Editor
//! Implements Office-style dropdown menu system

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Point,
};
use std::collections::HashMap;

use crate::i18n::{t, Language};
use crate::ui::icons::Icon;
use crate::ui::theme::{spacing, typography, PsocTheme};

/// Menu category identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuCategoryId {
    File,      // 文件
    Edit,      // 编辑
    Image,     // 图像
    Layer,     // 图层
    Text,      // 文字
    Select,    // 选择
    Filter,    // 滤镜
    View,      // 视图
    Window,    // 窗口
    Help,      // 帮助
}

impl MenuCategoryId {
    /// Get all menu categories in order
    pub fn all() -> Vec<Self> {
        vec![
            Self::File,
            Self::Edit,
            Self::Image,
            Self::Layer,
            Self::Text,
            Self::Select,
            Self::Filter,
            Self::View,
            Self::Window,
            Self::Help,
        ]
    }

    /// Get the localized title for this menu category
    pub fn title(self) -> String {
        match self {
            Self::File => t("menu-file"),
            Self::Edit => t("menu-edit"),
            Self::Image => t("menu-image"),
            Self::Layer => t("menu-layer"),
            Self::Text => t("menu-text"),
            Self::Select => t("menu-select"),
            Self::Filter => t("menu-filter"),
            Self::View => t("menu-view"),
            Self::Window => t("menu-window"),
            Self::Help => t("menu-help"),
        }
    }
}

/// Menu item definition
#[derive(Debug, Clone)]
pub struct MenuItem<Message> {
    /// Unique identifier for the menu item
    pub id: String,
    /// Display label (localized)
    pub label: String,
    /// Action to trigger when clicked
    pub action: Option<Message>,
    /// Optional icon
    pub icon: Option<Icon>,
    /// Optional keyboard shortcut display
    pub shortcut: Option<String>,
    /// Optional submenu items
    pub submenu: Option<Vec<MenuItem<Message>>>,
    /// Whether this is a separator
    pub is_separator: bool,
    /// Whether this item is enabled
    pub is_enabled: bool,
}

impl<Message> MenuItem<Message> {
    /// Create a new menu item
    pub fn new(id: impl Into<String>, label: impl Into<String>, action: Message) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            action: Some(action),
            icon: None,
            shortcut: None,
            submenu: None,
            is_separator: false,
            is_enabled: true,
        }
    }

    /// Create a separator menu item
    pub fn separator() -> Self {
        Self {
            id: "separator".to_string(),
            label: String::new(),
            action: None,
            icon: None,
            shortcut: None,
            submenu: None,
            is_separator: true,
            is_enabled: true,
        }
    }

    /// Add an icon to this menu item
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Add a keyboard shortcut display
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Add a submenu
    pub fn with_submenu(mut self, submenu: Vec<MenuItem<Message>>) -> Self {
        self.submenu = Some(submenu);
        self
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = enabled;
        self
    }
}

/// Menu category definition
#[derive(Debug, Clone)]
pub struct MenuCategory<Message> {
    /// Category identifier
    pub id: MenuCategoryId,
    /// Display title (localized)
    pub title: String,
    /// Menu items in this category
    pub items: Vec<MenuItem<Message>>,
    /// Position for dropdown menu
    pub position: Point,
    /// Whether the dropdown is currently open
    pub is_open: bool,
}

impl<Message> MenuCategory<Message> {
    /// Create a new menu category
    pub fn new(id: MenuCategoryId, items: Vec<MenuItem<Message>>) -> Self {
        Self {
            id,
            title: id.title(),
            items,
            position: Point::ORIGIN,
            is_open: false,
        }
    }
}

/// Animation state for menu transitions
#[derive(Debug, Clone, Copy)]
pub enum AnimationState {
    /// Menu is closed
    Closed,
    /// Menu is opening (progress 0.0 to 1.0)
    Opening(f32),
    /// Menu is fully open
    Open,
    /// Menu is closing (progress 1.0 to 0.0)
    Closing(f32),
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::Closed
    }
}

/// Main menu system state
#[derive(Debug, Clone)]
pub struct MenuSystem<Message> {
    /// All menu categories
    pub categories: Vec<MenuCategory<Message>>,
    /// Currently active (open) menu
    pub active_menu: Option<MenuCategoryId>,
    /// Currently hovered item (category_index, item_index)
    pub hover_item: Option<(usize, usize)>,
    /// Animation states for each menu
    pub animation_states: HashMap<MenuCategoryId, AnimationState>,
    /// Menu positions for dropdowns
    pub menu_positions: HashMap<MenuCategoryId, Point>,
}

impl<Message> MenuSystem<Message> {
    /// Create a new menu system
    pub fn new(categories: Vec<MenuCategory<Message>>) -> Self {
        let mut animation_states = HashMap::new();
        let mut menu_positions = HashMap::new();

        for category in &categories {
            animation_states.insert(category.id, AnimationState::Closed);
            menu_positions.insert(category.id, Point::ORIGIN);
        }

        Self {
            categories,
            active_menu: None,
            hover_item: None,
            animation_states,
            menu_positions,
        }
    }

    /// Open a menu category
    pub fn open_menu(&mut self, category_id: MenuCategoryId) {
        // Close any currently open menu
        if let Some(active) = self.active_menu {
            self.animation_states.insert(active, AnimationState::Closing(1.0));
        }

        // Open the new menu
        self.active_menu = Some(category_id);
        self.animation_states.insert(category_id, AnimationState::Opening(0.0));

        // Update category state
        for category in &mut self.categories {
            category.is_open = category.id == category_id;
        }
    }

    /// Close all menus
    pub fn close_all(&mut self) {
        if let Some(active) = self.active_menu {
            self.animation_states.insert(active, AnimationState::Closing(1.0));
        }
        self.active_menu = None;
        self.hover_item = None;

        for category in &mut self.categories {
            category.is_open = false;
        }
    }

    /// Check if a menu is open
    pub fn is_menu_open(&self, category_id: MenuCategoryId) -> bool {
        self.active_menu == Some(category_id)
    }

    /// Get the currently active menu category
    pub fn active_category(&self) -> Option<&MenuCategory<Message>> {
        self.active_menu
            .and_then(|id| self.categories.iter().find(|cat| cat.id == id))
    }

    /// Update animation states
    pub fn update_animations(&mut self, delta_time: f32) {
        let animation_speed = 5.0; // Animation speed multiplier

        for (category_id, state) in self.animation_states.iter_mut() {
            match state {
                AnimationState::Opening(progress) => {
                    *progress += delta_time * animation_speed;
                    if *progress >= 1.0 {
                        *progress = 1.0;
                        *state = AnimationState::Open;
                    }
                }
                AnimationState::Closing(progress) => {
                    *progress -= delta_time * animation_speed;
                    if *progress <= 0.0 {
                        *progress = 0.0;
                        *state = AnimationState::Closed;
                        // If this was the active menu, clear it
                        if self.active_menu == Some(*category_id) {
                            self.active_menu = None;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
