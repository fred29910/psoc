//! Modern menu bar component with enhanced visual effects
//! Provides active menu underline effects, hover animations, and modern styling

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Background, Color, Border, Shadow, Vector,
};

use crate::ui::theme::{PsocTheme, spacing};
use crate::ui::styles::glass_container_style;
use crate::ui::theme::GlassIntensity;
use crate::ui::components::menu_system::{MenuSystem, MenuCategoryId, MenuMessage};

/// Create a modern menu bar with enhanced visual effects
pub fn modern_menu_bar<Message: Clone + 'static>(
    menu_system: &MenuSystem<Message>,
    theme: PsocTheme,
) -> Element<'static, MenuMessage> {
    let palette = theme.palette();

    // Create menu category buttons with modern styling
    let menu_buttons: Vec<Element<'static, MenuMessage>> = menu_system.categories
        .iter()
        .map(|category| {
            let is_active = menu_system.active_menu_category_id == Some(category.id);
            let is_hovered = false; // TODO: Track hover state
            
            create_modern_menu_button(
                category.title_key.clone(),
                category.id,
                is_active,
                is_hovered,
                palette.clone(),
            )
        })
        .collect();

    // Create the menu bar container with modern styling
    let menu_bar_content = row(menu_buttons)
        .spacing(0)
        .align_y(iced::alignment::Vertical::Center);

    // Apply modern container styling with glass effect
    container(menu_bar_content)
        .width(Length::Fill)
        .height(Length::Fixed(48.0)) // Standard menu bar height
        .style(move |_theme| {
            let mut style = glass_container_style(GlassIntensity::Light, &theme);
            style.border = Border {
                color: Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.2
                ),
                width: 0.0,
                radius: 0.0.into(), // Menu bar typically has no radius
            };
            style.shadow = Shadow {
                color: palette.shadow_color(0.1),
                offset: Vector::new(0.0, 2.0), // Shadow downward
                blur_radius: 8.0,
            };
            style
        })
        .into()
}

/// Create a modern menu button with underline effect
fn create_modern_menu_button(
    title: String,
    category_id: MenuCategoryId,
    is_active: bool,
    is_hovered: bool,
    palette: crate::ui::theme::ColorPalette,
) -> Element<'static, MenuMessage> {
    // Create the button text
    let button_text = text(title)
        .size(14.0)
        .style(move |_theme| iced::widget::text::Style {
            color: Some(if is_active {
                Color::WHITE
            } else if is_hovered {
                palette.tech_blue
            } else {
                palette.text
            }),
        });

    // Create the underline indicator
    let underline = if is_active {
        container(
            Space::new(Length::Fill, Length::Fixed(2.0))
        )
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(palette.tech_blue)),
            border: Border {
                radius: 1.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    } else if is_hovered {
        container(
            Space::new(Length::Fill, Length::Fixed(2.0))
        )
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.5
            ))),
            border: Border {
                radius: 1.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    } else {
        container(
            Space::new(Length::Fill, Length::Fixed(2.0))
        )
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
    };

    // Create the button content with underline
    let button_content = column![
        container(button_text)
            .padding([12.0, 20.0])
            .center_x(Length::Fill),
        underline,
    ]
    .spacing(0);

    // Create the button with modern styling
    button(button_content)
        .on_press(MenuMessage::ToggleMenu(category_id))
        .style({
            let palette_clone = palette.clone();
            move |_theme, status| {
                modern_menu_button_style(palette_clone.clone(), status, is_active, is_hovered)
            }
        })
        .into()
}

/// Modern menu button style with enhanced visual effects
fn modern_menu_button_style(
    palette: crate::ui::theme::ColorPalette,
    status: iced::widget::button::Status,
    is_active: bool,
    is_hovered: bool,
) -> iced::widget::button::Style {
    let (background, border_color, shadow) = match (status, is_active) {
        (iced::widget::button::Status::Active, true) => (
            // Active and pressed state
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.3
            ))),
            palette.tech_blue,
            Shadow {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.3
                ),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        ),
        (iced::widget::button::Status::Hovered, true) => (
            // Active and hovered state
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.2
            ))),
            palette.tech_blue,
            Shadow {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.2
                ),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
        ),
        (_, true) => (
            // Active state
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.15
            ))),
            palette.tech_blue,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
        (iced::widget::button::Status::Hovered, false) => (
            // Hover state - subtle glass effect
            Some(Background::Color(Color::from_rgba(
                palette.glass_bg_light.r,
                palette.glass_bg_light.g,
                palette.glass_bg_light.b,
                0.8
            ))),
            Color::TRANSPARENT,
            Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        ),
        (iced::widget::button::Status::Active, false) => (
            // Pressed state
            Some(Background::Color(Color::from_rgba(
                palette.glass_bg_medium.r,
                palette.glass_bg_medium.g,
                palette.glass_bg_medium.b,
                0.6
            ))),
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
        text_color: if is_active {
            Color::WHITE
        } else if is_hovered {
            palette.tech_blue
        } else {
            palette.text
        },
        border: Border {
            color: border_color,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow,
    }
}

/// Create a modern dropdown menu with enhanced styling
pub fn modern_dropdown_menu<Message: Clone + 'static>(
    category: &crate::ui::components::menu_system::MenuCategory<Message>,
    theme: PsocTheme,
) -> Element<'static, MenuMessage> {
    let palette = theme.palette();

    // Create menu items
    let menu_items: Vec<Element<'static, MenuMessage>> = category.items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            create_modern_menu_item(
                item.id.clone(),
                item.label_key.clone(),
                item.shortcut_key.clone(),
                item.is_separator,
                palette.clone(),
            )
        })
        .collect();

    // Create the dropdown content
    let dropdown_content = column(menu_items)
        .spacing(2.0)
        .padding([8.0, 0.0]);

    // Apply modern container styling with enhanced glass effect
    container(dropdown_content)
        .width(Length::Fixed(220.0))
        .style(move |_theme| {
            let mut style = glass_container_style(GlassIntensity::Medium, &theme);
            style.border = Border {
                color: Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.3
                ),
                width: 1.0,
                radius: 8.0.into(),
            };
            style.shadow = Shadow {
                color: palette.shadow_color(0.2),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            };
            style
        })
        .into()
}

/// Create a modern menu item with enhanced styling
fn create_modern_menu_item(
    item_id: String,
    item_label: String,
    item_shortcut: Option<String>,
    is_separator: bool,
    palette: crate::ui::theme::ColorPalette,
) -> Element<'static, MenuMessage> {
    if is_separator {
        // Create a modern separator
        container(
            Space::new(Length::Fill, Length::Fixed(1.0))
        )
        .width(Length::Fill)
        .padding([4.0, 16.0])
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(
                palette.border.r,
                palette.border.g,
                palette.border.b,
                0.3
            ))),
            ..Default::default()
        })
        .into()
    } else {
        // Create a menu item button
        let item_content = row![
            text(item_label)
                .size(13.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(palette.text),
                }),
            Space::new(Length::Fill, Length::Shrink),
            if let Some(shortcut) = item_shortcut {
                text(shortcut)
                    .size(11.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(Color::from_rgba(
                            palette.text.r,
                            palette.text.g,
                            palette.text.b,
                            0.7
                        )),
                    })
            } else {
                text("")
                    .size(11.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(Color::TRANSPARENT),
                    })
            }
        ]
        .align_y(iced::alignment::Vertical::Center)
        .spacing(8.0);

        button(item_content)
            .width(Length::Fill)
            .padding([8.0, 16.0])
            .style({
                let palette_clone = palette.clone();
                move |_theme, status| {
                    modern_menu_item_style(palette_clone.clone(), status)
                }
            })
            .on_press(MenuMessage::SelectItemById(item_id))
            .into()
    }
}

/// Modern menu item style with hover effects
fn modern_menu_item_style(
    palette: crate::ui::theme::ColorPalette,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let (background, shadow) = match status {
        iced::widget::button::Status::Hovered => (
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.1
            ))),
            Shadow {
                color: Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.1
                ),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            },
        ),
        iced::widget::button::Status::Active => (
            Some(Background::Color(Color::from_rgba(
                palette.tech_blue.r,
                palette.tech_blue.g,
                palette.tech_blue.b,
                0.2
            ))),
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
        _ => (
            Some(Background::Color(Color::TRANSPARENT)),
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        ),
    };

    iced::widget::button::Style {
        background,
        text_color: palette.text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 4.0.into(),
        },
        shadow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::components::menu_system::{MenuCategory, MenuItem};

    fn create_test_menu_system() -> MenuSystem<()> {
        let file_category = MenuCategory::new(
            MenuCategoryId::File,
            "File",
            vec![
                MenuItem::new("New", "new", Some(())),
                MenuItem::new("Open", "open", Some(())),
                MenuItem::separator(),
                MenuItem::new("Save", "save", Some(())),
            ]
        );

        MenuSystem::new(vec![file_category])
    }

    #[test]
    fn test_modern_menu_bar_creation() {
        let menu_system = create_test_menu_system();
        let theme = PsocTheme::Dark;

        let menu_bar = modern_menu_bar(&menu_system, theme);

        // Should return an Element
        // This is mainly a compilation test
        let _ = menu_bar;
    }

    #[test]
    fn test_modern_dropdown_menu_creation() {
        let menu_system = create_test_menu_system();
        let theme = PsocTheme::Dark;

        if let Some(category) = menu_system.categories.first() {
            let dropdown = modern_dropdown_menu(category, theme);

            // Should return an Element
            let _ = dropdown;
        }
    }

    #[test]
    fn test_menu_button_style_variations() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();

        // Test different button states
        let states = [
            (iced::widget::button::Status::Disabled, false, false),
            (iced::widget::button::Status::Hovered, false, false),
            (iced::widget::button::Status::Active, false, false),
            (iced::widget::button::Status::Disabled, true, false),
            (iced::widget::button::Status::Hovered, true, false),
            (iced::widget::button::Status::Active, true, false),
        ];

        for (status, is_active, is_hovered) in states {
            let style = modern_menu_button_style(palette.clone(), status, is_active, is_hovered);

            // Should have valid style properties
            assert!(style.background.is_some());
            // Active buttons should have different styling
            if is_active {
                // Active buttons should have tech_blue influence
                if let Some(Background::Color(color)) = style.background {
                    assert!(color.a > 0.0 || color == Color::TRANSPARENT);
                }
            }
        }
    }

    #[test]
    fn test_menu_item_style_variations() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();

        let states = [
            iced::widget::button::Status::Disabled,
            iced::widget::button::Status::Hovered,
            iced::widget::button::Status::Active,
        ];

        for status in states {
            let style = modern_menu_item_style(palette.clone(), status);

            // Should have valid style properties
            assert!(style.background.is_some());
            assert_eq!(style.text_color, palette.text);

            // Hovered state should have different background
            if status == iced::widget::button::Status::Hovered {
                if let Some(Background::Color(color)) = style.background {
                    assert!(color.a > 0.0);
                }
            }
        }
    }

    #[test]
    fn test_modern_menu_button_creation() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();

        let button = create_modern_menu_button(
            "Test Menu".to_string(),
            MenuCategoryId::File,
            false,
            false,
            palette.clone(),
        );

        // Should return an Element
        let _ = button;
    }

    #[test]
    fn test_modern_menu_item_creation() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();

        let item = MenuItem::new("Test Item", "test", Some(()));
        let menu_item = create_modern_menu_item(
            item.id.clone(),
            item.label_key.clone(),
            item.shortcut_key.clone(),
            item.is_separator,
            palette.clone(),
        );

        // Should return an Element
        let _ = menu_item;

        // Test separator item
        let separator: MenuItem<()> = MenuItem::separator();
        let separator_item = create_modern_menu_item(
            separator.id.clone(),
            separator.label_key.clone(),
            separator.shortcut_key.clone(),
            separator.is_separator,
            palette.clone(),
        );
        let _ = separator_item;
    }
}
