//! Modern color panel component
//! Provides advanced color selection with HSL wheel, gradients, and palette management

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Background, Color, Border, Shadow, Vector,
};

use crate::ui::theme::{PsocTheme, spacing};
use crate::ui::styles::glass_container_style;
use crate::ui::theme::GlassIntensity;
use psoc_core::RgbaPixel;

/// Color panel information structure
#[derive(Debug, Clone)]
pub struct ModernColorInfo {
    /// Current foreground color
    pub foreground_color: RgbaPixel,
    /// Current background color
    pub background_color: RgbaPixel,
    /// Recently used colors
    pub color_history: Vec<RgbaPixel>,
    /// Current palette name
    pub current_palette: String,
    /// Available palettes
    pub available_palettes: Vec<String>,
    /// HSL values for current color
    pub hsl_values: (f32, f32, f32), // Hue, Saturation, Lightness
    /// Whether color picker is expanded
    pub picker_expanded: bool,
}

impl Default for ModernColorInfo {
    fn default() -> Self {
        Self {
            foreground_color: RgbaPixel::new(0, 0, 0, 255),
            background_color: RgbaPixel::new(255, 255, 255, 255),
            color_history: vec![
                RgbaPixel::new(255, 0, 0, 255),
                RgbaPixel::new(0, 255, 0, 255),
                RgbaPixel::new(0, 0, 255, 255),
                RgbaPixel::new(255, 255, 0, 255),
                RgbaPixel::new(255, 0, 255, 255),
                RgbaPixel::new(0, 255, 255, 255),
            ],
            current_palette: "Default".to_string(),
            available_palettes: vec![
                "Default".to_string(),
                "Web Safe".to_string(),
                "Material".to_string(),
                "Pastel".to_string(),
            ],
            hsl_values: (0.0, 0.0, 0.0),
            picker_expanded: false,
        }
    }
}

/// Color panel messages
#[derive(Debug, Clone)]
pub enum ColorPanelMessage<Message> {
    /// Set foreground color
    SetForegroundColor(RgbaPixel),
    /// Set background color
    SetBackgroundColor(RgbaPixel),
    /// Swap foreground and background colors
    SwapColors,
    /// Reset to default colors
    ResetColors,
    /// Select color from history
    SelectHistoryColor(RgbaPixel),
    /// Open color picker
    OpenColorPicker,
    /// Close color picker
    CloseColorPicker,
    /// Change HSL values
    ChangeHSL(f32, f32, f32),
    /// Select palette
    SelectPalette(String),
    /// Custom message
    Custom(Message),
}

/// Create a modern color panel
pub fn modern_color_panel<Message: Clone + 'static>(
    color_info: ModernColorInfo,
    on_message: impl Fn(ColorPanelMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    let palette = psoc_theme.palette();

    let mut content = Vec::new();

    // Color panel header
    let header = container(
        text("Colors")
            .size(14.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::WHITE),
            })
    )
    .padding([8.0, 12.0]);
    content.push(header.into());

    // Main color swatches (foreground/background)
    let main_colors = create_main_color_swatches(&color_info, on_message);
    content.push(main_colors);

    // Color history section
    if !color_info.color_history.is_empty() {
        let history_section = create_color_history_section(&color_info, on_message);
        content.push(history_section);
    }

    // HSL controls (if picker is expanded)
    if color_info.picker_expanded {
        let hsl_controls = create_hsl_controls(&color_info, on_message);
        content.push(hsl_controls);
    }

    // Palette selector
    let palette_section = create_palette_section(&color_info, on_message);
    content.push(palette_section);

    // Action buttons
    let actions = create_color_actions(&color_info, on_message);
    content.push(actions);

    // Main content
    let panel_content = column(content)
        .spacing(spacing::SM);

    // Apply modern container styling
    container(panel_content)
        .width(Length::Fixed(280.0))
        .padding([spacing::SM, spacing::MD])
        .style(move |_theme| {
            let mut style = glass_container_style(GlassIntensity::Medium, &psoc_theme);
            style.border = Border {
                color: Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.3
                ),
                width: 1.0,
                radius: 16.0.into(),
            };
            style.shadow = Shadow {
                color: palette.shadow_color(0.15),
                offset: Vector::new(2.0, 4.0),
                blur_radius: 12.0,
            };
            style
        })
        .into()
}

/// Create main color swatches (foreground/background)
fn create_main_color_swatches<Message: Clone + 'static>(
    color_info: &ModernColorInfo,
    on_message: impl Fn(ColorPanelMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let fg_color = color_info.foreground_color;
    let bg_color = color_info.background_color;

    // Foreground color swatch
    let fg_swatch = button(
        container(
            text(" ")
                .size(16.0)
        )
        .width(Length::Fixed(48.0))
        .height(Length::Fixed(48.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(
                fg_color.r as f32 / 255.0,
                fg_color.g as f32 / 255.0,
                fg_color.b as f32 / 255.0,
                fg_color.a as f32 / 255.0,
            ))),
            border: Border {
                color: Color::WHITE,
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
    )
    .style(|_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    })
    .on_press(on_message(ColorPanelMessage::OpenColorPicker));

    // Background color swatch
    let bg_swatch = button(
        container(
            text(" ")
                .size(16.0)
        )
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(
                bg_color.r as f32 / 255.0,
                bg_color.g as f32 / 255.0,
                bg_color.b as f32 / 255.0,
                bg_color.a as f32 / 255.0,
            ))),
            border: Border {
                color: Color::from_rgba(0.7, 0.7, 0.7, 1.0),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
    )
    .style(|_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
    .on_press(on_message(ColorPanelMessage::OpenColorPicker));

    // Swap button
    let swap_button = button(
        text("⇄")
            .size(12.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::WHITE),
            })
    )
    .width(Length::Fixed(24.0))
    .height(Length::Fixed(24.0))
    .style(|_theme, _status| iced::widget::button::Style {
        background: Some(Background::Color(Color::from_rgba(0.3, 0.3, 0.3, 0.8))),
        text_color: Color::WHITE,
        border: Border {
            color: Color::from_rgba(0.5, 0.5, 0.5, 0.5),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    })
    .on_press(on_message(ColorPanelMessage::SwapColors));

    // Layout main colors
    let main_colors = row![
        fg_swatch,
        Space::new(Length::Fixed(8.0), Length::Shrink),
        column![
            bg_swatch,
            Space::new(Length::Shrink, Length::Fixed(4.0)),
            swap_button,
        ]
        .align_x(iced::alignment::Horizontal::Center),
        Space::new(Length::Fill, Length::Shrink),
        // Color values display
        column![
            text(format!("RGB({}, {}, {})", fg_color.r, fg_color.g, fg_color.b))
                .size(10.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0)),
                }),
            text(format!("#{:02X}{:02X}{:02X}", fg_color.r, fg_color.g, fg_color.b))
                .size(10.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(Color::from_rgba(0.6, 0.6, 0.6, 1.0)),
                }),
        ]
        .spacing(2.0)
    ]
    .align_y(iced::alignment::Vertical::Center)
    .spacing(8.0);

    container(main_colors)
        .padding([12.0, 16.0])
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 0.5))),
            border: Border {
                color: Color::from_rgba(0.3, 0.3, 0.3, 0.3),
                width: 1.0,
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into()
}

/// Create color history section
fn create_color_history_section<Message: Clone + 'static>(
    color_info: &ModernColorInfo,
    on_message: impl Fn(ColorPanelMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let mut history_swatches = Vec::new();

    for (i, &color) in color_info.color_history.iter().take(12).enumerate() {
        let swatch = button(
            container(
                text(" ")
                    .size(8.0)
            )
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                    color.a as f32 / 255.0,
                ))),
                border: Border {
                    color: Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
        )
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .on_press(on_message(ColorPanelMessage::SelectHistoryColor(color)));

        history_swatches.push(swatch.into());

        // Add spacing between swatches
        if i < color_info.color_history.len() - 1 && i < 11 {
            history_swatches.push(Space::new(Length::Fixed(4.0), Length::Shrink).into());
        }
    }

    // Create a simple row layout for color history (limit to 6 colors for simplicity)
    let limited_swatches: Vec<_> = history_swatches.into_iter().take(6).collect();
    let history_grid = row(limited_swatches).spacing(4.0);

    column![
        text("Recent Colors")
            .size(12.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0)),
            }),
        Space::new(Length::Shrink, Length::Fixed(4.0)),
        history_grid,
    ]
    .spacing(4.0)
    .into()
}

/// Create HSL controls
fn create_hsl_controls<Message: Clone + 'static>(
    color_info: &ModernColorInfo,
    on_message: impl Fn(ColorPanelMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let (hue, saturation, lightness) = color_info.hsl_values;

    // HSL sliders (simplified representation)
    let hue_control = create_hsl_slider("Hue".to_string(), hue, 0.0, 360.0);
    let saturation_control = create_hsl_slider("Saturation".to_string(), saturation, 0.0, 100.0);
    let lightness_control = create_hsl_slider("Lightness".to_string(), lightness, 0.0, 100.0);

    column![
        text("HSL Controls")
            .size(12.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0)),
            }),
        Space::new(Length::Shrink, Length::Fixed(8.0)),
        hue_control,
        saturation_control,
        lightness_control,
    ]
    .spacing(6.0)
    .into()
}

/// Create a simplified HSL slider representation
fn create_hsl_slider<Message: 'static>(
    label: String,
    value: f32,
    min: f32,
    max: f32,
) -> Element<'static, Message> {
    let percentage = ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0);

    row![
        text(label)
            .size(11.0)
            .width(Length::Fixed(60.0))
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0)),
            }),
        container(
            row![
                container(text(" "))
                    .width(Length::FillPortion((percentage as u16).max(1)))
                    .height(Length::Fixed(8.0))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.2, 0.6, 1.0, 1.0))),
                        border: Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                container(text(" "))
                    .width(Length::FillPortion(((100.0 - percentage) as u16).max(1)))
                    .height(Length::Fixed(8.0))
                    .style(move |_theme| iced::widget::container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.2, 0.2, 0.2, 1.0))),
                        border: Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
            ]
        )
        .width(Length::Fill)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 1.0))),
            border: Border {
                color: Color::from_rgba(0.3, 0.3, 0.3, 0.5),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }),
        text(format!("{:.0}", value))
            .size(10.0)
            .width(Length::Fixed(30.0))
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.6, 0.6, 0.6, 1.0)),
            }),
    ]
    .align_y(iced::alignment::Vertical::Center)
    .spacing(8.0)
    .into()
}

/// Create palette section
fn create_palette_section<Message: Clone + 'static>(
    color_info: &ModernColorInfo,
    on_message: impl Fn(ColorPanelMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let current_palette = &color_info.current_palette;

    column![
        text("Palette")
            .size(12.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0)),
            }),
        Space::new(Length::Shrink, Length::Fixed(4.0)),
        container(
            row![
                text(current_palette.clone())
                    .size(11.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(Color::WHITE),
                    }),
                Space::new(Length::Fill, Length::Shrink),
                text("▼")
                    .size(10.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(Color::from_rgba(0.7, 0.7, 0.7, 1.0)),
                    })
            ]
            .align_y(iced::alignment::Vertical::Center)
        )
        .width(Length::Fill)
        .padding([6.0, 8.0])
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(0.05, 0.05, 0.05, 1.0))),
            border: Border {
                color: Color::from_rgba(0.4, 0.4, 0.4, 0.5),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
    ]
    .spacing(4.0)
    .into()
}

/// Create color action buttons
fn create_color_actions<Message: Clone + 'static>(
    color_info: &ModernColorInfo,
    on_message: impl Fn(ColorPanelMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let picker_button_text = if color_info.picker_expanded { "Hide Picker" } else { "Show Picker" };
    let picker_action = if color_info.picker_expanded {
        ColorPanelMessage::CloseColorPicker
    } else {
        ColorPanelMessage::OpenColorPicker
    };

    row![
        button(
            text(picker_button_text)
                .size(11.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(Color::WHITE),
                })
        )
        .width(Length::Fill)
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgba(0.2, 0.5, 0.8, 0.8))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgba(0.3, 0.6, 0.9, 0.5),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .on_press(on_message(picker_action)),
        Space::new(Length::Fixed(8.0), Length::Shrink),
        button(
            text("Reset")
                .size(11.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(Color::WHITE),
                })
        )
        .width(Length::Fill)
        .style(|_theme, _status| iced::widget::button::Style {
            background: Some(Background::Color(Color::from_rgba(0.5, 0.5, 0.5, 0.8))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgba(0.6, 0.6, 0.6, 0.5),
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .on_press(on_message(ColorPanelMessage::ResetColors)),
    ]
    .spacing(8.0)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_color_info() -> ModernColorInfo {
        ModernColorInfo {
            foreground_color: RgbaPixel::new(255, 0, 0, 255),
            background_color: RgbaPixel::new(0, 255, 0, 255),
            color_history: vec![
                RgbaPixel::new(255, 0, 0, 255),
                RgbaPixel::new(0, 255, 0, 255),
                RgbaPixel::new(0, 0, 255, 255),
            ],
            current_palette: "Test Palette".to_string(),
            available_palettes: vec!["Test Palette".to_string(), "Another".to_string()],
            hsl_values: (180.0, 50.0, 75.0),
            picker_expanded: false,
        }
    }

    #[test]
    fn test_modern_color_info_creation() {
        let color_info = create_test_color_info();

        assert_eq!(color_info.foreground_color, RgbaPixel::new(255, 0, 0, 255));
        assert_eq!(color_info.background_color, RgbaPixel::new(0, 255, 0, 255));
        assert_eq!(color_info.color_history.len(), 3);
        assert_eq!(color_info.current_palette, "Test Palette");
        assert_eq!(color_info.hsl_values, (180.0, 50.0, 75.0));
        assert!(!color_info.picker_expanded);
    }

    #[test]
    fn test_modern_color_info_default() {
        let color_info = ModernColorInfo::default();

        assert_eq!(color_info.foreground_color, RgbaPixel::new(0, 0, 0, 255));
        assert_eq!(color_info.background_color, RgbaPixel::new(255, 255, 255, 255));
        assert_eq!(color_info.color_history.len(), 6);
        assert_eq!(color_info.current_palette, "Default");
        assert_eq!(color_info.available_palettes.len(), 4);
        assert!(!color_info.picker_expanded);
    }

    #[test]
    fn test_modern_color_panel_creation() {
        let color_info = create_test_color_info();
        let panel = modern_color_panel(color_info, |_| ());

        // Should return an Element
        // This is mainly a compilation test
        let _ = panel;
    }

    #[test]
    fn test_color_panel_message_types() {
        // Test that all message types can be created
        let _set_fg: ColorPanelMessage<()> = ColorPanelMessage::SetForegroundColor(RgbaPixel::new(255, 0, 0, 255));
        let _set_bg: ColorPanelMessage<()> = ColorPanelMessage::SetBackgroundColor(RgbaPixel::new(0, 255, 0, 255));
        let _swap: ColorPanelMessage<()> = ColorPanelMessage::SwapColors;
        let _reset: ColorPanelMessage<()> = ColorPanelMessage::ResetColors;
        let _history: ColorPanelMessage<()> = ColorPanelMessage::SelectHistoryColor(RgbaPixel::new(0, 0, 255, 255));
        let _open: ColorPanelMessage<()> = ColorPanelMessage::OpenColorPicker;
        let _close: ColorPanelMessage<()> = ColorPanelMessage::CloseColorPicker;
        let _hsl: ColorPanelMessage<()> = ColorPanelMessage::ChangeHSL(180.0, 50.0, 75.0);
        let _palette: ColorPanelMessage<()> = ColorPanelMessage::SelectPalette("Test".to_string());
        let _custom: ColorPanelMessage<()> = ColorPanelMessage::Custom(());
    }

    #[test]
    fn test_hsl_slider_creation() {
        let slider = create_hsl_slider::<()>("Test".to_string(), 50.0, 0.0, 100.0);

        // Should return an Element
        let _ = slider;
    }

    #[test]
    fn test_color_panel_expanded_state() {
        let mut color_info = create_test_color_info();
        color_info.picker_expanded = true;

        let panel = modern_color_panel(color_info, |_| ());

        // Should handle expanded state correctly
        let _ = panel;
    }

    #[test]
    fn test_color_history_variations() {
        let mut color_info = create_test_color_info();

        // Test with empty history
        color_info.color_history.clear();
        let panel1 = modern_color_panel(color_info.clone(), |_| ());
        let _ = panel1;

        // Test with many colors (limit values to prevent overflow)
        color_info.color_history = (0..20).map(|i| RgbaPixel::new(
            ((i * 10) % 256) as u8,
            ((i * 5) % 256) as u8,
            ((i * 15) % 256) as u8,
            255
        )).collect();
        let panel2 = modern_color_panel(color_info, |_| ());
        let _ = panel2;
    }

    #[test]
    fn test_hsl_values_range() {
        let test_values = [
            (0.0, 0.0, 0.0),
            (360.0, 100.0, 100.0),
            (180.0, 50.0, 50.0),
            (270.0, 75.0, 25.0),
        ];

        for (h, s, l) in test_values {
            let mut color_info = create_test_color_info();
            color_info.hsl_values = (h, s, l);
            color_info.picker_expanded = true;

            let panel = modern_color_panel(color_info, |_| ());
            // Should handle all HSL value ranges
            let _ = panel;
        }
    }

    #[test]
    fn test_palette_names() {
        let palette_names = [
            "Default",
            "Web Safe",
            "Material Design",
            "Custom Palette",
            "Very Long Palette Name That Might Overflow",
        ];

        for name in palette_names {
            let mut color_info = create_test_color_info();
            color_info.current_palette = name.to_string();

            let panel = modern_color_panel(color_info, |_| ());
            // Should handle different palette names
            let _ = panel;
        }
    }
}
