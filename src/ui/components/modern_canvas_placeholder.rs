//! Modern canvas placeholder component with enhanced visual design
//! Provides an attractive placeholder when no document is open

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Alignment,
};

use crate::ui::theme::PsocTheme;
use crate::ui::styles::modern_placeholder_style;
use super::Icon;

/// Message types for canvas placeholder interactions
#[derive(Debug, Clone)]
pub enum PlaceholderMessage<Message> {
    /// User clicked new document button
    NewDocument,
    /// User clicked open document button
    OpenDocument,
    /// Custom message from parent
    Custom(Message),
}

/// Modern canvas placeholder component
pub fn modern_canvas_placeholder<Message: Clone + 'static>(
    theme: PsocTheme,
    on_message: impl Fn(PlaceholderMessage<Message>) -> Message + 'static + Copy,
) -> Element<'static, Message> {
    let palette = theme.palette();

    // Create the main content
    let content = column![
        // Icon section
        container(
            text("🎨")
                .size(64.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(palette.tech_blue),
                })
        )
        .center_x(Length::Fill)
        .padding([0.0, 24.0]),

        // Title
        container(
            text("Welcome to PSOC Image Editor")
                .size(24.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(palette.text),
                })
        )
        .center_x(Length::Fill)
        .padding([0.0, 8.0]),

        // Subtitle
        container(
            text("Create a new document or open an existing one to get started")
                .size(14.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgba(
                        palette.text.r,
                        palette.text.g,
                        palette.text.b,
                        0.7
                    )),
                })
        )
        .center_x(Length::Fill)
        .padding([0.0, 32.0]),

        // Action buttons
        row![
            // New document button
            button(
                row![
                    text("📄")
                        .size(16.0)
                        .style(move |_theme| iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        }),
                    Space::new(Length::Fixed(8.0), Length::Shrink),
                    text("New Document")
                        .size(14.0)
                        .style(move |_theme| iced::widget::text::Style {
                            color: Some(iced::Color::WHITE),
                        })
                ]
                .align_y(Alignment::Center)
            )
            .padding([12.0, 24.0])
            .style({
                let palette_clone = palette.clone();
                move |_theme, status| {
                    modern_placeholder_button_style(palette_clone.clone(), status, true)
                }
            })
            .on_press(on_message(PlaceholderMessage::NewDocument)),

            Space::new(Length::Fixed(16.0), Length::Shrink),

            // Open document button
            button(
                row![
                    text("📁")
                        .size(16.0)
                        .style(move |_theme| iced::widget::text::Style {
                            color: Some(palette.text),
                        }),
                    Space::new(Length::Fixed(8.0), Length::Shrink),
                    text("Open Document")
                        .size(14.0)
                        .style(move |_theme| iced::widget::text::Style {
                            color: Some(palette.text),
                        })
                ]
                .align_y(Alignment::Center)
            )
            .padding([12.0, 24.0])
            .style({
                let palette_clone = palette.clone();
                move |_theme, status| {
                    modern_placeholder_button_style(palette_clone.clone(), status, false)
                }
            })
            .on_press(on_message(PlaceholderMessage::OpenDocument)),
        ]
        .align_y(Alignment::Center),

        Space::new(Length::Shrink, Length::Fixed(24.0)),

        // Quick tips section
        container(
            column![
                text("Quick Tips:")
                    .size(16.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(palette.text),
                    }),
                
                Space::new(Length::Shrink, Length::Fixed(8.0)),
                
                text("• Use Ctrl+N to create a new document")
                    .size(12.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgba(
                            palette.text.r,
                            palette.text.g,
                            palette.text.b,
                            0.8
                        )),
                    }),
                
                text("• Use Ctrl+O to open an existing document")
                    .size(12.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgba(
                            palette.text.r,
                            palette.text.g,
                            palette.text.b,
                            0.8
                        )),
                    }),
                
                text("• Drag and drop files to open them")
                    .size(12.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgba(
                            palette.text.r,
                            palette.text.g,
                            palette.text.b,
                            0.8
                        )),
                    }),
            ]
            .spacing(4.0)
        )
        .center_x(Length::Fill),
    ]
    .spacing(0.0)
    .align_x(Alignment::Center);

    // Wrap in styled container
    container(content)
        .width(Length::Fixed(400.0))
        .padding(48.0)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_theme| modern_placeholder_style(&theme))
        .into()
}

/// Modern button style for placeholder buttons
fn modern_placeholder_button_style(
    palette: crate::ui::theme::ColorPalette,
    status: iced::widget::button::Status,
    is_primary: bool,
) -> iced::widget::button::Style {
    let (background, text_color, shadow) = if is_primary {
        // Primary button (New Document)
        match status {
            iced::widget::button::Status::Hovered => (
                Some(iced::Background::Color(iced::Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.9
                ))),
                iced::Color::WHITE,
                iced::Shadow {
                    color: iced::Color::from_rgba(
                        palette.tech_blue.r,
                        palette.tech_blue.g,
                        palette.tech_blue.b,
                        0.4
                    ),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
            ),
            iced::widget::button::Status::Active => (
                Some(iced::Background::Color(iced::Color::from_rgba(
                    palette.tech_blue.r,
                    palette.tech_blue.g,
                    palette.tech_blue.b,
                    0.7
                ))),
                iced::Color::WHITE,
                iced::Shadow {
                    color: iced::Color::TRANSPARENT,
                    offset: iced::Vector::new(0.0, 0.0),
                    blur_radius: 0.0,
                },
            ),
            _ => (
                Some(iced::Background::Color(palette.tech_blue)),
                iced::Color::WHITE,
                iced::Shadow {
                    color: iced::Color::from_rgba(
                        palette.tech_blue.r,
                        palette.tech_blue.g,
                        palette.tech_blue.b,
                        0.2
                    ),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            ),
        }
    } else {
        // Secondary button (Open Document)
        match status {
            iced::widget::button::Status::Hovered => (
                Some(iced::Background::Color(iced::Color::from_rgba(
                    palette.glass_bg_medium.r,
                    palette.glass_bg_medium.g,
                    palette.glass_bg_medium.b,
                    0.8
                ))),
                palette.text,
                iced::Shadow {
                    color: palette.shadow_color(0.1),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            ),
            iced::widget::button::Status::Active => (
                Some(iced::Background::Color(iced::Color::from_rgba(
                    palette.glass_bg_medium.r,
                    palette.glass_bg_medium.g,
                    palette.glass_bg_medium.b,
                    0.6
                ))),
                palette.text,
                iced::Shadow {
                    color: iced::Color::TRANSPARENT,
                    offset: iced::Vector::new(0.0, 0.0),
                    blur_radius: 0.0,
                },
            ),
            _ => (
                Some(iced::Background::Color(iced::Color::from_rgba(
                    palette.glass_bg_light.r,
                    palette.glass_bg_light.g,
                    palette.glass_bg_light.b,
                    0.6
                ))),
                palette.text,
                iced::Shadow {
                    color: palette.shadow_color(0.05),
                    offset: iced::Vector::new(0.0, 1.0),
                    blur_radius: 4.0,
                },
            ),
        }
    };

    iced::widget::button::Style {
        background,
        text_color,
        border: iced::Border {
            color: if is_primary {
                iced::Color::TRANSPARENT
            } else {
                iced::Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.3
                )
            },
            width: if is_primary { 0.0 } else { 1.0 },
            radius: 8.0.into(),
        },
        shadow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_creation() {
        let theme = PsocTheme::Dark;
        
        let placeholder = modern_canvas_placeholder(theme, |msg| match msg {
            PlaceholderMessage::NewDocument => "new",
            PlaceholderMessage::OpenDocument => "open",
            PlaceholderMessage::Custom(_) => "custom",
        });
        
        // Should return an Element
        let _ = placeholder;
    }

    #[test]
    fn test_placeholder_button_styles() {
        let theme = PsocTheme::Dark;
        let palette = theme.palette();
        
        let states = [
            iced::widget::button::Status::Disabled,
            iced::widget::button::Status::Hovered,
            iced::widget::button::Status::Active,
        ];

        for status in states {
            // Test primary button
            let primary_style = modern_placeholder_button_style(palette.clone(), status, true);
            assert!(primary_style.background.is_some());
            
            // Test secondary button
            let secondary_style = modern_placeholder_button_style(palette.clone(), status, false);
            assert!(secondary_style.background.is_some());
            
            // Primary button should have different styling
            if let (Some(iced::Background::Color(primary_bg)), Some(iced::Background::Color(secondary_bg))) = 
                (primary_style.background, secondary_style.background) {
                // Primary should be more blue-ish
                assert!(primary_bg.b > secondary_bg.b || primary_bg.g > secondary_bg.g);
            }
        }
    }

    #[test]
    fn test_placeholder_message_types() {
        // Test message enum variants
        let _new_msg: PlaceholderMessage<&str> = PlaceholderMessage::NewDocument;
        let _open_msg: PlaceholderMessage<&str> = PlaceholderMessage::OpenDocument;
        let _custom_msg: PlaceholderMessage<&str> = PlaceholderMessage::Custom("test");
        
        // Should compile without issues
    }
}
