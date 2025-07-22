//! Animated tool options panel component
//! Provides smooth transitions and modern styling for tool option panels

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Background, Color, Border, Shadow, Vector,
};

use crate::ui::theme::{PsocTheme, spacing};
use crate::ui::styles::glass_container_style;
use crate::ui::theme::GlassIntensity;
use crate::ui::animations::ToolOptionAnimationState;
use crate::tools::ToolType;
use crate::ui::components::ToolOptionControl;

/// Create an animated tool options panel
pub fn animated_tool_options_panel<Message: Clone + 'static>(
    tool_type: ToolType,
    options: Vec<ToolOptionControl<Message>>,
    animation_state: Option<ToolOptionAnimationState>,
) -> Element<'static, Message> {
    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    
    // Apply animation transformations
    let (opacity, width_factor, offset_x, scale) = if let Some(ref state) = animation_state {
        (state.opacity, state.width_factor, state.offset_x, state.scale)
    } else {
        (1.0, 1.0, 0.0, 1.0)
    };

    // Create tool options header
    let header = container(
        row![
            text(format!("{:?} Options", tool_type))
                .size(14.0)
                .style(move |_theme| iced::widget::text::Style {
                    color: Some(Color::WHITE),
                }),
            Space::new(Length::Fill, Length::Shrink),
            button(text("×").size(16.0))
                .width(Length::Fixed(24.0))
                .height(Length::Fixed(24.0))
                .style(|_theme, _status| iced::widget::button::Style {
                    background: Some(Background::Color(Color::from_rgba(0.8, 0.2, 0.2, 0.8))),
                    text_color: Color::WHITE,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 12.0.into(),
                    },
                    ..Default::default()
                }),
        ]
        .align_y(iced::alignment::Vertical::Center)
        .spacing(8.0)
    )
    .padding([12.0, 16.0]);

    // Create option controls with animations
    let mut option_elements = vec![header.into()];
    
    // For now, just show a placeholder for options
    if !options.is_empty() {
        option_elements.push(
            container(
                text(format!("{} options available", options.len()))
                    .size(12.0)
                    .style(move |_theme| iced::widget::text::Style {
                        color: Some(Color::from_rgba(0.8, 0.8, 0.8, opacity)),
                    })
            )
            .padding([8.0, 12.0])
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 0.5 * opacity))),
                border: Border {
                    color: Color::from_rgba(0.3, 0.3, 0.3, 0.5 * opacity),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .into()
        );
    }

    // Main content
    let content = column(option_elements)
        .spacing(spacing::SM);

    // Apply animation transformations
    let animated_content = container(content)
        .width(Length::Fixed(280.0 * width_factor))
        .style(move |_theme| {
            let mut style = glass_container_style(GlassIntensity::Medium, &psoc_theme);
            style.border = Border {
                radius: 16.0.into(),
                ..style.border
            };
            
            // Apply opacity from animation
            if let Some(Background::Color(color)) = style.background {
                style.background = Some(Background::Color(Color {
                    a: color.a * opacity,
                    ..color
                }));
            }
            
            style
        });

    // Apply transform effects
    let transformed_content = if scale != 1.0 || offset_x != 0.0 {
        // Note: iced doesn't have built-in transform support, so we simulate with padding/margin
        container(animated_content)
            .padding([offset_x.max(0.0), (-offset_x).max(0.0)])
            .into()
    } else {
        animated_content.into()
    };

    transformed_content
}


