//! About dialog for PSOC Image Editor

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length,
};

use super::super::theme::spacing;
use crate::{DESCRIPTION, NAME, VERSION};

/// About dialog component
#[derive(Debug, Clone, Default)]
pub struct AboutDialog {
    /// Whether the dialog is visible
    pub visible: bool,
}

/// Messages for the About dialog
#[derive(Debug, Clone)]
pub enum AboutMessage {
    /// Show the dialog
    Show,
    /// Hide the dialog
    Hide,
}

impl AboutDialog {
    /// Create a new About dialog
    pub fn new() -> Self {
        Self { visible: false }
    }

    /// Show the dialog
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Update the dialog state
    pub fn update(&mut self, message: AboutMessage) {
        match message {
            AboutMessage::Show => self.show(),
            AboutMessage::Hide => self.hide(),
        }
    }

    /// Render the About dialog
    pub fn view<Message>(&self, on_close: Message) -> Element<Message>
    where
        Message: Clone + 'static,
    {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        // Create the dialog content
        let content =
            container(
                column![
                    // Header
                    container(
                        column![
                            text(NAME)
                                .size(24.0)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE)
                                }),
                            text(format!("Version {}", VERSION))
                                .size(14.0)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8))
                                }),
                        ]
                        .align_x(iced::alignment::Horizontal::Center)
                        .spacing(spacing::SM)
                    )
                    .padding(spacing::LG)
                    .width(Length::Fill),
                    // Description
                    container(text(DESCRIPTION).size(12.0).style(|_theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9)),
                        }
                    }))
                    .padding(spacing::MD)
                    .width(Length::Fill),
                    // System Information
                    container(
                        column![
                            text("System Information").size(14.0).style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            }),
                            Space::new(Length::Shrink, Length::Fixed(8.0)),
                            info_row("Rust Version", "1.70+"),
                            info_row("Target", std::env::consts::ARCH),
                            info_row(
                                "Build Profile",
                                if cfg!(debug_assertions) {
                                    "Debug"
                                } else {
                                    "Release"
                                }
                            ),
                            info_row("GUI Framework", "iced"),
                        ]
                        .spacing(spacing::SM)
                    )
                    .padding(spacing::MD)
                    .width(Length::Fill),
                    // License Information
                    container(
                        column![
                            text("License")
                                .size(14.0)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE)
                                }),
                            Space::new(Length::Shrink, Length::Fixed(8.0)),
                            text("Licensed under MIT OR Apache-2.0")
                                .size(11.0)
                                .style(|_theme| iced::widget::text::Style {
                                    color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8))
                                }),
                        ]
                        .spacing(spacing::SM)
                    )
                    .padding(spacing::MD)
                    .width(Length::Fill),
                    // Credits
                    container(
                        column![
                            text("Development Team").size(14.0).style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::WHITE),
                                }
                            }),
                            Space::new(Length::Shrink, Length::Fixed(8.0)),
                            text("PSOC Development Team").size(11.0).style(|_theme| {
                                iced::widget::text::Style {
                                    color: Some(iced::Color::from_rgb(0.8, 0.8, 0.8)),
                                }
                            }),
                        ]
                        .spacing(spacing::SM)
                    )
                    .padding(spacing::MD)
                    .width(Length::Fill),
                    // Close button
                    container(
                        button(text("Close").size(12.0).style(|_theme| {
                            iced::widget::text::Style {
                                color: Some(iced::Color::WHITE),
                            }
                        }))
                        .on_press(on_close)
                        .padding([spacing::SM, spacing::LG])
                    )
                    .padding(spacing::LG)
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                ]
                .spacing(spacing::SM),
            )
            .width(Length::Fixed(400.0))
            .height(Length::Shrink)
            .padding(spacing::LG);

        // Create modal overlay
        container(
            container(content)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgba(
                0.0, 0.0, 0.0, 0.7,
            ))),
            ..Default::default()
        })
        .into()
    }
}

/// Create an information row for the about dialog
fn info_row<Message: 'static>(
    label: &'static str,
    value: &'static str,
) -> Element<'static, Message> {
    row![
        text(format!("{}:", label))
            .size(11.0)
            .width(Length::Fixed(100.0))
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7))
            }),
        text(value)
            .size(11.0)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.9, 0.9, 0.9))
            }),
    ]
    .spacing(spacing::SM)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_about_dialog_creation() {
        let dialog = AboutDialog::new();
        assert!(!dialog.visible);
    }

    #[test]
    fn test_about_dialog_show_hide() {
        let mut dialog = AboutDialog::new();

        dialog.show();
        assert!(dialog.visible);

        dialog.hide();
        assert!(!dialog.visible);
    }

    #[test]
    fn test_about_dialog_update() {
        let mut dialog = AboutDialog::new();

        dialog.update(AboutMessage::Show);
        assert!(dialog.visible);

        dialog.update(AboutMessage::Hide);
        assert!(!dialog.visible);
    }
}
