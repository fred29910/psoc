//! Modern UI components for PSOC Image Editor

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length,
};

use super::icons::{icon_button, simple_icon_button, tool_button, Icon};

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

/// Create a modern status bar
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

/// Create an advanced layer panel with controls
pub fn layer_panel<Message: Clone + 'static>(
    layers: Vec<(String, bool, bool, Message, Message)>, // (name, visible, selected, toggle_vis, select)
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
            .on_press(add_layer)
            .padding([4.0, 8.0]),
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
        for (name, is_visible, is_selected, toggle_visibility, select_layer) in layers {
            content.push(layer_item(
                name,
                is_visible,
                is_selected,
                toggle_visibility,
                select_layer,
            ));
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
