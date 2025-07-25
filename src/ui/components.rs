//! Modern UI components for PSOC Image Editor

use iced::{
    widget::{button, checkbox, column, container, row, slider, text, text_input, Row, Space},
    Element, Length,
};

use super::icons::{icon_button, simple_icon_button, tool_button, Icon};
use crate::i18n::{t, Language};
use psoc_core::{HistoryEntry, RgbaPixel};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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

/// Create a modern toolbar with icons and proper spacing
pub fn toolbar<Message: Clone + 'static>(
    tools: Vec<(Icon, Message, bool)>, // (icon, message, is_active)
    zoom_in: Message,
    zoom_out: Message,
    zoom_reset: Message,
) -> Element<'static, Message> {
    let tool_buttons: Vec<Element<Message>> = tools
        .into_iter()
        .map(|(icon, message, is_active)| tool_button(icon, message, is_active).into())
        .collect();

    container(
        row![
            // Tool section
            container(
                row(tool_buttons)
                    .spacing(8.0)
                    .align_y(iced::alignment::Vertical::Center)
            )
            .padding(8.0),
            // Separator
            text("|").size(20.0),
            // Zoom section
            container(
                row![
                    simple_icon_button(Icon::ZoomOut, zoom_out),
                    simple_icon_button(Icon::ZoomActual, zoom_reset),
                    simple_icon_button(Icon::ZoomIn, zoom_in),
                ]
                .spacing(8.0)
                .align_y(iced::alignment::Vertical::Center)
            )
            .padding(8.0),
        ]
        .spacing(16.0)
        .align_y(iced::alignment::Vertical::Center),
    )
    .padding(16.0)
    .width(Length::Fill)
    .into()
}

/// Create a modern menu bar
#[allow(clippy::too_many_arguments)]
pub fn menu_bar<Message: Clone + 'static>(
    new_doc: Message,
    open_doc: Message,
    save_doc: Message,
    save_as_doc: Message,
    undo: Message,
    redo: Message,
    brightness_contrast: Message,
    hsl: Message,
    grayscale: Message,
    color_balance: Message,
    curves: Message,
    levels: Message,
    gaussian_blur: Message,
    unsharp_mask: Message,
    add_noise: Message,
    show_color_picker: Message,
    show_color_palette: Message,
    create_smart_object: Message,
    toggle_rulers: Message,
    toggle_grid: Message,
    toggle_guides: Message,
    show_about: Message,
    exit_app: Message,
) -> Element<'static, Message> {
    container(
        row![
            // File menu section
            container(
                row![
                    icon_button(Icon::New, new_doc),
                    icon_button(Icon::Open, open_doc),
                    icon_button(Icon::Save, save_doc),
                    icon_button(Icon::SaveAs, save_as_doc),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Edit menu section
            container(
                row![icon_button(Icon::Undo, undo), icon_button(Icon::Redo, redo),].spacing(8.0)
            )
            .padding(8.0),
            // Image menu section
            container(
                row![
                    button(text("Brightness/Contrast").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(brightness_contrast)
                    .padding([4.0, 8.0]),
                    button(text("HSL").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(hsl)
                    .padding([4.0, 8.0]),
                    button(text("Grayscale").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(grayscale)
                    .padding([4.0, 8.0]),
                    button(text("Color Balance").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(color_balance)
                    .padding([4.0, 8.0]),
                    button(text("Curves").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(curves)
                    .padding([4.0, 8.0]),
                    button(text("Levels").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(levels)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Filter menu section
            container(
                row![
                    button(text("Gaussian Blur").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(gaussian_blur)
                    .padding([4.0, 8.0]),
                    button(text("Unsharp Mask").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(unsharp_mask)
                    .padding([4.0, 8.0]),
                    button(text("Add Noise").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(add_noise)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Color menu section
            container(
                row![
                    button(text("Color Picker").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(show_color_picker)
                    .padding([4.0, 8.0]),
                    button(text("Color Palette").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(show_color_palette)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Layer menu section
            container(
                row![button(text("Smart Object").size(12.0).style(|_theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::WHITE),
                    }
                }))
                .on_press(create_smart_object)
                .padding([4.0, 8.0]),]
                .spacing(8.0)
            )
            .padding(8.0),
            // View menu section
            container(
                row![
                    button(text("Rulers").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(toggle_rulers)
                    .padding([4.0, 8.0]),
                    button(text("Grid").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(toggle_grid)
                    .padding([4.0, 8.0]),
                    button(text("Guides").size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(toggle_guides)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Spacer
            Space::new(Length::Fill, Length::Shrink),
            // Help menu section
            container(
                row![button(
                    text("About")
                        .size(12.0)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(iced::Color::WHITE)
                        })
                )
                .on_press(show_about)
                .padding([4.0, 8.0]),]
                .spacing(8.0)
            )
            .padding(8.0),
            // App controls
            container(simple_icon_button(Icon::Close, exit_app)).padding(8.0),
        ]
        .align_y(iced::alignment::Vertical::Center),
    )
    .padding(16.0)
    .width(Length::Fill)
    .into()
}

/// Create a modern side panel
pub fn side_panel<Message: 'static>(
    title: String,
    content: Vec<Element<'static, Message>>,
    width: f32,
) -> Element<'static, Message> {
    container(column![
        // Panel header
        container(
            text(title)
                .size(16.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::WHITE)
                })
        )
        .padding(16.0)
        .width(Length::Fill),
        // Panel content
        container(column(content).spacing(16.0).padding(16.0))
            .width(Length::Fill)
            .height(Length::Fill),
    ])
    .width(Length::Fixed(width))
    .height(Length::Fill)
    .into()
}

/// Create a modern status bar with comprehensive information
pub fn status_bar<Message: 'static>(
    status_text: String,
    zoom_level: f32,
) -> Element<'static, Message> {
    container(
        row![
            // Status text
            text(status_text).size(12.0),
            // Spacer
            Space::new(Length::Fill, Length::Shrink),
            // Zoom indicator
            text(format!("Zoom: {:.0}%", zoom_level * 100.0)).size(12.0),
        ]
        .align_y(iced::alignment::Vertical::Center),
    )
    .padding(8.0)
    .width(Length::Fill)
    .into()
}

/// Create an enhanced status bar with detailed information
pub fn enhanced_status_bar<Message: 'static>(
    status_info: &crate::ui::application::StatusInfo,
) -> Element<'static, Message> {
    let mut status_elements = Vec::new();

    // Document status
    status_elements.push(
        text(format!("Status: {}", status_info.document_status))
            .size(12.0)
            .into(),
    );

    // Image dimensions
    if let Some((width, height)) = status_info.image_size {
        status_elements.push(text(" | ").size(12.0).into());
        status_elements.push(
            text(format!("Size: {}×{}", width, height))
                .size(12.0)
                .into(),
        );
    }

    // Color mode
    if let Some(ref color_mode) = status_info.color_mode {
        status_elements.push(text(" | ").size(12.0).into());
        status_elements.push(text(format!("Mode: {}", color_mode)).size(12.0).into());
    }

    // Mouse coordinates
    if let Some((x, y)) = status_info.mouse_position {
        status_elements.push(text(" | ").size(12.0).into());
        status_elements.push(text(format!("Pos: ({:.0}, {:.0})", x, y)).size(12.0).into());
    }

    // Pixel color
    if let Some(color) = status_info.pixel_color {
        status_elements.push(text(" | ").size(12.0).into());
        status_elements.push(
            text(format!("RGB: ({}, {}, {})", color.r, color.g, color.b))
                .size(12.0)
                .into(),
        );

        if color.a < 255 {
            status_elements.push(text(format!(" A: {}", color.a)).size(12.0).into());
        }
    }

    // Add spacer and zoom
    status_elements.push(Space::new(Length::Fill, Length::Shrink).into());
    status_elements.push(
        text(format!("Zoom: {:.0}%", status_info.zoom_level * 100.0))
            .size(12.0)
            .into(),
    );

    container(
        Row::with_children(status_elements)
            .align_y(iced::alignment::Vertical::Center)
            .spacing(0),
    )
    .padding(8.0)
    .width(Length::Fill)
    .into()
}

/// Create a modern card container
pub fn card<Message: 'static>(content: Element<'static, Message>) -> Element<'static, Message> {
    container(content).padding(24.0).into()
}

/// Create a property row (label + value)
pub fn property_row<Message: 'static>(label: String, value: String) -> Element<'static, Message> {
    row![
        text(label).size(12.0).width(Length::Fixed(80.0)),
        text(value).size(12.0),
    ]
    .spacing(16.0)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

/// Create a section header
pub fn section_header<Message: 'static>(title: String) -> Element<'static, Message> {
    container(
        text(title)
            .size(16.0)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::WHITE),
            }),
    )
    .padding(iced::Padding::new(16.0).left(0.0).right(0.0).bottom(8.0))
    .into()
}

/// Create a tool palette
pub fn tool_palette<Message: Clone + 'static>(
    tools: Vec<(Icon, Message, bool)>, // (icon, message, is_active)
) -> Element<'static, Message> {
    let tool_grid: Vec<Element<Message>> = tools
        .chunks(2) // Create rows of 2 tools
        .map(|chunk| {
            let row_tools: Vec<Element<Message>> = chunk
                .iter()
                .map(|(icon, message, is_active)| {
                    tool_button(*icon, message.clone(), *is_active)
                        .width(Length::Fixed(60.0))
                        .height(Length::Fixed(60.0))
                        .into()
                })
                .collect();

            row(row_tools).spacing(8.0).into()
        })
        .collect();

    container(column(tool_grid).spacing(8.0))
        .padding(16.0)
        .into()
}

/// Create a layer item
pub fn layer_item<Message: Clone + 'static>(
    name: String,
    is_visible: bool,
    is_selected: bool,
    toggle_visibility: Message,
    select_layer: Message,
) -> Element<'static, Message> {
    let visibility_icon = if is_visible {
        Icon::LayerVisible
    } else {
        Icon::LayerHidden
    };

    // Create layer item with selection highlighting
    let layer_button = if is_selected {
        button(text(name).size(12.0))
            .on_press(select_layer)
            .width(Length::Fill)
            .style(button::primary)
    } else {
        button(text(name).size(12.0))
            .on_press(select_layer)
            .width(Length::Fill)
    };

    let layer_content = row![
        // Visibility toggle
        simple_icon_button(visibility_icon, toggle_visibility),
        // Layer name (clickable)
        layer_button,
    ]
    .spacing(8.0)
    .align_y(iced::alignment::Vertical::Center);

    let layer_container = if is_selected {
        container(layer_content)
            .padding(8.0)
            .width(Length::Fill)
            .style(container::bordered_box)
    } else {
        container(layer_content).padding(8.0).width(Length::Fill)
    };

    layer_container.into()
}

/// Create an advanced layer item with blend mode and opacity controls
#[allow(clippy::too_many_arguments)]
pub fn layer_item_advanced<Message: Clone + 'static>(
    name: String,
    is_visible: bool,
    is_selected: bool,
    opacity: f32,
    blend_mode: psoc_core::BlendMode,
    toggle_visibility: Message,
    select_layer: Message,
    _opacity_change: Message,
    _blend_change: Message,
) -> Element<'static, Message> {
    let visibility_icon = if is_visible {
        Icon::LayerVisible
    } else {
        Icon::LayerHidden
    };

    // Create layer item with selection highlighting
    let layer_button = if is_selected {
        button(text(name).size(12.0))
            .on_press(select_layer)
            .width(Length::Fill)
            .style(button::primary)
    } else {
        button(text(name).size(12.0))
            .on_press(select_layer)
            .width(Length::Fill)
    };

    // Blend mode options
    let _blend_modes = psoc_core::BlendMode::all();

    // Create the layer content with controls
    let layer_content = column![
        // Top row: visibility and name
        row![
            simple_icon_button(visibility_icon, toggle_visibility),
            layer_button,
        ]
        .spacing(8.0)
        .align_y(iced::alignment::Vertical::Center),
        // Controls row (only show if selected)
        if is_selected {
            column![
                // Opacity control
                row![
                    text("Opacity:").size(10.0).width(Length::Fixed(50.0)),
                    text(format!("{:.0}%", opacity * 100.0))
                        .size(10.0)
                        .width(Length::Fixed(35.0)),
                ]
                .spacing(4.0)
                .align_y(iced::alignment::Vertical::Center),
                // Blend mode control - show current mode
                row![
                    text("Blend:").size(10.0).width(Length::Fixed(50.0)),
                    text(blend_mode.name()).size(10.0),
                ]
                .spacing(4.0)
                .align_y(iced::alignment::Vertical::Center),
            ]
            .spacing(4.0)
            .padding([4.0, 8.0])
        } else {
            column![]
        }
    ]
    .spacing(4.0);

    let layer_container = if is_selected {
        container(layer_content)
            .padding(8.0)
            .width(Length::Fill)
            .style(container::bordered_box)
    } else {
        container(layer_content).padding(8.0).width(Length::Fill)
    };

    layer_container.into()
}

/// Parameters for simple layer item
pub struct LayerItemSimpleParams<Message: Clone + 'static> {
    pub name: String,
    pub is_visible: bool,
    pub is_selected: bool,
    pub opacity: f32,
    pub blend_mode: psoc_core::BlendMode,
    pub layer_type: Option<String>, // For adjustment layers, show the adjustment type
    pub has_mask: bool,
    pub toggle_visibility: Message,
    pub select_layer: Message,
}

/// Create a simple layer item with blend mode and opacity display (no interactive controls)
#[allow(clippy::too_many_arguments)]
pub fn layer_item_simple<Message: Clone + 'static>(
    params: LayerItemSimpleParams<Message>,
) -> Element<'static, Message> {
    let visibility_icon = if params.is_visible {
        Icon::LayerVisible
    } else {
        Icon::LayerHidden
    };

    // Create layer name with type indicator for adjustment layers, smart objects, and mask indicator
    let display_name = if let Some(layer_type_str) = params.layer_type {
        let type_indicator = match layer_type_str.as_str() {
            "SmartObject" => "📦",                 // Box emoji for smart objects
            _ => &format!("[{}]", layer_type_str), // Adjustment layers
        };

        if params.has_mask {
            format!("{} {} 🎭", params.name, type_indicator)
        } else {
            format!("{} {}", params.name, type_indicator)
        }
    } else if params.has_mask {
        format!("{} 🎭", params.name)
    } else {
        params.name
    };

    // Create layer item with selection highlighting
    let layer_button = if params.is_selected {
        button(text(display_name).size(12.0))
            .on_press(params.select_layer)
            .width(Length::Fill)
            .style(button::primary)
    } else {
        button(text(display_name).size(12.0))
            .on_press(params.select_layer)
            .width(Length::Fill)
    };

    // Create the layer content with controls
    let layer_content = column![
        // Top row: visibility and name
        row![
            simple_icon_button(visibility_icon, params.toggle_visibility),
            layer_button,
        ]
        .spacing(8.0)
        .align_y(iced::alignment::Vertical::Center),
        // Properties row (only show if selected)
        if params.is_selected {
            column![
                // Opacity display
                row![
                    text("Opacity:").size(10.0).width(Length::Fixed(50.0)),
                    text(format!("{:.0}%", params.opacity * 100.0)).size(10.0),
                ]
                .spacing(4.0)
                .align_y(iced::alignment::Vertical::Center),
                // Blend mode display
                row![
                    text("Blend:").size(10.0).width(Length::Fixed(50.0)),
                    text(params.blend_mode.name()).size(10.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.3, 0.6, 1.0)),
                        }
                    }),
                ]
                .spacing(4.0)
                .align_y(iced::alignment::Vertical::Center),
            ]
            .spacing(4.0)
            .padding([4.0, 8.0])
        } else {
            column![]
        }
    ]
    .spacing(4.0);

    let layer_container = if params.is_selected {
        container(layer_content)
            .padding(8.0)
            .width(Length::Fill)
            .style(container::bordered_box)
    } else {
        container(layer_content).padding(8.0).width(Length::Fill)
    };

    layer_container.into()
}

/// Create an advanced layer panel with controls
#[allow(clippy::type_complexity)]
pub fn layer_panel<Message: Clone + 'static>(
    layers: Vec<LayerInfoSimple<Message>>, // (name, visible, selected, opacity, blend_mode, layer_type, has_mask, toggle_vis, select)
    add_layer: Message,
    delete_layer: Option<Message>,
    duplicate_layer: Option<Message>,
    move_up: Option<Message>,
    move_down: Option<Message>,
) -> Element<'static, Message> {
    let mut content = Vec::new();

    // Layer controls
    let controls = row![
        button(text("Add").size(10.0))
            .on_press(add_layer.clone())
            .padding([4.0, 8.0]),
        button(text("Adj").size(10.0))
            .on_press(add_layer) // For now, use the same as Add - will be enhanced later
            .padding([4.0, 8.0])
            .style(button::secondary),
        button(text("Del").size(10.0))
            .on_press_maybe(delete_layer)
            .padding([4.0, 8.0]),
        button(text("Dup").size(10.0))
            .on_press_maybe(duplicate_layer)
            .padding([4.0, 8.0]),
        Space::new(Length::Fill, Length::Shrink),
        button(text("↑").size(10.0))
            .on_press_maybe(move_up)
            .padding([4.0, 6.0]),
        button(text("↓").size(10.0))
            .on_press_maybe(move_down)
            .padding([4.0, 6.0]),
    ]
    .spacing(4.0)
    .align_y(iced::alignment::Vertical::Center);

    content.push(container(controls).padding(8.0).into());

    // Layer list
    if layers.is_empty() {
        content.push(
            container(
                text("No layers")
                    .size(12.0)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            )
            .padding(16.0)
            .center_x(Length::Fill)
            .into(),
        );
    } else {
        let layer_count = layers.len();
        for (
            index,
            (
                name,
                is_visible,
                is_selected,
                opacity,
                blend_mode,
                layer_type,
                has_mask,
                toggle_visibility,
                select_layer,
            ),
        ) in layers.into_iter().enumerate()
        {
            // Calculate the actual layer index (reverse order)
            let _layer_index = layer_count - 1 - index;
            content.push(layer_item_simple(LayerItemSimpleParams {
                name,
                is_visible,
                is_selected,
                opacity,
                blend_mode,
                layer_type: Some(layer_type),
                has_mask,
                toggle_visibility,
                select_layer,
            }));
        }
    }

    side_panel("Layers".to_string(), content, 250.0)
}

/// Create a modern canvas area placeholder
pub fn canvas_placeholder<Message: 'static>(
    zoom_level: f32,
    pan_offset: (f32, f32),
    current_tool: &str,
) -> Element<'static, Message> {
    container(
        column![
            text("Canvas Area")
                .size(18.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7))
                }),
            Space::new(Length::Shrink, Length::Fixed(24.0)),
            property_row("Zoom".to_string(), format!("{:.0}%", zoom_level * 100.0)),
            property_row("Pan X".to_string(), format!("{:.1}", pan_offset.0)),
            property_row("Pan Y".to_string(), format!("{:.1}", pan_offset.1)),
            property_row("Tool".to_string(), current_tool.to_string()),
            Space::new(Length::Shrink, Length::Fixed(24.0)),
            text("Click 'New' to create a document")
                .size(12.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5))
                }),
        ]
        .align_x(iced::alignment::Horizontal::Center)
        .spacing(8.0),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

/// Create a tool options panel
pub fn tool_options_panel<Message: Clone + 'static>(
    tool_name: String,
    options: Vec<ToolOptionControl<Message>>,
) -> Element<'static, Message> {
    let mut content = vec![section_header(format!("{} Options", tool_name))];

    if options.is_empty() {
        content.push(
            container(text("No options available").size(12.0).style(|_theme| {
                iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                }
            }))
            .padding(16.0)
            .center_x(Length::Fill)
            .into(),
        );
    } else {
        for option in options {
            content.push(option.into_element());
        }
    }

    side_panel("Tool Options".to_string(), content, 250.0)
}

/// Create a history panel showing command history
pub fn history_panel<Message: Clone + 'static>(
    history_entries: Vec<HistoryEntry>,
    navigate_to: impl Fn(usize) -> Message + 'static,
    clear_history: Message,
) -> Element<'static, Message> {
    let mut content = vec![section_header("History".to_string())];

    // Add clear history button
    content.push(
        container(
            button(text("Clear History").size(12.0))
                .on_press(clear_history)
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    button::Style {
                        background: Some(iced::Background::Color(palette.danger.base.color)),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..button::primary(theme, status)
                    }
                }),
        )
        .padding(8.0)
        .width(Length::Fill)
        .into(),
    );

    if history_entries.is_empty() {
        content.push(
            container(
                text("No history")
                    .size(12.0)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            )
            .padding(16.0)
            .center_x(Length::Fill)
            .into(),
        );
    } else {
        // Add history entries
        for entry in history_entries {
            content.push(history_entry_item(
                entry.description.clone(),
                entry.is_current,
                navigate_to(entry.index + 1), // +1 because we want to navigate to the position after this command
            ));
        }
    }

    side_panel("History".to_string(), content, 250.0)
}

/// Create a single history entry item
fn history_entry_item<Message: Clone + 'static>(
    description: String,
    is_current: bool,
    navigate_message: Message,
) -> Element<'static, Message> {
    let text_color = if is_current {
        iced::Color::from_rgb(0.2, 0.6, 1.0) // Blue for current
    } else {
        iced::Color::WHITE
    };

    let entry_content = button(
        text(description)
            .size(12.0)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(text_color),
            }),
    )
    .on_press(navigate_message)
    .style(move |theme: &iced::Theme, status| {
        let palette = theme.extended_palette();
        if is_current {
            button::Style {
                background: Some(iced::Background::Color(palette.primary.weak.color)),
                text_color,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..button::secondary(theme, status)
            }
        } else {
            button::Style {
                background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                text_color,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..button::text(theme, status)
            }
        }
    })
    .width(Length::Fill);

    container(entry_content)
        .padding(2.0)
        .width(Length::Fill)
        .into()
}

/// Tool option control types
pub enum ToolOptionControl<Message> {
    FloatSlider {
        label: String,
        value: f32,
        min: f32,
        max: f32,
        step: f32,
        on_change: Box<dyn Fn(f32) -> Message + 'static>,
    },
    IntSlider {
        label: String,
        value: i32,
        min: i32,
        max: i32,
        on_change: Box<dyn Fn(i32) -> Message + 'static>,
    },
    ColorPicker {
        label: String,
        value: [u8; 4], // RGBA
        on_change: Box<dyn Fn([u8; 4]) -> Message + 'static>,
    },
    Checkbox {
        label: String,
        value: bool,
        on_change: Box<dyn Fn(bool) -> Message + 'static>,
    },
    TextInput {
        label: String,
        value: String,
        placeholder: String,
        on_change: Box<dyn Fn(String) -> Message + 'static>,
    },
    Dropdown {
        label: String,
        options: Vec<String>,
        selected: String,
        on_change: Box<dyn Fn(String) -> Message + 'static>,
    },
}

impl<Message: Clone + 'static> ToolOptionControl<Message> {
    /// Convert the control to an iced Element
    pub fn into_element(self) -> Element<'static, Message> {
        match self {
            ToolOptionControl::FloatSlider {
                label,
                value,
                min,
                max,
                step,
                on_change,
            } => {
                let slider = slider(min..=max, value, on_change).step(step);

                column![
                    row![
                        text(label.clone()).size(12.0),
                        Space::new(Length::Fill, Length::Shrink),
                        text(format!("{:.2}", value)).size(12.0),
                    ]
                    .align_y(iced::alignment::Vertical::Center),
                    slider,
                ]
                .spacing(4.0)
                .padding(8.0)
                .into()
            }
            ToolOptionControl::IntSlider {
                label,
                value,
                min,
                max,
                on_change,
            } => {
                let slider = slider(min..=max, value, on_change);

                column![
                    row![
                        text(label.clone()).size(12.0),
                        Space::new(Length::Fill, Length::Shrink),
                        text(value.to_string()).size(12.0),
                    ]
                    .align_y(iced::alignment::Vertical::Center),
                    slider,
                ]
                .spacing(4.0)
                .padding(8.0)
                .into()
            }
            ToolOptionControl::ColorPicker {
                label,
                value,
                on_change: _,
            } => {
                // Simple color display for now - full color picker would be more complex
                let color_display = container(Space::new(Length::Fixed(20.0), Length::Fixed(20.0)))
                    .style(move |_theme| container::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba8(
                            value[0],
                            value[1],
                            value[2],
                            value[3] as f32 / 255.0,
                        ))),
                        border: iced::Border {
                            color: iced::Color::BLACK,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    });

                row![
                    text(label.clone()).size(12.0),
                    Space::new(Length::Fill, Length::Shrink),
                    color_display,
                ]
                .align_y(iced::alignment::Vertical::Center)
                .padding(8.0)
                .into()
            }
            ToolOptionControl::Checkbox {
                label,
                value,
                on_change,
            } => row![checkbox(label.clone(), value).on_toggle(on_change),]
                .align_y(iced::alignment::Vertical::Center)
                .padding(8.0)
                .into(),
            ToolOptionControl::TextInput {
                label,
                value,
                placeholder,
                on_change,
            } => column![
                text(label.clone()).size(12.0),
                text_input(&placeholder, &value).on_input(on_change),
            ]
            .spacing(4.0)
            .padding(8.0)
            .into(),
            ToolOptionControl::Dropdown {
                label,
                options: _,
                selected,
                on_change: _,
            } => {
                // For now, create a simple text display since iced doesn't have a built-in dropdown
                // In a real implementation, you'd use a pick_list or custom dropdown
                column![
                    text(label.clone()).size(12.0),
                    row![
                        text("Current: ").size(10.0),
                        text(selected.clone()).size(10.0).style(|_theme| {
                            iced::widget::text::Style {
                                color: Some(iced::Color::from_rgb(0.3, 0.6, 1.0)),
                            }
                        }),
                    ]
                    .align_y(iced::alignment::Vertical::Center),
                    // TODO: Replace with actual dropdown/pick_list when available
                    text("(Use keyboard shortcuts to change)")
                        .size(8.0)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        }),
                ]
                .spacing(4.0)
                .padding(8.0)
                .into()
            }
        }
    }
}

/// Maximum number of colors to keep in history
const MAX_HISTORY_SIZE: usize = 20;

/// Color history messages
#[derive(Debug, Clone)]
pub enum ColorHistoryMessage {
    /// Select a color from history
    SelectColor(RgbaPixel),
    /// Clear all history
    ClearHistory,
}

/// Color history component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorHistory {
    /// Recently used colors (most recent first)
    colors: VecDeque<RgbaPixel>,
}

impl Default for ColorHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorHistory {
    /// Create a new color history
    pub fn new() -> Self {
        Self {
            colors: VecDeque::new(),
        }
    }

    /// Add a color to the history
    pub fn add_color(&mut self, color: RgbaPixel) {
        // Remove the color if it already exists
        self.colors.retain(|&c| c != color);

        // Add to front
        self.colors.push_front(color);

        // Limit size
        if self.colors.len() > MAX_HISTORY_SIZE {
            self.colors.pop_back();
        }
    }

    /// Get all colors in history (most recent first)
    pub fn colors(&self) -> &VecDeque<RgbaPixel> {
        &self.colors
    }

    /// Get the most recent color
    pub fn most_recent(&self) -> Option<RgbaPixel> {
        self.colors.front().copied()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.colors.clear();
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Get the number of colors in history
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Handle color history messages
    pub fn update(&mut self, message: ColorHistoryMessage) {
        match message {
            ColorHistoryMessage::SelectColor(_color) => {
                // Color selection will be handled by parent
            }
            ColorHistoryMessage::ClearHistory => {
                self.clear();
            }
        }
    }
}

/// Create a language selector component
pub fn language_selector<Message: Clone + 'static>(
    current_language: Language,
    on_language_change: impl Fn(Language) -> Message + 'static,
) -> Element<'static, Message> {
    use iced::widget::pick_list;

    let languages = Language::all();

    container(
        column![
            text(t("language-selector-title"))
                .size(14.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(iced::Color::WHITE)
                }),
            pick_list(languages, Some(current_language), move |language| {
                on_language_change(language)
            })
            .placeholder(t("language-selector-placeholder"))
            .width(Length::Fixed(150.0))
        ]
        .spacing(8.0),
    )
    .padding(8.0)
    .into()
}

/// Create a localized menu bar with translation support
#[allow(clippy::too_many_arguments)]
pub fn localized_menu_bar<Message: Clone + 'static>(
    new_doc: Message,
    open_doc: Message,
    save_doc: Message,
    save_as_doc: Message,
    undo: Message,
    redo: Message,
    brightness_contrast: Message,
    hsl: Message,
    grayscale: Message,
    color_balance: Message,
    curves: Message,
    levels: Message,
    gaussian_blur: Message,
    unsharp_mask: Message,
    add_noise: Message,
    show_color_picker: Message,
    show_color_palette: Message,
    show_preferences: Message,
    create_smart_object: Message,
    toggle_rulers: Message,
    toggle_grid: Message,
    toggle_guides: Message,
    show_about: Message,
    exit_app: Message,
    language_change: impl Fn(Language) -> Message + 'static,
    current_language: Language,
) -> Element<'static, Message> {
    container(
        row![
            // File menu section
            container(
                row![
                    icon_button(Icon::New, new_doc),
                    icon_button(Icon::Open, open_doc),
                    icon_button(Icon::Save, save_doc),
                    icon_button(Icon::SaveAs, save_as_doc),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Edit menu section
            container(
                row![icon_button(Icon::Undo, undo), icon_button(Icon::Redo, redo),].spacing(8.0)
            )
            .padding(8.0),
            // Image menu section
            container(
                row![
                    button(
                        text(t("menu-image-brightness-contrast"))
                            .size(12.0)
                            .style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            })
                    )
                    .on_press(brightness_contrast)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-image-hsl")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(hsl)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-image-grayscale")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(grayscale)
                    .padding([4.0, 8.0]),
                    button(
                        text(t("menu-image-color-balance"))
                            .size(12.0)
                            .style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            })
                    )
                    .on_press(color_balance)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-image-curves")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(curves)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-image-levels")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(levels)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Filter menu section
            container(
                row![
                    button(
                        text(t("menu-filter-gaussian-blur"))
                            .size(12.0)
                            .style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            })
                    )
                    .on_press(gaussian_blur)
                    .padding([4.0, 8.0]),
                    button(
                        text(t("menu-filter-unsharp-mask"))
                            .size(12.0)
                            .style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            })
                    )
                    .on_press(unsharp_mask)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-filter-add-noise")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(add_noise)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Color menu section
            container(
                row![
                    button(
                        text(t("menu-tools-color-picker"))
                            .size(12.0)
                            .style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            })
                    )
                    .on_press(show_color_picker)
                    .padding([4.0, 8.0]),
                    button(
                        text(t("menu-tools-color-palette"))
                            .size(12.0)
                            .style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            })
                    )
                    .on_press(show_color_palette)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Layer menu section
            container(
                row![button(text("Smart Object").size(12.0).style(|_theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::WHITE),
                    }
                }))
                .on_press(create_smart_object)
                .padding([4.0, 8.0]),]
                .spacing(8.0)
            )
            .padding(8.0),
            // View menu section
            container(
                row![
                    button(text(t("menu-view-rulers")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(toggle_rulers)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-view-grid")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(toggle_grid)
                    .padding([4.0, 8.0]),
                    button(text(t("menu-view-guides")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(toggle_guides)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Language selector
            language_selector(current_language, language_change),
            // Spacer
            Space::new(Length::Fill, Length::Shrink),
            // Settings menu section
            container(
                row![
                    button(text(t("menu-edit-preferences")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(show_preferences)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // Help menu section
            container(
                row![
                    button(text(t("menu-help-about")).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }
                    }))
                    .on_press(show_about)
                    .padding([4.0, 8.0]),
                ]
                .spacing(8.0)
            )
            .padding(8.0),
            // App controls
            container(simple_icon_button(Icon::Close, exit_app)).padding(8.0),
        ]
        .align_y(iced::alignment::Vertical::Center),
    )
    .padding(16.0)
    .width(Length::Fill)
    .into()
}
