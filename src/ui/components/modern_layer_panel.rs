//! Modern layer panel components with enhanced visual design
//! Provides modern layer cards, drag-and-drop support, and smooth animations

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Background, Color, Border, Shadow, Vector,
};

use super::Icon;
use crate::ui::theme::{PsocTheme, spacing};
use crate::ui::styles::glass_container_style;
use crate::ui::theme::GlassIntensity;

/// Modern layer information structure
#[derive(Debug, Clone)]
pub struct ModernLayerInfo {
    pub name: String,
    pub visible: bool,
    pub selected: bool,
    pub opacity: f32,
    pub blend_mode: psoc_core::BlendMode,
    pub layer_type: String,
    pub has_mask: bool,
    pub thumbnail: Option<iced::widget::image::Handle>, // Layer thumbnail
}

/// Layer panel messages
#[derive(Debug, Clone)]
pub enum LayerPanelMessage<Message> {
    /// Toggle layer visibility
    ToggleVisibility(usize),
    /// Select layer
    SelectLayer(usize),
    /// Change layer opacity
    ChangeOpacity(usize, f32),
    /// Change blend mode
    ChangeBlendMode(usize, psoc_core::BlendMode),
    /// Start dragging layer
    StartDrag(usize),
    /// Drop layer at position
    DropLayer(usize, usize),
    /// Add new layer
    AddLayer,
    /// Delete layer
    DeleteLayer(usize),
    /// Duplicate layer
    DuplicateLayer(usize),
    /// Custom message
    Custom(Message),
}

/// Create a modern layer card with enhanced styling
pub fn modern_layer_card<Message: Clone + 'static>(
    _index: usize,
    layer_info: ModernLayerInfo,
    on_toggle_visibility: Message,
    on_select: Message,
) -> Element<'static, Message> {
    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    let palette = psoc_theme.palette();

    // Layer thumbnail (placeholder for now)
    let thumbnail = container(
        text("🖼")
            .size(24.0)
            .style(|_theme| iced::widget::text::Style {
                color: Some(Color::from_rgb(0.7, 0.7, 0.7)),
            })
    )
    .width(Length::Fixed(40.0))
    .height(Length::Fixed(40.0))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme| iced::widget::container::Style {
        background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 0.8))),
        border: Border {
            color: Color::from_rgba(0.4, 0.4, 0.4, 0.5),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    });

    // Visibility toggle button with modern styling
    let visibility_icon = if layer_info.visible { "👁" } else { "🚫" };
    let tech_blue = palette.tech_blue;
    let is_visible = layer_info.visible;
    let visibility_button = button(
        text(visibility_icon)
            .size(16.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(if is_visible {
                    tech_blue
                } else {
                    Color::from_rgb(0.5, 0.5, 0.5)
                }),
            })
    )
    .on_press(on_toggle_visibility)
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(32.0))
    .style(|_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 16.0.into(),
        },
        ..Default::default()
    });

    // Layer name and type
    let layer_name = text(layer_info.name.clone())
        .size(13.0)
        .style(|_theme| iced::widget::text::Style {
            color: Some(Color::WHITE),
        });

    let tech_blue_variant = palette.tech_blue_variant(70);
    let layer_type_badge = container(
        text(layer_info.layer_type.clone())
            .size(10.0)
            .style(|_theme| iced::widget::text::Style {
                color: Some(Color::WHITE),
            })
    )
    .padding([2.0, 6.0])
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(tech_blue_variant)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });

    // Opacity display
    let opacity_text = text(format!("{}%", (layer_info.opacity * 100.0) as u8))
        .size(11.0)
        .style(|_theme| iced::widget::text::Style {
            color: Some(Color::from_rgb(0.8, 0.8, 0.8)),
        });

    // Blend mode indicator (if not Normal)
    let tech_blue_variant_80 = palette.tech_blue_variant(80);
    let blend_mode_indicator = if layer_info.blend_mode != psoc_core::BlendMode::Normal {
        Some(
            text(format!("{:?}", layer_info.blend_mode))
                .size(10.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(tech_blue_variant_80),
                })
        )
    } else {
        None
    };

    // Mask indicator
    let tech_blue_mask = palette.tech_blue;
    let mask_indicator = if layer_info.has_mask {
        Some(
            text("🎭")
                .size(12.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(tech_blue_mask),
                })
        )
    } else {
        None
    };

    // Build the layer info section
    let mut info_column = vec![
        row![
            layer_name,
            Space::new(Length::Fill, Length::Shrink),
            layer_type_badge,
        ]
        .align_y(iced::alignment::Vertical::Center)
        .into()
    ];

    let mut bottom_row = vec![opacity_text.into()];
    if let Some(blend_indicator) = blend_mode_indicator {
        bottom_row.push(text(" • ").size(10.0).into());
        bottom_row.push(blend_indicator.into());
    }
    if let Some(mask_ind) = mask_indicator {
        bottom_row.push(Space::new(Length::Fill, Length::Shrink).into());
        bottom_row.push(mask_ind.into());
    }

    info_column.push(
        row(bottom_row)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    );

    let layer_info_section = column(info_column)
        .spacing(4.0)
        .width(Length::Fill);

    // Main layer card content
    let card_content = row![
        thumbnail,
        container(layer_info_section)
            .padding([0.0, 12.0])
            .width(Length::Fill),
        visibility_button,
    ]
    .spacing(8.0)
    .align_y(iced::alignment::Vertical::Center);

    // Create the clickable card container
    let is_selected = layer_info.selected;
    let card = button(card_content)
        .on_press(on_select)
        .width(Length::Fill)
        .style(move |theme, status| modern_layer_card_style(theme, status, is_selected));

    container(card)
        .padding(2.0)
        .width(Length::Fill)
        .into()
}

/// Modern layer card styling
fn modern_layer_card_style(
    _theme: &iced::Theme,
    _status: iced::widget::button::Status,
    is_selected: bool,
) -> iced::widget::button::Style {
    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    let palette = psoc_theme.palette();

    let (background, border_color, shadow) = if is_selected {
        (
            Some(Background::Color(palette.glass_bg_medium)),
            palette.tech_blue,
            Shadow {
                color: palette.tech_blue_glow(),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 8.0,
            },
        )
    } else {
        (
            Some(Background::Color(palette.glass_bg_light)),
            Color::from_rgba(0.3, 0.3, 0.3, 0.5),
            Shadow::default(),
        )
    };

    iced::widget::button::Style {
        background,
        text_color: Color::WHITE,
        border: Border {
            color: border_color,
            width: if is_selected { 2.0 } else { 1.0 },
            radius: 12.0.into(),
        },
        shadow,
    }
}

/// Create modern layer panel with enhanced styling
pub fn modern_layer_panel<Message: Clone + 'static>(
    layers: Vec<ModernLayerInfo>,
    on_add_layer: Message,
    on_delete_layer: Option<Message>,
    on_duplicate_layer: Option<Message>,
    layer_messages: Vec<(Message, Message)>, // (toggle_visibility, select_layer) for each layer
) -> Element<'static, Message> {
    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    
    // Header with title and action buttons
    let header = container(
        row![
            text("Layers")
                .size(14.0)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(Color::WHITE),
                }),
            Space::new(Length::Fill, Length::Shrink),
            button(text("+").size(16.0))
                .on_press(on_add_layer)
                .width(Length::Fixed(28.0))
                .height(Length::Fixed(28.0))
                .style(|_theme, _status| iced::widget::button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.75, 1.0, 0.8))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 14.0.into(),
                    },
                    ..Default::default()
                }),
        ]
        .align_y(iced::alignment::Vertical::Center)
        .spacing(8.0)
    )
    .padding([12.0, 16.0]);

    // Layer cards
    let mut layer_elements = vec![header.into()];
    
    for (index, layer) in layers.iter().enumerate() {
        if let Some((toggle_vis, select)) = layer_messages.get(index) {
            let layer_card = modern_layer_card(
                index,
                layer.clone(),
                toggle_vis.clone(),
                select.clone(),
            );
            layer_elements.push(layer_card);
        }
    }

    // Action buttons at bottom
    let mut action_buttons = vec![];

    // Delete button
    let delete_button = button(text("🗑").size(14.0))
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgba(0.8, 0.2, 0.2, 0.8))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 16.0.into(),
            },
            ..Default::default()
        });

    let delete_button = if let Some(delete_msg) = on_delete_layer {
        delete_button.on_press(delete_msg)
    } else {
        delete_button
    };

    action_buttons.push(delete_button.into());

    if let Some(duplicate_msg) = on_duplicate_layer {
        action_buttons.push(
            button(text("📋").size(14.0))
                .on_press(duplicate_msg)
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0))
                .style(|_theme, _status| iced::widget::button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.2, 0.6, 0.8, 0.8))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 16.0.into(),
                    },
                    ..Default::default()
                })
                .into()
        );
    }

    let actions = container(
        row(action_buttons)
            .spacing(8.0)
            .align_y(iced::alignment::Vertical::Center)
    )
    .padding([8.0, 16.0]);

    layer_elements.push(actions.into());

    // Main panel container with glass effect
    let content = column(layer_elements)
        .spacing(4.0);

    container(content)
        .width(Length::Fixed(280.0))
        .style(move |_theme| {
            let mut style = glass_container_style(GlassIntensity::Medium, &psoc_theme);
            style.border = Border {
                radius: 16.0.into(),
                ..style.border
            };
            style
        })
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_layer_info() -> ModernLayerInfo {
        ModernLayerInfo {
            name: "Test Layer".to_string(),
            visible: true,
            selected: false,
            opacity: 1.0,
            blend_mode: psoc_core::BlendMode::Normal,
            layer_type: "Pixel".to_string(),
            has_mask: false,
            thumbnail: None,
        }
    }

    #[test]
    fn test_modern_layer_info_creation() {
        let layer_info = create_test_layer_info();
        assert_eq!(layer_info.name, "Test Layer");
        assert!(layer_info.visible);
        assert!(!layer_info.selected);
        assert_eq!(layer_info.opacity, 1.0);
        assert_eq!(layer_info.blend_mode, psoc_core::BlendMode::Normal);
        assert_eq!(layer_info.layer_type, "Pixel");
        assert!(!layer_info.has_mask);
    }

    #[test]
    fn test_modern_layer_card_creation() {
        let layer_info = create_test_layer_info();
        let card = modern_layer_card(0, layer_info, (), ());

        // Should return an Element
        // This is mainly a compilation test
        let _ = card;
    }

    #[test]
    fn test_modern_layer_panel_creation() {
        let layers = vec![
            create_test_layer_info(),
            ModernLayerInfo {
                name: "Layer 2".to_string(),
                visible: false,
                selected: true,
                opacity: 0.8,
                blend_mode: psoc_core::BlendMode::Multiply,
                layer_type: "Adjustment".to_string(),
                has_mask: true,
                thumbnail: None,
            },
        ];

        let layer_messages = vec![((), ()), ((), ())];
        let panel = modern_layer_panel(layers, (), Some(()), Some(()), layer_messages);

        // Should return an Element
        // This is mainly a compilation test
        let _ = panel;
    }

    #[test]
    fn test_modern_layer_card_style() {
        let theme = iced::Theme::Dark;
        let status = iced::widget::button::Status::Active;

        // Test selected layer style
        let selected_style = modern_layer_card_style(&theme, status, true);
        assert!(selected_style.background.is_some());
        assert!(selected_style.border.width > 1.0); // Selected should have thicker border

        // Test unselected layer style
        let unselected_style = modern_layer_card_style(&theme, status, false);
        assert!(unselected_style.background.is_some());
        assert_eq!(unselected_style.border.width, 1.0); // Unselected should have normal border
    }

    #[test]
    fn test_layer_panel_message_types() {
        // Test that all message types can be created
        let _toggle_vis: LayerPanelMessage<()> = LayerPanelMessage::ToggleVisibility(0);
        let _select: LayerPanelMessage<()> = LayerPanelMessage::SelectLayer(0);
        let _opacity: LayerPanelMessage<()> = LayerPanelMessage::ChangeOpacity(0, 0.5);
        let _blend: LayerPanelMessage<()> = LayerPanelMessage::ChangeBlendMode(0, psoc_core::BlendMode::Screen);
        let _drag: LayerPanelMessage<()> = LayerPanelMessage::StartDrag(0);
        let _drop: LayerPanelMessage<()> = LayerPanelMessage::DropLayer(0, 1);
        let _add: LayerPanelMessage<()> = LayerPanelMessage::AddLayer;
        let _delete: LayerPanelMessage<()> = LayerPanelMessage::DeleteLayer(0);
        let _duplicate: LayerPanelMessage<()> = LayerPanelMessage::DuplicateLayer(0);
        let _custom: LayerPanelMessage<()> = LayerPanelMessage::Custom(());
    }

    #[test]
    fn test_layer_info_with_different_blend_modes() {
        let blend_modes = [
            psoc_core::BlendMode::Normal,
            psoc_core::BlendMode::Multiply,
            psoc_core::BlendMode::Screen,
            psoc_core::BlendMode::Overlay,
        ];

        for blend_mode in blend_modes {
            let layer_info = ModernLayerInfo {
                name: format!("Layer {:?}", blend_mode),
                visible: true,
                selected: false,
                opacity: 1.0,
                blend_mode,
                layer_type: "Pixel".to_string(),
                has_mask: false,
                thumbnail: None,
            };

            // Should be able to create layer card with any blend mode
            let _card = modern_layer_card(0, layer_info, (), ());
        }
    }

    #[test]
    fn test_layer_info_with_mask() {
        let layer_with_mask = ModernLayerInfo {
            name: "Masked Layer".to_string(),
            visible: true,
            selected: false,
            opacity: 1.0,
            blend_mode: psoc_core::BlendMode::Normal,
            layer_type: "Pixel".to_string(),
            has_mask: true,
            thumbnail: None,
        };

        let _card = modern_layer_card(0, layer_with_mask, (), ());
        // Should handle masked layers correctly
    }
}
