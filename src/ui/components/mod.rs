//! UI Components module for PSOC Image Editor

use iced::{
    widget::{button, checkbox, column, container, row, slider, text, text_input, tooltip, Row, Space},
    Element, Length,
};

use super::icons::{icon_button, simple_icon_button, tool_button, Icon};
use crate::i18n::{t, Language};
use psoc_core::{HistoryEntry, RgbaPixel};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub mod menu_system;
// pub mod modern_menu; // Assuming this will be replaced or integrated with menu_system
pub mod menu_factory;
pub mod responsive_layout;
pub mod keyboard_navigation;
pub mod modern_layer_panel;
pub mod animated_tool_options;
pub mod modern_status_panel;
pub mod modern_color_panel;
pub mod modern_menu_bar;
pub mod modern_canvas_placeholder;
pub mod smart_panel_system;
pub mod adaptive_toolbar;

// Re-export main components from the new menu_system
pub use menu_system::{
    MenuCategory, MenuCategoryId, MenuItem, MenuSystem, MenuMessage, menu_system_view, // Added menu_system_view
    // AnimationState, // Will be re-added if used by menu_system.rs
};

// Re-export modern layer panel components
pub use modern_layer_panel::{
    ModernLayerInfo, LayerPanelMessage, modern_layer_card, modern_layer_panel,
};

// Re-export animated tool options components
pub use animated_tool_options::{
    animated_tool_options_panel,
};

// Re-export modern status panel components
pub use modern_status_panel::{
    ModernStatusInfo, modern_status_panel, animated_status_indicator,
};

// Re-export modern color panel components
pub use modern_color_panel::{
    ModernColorInfo, ColorPanelMessage, modern_color_panel,
};

// Re-export modern menu bar components
pub use modern_menu_bar::{
    modern_menu_bar, modern_dropdown_menu,
};

// Re-export modern canvas placeholder components
pub use modern_canvas_placeholder::{
    modern_canvas_placeholder, PlaceholderMessage,
};

// Re-export smart panel system components
pub use smart_panel_system::{
    SmartPanelSystem, SmartPanelConfig, PanelUsageStats, FoldingStrategy,
    PanelPriority, PanelFoldingAction,
};

// Re-export adaptive toolbar components
pub use adaptive_toolbar::{
    AdaptiveToolbar, AdaptiveToolbarConfig, ToolbarStrategy, ToolGroup,
};

// Commenting out old modern_menu exports, assuming they will be replaced.
// If modern_menu.rs is still needed and distinct, these can be reinstated or adapted.
// pub use modern_menu::{
//     dropdown_menu, menu_bar, menu_system_view,
//     enhanced_menu_bar, enhanced_dropdown_menu, EnhancedMenuState,
//     KeyboardNavigationMessage, KeyboardNavigationState,
// };

pub use responsive_layout::{
    ResponsiveLayoutManager, ResponsiveLayoutMessage, PanelId, PanelState, ScreenSize,
};
pub use keyboard_navigation::{
    KeyboardNavigationManager, KeyboardNavigationMessage as KbNavMessage,
    FocusTarget, NavigationAction, TabOrder,
};
pub use menu_factory::MenuFactory; // MenuFactory will need to be updated to use new structures

// Type aliases for complex layer information tuples
#[allow(clippy::type_complexity)]
type LayerInfoSimple<Message> = (
    String,               // name
    bool,                 // visible
    bool,                 // selected
    f32,                  // opacity
    psoc_core::BlendMode, // blend_mode
    String,               // layer_type
    bool,                 // has_mask
    Message,              // toggle_visibility
    Message,              // select_layer
);

/// Color history for tracking recently used colors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorHistory {
    colors: VecDeque<RgbaPixel>,
    max_size: usize,
}

impl Default for ColorHistory {
    fn default() -> Self {
        Self {
            colors: VecDeque::new(),
            max_size: 16,
        }
    }
}

impl ColorHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            colors: VecDeque::new(),
            max_size,
        }
    }

    pub fn add_color(&mut self, color: RgbaPixel) {
        // Remove if already exists
        self.colors.retain(|&c| c != color);

        // Add to front
        self.colors.push_front(color);

        // Trim to max size
        while self.colors.len() > self.max_size {
            self.colors.pop_back();
        }
    }

    pub fn colors(&self) -> &VecDeque<RgbaPixel> {
        &self.colors
    }
}

/// Create a modern vertical toolbar with tools and zoom controls
pub fn toolbar<Message: Clone + 'static>(
    tools: Vec<(Icon, Message, bool)>, // (icon, message, is_active)
    zoom_in: Message,
    zoom_out: Message,
    zoom_reset: Message,
) -> Element<'static, Message> {
    enhanced_toolbar_with_tooltips(tools, zoom_in, zoom_out, zoom_reset)
}

/// Create an enhanced toolbar with tooltips for better user experience
pub fn enhanced_toolbar_with_tooltips<Message: Clone + 'static>(
    tools: Vec<(Icon, Message, bool)>, // (icon, message, is_active)
    zoom_in: Message,
    zoom_out: Message,
    zoom_reset: Message,
) -> Element<'static, Message> {
    let tool_buttons: Vec<Element<Message>> = tools
        .into_iter()
        .map(|(icon, message, is_active)| {
            let tooltip_text = get_tool_tooltip_text(icon);
            modern_tool_button_with_tooltip(icon, message, is_active, Some(tooltip_text)).into()
        })
        .collect();

    // Create vertical tool column with modern spacing
    let tools_column = column(tool_buttons).spacing(6.0);

    // Create zoom controls in vertical layout
    let zoom_controls = column![
        Space::new(Length::Shrink, Length::Fixed(16.0)), // Spacer
        icon_button(Icon::ZoomIn, zoom_in),
        icon_button(Icon::ZoomReset, zoom_reset),
        icon_button(Icon::ZoomOut, zoom_out),
    ]
    .spacing(4.0);

    // Use modern container with glass effect
    modern_toolbar_container(
        column![tools_column, Space::new(Length::Shrink, Length::Fill), zoom_controls]
            .spacing(0)
            .align_x(iced::alignment::Horizontal::Center)
    )
}

/// Create a modern tool button with enhanced styling and tooltip
pub fn modern_tool_button<Message: Clone + 'static>(
    icon: Icon,
    message: Message,
    is_active: bool,
) -> Element<'static, Message> {
    modern_tool_button_with_tooltip(icon, message, is_active, None)
}

/// Create a modern tool button with enhanced styling and optional tooltip
pub fn modern_tool_button_with_tooltip<Message: Clone + 'static>(
    icon: Icon,
    message: Message,
    is_active: bool,
    tooltip_text: Option<&'static str>,
) -> Element<'static, Message> {
    let button_content = container(
        text(icon.unicode())
            .size(20.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(if is_active { iced::Color::WHITE } else { iced::Color::from_rgb(0.8, 0.8, 0.8) }),
            })
    )
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    let button_element = button(button_content)
        .on_press(message)
        .style(move |theme, status| {
            modern_tool_button_style(theme, status, is_active)
        });

    // Add tooltip if provided
    if let Some(tooltip_text) = tooltip_text {
        iced::widget::tooltip(
            button_element,
            tooltip_text,
            iced::widget::tooltip::Position::Right
        )
        .style(|_theme| {
            use crate::ui::theme::PsocTheme;
            use iced::{Background, Border, Color};

            let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
            let palette = psoc_theme.palette();

            iced::widget::container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(palette.glass_bg_heavy)),
                border: Border {
                    color: palette.tech_blue_variant(50),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        })
        .into()
    } else {
        button_element.into()
    }
}

/// Get tooltip text for tool icons
fn get_tool_tooltip_text(icon: Icon) -> &'static str {
    match icon {
        Icon::Select => "Selection Tool (V)",
        Icon::Brush => "Brush Tool (B)",
        Icon::Eraser => "Eraser Tool (E)",
        Icon::Move => "Move Tool (M)",
        Icon::Transform => "Transform Tool (T)",
        Icon::Gradient => "Gradient Tool (G)",
        Icon::Crop => "Crop Tool (C)",
        Icon::Eyedropper => "Eyedropper Tool (I)",
        Icon::Rectangle => "Rectangle Tool (R)",
        Icon::Ellipse => "Ellipse Tool (U)",
        Icon::Line => "Line Tool (L)",
        Icon::Polygon => "Polygon Tool (P)",
        Icon::ZoomIn => "Zoom In (+)",
        Icon::ZoomOut => "Zoom Out (-)",
        Icon::ZoomReset => "Reset Zoom (0)",
        _ => "Tool",
    }
}

/// Modern toolbar container with glass effect
pub fn modern_toolbar_container<Message: 'static>(
    content: impl Into<Element<'static, Message>>,
) -> Element<'static, Message> {
    container(content)
        .width(Length::Fixed(64.0)) // Vertical toolbar width
        .height(Length::Fill)
        .padding(8.0)
        .style(|theme| {
            use crate::ui::theme::PsocTheme;
            use crate::ui::styles::glass_container_style;
            use crate::ui::theme::GlassIntensity;

            let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
            glass_container_style(GlassIntensity::Heavy, &psoc_theme)
        })
        .into()
}

/// Enhanced modern tool button styling function with improved visual feedback
fn modern_tool_button_style(
    theme: &iced::Theme,
    status: iced::widget::button::Status,
    is_active: bool,
) -> iced::widget::button::Style {
    use iced::{Background, Border, Color, Shadow, Vector};
    use crate::ui::theme::PsocTheme;

    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    let palette = psoc_theme.palette();

    let (background, border_color, shadow, text_color) = match (status, is_active) {
        (iced::widget::button::Status::Hovered, true) => (
            // Active tool hovered - even brighter with pulsing effect
            Some(Background::Color(palette.tech_blue)),
            palette.tech_blue_variant(120),
            Shadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 16.0, // Enhanced glow on hover
            },
            Color::WHITE,
        ),
        (iced::widget::button::Status::Pressed, true) => (
            // Active tool pressed - slightly dimmed
            Some(Background::Color(palette.tech_blue_variant(70))),
            palette.tech_blue_variant(80),
            Shadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 8.0,
            },
            Color::WHITE,
        ),
        (_, true) => (
            // Active tool - enhanced tech blue background with stronger glow
            Some(Background::Color(palette.tech_blue_variant(90))),
            palette.tech_blue,
            Shadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 12.0, // Increased blur for more prominent glow
            },
            Color::WHITE,
        ),
        (iced::widget::button::Status::Hovered, false) => (
            // Inactive tool hovered - enhanced glass effect with tech blue hint
            Some(Background::Color(palette.glass_bg_medium)),
            palette.tech_blue_variant(60),
            Shadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 6.0, // Subtle glow hint
            },
            palette.tech_blue_variant(120),
        ),
        (iced::widget::button::Status::Pressed, false) => (
            // Inactive tool pressed - darker glass with tech blue accent
            Some(Background::Color(palette.glass_bg_heavy)),
            palette.tech_blue_variant(40),
            Shadow {
                color: palette.shadow_color(0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            palette.tech_blue_variant(100),
        ),
        _ => (
            // Default state - subtle glass background
            Some(Background::Color(palette.glass_bg_light)),
            Color::TRANSPARENT,
            Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
            palette.text,
        ),
    };

    iced::widget::button::Style {
        background,
        text_color,
        border: Border {
            color: border_color,
            width: if is_active { 2.0 } else { if matches!(status, iced::widget::button::Status::Hovered) { 1.0 } else { 0.0 } },
            radius: 8.0.into(),
        },
        shadow,
    }
}

/// Create a modern side panel with title and content using glass effect
pub fn side_panel<Message: Clone + 'static>(
    title: String,
    content: Vec<Element<'static, Message>>,
    width: f32,
) -> Element<'static, Message> {
    let header = container(
        text(title)
            .size(14.0)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::WHITE),
            })
    )
    .padding([12.0, 16.0])
    .style(|theme| {
        use crate::ui::theme::PsocTheme;
        use iced::{Background, Border, Color};

        let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
        let palette = psoc_theme.palette();

        iced::widget::container::Style {
            background: Some(Background::Color(palette.glass_bg_medium)),
            border: Border {
                color: Color::from_rgba(palette.border.r, palette.border.g, palette.border.b, 0.2),
                width: 0.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        }
    });

    let content_container = container(
        column(content).spacing(8.0).padding([8.0, 16.0])
    )
    .style(|theme| {
        use crate::ui::theme::PsocTheme;
        use crate::ui::styles::glass_container_style;
        use crate::ui::theme::GlassIntensity;
        use iced::{Border};

        let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
        let mut style = glass_container_style(GlassIntensity::Medium, &psoc_theme);

        // Adjust border radius for content area
        style.border = Border {
            radius: 12.0.into(),
            ..style.border
        };

        style
    });

    container(
        column![header, content_container]
    )
    .width(Length::Fixed(width))
    .into()
}

/// Create a section header
pub fn section_header<Message: 'static>(title: String) -> Element<'static, Message> {
    container(
        text(title)
            .size(12.0)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
            })
    )
    .padding([8.0, 4.0])
    .into()
}

/// Create a property row with label and value
pub fn property_row<Message: 'static>(label: String, value: String) -> Element<'static, Message> {
    container(
        row![
            text(label)
                .size(11.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                }),
            Space::new(Length::Fill, Length::Shrink),
            text(value)
                .size(11.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::WHITE),
                }),
        ]
        .spacing(8.0)
        .align_y(iced::alignment::Vertical::Center)
    )
    .padding([2.0, 8.0])
    .into()
}

/// Create a tool palette
pub fn tool_palette<Message: Clone + 'static>(
    tools: Vec<(Icon, Message, bool)>, // (icon, message, is_active)
) -> Element<'static, Message> {
    let tool_buttons: Vec<Element<Message>> = tools
        .into_iter()
        .map(|(icon, message, is_active)| {
            tool_button(icon, message, is_active).into()
        })
        .collect();

    column(tool_buttons).spacing(4.0).into()
}

/// Tool option control type
pub enum ToolOptionControl<Message> {
    Slider {
        label: String,
        value: f32,
        min: f32,
        max: f32,
        on_change: Box<dyn Fn(f32) -> Message>,
    },
    Checkbox {
        label: String,
        checked: bool,
        on_toggle: Message,
    },
    Text {
        label: String,
        value: String,
        on_change: Box<dyn Fn(String) -> Message>,
    },
}

/// Create a tool options panel
pub fn tool_options_panel<Message: Clone + 'static>(
    tool_name: String,
    controls: Vec<ToolOptionControl<Message>>,
) -> Element<'static, Message> {
    let mut elements = vec![
        section_header(format!("{} Options", tool_name))
    ];

    for control in controls {
        match control {
            ToolOptionControl::Slider { label, value, min, max, on_change: _ } => {
                elements.push(property_row(label, format!("{:.1}", value)));
            }
            ToolOptionControl::Checkbox { label, checked, on_toggle: _ } => {
                elements.push(property_row(label, if checked { "On" } else { "Off" }.to_string()));
            }
            ToolOptionControl::Text { label, value, on_change: _ } => {
                elements.push(property_row(label, value));
            }
        }
    }

    column(elements).spacing(4.0).into()
}

/// Status information for the status bar
pub struct StatusInfo {
    pub cursor_position: Option<(f32, f32)>,
    pub zoom_level: f32,
    pub document_size: Option<(u32, u32)>,
    pub tool_info: String,
}

/// Create an enhanced status bar
pub fn enhanced_status_bar<Message: 'static>(status_info: &StatusInfo) -> Element<Message> {
    let position_text = if let Some((x, y)) = status_info.cursor_position {
        format!("X: {:.0}, Y: {:.0}", x, y)
    } else {
        "No position".to_string()
    };

    let zoom_text = format!("Zoom: {:.0}%", status_info.zoom_level * 100.0);

    let size_text = if let Some((w, h)) = status_info.document_size {
        format!("Size: {}x{}", w, h)
    } else {
        "No document".to_string()
    };

    container(
        row![
            text(&status_info.tool_info).size(12.0),
            Space::new(Length::Fill, Length::Shrink),
            text(position_text).size(12.0),
            text(" | ").size(12.0),
            text(zoom_text).size(12.0),
            text(" | ").size(12.0),
            text(size_text).size(12.0),
        ]
        .spacing(8.0)
        .align_y(iced::alignment::Vertical::Center)
    )
    .padding(4.0)
    .into()
}

/// Layer information for the layer panel
pub type LayerInfo<Message> = (
    String,               // name
    bool,                 // visible
    bool,                 // selected
    f32,                  // opacity
    psoc_core::BlendMode, // blend_mode
    String,               // layer_type
    bool,                 // has_mask
    Message,              // toggle_visibility
    Message,              // select_layer
);

/// Create a layer panel
pub fn layer_panel<Message: Clone + 'static>(
    layers: Vec<LayerInfo<Message>>,
    add_layer: Message,
    delete_layer: Message,
    duplicate_layer: Message,
) -> Element<'static, Message> {
    let mut elements = vec![
        section_header("Layers".to_string()),
        container(
            row![
                icon_button(Icon::LayerAdd, add_layer),
                icon_button(Icon::LayerDelete, delete_layer),
                icon_button(Icon::LayerDuplicate, duplicate_layer),
            ]
            .spacing(4.0)
        )
        .padding(4.0)
        .into()
    ];

    for (name, visible, selected, opacity, _blend_mode, _layer_type, _has_mask, toggle_vis, select) in layers {
        let layer_row = container(
            row![
                button(if visible { "●" } else { "○" })
                    .on_press(toggle_vis)
                    .width(Length::Fixed(20.0)),
                button(text(name.clone()).size(11.0))
                    .on_press(select)
                    .width(Length::Fill),
                text(format!("{:.0}%", opacity * 100.0))
                    .size(10.0)
                    .width(Length::Fixed(40.0)),
            ]
            .spacing(4.0)
            .align_y(iced::alignment::Vertical::Center)
        )
        .padding(2.0)
        .style(move |_theme| {
            if selected {
                container::Style {
                    background: Some(iced::Color::from_rgba(0.0, 0.75, 1.0, 0.2).into()),
                    ..Default::default()
                }
            } else {
                container::Style::default()
            }
        });

        elements.push(layer_row.into());
    }

    column(elements).spacing(2.0).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::PsocTheme;

    #[test]
    fn test_modern_tool_button_style() {
        let theme = iced::Theme::Dark;

        // Test active button style
        let active_style = modern_tool_button_style(
            &theme,
            iced::widget::button::Status::Active,
            true
        );

        // Active button should have tech blue background
        assert!(active_style.background.is_some());

        // Test inactive button style
        let inactive_style = modern_tool_button_style(
            &theme,
            iced::widget::button::Status::Active,
            false
        );

        // Inactive button should have transparent background
        assert!(inactive_style.background.is_some());
    }

    #[test]
    fn test_modern_toolbar_container() {
        // Test that modern toolbar container can be created
        let content: iced::Element<'static, ()> = text("Test").into();
        let toolbar = modern_toolbar_container(content);

        // Should return an Element
        // This is mainly a compilation test
        let _ = toolbar;
    }

    #[test]
    fn test_modern_tool_button() {
        use crate::ui::icons::Icon;

        // Test that modern tool button can be created
        let button = modern_tool_button(Icon::Brush, (), true);

        // Should return an Element
        // This is mainly a compilation test
        let _ = button;
    }

    #[test]
    fn test_side_panel_creation() {
        // Test that side panel can be created with modern styling
        let content: Vec<iced::Element<'static, ()>> = vec![text("Test content").into()];
        let panel = side_panel::<()>("Test Panel".to_string(), content, 250.0);

        // Should return an Element
        // This is mainly a compilation test
        let _ = panel;
    }

    #[test]
    fn test_modern_tool_button_with_tooltip() {
        use crate::ui::icons::Icon;

        // Test that modern tool button with tooltip can be created
        let button = modern_tool_button_with_tooltip(Icon::Brush, (), true, Some("Test Tooltip"));

        // Should return an Element
        // This is mainly a compilation test
        let _ = button;
    }

    #[test]
    fn test_enhanced_toolbar_with_tooltips() {
        use crate::ui::icons::Icon;

        // Test that enhanced toolbar with tooltips can be created
        let tools = vec![
            (Icon::Brush, (), true),
            (Icon::Eraser, (), false),
        ];
        let toolbar = enhanced_toolbar_with_tooltips(tools, (), (), ());

        // Should return an Element
        // This is mainly a compilation test
        let _ = toolbar;
    }

    #[test]
    fn test_get_tool_tooltip_text() {
        use crate::ui::icons::Icon;

        // Test tooltip text mapping
        assert_eq!(get_tool_tooltip_text(Icon::Brush), "Brush Tool (B)");
        assert_eq!(get_tool_tooltip_text(Icon::Eraser), "Eraser Tool (E)");
        assert_eq!(get_tool_tooltip_text(Icon::Select), "Selection Tool (V)");
        assert_eq!(get_tool_tooltip_text(Icon::ZoomIn), "Zoom In (+)");
    }

    #[test]
    fn test_enhanced_tool_button_styling() {
        let theme = iced::Theme::Dark;

        // Test enhanced active button style with hover
        let active_hover_style = modern_tool_button_style(
            &theme,
            iced::widget::button::Status::Hovered,
            true
        );

        // Active hovered button should have enhanced glow
        assert!(active_hover_style.background.is_some());
        assert!(active_hover_style.shadow.blur_radius > 12.0); // Enhanced glow

        // Test inactive button hover style
        let inactive_hover_style = modern_tool_button_style(
            &theme,
            iced::widget::button::Status::Hovered,
            false
        );

        // Inactive hovered button should have subtle tech blue hint
        assert!(inactive_hover_style.background.is_some());
        assert!(inactive_hover_style.border.width > 0.0); // Should have border on hover
    }
}

/// Create a history panel
pub fn history_panel<Message: Clone + 'static>(
    history: Vec<HistoryEntry>,
    current_position: usize,
    navigate_to: impl Fn(usize) -> Message + Clone + 'static,
    clear_history: Message,
) -> Element<'static, Message> {
    let mut elements = vec![
        section_header("History".to_string()),
        container(
            button("Clear")
                .on_press(clear_history)
                .width(Length::Fill)
        )
        .padding(4.0)
        .into()
    ];

    for (index, entry) in history.iter().enumerate() {
        let is_current = index == current_position;
        let navigate_msg = navigate_to.clone()(index);
        let description = entry.description.clone();

        let history_item = container(
            button(text(description).size(11.0))
                .on_press(navigate_msg)
                .width(Length::Fill)
        )
        .padding(1.0)
        .style(move |_theme| {
            if is_current {
                container::Style {
                    background: Some(iced::Color::from_rgba(0.0, 0.75, 1.0, 0.2).into()),
                    ..Default::default()
                }
            } else {
                container::Style::default()
            }
        });

        elements.push(history_item.into());
    }

    column(elements).spacing(1.0).into()
}
