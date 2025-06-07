//! Gaussian Blur filter dialog for PSOC Image Editor

use iced::{
    widget::{button, column, container, row, slider, text, text_input, Space},
    Element, Length,
};

use super::super::theme::spacing;

/// Gaussian Blur filter dialog component
#[derive(Debug, Clone)]
pub struct GaussianBlurDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current blur radius value (0.0 to 100.0)
    pub radius: f32,
    /// Current quality factor (1.0 to 3.0)
    pub quality: f32,
    /// Radius text input value
    pub radius_text: String,
    /// Quality text input value
    pub quality_text: String,
    /// Whether real-time preview is enabled
    pub preview_enabled: bool,
    /// Whether changes have been applied
    pub has_changes: bool,
}

/// Messages for the Gaussian Blur dialog
#[derive(Debug, Clone)]
pub enum GaussianBlurMessage {
    /// Show the dialog
    Show,
    /// Hide the dialog
    Hide,
    /// Radius value changed via slider
    RadiusChanged(f32),
    /// Quality value changed via slider
    QualityChanged(f32),
    /// Radius text input changed
    RadiusTextChanged(String),
    /// Quality text input changed
    QualityTextChanged(String),
    /// Toggle preview mode
    TogglePreview,
    /// Reset values to defaults
    Reset,
    /// Apply the filter
    Apply,
    /// Cancel and close dialog
    Cancel,
}

impl Default for GaussianBlurDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl GaussianBlurDialog {
    /// Create a new Gaussian Blur dialog
    pub fn new() -> Self {
        Self {
            visible: false,
            radius: 1.0,
            quality: 2.0,
            radius_text: "1.0".to_string(),
            quality_text: "2.0".to_string(),
            preview_enabled: false,
            has_changes: false,
        }
    }

    /// Show the dialog
    pub fn show(&mut self) {
        self.visible = true;
        self.has_changes = false;
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.visible = false;
        self.has_changes = false;
    }

    /// Set the radius value
    pub fn set_radius(&mut self, radius: f32) {
        let clamped_radius = radius.clamp(0.0, 100.0);
        if (self.radius - clamped_radius).abs() > f32::EPSILON {
            self.radius = clamped_radius;
            self.radius_text = format!("{:.1}", clamped_radius);
            self.has_changes = true;
        }
    }

    /// Set the quality value
    pub fn set_quality(&mut self, quality: f32) {
        let clamped_quality = quality.clamp(1.0, 3.0);
        if (self.quality - clamped_quality).abs() > f32::EPSILON {
            self.quality = clamped_quality;
            self.quality_text = format!("{:.1}", clamped_quality);
            self.has_changes = true;
        }
    }

    /// Set radius from text input
    pub fn set_radius_text(&mut self, text: String) {
        self.radius_text = text.clone();
        if let Ok(value) = text.parse::<f32>() {
            self.set_radius(value);
        }
    }

    /// Set quality from text input
    pub fn set_quality_text(&mut self, text: String) {
        self.quality_text = text.clone();
        if let Ok(value) = text.parse::<f32>() {
            self.set_quality(value);
        }
    }

    /// Toggle preview mode
    pub fn toggle_preview(&mut self) {
        self.preview_enabled = !self.preview_enabled;
    }

    /// Reset values to defaults
    pub fn reset(&mut self) {
        self.radius = 1.0;
        self.quality = 2.0;
        self.radius_text = "1.0".to_string();
        self.quality_text = "2.0".to_string();
        self.has_changes = false;
    }

    /// Get the current radius value
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get the current quality value
    pub fn quality(&self) -> f32 {
        self.quality
    }

    /// Check if preview is enabled
    pub fn preview_enabled(&self) -> bool {
        self.preview_enabled
    }

    /// Check if there are unsaved changes
    pub fn has_changes(&self) -> bool {
        self.has_changes
    }

    /// Update the dialog state based on a message
    pub fn update(&mut self, message: GaussianBlurMessage) {
        match message {
            GaussianBlurMessage::Show => self.show(),
            GaussianBlurMessage::Hide => self.hide(),
            GaussianBlurMessage::RadiusChanged(value) => self.set_radius(value),
            GaussianBlurMessage::QualityChanged(value) => self.set_quality(value),
            GaussianBlurMessage::RadiusTextChanged(text) => self.set_radius_text(text),
            GaussianBlurMessage::QualityTextChanged(text) => self.set_quality_text(text),
            GaussianBlurMessage::TogglePreview => self.toggle_preview(),
            GaussianBlurMessage::Reset => self.reset(),
            GaussianBlurMessage::Apply => {
                // Apply logic handled by parent
                self.has_changes = false;
            }
            GaussianBlurMessage::Cancel => {
                self.reset();
                self.hide();
            }
        }
    }

    /// Render the Gaussian Blur dialog
    pub fn view<Message>(
        &self,
        message_mapper: fn(GaussianBlurMessage) -> Message,
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
                container(text("Gaussian Blur").size(18.0).style(|_theme| {
                    iced::widget::text::Style {
                        color: Some(iced::Color::WHITE),
                    }
                }))
                .padding(spacing::MD)
                .width(Length::Fill),
                // Radius control
                container(
                    column![
                        text("Radius")
                            .size(14.0)
                            .style(|_theme| iced::widget::text::Style {
                                color: Some(iced::Color::WHITE)
                            }),
                        row![
                            slider(0.0..=100.0, self.radius, move |value| {
                                message_mapper(GaussianBlurMessage::RadiusChanged(value))
                            })
                            .width(Length::FillPortion(3)),
                            text_input("1.0", &self.radius_text)
                                .on_input(move |text| {
                                    message_mapper(GaussianBlurMessage::RadiusTextChanged(text))
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
                // Quality control
                container(
                    column![
                        text("Quality")
                            .size(14.0)
                            .style(|_theme| iced::widget::text::Style {
                                color: Some(iced::Color::WHITE)
                            }),
                        row![
                            slider(1.0..=3.0, self.quality, move |value| {
                                message_mapper(GaussianBlurMessage::QualityChanged(value))
                            })
                            .width(Length::FillPortion(3)),
                            text_input("2.0", &self.quality_text)
                                .on_input(move |text| {
                                    message_mapper(GaussianBlurMessage::QualityTextChanged(text))
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
                    row![
                        button(text("Preview").size(12.0))
                            .on_press(message_mapper(GaussianBlurMessage::TogglePreview))
                            .padding([4.0, 8.0]),
                        text(if self.preview_enabled { "On" } else { "Off" })
                            .size(12.0)
                            .style(|_theme| iced::widget::text::Style {
                                color: Some(if self.preview_enabled {
                                    iced::Color::from_rgb(0.0, 1.0, 0.0)
                                } else {
                                    iced::Color::from_rgb(0.7, 0.7, 0.7)
                                })
                            }),
                    ]
                    .spacing(spacing::SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .padding(spacing::MD)
                .width(Length::Fill),
                // Action buttons
                container(
                    row![
                        button(text("Reset").size(12.0))
                            .on_press(message_mapper(GaussianBlurMessage::Reset))
                            .padding([6.0, 12.0]),
                        Space::new(Length::Fill, Length::Shrink),
                        button(text("Cancel").size(12.0))
                            .on_press(message_mapper(GaussianBlurMessage::Cancel))
                            .padding([6.0, 12.0]),
                        button(text("Apply").size(12.0))
                            .on_press(message_mapper(GaussianBlurMessage::Apply))
                            .padding([6.0, 12.0]),
                    ]
                    .spacing(spacing::SM)
                    .align_y(iced::alignment::Vertical::Center)
                )
                .padding(spacing::MD)
                .width(Length::Fill),
            ]
            .spacing(spacing::SM),
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
    fn test_gaussian_blur_dialog_creation() {
        let dialog = GaussianBlurDialog::new();
        assert!(!dialog.visible);
        assert_eq!(dialog.radius, 1.0);
        assert_eq!(dialog.quality, 2.0);
        assert_eq!(dialog.radius_text, "1.0");
        assert_eq!(dialog.quality_text, "2.0");
        assert!(!dialog.preview_enabled);
        assert!(!dialog.has_changes);
    }

    #[test]
    fn test_gaussian_blur_dialog_show_hide() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.show();
        assert!(dialog.visible);
        assert!(!dialog.has_changes);

        dialog.hide();
        assert!(!dialog.visible);
        assert!(!dialog.has_changes);
    }

    #[test]
    fn test_gaussian_blur_dialog_radius_setting() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.set_radius(5.0);
        assert_eq!(dialog.radius, 5.0);
        assert_eq!(dialog.radius_text, "5.0");
        assert!(dialog.has_changes);

        // Test clamping
        dialog.set_radius(-1.0);
        assert_eq!(dialog.radius, 0.0);

        dialog.set_radius(150.0);
        assert_eq!(dialog.radius, 100.0);
    }

    #[test]
    fn test_gaussian_blur_dialog_quality_setting() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.set_quality(2.5);
        assert_eq!(dialog.quality, 2.5);
        assert_eq!(dialog.quality_text, "2.5");
        assert!(dialog.has_changes);

        // Test clamping
        dialog.set_quality(0.5);
        assert_eq!(dialog.quality, 1.0);

        dialog.set_quality(5.0);
        assert_eq!(dialog.quality, 3.0);
    }

    #[test]
    fn test_gaussian_blur_dialog_text_input() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.set_radius_text("3.5".to_string());
        assert_eq!(dialog.radius_text, "3.5");
        assert_eq!(dialog.radius, 3.5);

        dialog.set_quality_text("1.8".to_string());
        assert_eq!(dialog.quality_text, "1.8");
        assert_eq!(dialog.quality, 1.8);

        // Test invalid input
        dialog.set_radius_text("invalid".to_string());
        assert_eq!(dialog.radius_text, "invalid");
        // Radius should remain unchanged
        assert_eq!(dialog.radius, 3.5);
    }

    #[test]
    fn test_gaussian_blur_dialog_preview_toggle() {
        let mut dialog = GaussianBlurDialog::new();

        assert!(!dialog.preview_enabled);

        dialog.toggle_preview();
        assert!(dialog.preview_enabled);

        dialog.toggle_preview();
        assert!(!dialog.preview_enabled);
    }

    #[test]
    fn test_gaussian_blur_dialog_reset() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.set_radius(10.0);
        dialog.set_quality(2.8);
        dialog.toggle_preview();

        dialog.reset();
        assert_eq!(dialog.radius, 1.0);
        assert_eq!(dialog.quality, 2.0);
        assert_eq!(dialog.radius_text, "1.0");
        assert_eq!(dialog.quality_text, "2.0");
        assert!(!dialog.has_changes);
        // Preview state should remain unchanged
        assert!(dialog.preview_enabled);
    }

    #[test]
    fn test_gaussian_blur_dialog_getters() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.set_radius(7.5);
        dialog.set_quality(2.2);
        dialog.toggle_preview();
        dialog.has_changes = true;

        assert_eq!(dialog.radius(), 7.5);
        assert_eq!(dialog.quality(), 2.2);
        assert!(dialog.preview_enabled());
        assert!(dialog.has_changes());
    }

    #[test]
    fn test_gaussian_blur_dialog_update() {
        let mut dialog = GaussianBlurDialog::new();

        dialog.update(GaussianBlurMessage::Show);
        assert!(dialog.visible);

        dialog.update(GaussianBlurMessage::RadiusChanged(8.0));
        assert_eq!(dialog.radius, 8.0);

        dialog.update(GaussianBlurMessage::QualityChanged(2.7));
        assert_eq!(dialog.quality, 2.7);

        dialog.update(GaussianBlurMessage::TogglePreview);
        assert!(dialog.preview_enabled);

        dialog.update(GaussianBlurMessage::Reset);
        assert_eq!(dialog.radius, 1.0);
        assert_eq!(dialog.quality, 2.0);

        dialog.update(GaussianBlurMessage::Apply);
        assert!(!dialog.has_changes);

        dialog.update(GaussianBlurMessage::Cancel);
        assert!(!dialog.visible);
        assert_eq!(dialog.radius, 1.0);
    }
}
