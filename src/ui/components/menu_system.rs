// src/ui/components/menu_system.rs

//! Core components for the new Office-style menu system.

use iced::{Point, Element, Color}; // Removed Theme, Added Element, Color
use iced::widget::{container, text}; // Removed Element from here
use std::collections::HashMap;

// Attempt to use rand crate
// use rand::Rng; // Temporarily commented out to avoid build issues if crate not found by builder

use crate::ui::icons::Icon;
use crate::ui::theme::PsocTheme;
use crate::Message as AppMessageGlobal;
use crate::i18n::t;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuCategoryId {
    File, Edit, Image, Layer, Text, Select, Filter, View, Window, Help,
}

impl MenuCategoryId {
    pub fn all() -> Vec<Self> {
        vec![ Self::File, Self::Edit, Self::Image, Self::Layer, Self::Text, Self::Select, Self::Filter, Self::View, Self::Window, Self::Help]
    }
    // title() method removed, title_key will be used directly with t()
}

#[derive(Debug, Clone)]
pub struct MenuCategory<M> {
    pub id: MenuCategoryId,
    pub title_key: String, // Key for localization
    pub items: Vec<MenuItem<M>>,
    pub position: Point,
    pub is_open: bool,
}

impl<M> MenuCategory<M> {
    // Constructor updated to take title_key
    pub fn new(id: MenuCategoryId, title_key: &str, items: Vec<MenuItem<M>>) -> Self {
        Self {
            id,
            title_key: title_key.to_string(),
            items,
            position: Point::ORIGIN,
            is_open: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MenuItem<M> {
    pub id: String,
    pub label_key: String,
    pub action: Option<M>,
    pub icon: Option<Icon>,
    pub shortcut_key: Option<String>,
    pub submenu: Option<Vec<MenuItem<M>>>,
    pub is_separator: bool,
    pub is_enabled: bool,
}

impl<M> MenuItem<M> {
    pub fn new(id: &str, label_key: &str, action: Option<M>) -> Self { // action is Option<M>
        Self {
            id: id.to_string(),
            label_key: label_key.to_string(),
            action,
            icon: None,
            shortcut_key: None,
            submenu: None,
            is_separator: false,
            is_enabled: true,
        }
    }

    pub fn separator() -> Self {
        Self {
            id: "---SEPARATOR---".to_string(), // Fully static ID
            label_key: String::new(),
            action: None, icon: None, shortcut_key: None, submenu: None,
            is_separator: true, is_enabled: false,
        }
    }

    pub fn with_icon(mut self, icon: Icon) -> Self { self.icon = Some(icon); self }
    pub fn with_shortcut(mut self, shortcut_key: &str) -> Self { self.shortcut_key = Some(shortcut_key.to_string()); self }
    pub fn with_submenu(mut self, submenu: Vec<MenuItem<M>>) -> Self { self.submenu = Some(submenu); self }
    pub fn enabled(mut self, enabled: bool) -> Self { self.is_enabled = enabled; self } // Corrected from disabled()
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState { Closed, Opening(f32), Open, Closing(f32) }
impl Default for AnimationState { fn default() -> Self { Self::Closed } }

#[derive(Debug, Clone)]
pub struct MenuSystem<M> {
    pub categories: Vec<MenuCategory<M>>,
    pub active_menu_category_id: Option<MenuCategoryId>,
    pub focused_item_path: Option<Vec<usize>>,
    pub hover_item_path: Option<Vec<usize>>,
    pub animation_states: HashMap<MenuCategoryId, AnimationState>,
    pub menu_positions: HashMap<MenuCategoryId, Point>,
    pub focused_category_index: Option<usize>,
}

impl<M: Clone> MenuSystem<M> { // Added Clone bound for M due to item.action.clone()
    pub fn new(categories: Vec<MenuCategory<M>>) -> Self {
        let mut animation_states = HashMap::new();
        let mut menu_positions = HashMap::new();
        for category in &categories {
            animation_states.insert(category.id, AnimationState::default());
            menu_positions.insert(category.id, Point::ORIGIN);
        }
        Self {
            categories, active_menu_category_id: None, focused_item_path: None,
            hover_item_path: None, animation_states, menu_positions, focused_category_index: None,
        }
    }

    pub fn open_menu(&mut self, category_id: MenuCategoryId, category_idx: usize) {
        if let Some(active_id) = self.active_menu_category_id {
            if active_id != category_id {
                self.animation_states.insert(active_id, AnimationState::Closing(1.0));
            }
        }
        self.active_menu_category_id = Some(category_id);
        self.animation_states.insert(category_id, AnimationState::Opening(0.0));
        self.focused_item_path = Some(vec![category_idx, 0]);
        self.hover_item_path = None;
        for cat in &mut self.categories { cat.is_open = cat.id == category_id; }
    }

    pub fn toggle_menu(&mut self, category_id: MenuCategoryId, category_idx: usize) {
        if self.active_menu_category_id == Some(category_id) {
            self.close_active_menu();
        } else {
            self.open_menu(category_id, category_idx);
            self.focused_category_index = Some(category_idx);
        }
    }

    pub fn close_active_menu(&mut self) {
        if let Some(active_id) = self.active_menu_category_id.take() {
            self.animation_states.insert(active_id, AnimationState::Closing(1.0));
            if let Some(cat_idx) = self.categories.iter().position(|c| c.id == active_id) {
                 self.categories[cat_idx].is_open = false;
            }
        }
        self.focused_item_path = self.focused_category_index.map(|idx| vec![idx]);
        self.hover_item_path = None;
    }

    pub fn close_all_force(&mut self) {
        if let Some(active_id) = self.active_menu_category_id.take() {
            self.animation_states.insert(active_id, AnimationState::Closed);
            if let Some(cat_idx) = self.categories.iter().position(|c| c.id == active_id) {
                 self.categories[cat_idx].is_open = false;
            }
        }
        self.focused_item_path = None; self.hover_item_path = None; self.focused_category_index = None;
    }

    pub fn is_menu_open_or_opening(&self, category_id: MenuCategoryId) -> bool {
        match self.animation_states.get(&category_id) {
            Some(AnimationState::Opening(_)) | Some(AnimationState::Open) => true,
            _ => self.active_menu_category_id == Some(category_id),
        }
    }

    pub fn get_active_category(&self) -> Option<&MenuCategory<M>> {
        self.active_menu_category_id.and_then(|id| self.categories.iter().find(|cat| cat.id == id))
    }

    pub fn update_animations(&mut self, delta_time: f32) {
        let animation_speed = 5.0;
        for (category_id, state) in self.animation_states.iter_mut() {
            match state {
                AnimationState::Opening(progress) => {
                    *progress = (*progress + delta_time * animation_speed).min(1.0);
                    if *progress >= 1.0 { *state = AnimationState::Open; }
                }
                AnimationState::Closing(progress) => {
                    *progress = (*progress - delta_time * animation_speed).max(0.0);
                    if *progress <= 0.0 {
                        *state = AnimationState::Closed;
                        if self.active_menu_category_id == Some(*category_id) { // Should be already None if closed via close_active_menu
                           if let Some(idx) = self.categories.iter().position(|c| c.id == *category_id) {
                               self.categories[idx].is_open = false;
                           }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn focus_menu_bar(&mut self, focus_first_category: bool) {
        self.close_active_menu();
        if focus_first_category && !self.categories.is_empty() {
            self.focused_category_index = Some(0);
            self.focused_item_path = Some(vec![0]);
        } else {
            self.focused_category_index = None;
            self.focused_item_path = None;
        }
    }

    pub fn focus_next_menu_category(&mut self) {
        if self.categories.is_empty() { return; }
        let current_idx = self.focused_category_index.unwrap_or(self.categories.len() -1);
        let next_idx = (current_idx + 1) % self.categories.len();
        self.focused_category_index = Some(next_idx);
        self.focused_item_path = Some(vec![next_idx]);
        if self.active_menu_category_id.is_some() { // If a menu was open, switch to the new one
            self.open_menu(self.categories[next_idx].id, next_idx);
        }
    }

    pub fn focus_prev_menu_category(&mut self) {
        if self.categories.is_empty() { return; }
        let current_idx = self.focused_category_index.unwrap_or(0);
        let prev_idx = if current_idx == 0 { self.categories.len() - 1 } else { current_idx - 1 };
        self.focused_category_index = Some(prev_idx);
        self.focused_item_path = Some(vec![prev_idx]);
        if self.active_menu_category_id.is_some() { // If a menu was open, switch to the new one
            self.open_menu(self.categories[prev_idx].id, prev_idx);
        }
    }

    pub fn open_focused_menu_category_dropdown(&mut self) {
        if let Some(idx) = self.focused_category_index {
            if idx < self.categories.len() {
                let category_id = self.categories[idx].id;
                self.open_menu(category_id, idx);
                self.focus_item_in_current_dropdown(true); // Focus first valid item
            }
        }
    }

    fn get_menu_items_at_path(&self, path_prefix: &[usize]) -> Option<&Vec<MenuItem<M>>> {
        if path_prefix.is_empty() { return None; }
        let category = self.categories.get(path_prefix[0])?;
        if path_prefix.len() == 1 { return Some(&category.items); }
        let mut current_items = &category.items;
        for &index in &path_prefix[1..] {
            current_items = current_items.get(index)?.submenu.as_ref()?;
        }
        Some(current_items)
    }

    fn focus_item_in_current_dropdown(&mut self, forward: bool) {
        let current_path = match &self.focused_item_path {
            Some(p) => p.clone(),
            None => { // No focus path, try to set initial focus
                if let Some(active_cat_id) = self.active_menu_category_id {
                    if let Some(cat_idx) = self.categories.iter().position(|c| c.id == active_cat_id) {
                        if let Some(items) = self.get_menu_items_at_path(&[cat_idx]) {
                            if let Some(first_valid_idx) = items.iter().position(|item| !item.is_separator && item.is_enabled) {
                                self.focused_item_path = Some(vec![cat_idx, first_valid_idx]);
                            }
                        }
                    }
                }
                return;
            }
        };

        if current_path.len() < 2 { return; } // Path must be at least [cat_idx, item_idx]

        let item_list_path_prefix = &current_path[..current_path.len()-1];
        if let Some(items) = self.get_menu_items_at_path(item_list_path_prefix) {
            if items.is_empty() { return; }
            let mut current_item_idx = *current_path.last().unwrap();
            let initial_idx = current_item_idx;
            loop {
                if forward {
                    current_item_idx = (current_item_idx + 1) % items.len();
                } else {
                    current_item_idx = if current_item_idx == 0 { items.len() - 1 } else { current_item_idx - 1 };
                }
                if let Some(item) = items.get(current_item_idx) {
                    if !item.is_separator && item.is_enabled {
                        let mut new_path = item_list_path_prefix.to_vec();
                        new_path.push(current_item_idx);
                        self.focused_item_path = Some(new_path);
                        break;
                    }
                }
                if current_item_idx == initial_idx { break; } // Full circle, no valid item found
            }
        }
    }

    pub fn focus_next_item_in_dropdown(&mut self) { self.focus_item_in_current_dropdown(true); }
    pub fn focus_prev_item_in_dropdown(&mut self) { self.focus_item_in_current_dropdown(false); }

    pub fn get_focused_item(&self) -> Option<&MenuItem<M>> {
        let path = self.focused_item_path.as_ref()?;
        if path.len() < 2 { return None; } // Needs at least category and item index
        let category = self.categories.get(path[0])?;
        let mut current_items = &category.items;
        for &index in &path[1..path.len()-1] {
            current_items = current_items.get(index)?.submenu.as_ref()?;
        }
        current_items.get(*path.last().unwrap())
    }

    pub fn open_focused_submenu(&mut self) {
        let mut should_push_to_path = false;
        let mut first_valid_submenu_idx = 0;

        if let Some(item) = self.get_focused_item() {
            if let Some(submenu_items) = &item.submenu {
                if !submenu_items.is_empty() {
                    if let Some(idx) = submenu_items.iter().position(|si| !si.is_separator && si.is_enabled) {
                        should_push_to_path = true;
                        first_valid_submenu_idx = idx;
                    }
                }
            }
        }

        if should_push_to_path {
            if let Some(path) = self.focused_item_path.as_mut() {
                path.push(first_valid_submenu_idx);
            }
        }
    }

    pub fn close_submenu_or_active_menu(&mut self) {
        if let Some(path) = &mut self.focused_item_path {
            if path.len() > 2 { path.pop(); }
            else if path.len() == 2 || path.len() == 1 { self.close_active_menu(); }
        } else { self.close_active_menu(); }
    }
}

pub fn menu_system_view<'a, M: 'a + Clone>( // M is the app's main message type (AppMessageGlobal)
    menu_system: &'a MenuSystem<M>,      // MenuSystem stores actions of type M
    theme: PsocTheme,
) -> Element<'a, MenuMessage> { // This view function emits its own local MenuMessage
    use iced::widget::{button, row, text, container};
    use iced::{Background, Border, Color, Length, Shadow, Vector};

    let palette = theme.palette();

    // Create menu category buttons
    let menu_buttons: Vec<iced::Element<'_, MenuMessage>> = menu_system.categories
        .iter()
        .map(|category| {
            let is_active = menu_system.active_menu_category_id == Some(category.id);
            let palette_clone = palette.clone();

            button(
                text(&category.title_key)
                    .size(14.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(if is_active { Color::WHITE } else { palette_clone.text }),
                    })
            )
            .on_press(MenuMessage::ToggleMenu(category.id))
            .style(move |_theme, status| {
                let palette_clone2 = palette_clone.clone();
                modern_menu_button_style(&palette_clone2, status, is_active)
            })
            .padding([8.0, 16.0])
            .into()
        })
        .collect();

    // Create modern menu bar with glass effect
    let palette_for_container = palette.clone();
    container(
        row(menu_buttons)
            .spacing(0)
            .align_y(iced::alignment::Vertical::Center)
    )
    .width(Length::Fill)
    .height(Length::Fixed(40.0))
    .padding([0.0, 16.0])
    .style(move |_theme| {
        iced::widget::container::Style {
            text_color: Some(palette_for_container.text),
            background: Some(Background::Color(palette_for_container.glass_bg_heavy)),
            border: Border {
                color: Color::from_rgba(palette_for_container.border.r, palette_for_container.border.g, palette_for_container.border.b, 0.1),
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: palette_for_container.shadow_color(0.1),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    })
    .into()
}

/// Modern menu button styling
fn modern_menu_button_style(
    palette: &crate::ui::theme::ColorPalette,
    status: iced::widget::button::Status,
    is_active: bool,
) -> iced::widget::button::Style {
    use iced::{Background, Border, Color, Shadow, Vector};

    let (background, border_color, shadow) = match (status, is_active) {
        (_, true) => (
            // Active menu - tech blue background
            Some(Background::Color(palette.tech_blue_variant(20))),
            palette.tech_blue,
            Shadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 4.0,
            },
        ),
        (iced::widget::button::Status::Hovered, false) => (
            // Hover state - subtle glass effect
            Some(Background::Color(palette.glass_bg_light)),
            Color::TRANSPARENT,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
        _ => (
            // Default state - transparent
            Some(Background::Color(Color::TRANSPARENT)),
            Color::TRANSPARENT,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
    };

    iced::widget::button::Style {
        background,
        text_color: if is_active { Color::WHITE } else { palette.text },
        border: Border {
            color: border_color,
            width: if is_active { 0.0 } else { 0.0 },
            radius: 0.0.into(),
        },
        shadow,
    }
}

#[derive(Debug, Clone)]
pub enum MenuMessage { // This is crate::ui::components::menu_system::MenuMessage
    ToggleMenu(MenuCategoryId), CloseAllMenus, SelectItemById(String),
    HoverItemPath(Option<Vec<usize>>), FocusMenuBar, FocusNextMenuCategory,
    FocusPrevMenuCategory, OpenFocusedMenuCategory, FocusNextItem, FocusPrevItem,
    ActivateFocusedItem, OpenFocusedSubmenu, CloseSubmenuToParent, HandleEscape,
    UpdateAnimations,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuElement {
    TopBar, TopBarCategory(bool, bool), DropdownContainer, DropdownItem(bool, bool),
    DropdownSeparator, DropdownIcon, DropdownLabel, DropdownShortcut, SubmenuIndicator,
}

pub trait MenuStyleSheet {
    type Style: Default + Clone;
    fn appearance(&self, element: MenuElement) -> container::Style; // Changed Appearance to Style
}

#[derive(Default, Clone)]
pub struct MenuAppearance {} // Placeholder

impl MenuStyleSheet for PsocTheme {
    type Style = container::Style; // Use concrete type container::Style
    fn appearance(&self, element: MenuElement) -> container::Style { // Changed Appearance to Style
        let palette = self.palette();
        match element {
            MenuElement::TopBar => container::Style { background: Some(palette.dark_bg.into()), ..Default::default() },
            MenuElement::TopBarCategory(is_hovered, is_open) => {
                let bg = if is_open { palette.menu_active } else if is_hovered { palette.menu_hover } else { palette.dark_bg };
                container::Style { background: Some(bg.into()), ..Default::default() }
            }
            MenuElement::DropdownContainer => container::Style { background: Some(palette.dark_panel.into()), ..Default::default() },
            MenuElement::DropdownItem(is_hovered, is_enabled) => {
                let bg = if is_enabled && is_hovered { palette.menu_hover } else { Color::TRANSPARENT }; // iced::Color used here
                container::Style { background: Some(bg.into()), ..Default::default() }
            }
            MenuElement::DropdownSeparator => container::Style { background: Some(palette.border.into()), ..Default::default() },
            _ => Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::PsocTheme;

    #[test]
    fn test_modern_menu_button_style() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();

        // Test active menu button style
        let active_style = modern_menu_button_style(
            &palette,
            iced::widget::button::Status::Active,
            true
        );

        // Active button should have tech blue background
        assert!(active_style.background.is_some());
        assert_eq!(active_style.text_color, iced::Color::WHITE);

        // Test inactive button style
        let inactive_style = modern_menu_button_style(
            &palette,
            iced::widget::button::Status::Active,
            false
        );

        // Inactive button should have transparent background
        assert!(inactive_style.background.is_some());
        assert_eq!(inactive_style.text_color, palette.text);
    }

    #[test]
    fn test_modern_menu_button_hover_style() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();

        // Test hover state for inactive button
        let hover_style = modern_menu_button_style(
            &palette,
            iced::widget::button::Status::Hovered,
            false
        );

        // Hover button should have glass background
        assert!(hover_style.background.is_some());
    }

    #[test]
    fn test_menu_system_view_creation() {
        // Create a simple menu system for testing
        let categories: Vec<MenuCategory<()>> = vec![MenuCategory {
            id: MenuCategoryId::File,
            title_key: "File".to_string(),
            items: vec![],
            position: iced::Point::new(0.0, 0.0),
            is_open: false,
        }];
        let menu_system = MenuSystem::new(categories);

        let theme = PsocTheme::Dark;

        // Test that menu system view can be created
        let view = menu_system_view(&menu_system, theme);

        // Should return an Element
        // This is mainly a compilation test
        let _ = view;
    }
}
