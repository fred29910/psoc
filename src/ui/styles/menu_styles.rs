// src/ui/styles/menu_styles.rs

//! Defines the visual styling for the menu system components.

use iced::{Background, Border, Color, Shadow, Vector, theme::Container};
use iced::widget::container;

use crate::ui::theme::{PsocTheme, ColorPalette, VisualStyle as PsocVisualStyle}; // Renamed to avoid conflict
use crate::ui::components::menu_system::{MenuElement, MenuStyleSheet as MenuStyleSheetTrait};
use crate::ui::styles::visual_effects::{self, VisualEffectStyle};

// You can define a concrete struct that holds all style definitions if needed,
// or implement the trait directly on PsocTheme. The latter is chosen here
// to align with how it was provisionally defined in menu_system.rs.

impl MenuStyleSheetTrait for PsocTheme {
    type Style = Container; // iced::theme::Container is an alias for container::Style

    fn appearance(&self, element: MenuElement) -> container::Appearance {
        let palette = self.palette();
        let base_theme_style = self.enhanced_container_style(PsocVisualStyle::None); // Get a base style

        match element {
            MenuElement::TopBar => {
                container::Appearance {
                    background: Some(Background::Color(palette.dark_bg)), // Or surface
                    // No border for the top bar itself by default
                    ..base_theme_style
                }
            }
            MenuElement::TopBarCategory(is_hovered, is_open) => {
                let bg_color = if is_open {
                    palette.menu_active // A distinct color for an open category
                } else if is_hovered {
                    palette.menu_hover
                } else {
                    palette.dark_bg // Same as TopBar background, effectively transparent
                };
                container::Appearance {
                    background: Some(Background::Color(bg_color)),
                    text_color: Some(palette.text),
                    border: Border { // Subtle bottom border if open, as in design
                        color: if is_open { palette.tech_blue } else { Color::TRANSPARENT },
                        width: if is_open { 2.0 } else { 0.0 },
                        radius: 0.0.into(),
                    },
                    ..base_theme_style
                }
            }
            MenuElement::DropdownContainer => {
                // Use the frosted glass effect for dropdowns
                let effect = VisualEffectStyle::dropdown_menu(self);
                visual_effects::apply_visual_effects(&effect, Some(palette.glass_bg))
            }
            MenuElement::DropdownItem(is_hovered, is_enabled) => {
                let bg_color = if is_enabled && is_hovered {
                    palette.menu_hover // Use the menu_hover color
                } else {
                    Color::TRANSPARENT // Transparent background for items, relies on DropdownContainer bg
                };
                let text_color = if is_enabled {
                    palette.text
                } else {
                    palette.text_secondary // Muted color for disabled items
                };
                container::Appearance {
                    background: Some(Background::Color(bg_color)),
                    text_color: Some(text_color),
                    border: Border::default(), // No border for individual items by default
                    shadow: Shadow::default(),
                    ..base_theme_style
                }
            }
            MenuElement::DropdownSeparator => {
                container::Appearance {
                    background: Some(Background::Color(palette.menu_separator())),
                    // Separator is usually a thin line, height will be controlled by the widget using it
                    ..base_theme_style
                }
            }
            MenuElement::DropdownIcon => { // Style for the icon area within a menu item
                container::Appearance {
                    text_color: Some(palette.text_secondary), // Icons can be slightly muted or match text
                    ..base_theme_style
                }
            }
            MenuElement::DropdownLabel => { // Style for the label area
                container::Appearance {
                    // text_color is handled by DropdownItem based on enabled state
                    ..base_theme_style
                }
            }
            MenuElement::DropdownShortcut => { // Style for the shortcut text area
                container::Appearance {
                    text_color: Some(palette.text_secondary), // Shortcuts are often muted
                    ..base_theme_style
                }
            }
            MenuElement::SubmenuIndicator => { // Style for the ">" arrow for submenus
                 container::Appearance {
                    text_color: Some(palette.text_secondary), // Usually a muted color
                    ..base_theme_style
                }
            }
        }
    }

    // Example of how you might provide more specific styles if the trait were extended:
    // fn text_color(&self, element: MenuElement) -> Color {
    //     let palette = self.palette();
    //     match element {
    //         MenuElement::DropdownItem(_, is_enabled) => if is_enabled { palette.text } else { palette.text_secondary },
    //         MenuElement::DropdownShortcut => palette.text_secondary,
    //         _ => palette.text,
    //     }
    // }
}

// To make this usable, you might also want to define a concrete struct that wraps PsocTheme
// if you need to pass around a style sheet object explicitly, though the trait implementation
// on PsocTheme itself is often sufficient for iced.

pub struct PsocMenuStylesheet(pub PsocTheme);

impl container::StyleSheet for PsocMenuStylesheet {
    type Style = PsocTheme; // Using PsocTheme as the Style enum to select variants

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        // This implementation is a bit awkward because container::StyleSheet's `style`
        // argument is the same as `Self::Style`.
        // This would be better if MenuElement was part of the `style: &Self::Style` argument.
        // For now, let's assume a default element or make it configurable.
        // This part highlights a potential area for refactoring how styles are applied.
        // The MenuStyleSheetTrait approach is more direct for the menu system.

        // Fallback to a default appearance or a specific one.
        // This specific struct might not be the primary way styles are fetched if
        // the trait on PsocTheme is used directly by the view logic.
        self.0.appearance(MenuElement::DropdownContainer) // Default to DropdownContainer for example
    }
}

// Helper function to get a menu style (alternative to direct trait use, if preferred)
pub fn get_menu_appearance(theme: &PsocTheme, element: MenuElement) -> container::Appearance {
    theme.appearance(element)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::components::menu_system::MenuElement;
    use crate::ui::theme::PsocTheme;

    #[test]
    fn top_bar_appearance() {
        let theme = PsocTheme::Dark;
        let appearance = theme.appearance(MenuElement::TopBar);
        assert!(appearance.background.is_some());
        if let Some(Background::Color(color)) = appearance.background {
            assert_eq!(color, theme.palette().dark_bg);
        } else {
            panic!("Background was not a color");
        }
    }

    #[test]
    fn dropdown_container_appearance() {
        let theme = PsocTheme::Dark;
        let appearance = theme.appearance(MenuElement::DropdownContainer);
        // Check if it resembles a frosted glass style
        assert!(appearance.background.is_some());
        if let Some(Background::Color(color)) = appearance.background {
            // Expected to be glass_bg from the VisualEffectStyle::dropdown_menu
             assert_eq!(color.a, theme.palette().glass_bg.a, "Alpha channel for glass_bg mismatch");
        } else {
            panic!("Background was not a color");
        }
        assert_ne!(appearance.shadow.offset, Vector::default(), "Dropdown should have a shadow");
    }

    #[test]
    fn hovered_menu_item_appearance() {
        let theme = PsocTheme::Dark;
        let appearance = theme.appearance(MenuElement::DropdownItem(true, true)); // Hovered, enabled
        assert!(appearance.background.is_some());
        if let Some(Background::Color(color)) = appearance.background {
            assert_eq!(color, theme.palette().menu_hover);
        } else {
            panic!("Background was not a color");
        }
    }

    #[test]
    fn disabled_menu_item_text_color() {
        let theme = PsocTheme::Dark;
        let appearance = theme.appearance(MenuElement::DropdownItem(false, false)); // Not hovered, disabled
         assert!(appearance.text_color.is_some());
        if let Some(color) = appearance.text_color {
            assert_eq!(color, theme.palette().text_secondary);
        } else {
            panic!("Text color was None");
        }
    }
}
