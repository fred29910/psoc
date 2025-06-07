//! Brightness/Contrast adjustment dialog for PSOC Image Editor

use iced::{
    widget::{button, column, container, row, slider, text, text_input, Space},
    Element, Length,
};

use super::super::theme::spacing;

/// Brightness/Contrast adjustment dialog component
#[derive(Debug, Clone)]
pub struct BrightnessContrastDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current brightness value (-1.0 to 1.0)
    pub brightness: f32,
    /// Current contrast value (-1.0 to 1.0)
    pub contrast: f32,
    /// Brightness text input value
    pub brightness_text: String,
    /// Contrast text input value
    pub contrast_text: String,
    /// Whether real-time preview is enabled
    pub preview_enabled: bool,
    /// Whether changes have been applied
    pub has_changes: bool,
}

/// Messages for the Brightness/Contrast dialog
#[derive(Debug, Clone)]
pub enum BrightnessContrastMessage {
    /// Show the dialog
    Show,
    /// Hide the dialog
    Hide,
    /// Brightness slider changed
    BrightnessChanged(f32),
    /// Contrast slider changed
    ContrastChanged(f32),
    /// Brightness text input changed
    BrightnessTextChanged(String),
    /// Contrast text input changed
    ContrastTextChanged(String),
    /// Toggle preview mode
    TogglePreview,
    /// Reset values to defaults
    Reset,
    /// Apply changes
    Apply,
    /// Cancel changes
    Cancel,
}

impl Default for BrightnessContrastDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl BrightnessContrastDialog {
    /// Create a new Brightness/Contrast dialog
    pub fn new() -> Self {
        Self {
            visible: false,
            brightness: 0.0,
            contrast: 0.0,
            brightness_text: "0.0".to_string(),
            contrast_text: "0.0".to_string(),
            preview_enabled: true,
            has_changes: false,
        }
    }

    /// Show the dialog
    pub fn show(&mut self) {
        self.visible = true;
        self.reset_values();
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.visible = false;
        self.has_changes = false;
    }

    /// Reset values to defaults
    pub fn reset_values(&mut self) {
        self.brightness = 0.0;
        self.contrast = 0.0;
        self.brightness_text = "0.0".to_string();
        self.contrast_text = "0.0".to_string();
        self.has_changes = false;
    }

    /// Set brightness value
    pub fn set_brightness(&mut self, value: f32) {
        let clamped = value.clamp(-1.0, 1.0);
        self.brightness = clamped;
        self.brightness_text = format!("{:.2}", clamped);
        self.has_changes = self.brightness != 0.0 || self.contrast != 0.0;
    }

    /// Set contrast value
    pub fn set_contrast(&mut self, value: f32) {
        let clamped = value.clamp(-1.0, 1.0);
        self.contrast = clamped;
        self.contrast_text = format!("{:.2}", clamped);
        self.has_changes = self.brightness != 0.0 || self.contrast != 0.0;
    }

    /// Update brightness from text input
    pub fn update_brightness_from_text(&mut self, text: String) {
        self.brightness_text = text.clone();
        if let Ok(value) = text.parse::<f32>() {
            let clamped = value.clamp(-1.0, 1.0);
            self.brightness = clamped;
            self.has_changes = self.brightness != 0.0 || self.contrast != 0.0;
        }
    }

    /// Update contrast from text input
    pub fn update_contrast_from_text(&mut self, text: String) {
        self.contrast_text = text.clone();
        if let Ok(value) = text.parse::<f32>() {
            let clamped = value.clamp(-1.0, 1.0);
            self.contrast = clamped;
            self.has_changes = self.brightness != 0.0 || self.contrast != 0.0;
        }
    }

    /// Toggle preview mode
    pub fn toggle_preview(&mut self) {
        self.preview_enabled = !self.preview_enabled;
    }

    /// Update the dialog state
    pub fn update(&mut self, message: BrightnessContrastMessage) {
        match message {
            BrightnessContrastMessage::Show => self.show(),
            BrightnessContrastMessage::Hide => self.hide(),
            BrightnessContrastMessage::BrightnessChanged(value) => self.set_brightness(value),
            BrightnessContrastMessage::ContrastChanged(value) => self.set_contrast(value),
            BrightnessContrastMessage::BrightnessTextChanged(text) => {
                self.update_brightness_from_text(text)
            }
            BrightnessContrastMessage::ContrastTextChanged(text) => {
                self.update_contrast_from_text(text)
            }
            BrightnessContrastMessage::TogglePreview => self.toggle_preview(),
            BrightnessContrastMessage::Reset => self.reset_values(),
            BrightnessContrastMessage::Apply => {
                // Apply logic will be handled by the parent application
                self.has_changes = false;
            }
            BrightnessContrastMessage::Cancel => {
                self.reset_values();
                self.hide();
            }
        }
    }

    /// Get current brightness value
    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    /// Get current contrast value
    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    /// Check if preview is enabled
    pub fn is_preview_enabled(&self) -> bool {
        self.preview_enabled
    }

    /// Check if there are unsaved changes
    pub fn has_changes(&self) -> bool {
        self.has_changes
    }

    /// Render the Brightness/Contrast dialog
    pub fn view<Message>(
        &self,
        message_mapper: fn(BrightnessContrastMessage) -> Message,
    ) -> Element<Message>
    where
        Message: Clone + 'static,
    {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        // Create the dialog content
        let content = container(
            column![
                // Header
                container(text("Brightness/Contrast").size(18.0).style(|_theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::WHITE),
                    }
                }))
                .padding(spacing::MD)
                .width(Length::Fill),
                // Brightness control
                container(
                    column![
                        text("Brightness")
                            .size(14.0)
                            .style(|_theme| iced::widget::text::Style {
                                color: Some(iced::Color::WHITE)
                            }),
                        row![
                            slider(-1.0..=1.0, self.brightness, move |value| {
                                message_mapper(BrightnessContrastMessage::BrightnessChanged(value))
                            })
                            .width(Length::FillPortion(3)),
                            text_input("0.0", &self.brightness_text)
                                .on_input(move |text| {
                                    message_mapper(
                                        BrightnessContrastMessage::BrightnessTextChanged(text),
                                    )
                                })
                                .width(Length::FillPortion(1))
                                .size(12.0),
                        ]
                        .spacing(spacing::SM)
                        .align_y(iced::alignment::Vertical::Center),
                    ]
                    .spacing(spacing::XS)
                )
                .padding(spacing::MD)
                .width(Length::Fill),
                // Contrast control
                container(
                    column![
                        text("Contrast")
                            .size(14.0)
                            .style(|_theme| iced::widget::text::Style {
                                color: Some(iced::Color::WHITE)
                            }),
                        row![
                            slider(-1.0..=1.0, self.contrast, move |value| {
                                message_mapper(BrightnessContrastMessage::ContrastChanged(value))
                            })
                            .width(Length::FillPortion(3)),
                            text_input("0.0", &self.contrast_text)
                                .on_input(move |text| {
                                    message_mapper(BrightnessContrastMessage::ContrastTextChanged(
                                        text,
                                    ))
                                })
                                .width(Length::FillPortion(1))
                                .size(12.0),
                        ]
                        .spacing(spacing::SM)
                        .align_y(iced::alignment::Vertical::Center),
                    ]
                    .spacing(spacing::XS)
                )
                .padding(spacing::MD)
                .width(Length::Fill),
                // Preview toggle
                container(
                    button(
                        text(if self.preview_enabled {
                            "Preview: ON"
                        } else {
                            "Preview: OFF"
                        })
                        .size(12.0)
                    )
                    .on_press(message_mapper(BrightnessContrastMessage::TogglePreview))
                    .padding([4.0, 8.0])
                )
                .padding(spacing::SM)
                .width(Length::Fill),
                // Button row
                container(
                    row![
                        button(text("Reset").size(12.0))
                            .on_press(message_mapper(BrightnessContrastMessage::Reset))
                            .padding([6.0, 12.0]),
                        Space::new(Length::Fill, Length::Shrink),
                        button(text("Cancel").size(12.0))
                            .on_press(message_mapper(BrightnessContrastMessage::Cancel))
                            .padding([6.0, 12.0]),
                        button(text("Apply").size(12.0))
                            .on_press(message_mapper(BrightnessContrastMessage::Apply))
                            .padding([6.0, 12.0])
                            .style(button::primary),
                    ]
                    .spacing(spacing::SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .padding(spacing::MD)
                .width(Length::Fill),
            ]
            .spacing(spacing::XS),
        )
        .padding(spacing::LG)
        .width(Length::Fixed(400.0))
        .style(container::bordered_box);

        // Create modal overlay
        container(
            container(content)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.0, 0.0, 0.0, 0.7,
            ))),
            ..Default::default()
        })
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_contrast_dialog_creation() {
        let dialog = BrightnessContrastDialog::new();
        assert!(!dialog.visible);
        assert_eq!(dialog.brightness, 0.0);
        assert_eq!(dialog.contrast, 0.0);
        assert_eq!(dialog.brightness_text, "0.0");
        assert_eq!(dialog.contrast_text, "0.0");
        assert!(dialog.preview_enabled);
        assert!(!dialog.has_changes);
    }

    #[test]
    fn test_brightness_contrast_dialog_show_hide() {
        let mut dialog = BrightnessContrastDialog::new();

        // Test show
        dialog.show();
        assert!(dialog.visible);

        // Test hide
        dialog.hide();
        assert!(!dialog.visible);
        assert!(!dialog.has_changes);
    }

    #[test]
    fn test_brightness_contrast_dialog_value_setting() {
        let mut dialog = BrightnessContrastDialog::new();

        // Test brightness setting
        dialog.set_brightness(0.5);
        assert_eq!(dialog.brightness, 0.5);
        assert_eq!(dialog.brightness_text, "0.50");
        assert!(dialog.has_changes);

        // Test contrast setting
        dialog.set_contrast(-0.3);
        assert_eq!(dialog.contrast, -0.3);
        assert_eq!(dialog.contrast_text, "-0.30");
        assert!(dialog.has_changes);

        // Test clamping
        dialog.set_brightness(2.0);
        assert_eq!(dialog.brightness, 1.0);

        dialog.set_contrast(-2.0);
        assert_eq!(dialog.contrast, -1.0);
    }

    #[test]
    fn test_brightness_contrast_dialog_text_input() {
        let mut dialog = BrightnessContrastDialog::new();

        // Test valid text input
        dialog.update_brightness_from_text("0.75".to_string());
        assert_eq!(dialog.brightness_text, "0.75");
        assert_eq!(dialog.brightness, 0.75);

        dialog.update_contrast_from_text("-0.25".to_string());
        assert_eq!(dialog.contrast_text, "-0.25");
        assert_eq!(dialog.contrast, -0.25);

        // Test invalid text input (should keep text but not update value)
        let old_brightness = dialog.brightness;
        dialog.update_brightness_from_text("invalid".to_string());
        assert_eq!(dialog.brightness_text, "invalid");
        assert_eq!(dialog.brightness, old_brightness);
    }

    #[test]
    fn test_brightness_contrast_dialog_preview_toggle() {
        let mut dialog = BrightnessContrastDialog::new();
        assert!(dialog.preview_enabled);

        dialog.toggle_preview();
        assert!(!dialog.preview_enabled);

        dialog.toggle_preview();
        assert!(dialog.preview_enabled);
    }

    #[test]
    fn test_brightness_contrast_dialog_reset() {
        let mut dialog = BrightnessContrastDialog::new();

        // Set some values
        dialog.set_brightness(0.5);
        dialog.set_contrast(-0.3);
        dialog.toggle_preview();

        // Reset
        dialog.reset_values();
        assert_eq!(dialog.brightness, 0.0);
        assert_eq!(dialog.contrast, 0.0);
        assert_eq!(dialog.brightness_text, "0.0");
        assert_eq!(dialog.contrast_text, "0.0");
        assert!(!dialog.has_changes);
    }

    #[test]
    fn test_brightness_contrast_dialog_update() {
        let mut dialog = BrightnessContrastDialog::new();

        // Test show message
        dialog.update(BrightnessContrastMessage::Show);
        assert!(dialog.visible);

        // Test brightness change
        dialog.update(BrightnessContrastMessage::BrightnessChanged(0.4));
        assert_eq!(dialog.brightness, 0.4);

        // Test contrast change
        dialog.update(BrightnessContrastMessage::ContrastChanged(-0.2));
        assert_eq!(dialog.contrast, -0.2);

        // Test text changes
        dialog.update(BrightnessContrastMessage::BrightnessTextChanged(
            "0.8".to_string(),
        ));
        assert_eq!(dialog.brightness_text, "0.8");
        assert_eq!(dialog.brightness, 0.8);

        // Test preview toggle
        let old_preview = dialog.preview_enabled;
        dialog.update(BrightnessContrastMessage::TogglePreview);
        assert_eq!(dialog.preview_enabled, !old_preview);

        // Test reset
        dialog.update(BrightnessContrastMessage::Reset);
        assert_eq!(dialog.brightness, 0.0);
        assert_eq!(dialog.contrast, 0.0);

        // Test apply
        dialog.set_brightness(0.5);
        assert!(dialog.has_changes);
        dialog.update(BrightnessContrastMessage::Apply);
        assert!(!dialog.has_changes);

        // Test cancel
        dialog.set_brightness(0.3);
        dialog.update(BrightnessContrastMessage::Cancel);
        assert!(!dialog.visible);
        assert_eq!(dialog.brightness, 0.0);
    }

    #[test]
    fn test_brightness_contrast_dialog_getters() {
        let mut dialog = BrightnessContrastDialog::new();

        dialog.set_brightness(0.6);
        dialog.set_contrast(-0.4);
        dialog.toggle_preview();

        assert_eq!(dialog.brightness(), 0.6);
        assert_eq!(dialog.contrast(), -0.4);
        assert!(!dialog.is_preview_enabled());
        assert!(dialog.has_changes());
    }
}
