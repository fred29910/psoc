//! Color picker dialog for PSOC Image Editor

use iced::{
    widget::{button, column, container, row, slider, text, text_input, Space},
    Element, Length,
};
use psoc_core::{color::HslColor, RgbaPixel};
use serde::{Deserialize, Serialize};

use crate::ui::theme::{spacing, PsocTheme};

/// Color picker dialog messages
#[derive(Debug, Clone)]
pub enum ColorPickerMessage {
    /// Red component changed (0-255)
    RedChanged(f32),
    /// Green component changed (0-255)
    GreenChanged(f32),
    /// Blue component changed (0-255)
    BlueChanged(f32),
    /// Alpha component changed (0-255)
    AlphaChanged(f32),
    /// Hue changed (0-360)
    HueChanged(f32),
    /// Saturation changed (0-100)
    SaturationChanged(f32),
    /// Lightness changed (0-100)
    LightnessChanged(f32),
    /// Hex color text changed
    HexChanged(String),
    /// Apply the selected color
    Apply,
    /// Cancel color selection
    Cancel,
    /// Reset to default color
    Reset,
    /// Select a preset color
    SelectPreset(RgbaPixel),
}

/// Color picker dialog state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPickerDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current color (RGBA)
    pub current_color: RgbaPixel,
    /// Original color (for cancel)
    pub original_color: RgbaPixel,
    /// Hex color text input
    pub hex_text: String,
    /// Red component text
    pub red_text: String,
    /// Green component text
    pub green_text: String,
    /// Blue component text
    pub blue_text: String,
    /// Alpha component text
    pub alpha_text: String,
    /// Hue text
    pub hue_text: String,
    /// Saturation text
    pub saturation_text: String,
    /// Lightness text
    pub lightness_text: String,
}

impl Default for ColorPickerDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorPickerDialog {
    /// Create a new color picker dialog
    pub fn new() -> Self {
        let default_color = RgbaPixel::new(255, 255, 255, 255); // White
        Self {
            visible: false,
            current_color: default_color,
            original_color: default_color,
            hex_text: "FFFFFF".to_string(),
            red_text: "255".to_string(),
            green_text: "255".to_string(),
            blue_text: "255".to_string(),
            alpha_text: "255".to_string(),
            hue_text: "0".to_string(),
            saturation_text: "0".to_string(),
            lightness_text: "100".to_string(),
        }
    }

    /// Show the dialog with a specific color
    pub fn show(&mut self, color: RgbaPixel) {
        self.visible = true;
        self.current_color = color;
        self.original_color = color;
        self.update_text_fields();
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if the dialog is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the current color
    pub fn current_color(&self) -> RgbaPixel {
        self.current_color
    }

    /// Get the original color
    pub fn original_color(&self) -> RgbaPixel {
        self.original_color
    }

    /// Update all text fields based on current color
    fn update_text_fields(&mut self) {
        self.hex_text = format!(
            "{:02X}{:02X}{:02X}",
            self.current_color.r, self.current_color.g, self.current_color.b
        );
        self.red_text = self.current_color.r.to_string();
        self.green_text = self.current_color.g.to_string();
        self.blue_text = self.current_color.b.to_string();
        self.alpha_text = self.current_color.a.to_string();

        // Convert to HSL for HSL fields
        let hsl = HslColor::from_rgba(self.current_color);
        self.hue_text = format!("{:.0}", hsl.h);
        self.saturation_text = format!("{:.0}", hsl.s * 100.0);
        self.lightness_text = format!("{:.0}", hsl.l * 100.0);
    }

    /// Update the current color from RGB components
    fn update_color_from_rgb(&mut self) {
        self.update_text_fields();
    }

    /// Update the current color from HSL components
    fn update_color_from_hsl(&mut self, h: f32, s: f32, l: f32) {
        let hsl = HslColor::new(h, s / 100.0, l / 100.0, self.current_color.a as f32 / 255.0);
        self.current_color = hsl.to_rgba();
        self.update_text_fields();
    }

    /// Update the current color from hex string
    fn update_color_from_hex(&mut self, hex: &str) {
        if let Ok(color) = parse_hex_color(hex) {
            self.current_color.r = color.0;
            self.current_color.g = color.1;
            self.current_color.b = color.2;
            // Keep existing alpha
            self.update_text_fields();
        }
    }

    /// Handle color picker messages
    pub fn update(&mut self, message: ColorPickerMessage) {
        match message {
            ColorPickerMessage::RedChanged(value) => {
                self.current_color.r = value as u8;
                self.update_color_from_rgb();
            }
            ColorPickerMessage::GreenChanged(value) => {
                self.current_color.g = value as u8;
                self.update_color_from_rgb();
            }
            ColorPickerMessage::BlueChanged(value) => {
                self.current_color.b = value as u8;
                self.update_color_from_rgb();
            }
            ColorPickerMessage::AlphaChanged(value) => {
                self.current_color.a = value as u8;
                self.update_color_from_rgb();
            }
            ColorPickerMessage::HueChanged(value) => {
                let hsl = HslColor::from_rgba(self.current_color);
                self.update_color_from_hsl(value, hsl.s * 100.0, hsl.l * 100.0);
            }
            ColorPickerMessage::SaturationChanged(value) => {
                let hsl = HslColor::from_rgba(self.current_color);
                self.update_color_from_hsl(hsl.h, value, hsl.l * 100.0);
            }
            ColorPickerMessage::LightnessChanged(value) => {
                let hsl = HslColor::from_rgba(self.current_color);
                self.update_color_from_hsl(hsl.h, hsl.s * 100.0, value);
            }
            ColorPickerMessage::HexChanged(hex) => {
                self.hex_text = hex.clone();
                self.update_color_from_hex(&hex);
            }
            ColorPickerMessage::Apply => {
                // Color will be applied by the parent
            }
            ColorPickerMessage::Cancel => {
                self.current_color = self.original_color;
                self.update_text_fields();
            }
            ColorPickerMessage::Reset => {
                self.current_color = RgbaPixel::new(255, 255, 255, 255);
                self.update_text_fields();
            }
            ColorPickerMessage::SelectPreset(color) => {
                self.current_color = color;
                self.update_text_fields();
            }
        }
    }

    /// Get preset colors
    pub fn preset_colors() -> Vec<RgbaPixel> {
        vec![
            RgbaPixel::new(0, 0, 0, 255),       // Black
            RgbaPixel::new(255, 255, 255, 255), // White
            RgbaPixel::new(255, 0, 0, 255),     // Red
            RgbaPixel::new(0, 255, 0, 255),     // Green
            RgbaPixel::new(0, 0, 255, 255),     // Blue
            RgbaPixel::new(255, 255, 0, 255),   // Yellow
            RgbaPixel::new(255, 0, 255, 255),   // Magenta
            RgbaPixel::new(0, 255, 255, 255),   // Cyan
            RgbaPixel::new(128, 128, 128, 255), // Gray
            RgbaPixel::new(255, 128, 0, 255),   // Orange
            RgbaPixel::new(128, 0, 128, 255),   // Purple
            RgbaPixel::new(0, 128, 0, 255),     // Dark Green
        ]
    }

    /// Create the color picker dialog view
    pub fn view<'a, F>(&'a self, message_mapper: F) -> Element<'a, ColorPickerMessage>
    where
        F: Fn(ColorPickerMessage) -> ColorPickerMessage + Copy + 'a,
    {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        let color_preview =
            container(Space::new(Length::Fixed(60.0), Length::Fixed(60.0))).style(move |_theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgba8(
                        self.current_color.r,
                        self.current_color.g,
                        self.current_color.b,
                        self.current_color.a as f32 / 255.0,
                    ))),
                    border: iced::Border {
                        color: iced::Color::BLACK,
                        width: 2.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            });

        let rgb_controls = column![
            text("RGB").size(16.0),
            row![
                text("R:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=255.0, self.current_color.r as f32, move |value| {
                    message_mapper(ColorPickerMessage::RedChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("255", &self.red_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::RedChanged(value.clamp(0.0, 255.0)))
                        } else {
                            message_mapper(ColorPickerMessage::RedChanged(
                                self.current_color.r as f32,
                            ))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
            row![
                text("G:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=255.0, self.current_color.g as f32, move |value| {
                    message_mapper(ColorPickerMessage::GreenChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("255", &self.green_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::GreenChanged(
                                value.clamp(0.0, 255.0),
                            ))
                        } else {
                            message_mapper(ColorPickerMessage::GreenChanged(
                                self.current_color.g as f32,
                            ))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
            row![
                text("B:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=255.0, self.current_color.b as f32, move |value| {
                    message_mapper(ColorPickerMessage::BlueChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("255", &self.blue_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::BlueChanged(value.clamp(0.0, 255.0)))
                        } else {
                            message_mapper(ColorPickerMessage::BlueChanged(
                                self.current_color.b as f32,
                            ))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
            row![
                text("A:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=255.0, self.current_color.a as f32, move |value| {
                    message_mapper(ColorPickerMessage::AlphaChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("255", &self.alpha_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::AlphaChanged(
                                value.clamp(0.0, 255.0),
                            ))
                        } else {
                            message_mapper(ColorPickerMessage::AlphaChanged(
                                self.current_color.a as f32,
                            ))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
        ]
        .spacing(spacing::XS);

        let hsl = HslColor::from_rgba(self.current_color);
        let hsl_controls = column![
            text("HSL").size(16.0),
            row![
                text("H:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=360.0, hsl.h, move |value| {
                    message_mapper(ColorPickerMessage::HueChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("0", &self.hue_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::HueChanged(value.clamp(0.0, 360.0)))
                        } else {
                            message_mapper(ColorPickerMessage::HueChanged(hsl.h))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
            row![
                text("S:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=100.0, hsl.s * 100.0, move |value| {
                    message_mapper(ColorPickerMessage::SaturationChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("0", &self.saturation_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::SaturationChanged(
                                value.clamp(0.0, 100.0),
                            ))
                        } else {
                            message_mapper(ColorPickerMessage::SaturationChanged(hsl.s * 100.0))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
            row![
                text("L:").size(12.0).width(Length::Fixed(20.0)),
                slider(0.0..=100.0, hsl.l * 100.0, move |value| {
                    message_mapper(ColorPickerMessage::LightnessChanged(value))
                })
                .width(Length::FillPortion(3)),
                text_input("0", &self.lightness_text)
                    .on_input(move |text| {
                        if let Ok(value) = text.parse::<f32>() {
                            message_mapper(ColorPickerMessage::LightnessChanged(
                                value.clamp(0.0, 100.0),
                            ))
                        } else {
                            message_mapper(ColorPickerMessage::LightnessChanged(hsl.l * 100.0))
                        }
                    })
                    .width(Length::Fixed(60.0))
                    .size(12.0),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
        ]
        .spacing(spacing::XS);

        let hex_input = row![
            text("Hex:").size(12.0),
            text("#").size(12.0),
            text_input("FFFFFF", &self.hex_text)
                .on_input(move |text| message_mapper(ColorPickerMessage::HexChanged(text)))
                .width(Length::Fixed(80.0))
                .size(12.0),
        ]
        .spacing(spacing::SM)
        .align_y(iced::alignment::Vertical::Center);

        // Create preset color buttons
        let preset_colors = Self::preset_colors();
        let mut preset_rows = Vec::new();
        for chunk in preset_colors.chunks(6) {
            let mut preset_row = row![].spacing(spacing::XS);
            for &color in chunk {
                let color_button = button(Space::new(Length::Fixed(24.0), Length::Fixed(24.0)))
                    .on_press(message_mapper(ColorPickerMessage::SelectPreset(color)))
                    .style(move |_theme, _status| button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgba8(
                            color.r,
                            color.g,
                            color.b,
                            color.a as f32 / 255.0,
                        ))),
                        border: iced::Border {
                            color: iced::Color::BLACK,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    });
                preset_row = preset_row.push(color_button);
            }
            preset_rows.push(preset_row.into());
        }

        let presets = column(preset_rows).spacing(spacing::XS);

        let buttons = row![
            button("Apply")
                .on_press(message_mapper(ColorPickerMessage::Apply))
                .style(|theme, status| {
                    let palette = PsocTheme::Dark.palette();
                    button::Style {
                        background: Some(iced::Background::Color(palette.primary)),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            color: palette.border,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..button::primary(theme, status)
                    }
                }),
            button("Cancel")
                .on_press(message_mapper(ColorPickerMessage::Cancel))
                .style(|theme, status| {
                    let palette = PsocTheme::Dark.palette();
                    button::Style {
                        background: Some(iced::Background::Color(palette.secondary)),
                        text_color: iced::Color::WHITE,
                        border: iced::Border {
                            color: palette.border,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..button::secondary(theme, status)
                    }
                }),
            button("Reset")
                .on_press(message_mapper(ColorPickerMessage::Reset))
                .style(|theme, status| {
                    let palette = PsocTheme::Dark.palette();
                    button::Style {
                        background: Some(iced::Background::Color(palette.surface)),
                        text_color: palette.text,
                        border: iced::Border {
                            color: palette.border,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..button::secondary(theme, status)
                    }
                }),
        ]
        .spacing(spacing::MD);

        let content = column![
            text("Color Picker").size(18.0),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            row![
                color_preview,
                Space::new(Length::Fixed(spacing::MD), Length::Shrink),
                hex_input
            ]
            .align_y(iced::alignment::Vertical::Center),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            row![
                rgb_controls,
                Space::new(Length::Fixed(spacing::LG), Length::Shrink),
                hsl_controls
            ],
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            column![text("Presets").size(14.0), presets].spacing(spacing::SM),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            buttons,
        ]
        .spacing(spacing::SM)
        .padding(spacing::LG);

        container(content)
            .style(|_theme| {
                let palette = PsocTheme::Dark.palette();
                container::Style {
                    background: Some(iced::Background::Color(palette.surface)),
                    border: iced::Border {
                        color: palette.border,
                        width: 2.0,
                        radius: 8.0.into(),
                    },
                    shadow: iced::Shadow {
                        color: palette.shadow,
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 8.0,
                    },
                    ..Default::default()
                }
            })
            .width(Length::Fixed(500.0))
            .into()
    }
}

/// Parse hex color string (without #)
fn parse_hex_color(hex: &str) -> Result<(u8, u8, u8), std::num::ParseIntError> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        Ok((r, g, b))
    } else if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)?;
        let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
        let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
        Ok((r, g, b))
    } else {
        Err(u8::from_str_radix("", 16).unwrap_err())
    }
}
