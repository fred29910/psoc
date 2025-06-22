//! Modern menu components for PSOC Image Editor
//! Implements the visual rendering of the menu system with advanced effects

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Padding, Point, Color,
};
use std::time::Instant;

use super::menu_system::{MenuCategory, MenuCategoryId, MenuItem, MenuSystem};
use crate::ui::icons::{icon_button, Icon};
use crate::ui::theme::{spacing, typography, PsocTheme, VisualStyle};
use crate::ui::animations::{MenuAnimationManager, TransitionType};
use crate::ui::styles::{VisualEffectStyle, GlassEffect, FrostedGlassStyle, ShadowConfig};

/// Messages for menu interactions
#[derive(Debug, Clone)]
pub enum MenuMessage {
    /// Open a menu category
    OpenMenu(MenuCategoryId),
    /// Close all menus
    CloseAllMenus,
    /// Select a menu item by ID
    SelectItem(String),
    /// Hover over a menu item
    HoverItem(MenuCategoryId, usize),
    /// Leave hover state
    LeaveHover,
    /// Update animations
    UpdateAnimations,
    /// Keyboard navigation messages
    KeyboardNavigation(KeyboardNavigationMessage),
}

/// Keyboard navigation messages for menus
#[derive(Debug, Clone)]
pub enum KeyboardNavigationMessage {
    /// Activate menu bar (Alt key)
    ActivateMenuBar,
    /// Navigate to next menu category
    NextCategory,
    /// Navigate to previous menu category
    PreviousCategory,
    /// Navigate to next menu item
    NextItem,
    /// Navigate to previous menu item
    PreviousItem,
    /// Select current focused item
    SelectFocused,
    /// Close menu and return focus
    EscapeMenu,
    /// Navigate to submenu
    EnterSubmenu,
    /// Exit submenu
    ExitSubmenu,
}

/// Enhanced menu state with visual effects
#[derive(Debug)]
pub struct EnhancedMenuState {
    /// Animation manager
    pub animation_manager: MenuAnimationManager,
    /// Hover states for smooth transitions
    pub hover_states: std::collections::HashMap<MenuCategoryId, f32>,
    /// Last update time for animations
    pub last_update: Instant,
    /// Keyboard navigation state
    pub keyboard_state: KeyboardNavigationState,
}

/// Keyboard navigation state for menus
#[derive(Debug, Clone)]
pub struct KeyboardNavigationState {
    /// Whether menu bar is activated via keyboard
    pub menu_bar_active: bool,
    /// Currently focused menu category
    pub focused_category: Option<MenuCategoryId>,
    /// Currently focused menu item index
    pub focused_item: Option<usize>,
    /// Whether we're in a submenu
    pub in_submenu: bool,
    /// Submenu navigation stack
    pub submenu_stack: Vec<(MenuCategoryId, usize)>,
    /// Whether keyboard navigation is enabled
    pub enabled: bool,
}

impl Default for KeyboardNavigationState {
    fn default() -> Self {
        Self {
            menu_bar_active: false,
            focused_category: None,
            focused_item: None,
            in_submenu: false,
            submenu_stack: Vec::new(),
            enabled: true,
        }
    }
}

impl Default for EnhancedMenuState {
    fn default() -> Self {
        Self {
            animation_manager: MenuAnimationManager::new(),
            hover_states: std::collections::HashMap::new(),
            last_update: Instant::now(),
            keyboard_state: KeyboardNavigationState::default(),
        }
    }
}

impl EnhancedMenuState {
    /// Update animations and hover states
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Update animations
        let animations_active = self.animation_manager.update();

        // Update hover states (smooth transitions)
        let hover_speed = 8.0; // Hover transition speed
        for (_, hover_progress) in self.hover_states.iter_mut() {
            if *hover_progress > 0.0 {
                *hover_progress = (*hover_progress - delta * hover_speed).max(0.0);
            }
        }

        animations_active || self.hover_states.values().any(|&v| v > 0.0)
    }

    /// Start hover effect
    pub fn start_hover(&mut self, menu_id: MenuCategoryId) {
        self.hover_states.insert(menu_id, 1.0);
    }

    /// Get current hover intensity
    pub fn get_hover_intensity(&self, menu_id: MenuCategoryId) -> f32 {
        self.hover_states.get(&menu_id).copied().unwrap_or(0.0)
    }

    /// Handle keyboard navigation message
    pub fn handle_keyboard_navigation(&mut self, message: KeyboardNavigationMessage, menu_system: &mut MenuSystem<crate::ui::application::Message>) -> bool {
        if !self.keyboard_state.enabled {
            return false;
        }

        match message {
            KeyboardNavigationMessage::ActivateMenuBar => {
                self.keyboard_state.menu_bar_active = true;
                self.keyboard_state.focused_category = menu_system.categories.first().map(|c| c.id);
                true
            }
            KeyboardNavigationMessage::NextCategory => {
                if let Some(current) = self.keyboard_state.focused_category {
                    if let Some(current_index) = menu_system.categories.iter().position(|c| c.id == current) {
                        let next_index = (current_index + 1) % menu_system.categories.len();
                        self.keyboard_state.focused_category = Some(menu_system.categories[next_index].id);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            KeyboardNavigationMessage::PreviousCategory => {
                if let Some(current) = self.keyboard_state.focused_category {
                    if let Some(current_index) = menu_system.categories.iter().position(|c| c.id == current) {
                        let prev_index = if current_index == 0 {
                            menu_system.categories.len() - 1
                        } else {
                            current_index - 1
                        };
                        self.keyboard_state.focused_category = Some(menu_system.categories[prev_index].id);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            KeyboardNavigationMessage::SelectFocused => {
                if let Some(focused_category) = self.keyboard_state.focused_category {
                    if self.keyboard_state.focused_item.is_none() {
                        // Open the focused menu
                        menu_system.open_menu(focused_category);
                        self.keyboard_state.focused_item = Some(0);
                        true
                    } else {
                        // Select the focused item
                        // This would trigger the menu item action
                        true
                    }
                } else {
                    false
                }
            }
            KeyboardNavigationMessage::EscapeMenu => {
                if self.keyboard_state.in_submenu {
                    self.keyboard_state.in_submenu = false;
                    self.keyboard_state.submenu_stack.pop();
                } else if self.keyboard_state.focused_item.is_some() {
                    self.keyboard_state.focused_item = None;
                    menu_system.close_all();
                } else {
                    self.keyboard_state.menu_bar_active = false;
                    self.keyboard_state.focused_category = None;
                }
                true
            }
            _ => false,
        }
    }

    /// Get currently focused category
    pub fn get_focused_category(&self) -> Option<MenuCategoryId> {
        if self.keyboard_state.menu_bar_active {
            self.keyboard_state.focused_category
        } else {
            None
        }
    }

    /// Get currently focused item index
    pub fn get_focused_item(&self) -> Option<usize> {
        if self.keyboard_state.menu_bar_active {
            self.keyboard_state.focused_item
        } else {
            None
        }
    }
}

/// Create enhanced menu bar with visual effects
pub fn enhanced_menu_bar<'a, Message: Clone + 'static>(
    menu_system: &'a MenuSystem<Message>,
    enhanced_state: &'a EnhancedMenuState,
    theme: &'a PsocTheme,
) -> Element<'a, MenuMessage> {
    let mut menu_buttons = Vec::new();

    for category in &menu_system.categories {
        let hover_intensity = enhanced_state.get_hover_intensity(category.id);
        let is_active = menu_system.active_menu == Some(category.id);

        // Create button with enhanced styling
        let button_style = if is_active {
            theme.enhanced_container_style(VisualStyle::Active)
        } else if hover_intensity > 0.0 {
            // Interpolate between normal and hover styles
            theme.enhanced_container_style(VisualStyle::Hover)
        } else {
            theme.enhanced_container_style(VisualStyle::None)
        };

        let menu_button = button(
            text(&category.title)
                .size(14.0)
                .color(theme.palette().text)
        )
        .on_press(MenuMessage::OpenMenu(category.id))
        .style(move |_theme, status| {
            match status {
                iced::widget::button::Status::Hovered => {
                    iced::widget::button::Style {
                        background: Some(theme.palette().menu_hover.into()),
                        text_color: theme.palette().text,
                        border: iced::Border {
                            color: theme.palette().tech_blue_alpha(0.3),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        shadow: iced::Shadow {
                            color: theme.palette().tech_blue_alpha(0.2),
                            offset: iced::Vector::new(0.0, 1.0),
                            blur_radius: 4.0,
                        },
                    }
                }
                iced::widget::button::Status::Pressed => {
                    iced::widget::button::Style {
                        background: Some(theme.palette().menu_active.into()),
                        text_color: theme.palette().text,
                        border: iced::Border {
                            color: theme.palette().tech_blue_alpha(0.5),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        shadow: iced::Shadow::default(),
                    }
                }
                _ => {
                    iced::widget::button::Style {
                        background: if is_active {
                            Some(theme.palette().menu_active.into())
                        } else {
                            Some(Color::TRANSPARENT.into())
                        },
                        text_color: theme.palette().text,
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    }
                }
            }
        })
        .padding(Padding::from([8, 16]));

        menu_buttons.push(menu_button.into());
    }

    // Create menu bar container with frosted glass effect
    let glass_effect = GlassEffect::frosted(FrostedGlassStyle::Subtle, theme);
    let menu_bar_style = glass_effect.to_container_style();

    container(
        row(menu_buttons)
            .spacing(4.0)
            .padding(Padding::from([4, 8]))
    )
    .style(move |_theme| menu_bar_style.clone())
    .width(Length::Fill)
    .into()
}

/// Create enhanced dropdown menu with visual effects
pub fn enhanced_dropdown_menu<'a, Message: Clone + 'static>(
    category: &'a MenuCategory<Message>,
    enhanced_state: &'a EnhancedMenuState,
    theme: &'a PsocTheme,
    position: Point,
) -> Element<'a, MenuMessage> {
    let mut menu_items = Vec::new();

    for (index, item) in category.items.iter().enumerate() {
        if item.is_separator {
            // Separator with subtle styling
            let separator = container(
                Space::new(Length::Fill, Length::Fixed(1.0))
            )
            .style(move |_theme| {
                iced::widget::container::Style {
                    background: Some(theme.palette().menu_separator().into()),
                    ..Default::default()
                }
            })
            .width(Length::Fill)
            .padding(Padding::from([4, 16]));

            menu_items.push(separator.into());
        } else {
            // Regular menu item with hover effects
            let mut item_content = vec![
                text(&item.label)
                    .size(13.0)
                    .color(if item.is_enabled {
                        theme.palette().text
                    } else {
                        theme.palette().text_secondary
                    })
                    .into()
            ];

            // Add shortcut text if available
            if let Some(ref shortcut) = item.shortcut {
                item_content.push(Space::new(Length::Fill, Length::Shrink).into());
                item_content.push(
                    text(shortcut)
                        .size(11.0)
                        .color(theme.palette().text_secondary)
                        .into()
                );
            }

            let menu_item = button(
                row(item_content)
                    .align_y(iced::alignment::Vertical::Center)
                    .spacing(8.0)
            )
            .on_press_maybe(if item.is_enabled {
                Some(MenuMessage::SelectItem(item.id.clone()))
            } else {
                None
            })
            .style(move |_theme, status| {
                match status {
                    iced::widget::button::Status::Hovered if item.is_enabled => {
                        let hover_effect = VisualEffectStyle::menu_item_hover(theme);
                        iced::widget::button::Style {
                            background: Some(theme.palette().menu_hover.into()),
                            text_color: theme.palette().text,
                            border: iced::Border {
                                color: theme.palette().tech_blue_alpha(0.2),
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            shadow: iced::Shadow::default(),
                        }
                    }
                    iced::widget::button::Status::Pressed if item.is_enabled => {
                        iced::widget::button::Style {
                            background: Some(theme.palette().menu_active.into()),
                            text_color: theme.palette().text,
                            border: iced::Border {
                                color: theme.palette().tech_blue_alpha(0.3),
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            shadow: iced::Shadow::default(),
                        }
                    }
                    _ => {
                        iced::widget::button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            text_color: if item.is_enabled {
                                theme.palette().text
                            } else {
                                theme.palette().text_secondary
                            },
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }
                    }
                }
            })
            .width(Length::Fill)
            .padding(Padding::from([8, 16]));

            menu_items.push(menu_item.into());
        }
    }

    // Apply animation state if available
    let animation_state = enhanced_state.animation_manager.get_current_state(category.id);

    // Create dropdown container with frosted glass effect
    let _dropdown_effect = VisualEffectStyle::dropdown_menu(theme);
    let shadow_config = ShadowConfig::dropdown_menu(theme);

    let dropdown_style = iced::widget::container::Style {
        text_color: Some(theme.palette().text),
        background: Some(theme.palette().glass_bg.into()),
        border: iced::Border {
            color: theme.palette().tech_blue_alpha(0.1),
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: shadow_config.primary_iced_shadow(),
    };

    // Apply animation transform if available
    if let Some(state) = animation_state {
        // In a real implementation, we would apply transforms here
        // For now, we'll just apply opacity through the background color
        let animated_style = iced::widget::container::Style {
            text_color: Some(theme.palette().text),
            background: Some(Color::from_rgba(
                theme.palette().glass_bg.r,
                theme.palette().glass_bg.g,
                theme.palette().glass_bg.b,
                theme.palette().glass_bg.a * state.opacity,
            ).into()),
            border: iced::Border {
                color: Color::from_rgba(
                    theme.palette().tech_blue.r,
                    theme.palette().tech_blue.g,
                    theme.palette().tech_blue.b,
                    0.1 * state.opacity,
                ),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25 * state.opacity),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
        };

        container(
            column(menu_items)
                .spacing(2.0)
                .padding(Padding::from([8, 4]))
        )
        .style(move |_theme| animated_style.clone())
        .width(Length::Fixed(220.0))
        .into()
    } else {
        container(
            column(menu_items)
                .spacing(2.0)
                .padding(Padding::from([8, 4]))
        )
        .style(move |_theme| dropdown_style.clone())
        .width(Length::Fixed(220.0))
        .into()
    }
}

/// Create the top-level menu bar
pub fn menu_bar<Message: Clone + 'static>(
    menu_system: &MenuSystem<Message>,
    theme: PsocTheme,
) -> Element<MenuMessage> {
    let palette = theme.palette();
    let background_color = palette.background;
    let border_color = palette.border;
    let default_text_color = palette.text;

    let menu_items: Vec<Element<MenuMessage>> = menu_system
        .categories
        .iter()
        .map(|category| {
            let is_active = menu_system.is_menu_open(category.id);
            let text_color = if is_active {
                iced::Color::WHITE
            } else {
                palette.text
            };

            let _button_style = if is_active {
                button::primary
            } else {
                button::secondary
            };

            button(
                text(&category.title)
                    .size(14.0) // typography::NORMAL
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(text_color),
                    })
            )
            .on_press(MenuMessage::OpenMenu(category.id))
            .padding(Padding::from([8.0, 12.0])) // spacing::SM, spacing::MD
            .style(button::text)
            .into()
        })
        .collect();

    let menu_row = row(menu_items)
        .spacing(4.0) // spacing::XS
        .align_y(iced::alignment::Vertical::Center);

    container(menu_row)
        .padding(8.0) // spacing::SM
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            text_color: Some(default_text_color),
            background: Some(background_color.into()),
            border: iced::Border {
                color: border_color,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow::default(),
        })
        .into()
}

/// Create a dropdown menu for a category
pub fn dropdown_menu<Message: Clone + 'static>(
    category: &MenuCategory<Message>,
    theme: PsocTheme,
) -> Element<MenuMessage> {
    let palette = theme.palette();

    let menu_items: Vec<Element<MenuMessage>> = category
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            if item.is_separator {
                // Create separator
                container(Space::new(Length::Fill, Length::Fixed(1.0)))
                    .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                        text_color: None,
                        background: Some(palette.border.into()),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    })
                    .height(Length::Fixed(1.0))
                    .width(Length::Fill)
                    .into()
            } else {
                // Create menu item
                let item_content = row![
                    // Icon (if present)
                    if let Some(icon) = item.icon {
                        {
                            let icon_container: Element<MenuMessage> = container(
                                text("●") // Placeholder for icon
                                    .size(typography::SMALL)
                            )
                            .width(Length::Fixed(20.0))
                            .into();
                            icon_container
                        }
                    } else {
                        Space::new(Length::Fixed(20.0), Length::Shrink).into()
                    },
                    // Label
                    text(&item.label)
                        .size(typography::NORMAL)
                        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(if item.is_enabled {
                                palette.text
                            } else {
                                palette.text_secondary
                            }),
                        }),
                    // Spacer
                    Space::new(Length::Fill, Length::Shrink),
                    // Shortcut (if present)
                    if let Some(shortcut) = &item.shortcut {
                        {
                            let shortcut_text: Element<MenuMessage> = text(shortcut)
                                .size(typography::SMALL)
                                .into();
                            shortcut_text
                        }
                    } else {
                        Space::new(Length::Shrink, Length::Shrink).into()
                    },
                ]
                .spacing(spacing::SM)
                .align_y(iced::alignment::Vertical::Center);

                let menu_item_button = if item.is_enabled && item.action.is_some() {
                    button(item_content)
                        .on_press(MenuMessage::SelectItem(item.id.clone()))
                        .width(Length::Fill)
                        .padding(Padding::from([spacing::SM, spacing::MD]))
                        .style(button::text)
                } else {
                    button(item_content)
                        .width(Length::Fill)
                        .padding(Padding::from([spacing::SM, spacing::MD]))
                        .style(button::text)
                };

                container(menu_item_button)
                    .width(Length::Fill)
                    .into()
            }
        })
        .collect();

    let menu_column = column(menu_items)
        .spacing(spacing::XS)
        .width(Length::Fixed(250.0));

    let surface_color = palette.surface;
    let border_color = palette.border;
    let shadow_color = palette.shadow;

    container(menu_column)
        .padding(8.0) // spacing::SM
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            text_color: None,
            background: Some(surface_color.into()),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(), // spacing::SM
            },
            shadow: iced::Shadow {
                color: shadow_color,
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
        })
        .into()
}

/// Create the complete menu system view
pub fn menu_system_view<Message: Clone + 'static>(
    menu_system: &MenuSystem<Message>,
    theme: PsocTheme,
) -> Element<MenuMessage> {
    let menu_bar_element = menu_bar(menu_system, theme);

    // If there's an active menu, overlay the dropdown
    if let Some(active_category) = menu_system.active_category() {
        // For now, we'll stack the dropdown below the menu bar
        // In a real implementation, this would be positioned absolutely
        let dropdown = dropdown_menu(active_category, theme);
        
        column![menu_bar_element, dropdown]
            .spacing(0)
            .into()
    } else {
        menu_bar_element
    }
}

/// Helper function to create menu items with common patterns
pub mod menu_builders {
    use super::*;

    /// Create a standard menu item with icon and shortcut
    pub fn menu_item<Message>(
        id: impl Into<String>,
        label: impl Into<String>,
        action: Message,
        icon: Option<Icon>,
        shortcut: Option<impl Into<String>>,
    ) -> MenuItem<Message> {
        let mut item = MenuItem::new(id, label, action);
        
        if let Some(icon) = icon {
            item = item.with_icon(icon);
        }
        
        if let Some(shortcut) = shortcut {
            item = item.with_shortcut(shortcut);
        }
        
        item
    }

    /// Create a menu item with submenu
    pub fn submenu_item<Message>(
        id: impl Into<String>,
        label: impl Into<String>,
        submenu: Vec<MenuItem<Message>>,
        icon: Option<Icon>,
    ) -> MenuItem<Message> {
        MenuItem {
            id: id.into(),
            label: label.into(),
            action: None,
            icon,
            shortcut: None,
            submenu: Some(submenu),
            is_separator: false,
            is_enabled: true,
        }
    }
}
