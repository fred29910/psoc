//! UI Components module for PSOC Image Editor

use iced::{
    widget::{button, checkbox, column, container, row, slider, text, text_input, Row, Space},
    Element, Length,
};

use super::icons::{icon_button, simple_icon_button, tool_button, Icon};
use crate::i18n::{t, Language};
use psoc_core::{HistoryEntry, RgbaPixel};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub mod menu_system;
pub mod modern_menu;
pub mod menu_factory;
pub mod responsive_layout;
pub mod keyboard_navigation;

// Re-export main components
pub use menu_system::{
    AnimationState, MenuCategory, MenuCategoryId, MenuItem, MenuSystem,
};
pub use modern_menu::{
    dropdown_menu, menu_bar, menu_system_view, MenuMessage,
    enhanced_menu_bar, enhanced_dropdown_menu, EnhancedMenuState,
    KeyboardNavigationMessage, KeyboardNavigationState,
};
pub use responsive_layout::{
    ResponsiveLayoutManager, ResponsiveLayoutMessage, PanelId, PanelState, ScreenSize,
};
pub use keyboard_navigation::{
    KeyboardNavigationManager, KeyboardNavigationMessage as KbNavMessage,
    FocusTarget, NavigationAction, TabOrder,
};
pub use menu_factory::MenuFactory;

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

/// Create a toolbar with tools and zoom controls
pub fn toolbar<Message: Clone + 'static>(
    tools: Vec<(Icon, Message, bool)>, // (icon, message, is_active)
    zoom_in: Message,
    zoom_out: Message,
    zoom_reset: Message,
) -> Element<'static, Message> {
    let tool_buttons: Vec<Element<Message>> = tools
        .into_iter()
        .map(|(icon, message, is_active)| {
            tool_button(icon, message, is_active).into()
        })
        .collect();

    let tools_row = row(tool_buttons).spacing(4.0);

    let zoom_controls = row![
        icon_button(Icon::ZoomOut, zoom_out),
        icon_button(Icon::ZoomReset, zoom_reset),
        icon_button(Icon::ZoomIn, zoom_in),
    ]
    .spacing(4.0);

    container(
        row![tools_row, Space::new(Length::Fill, Length::Shrink), zoom_controls]
            .spacing(16.0)
            .align_y(iced::alignment::Vertical::Center)
    )
    .padding(8.0)
    .into()
}

/// Create a side panel with title and content
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
    .padding(8.0);

    let content_column = column(content).spacing(4.0);

    container(
        column![header, content_column]
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
