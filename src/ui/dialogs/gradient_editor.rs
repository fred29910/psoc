//! Gradient editor dialog for creating and editing gradients
//!
//! This module provides a comprehensive gradient editor interface that allows users to:
//! - Create and edit gradients
//! - Manage color stops
//! - Preview gradients in real-time
//! - Select gradient types and interpolation methods

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length,
};
use psoc_core::{Gradient, GradientType, InterpolationMethod, RgbaPixel};

use crate::ui::theme::spacing;

/// Messages for the gradient editor dialog
#[derive(Debug, Clone)]
pub enum GradientEditorMessage {
    /// Show the gradient editor dialog
    Show,
    /// Hide the gradient editor dialog
    Hide,
    /// Set gradient type
    SetGradientType(GradientType),
    /// Set interpolation method
    SetInterpolationMethod(InterpolationMethod),
    /// Toggle repeat mode
    ToggleRepeat(bool),
    /// Add a new color stop
    AddColorStop,
    /// Remove a color stop
    RemoveColorStop(u32),
    /// Update color stop position
    UpdateColorStopPosition(u32, f32),
    /// Update color stop color
    UpdateColorStopColor(u32, RgbaPixel),
    /// Select a color stop for editing
    SelectColorStop(u32),
    /// Apply the gradient
    Apply,
    /// Cancel editing
    Cancel,
    /// Reset to default gradient
    Reset,
}

/// Gradient editor dialog state
#[derive(Debug, Clone)]
pub struct GradientEditor {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current gradient being edited
    pub current_gradient: Gradient,
    /// Selected color stop index
    pub selected_stop: Option<u32>,
    /// Whether changes have been made
    pub has_changes: bool,
}

impl Default for GradientEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientEditor {
    /// Create a new gradient editor
    pub fn new() -> Self {
        Self {
            visible: false,
            current_gradient: Gradient::default(),
            selected_stop: None,
            has_changes: false,
        }
    }

    /// Show the gradient editor with a specific gradient
    pub fn show_with_gradient(&mut self, gradient: Gradient) {
        self.current_gradient = gradient;
        self.selected_stop = None;
        self.has_changes = false;
        self.visible = true;
    }

    /// Show the gradient editor
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the gradient editor
    pub fn hide(&mut self) {
        self.visible = false;
        self.has_changes = false;
    }

    /// Update the gradient editor state
    pub fn update(&mut self, message: GradientEditorMessage) -> Option<Gradient> {
        match message {
            GradientEditorMessage::Show => {
                self.show();
                None
            }
            GradientEditorMessage::Hide => {
                self.hide();
                None
            }
            GradientEditorMessage::SetGradientType(gradient_type) => {
                self.current_gradient.gradient_type = gradient_type;
                self.has_changes = true;
                None
            }
            GradientEditorMessage::SetInterpolationMethod(method) => {
                self.current_gradient.interpolation = method;
                self.has_changes = true;
                None
            }
            GradientEditorMessage::ToggleRepeat(repeat) => {
                self.current_gradient.repeat = repeat;
                self.has_changes = true;
                None
            }
            GradientEditorMessage::AddColorStop => {
                let position = if self.current_gradient.stops.is_empty() {
                    0.5
                } else {
                    // Find a good position between existing stops
                    let sorted_stops = self.current_gradient.sorted_stops();
                    if sorted_stops.len() == 1 {
                        1.0
                    } else {
                        // Find the largest gap between stops
                        let mut max_gap = 0.0;
                        let mut best_position = 0.5;

                        for i in 0..sorted_stops.len() - 1 {
                            let gap = sorted_stops[i + 1].position - sorted_stops[i].position;
                            if gap > max_gap {
                                max_gap = gap;
                                best_position =
                                    (sorted_stops[i].position + sorted_stops[i + 1].position) / 2.0;
                            }
                        }
                        best_position
                    }
                };

                let color_stop =
                    psoc_core::ColorStop::new(position, RgbaPixel::new(128, 128, 128, 255));
                let key = self.current_gradient.add_stop(color_stop);
                self.selected_stop = Some(key);
                self.has_changes = true;
                None
            }
            GradientEditorMessage::RemoveColorStop(key) => {
                if self.current_gradient.stops.len() > 2 {
                    self.current_gradient.remove_stop(key);
                    if self.selected_stop == Some(key) {
                        self.selected_stop = None;
                    }
                    self.has_changes = true;
                }
                None
            }
            GradientEditorMessage::UpdateColorStopPosition(key, position) => {
                if let Some(stop) = self.current_gradient.stops.get_mut(&key) {
                    stop.position = position.clamp(0.0, 1.0);
                    self.has_changes = true;
                }
                None
            }
            GradientEditorMessage::UpdateColorStopColor(key, color) => {
                if let Some(stop) = self.current_gradient.stops.get_mut(&key) {
                    stop.color = color;
                    self.has_changes = true;
                }
                None
            }
            GradientEditorMessage::SelectColorStop(key) => {
                self.selected_stop = Some(key);
                None
            }
            GradientEditorMessage::Apply => {
                let gradient = self.current_gradient.clone();
                self.hide();
                Some(gradient)
            }
            GradientEditorMessage::Cancel => {
                self.hide();
                None
            }
            GradientEditorMessage::Reset => {
                self.current_gradient = Gradient::default();
                self.selected_stop = None;
                self.has_changes = true;
                None
            }
        }
    }

    /// Create the gradient editor dialog view
    pub fn view<'a, F>(&'a self, message_mapper: F) -> Element<'a, GradientEditorMessage>
    where
        F: Fn(GradientEditorMessage) -> GradientEditorMessage + Copy + 'a,
    {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        let title = text("Gradient Editor").size(18.0);

        // Gradient type selection
        let gradient_type_controls = row![
            text("Type:").size(14.0).width(Length::Fixed(80.0)),
            button("Linear").on_press(message_mapper(GradientEditorMessage::SetGradientType(
                GradientType::Linear
            ))),
            button("Radial").on_press(message_mapper(GradientEditorMessage::SetGradientType(
                GradientType::Radial
            ))),
            button("Angular").on_press(message_mapper(GradientEditorMessage::SetGradientType(
                GradientType::Angular
            ))),
            button("Diamond").on_press(message_mapper(GradientEditorMessage::SetGradientType(
                GradientType::Diamond
            ))),
        ]
        .spacing(spacing::SM);

        // Interpolation method selection
        let interpolation_controls = row![
            text("Interpolation:").size(14.0).width(Length::Fixed(80.0)),
            button("Linear").on_press(message_mapper(
                GradientEditorMessage::SetInterpolationMethod(InterpolationMethod::Linear)
            )),
            button("HSL").on_press(message_mapper(
                GradientEditorMessage::SetInterpolationMethod(InterpolationMethod::Hsl)
            )),
            button("HSV").on_press(message_mapper(
                GradientEditorMessage::SetInterpolationMethod(InterpolationMethod::Hsv)
            )),
            button("Smooth").on_press(message_mapper(
                GradientEditorMessage::SetInterpolationMethod(InterpolationMethod::Smooth)
            )),
        ]
        .spacing(spacing::SM);

        // Gradient preview (simplified for now)
        let preview =
            container(Space::new(Length::Fixed(300.0), Length::Fixed(40.0))).style(|_theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.8, 0.8, 0.8,
                    ))),
                    border: iced::Border {
                        color: iced::Color::BLACK,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            });

        // Action buttons
        let buttons = row![
            button("Add Stop").on_press(message_mapper(GradientEditorMessage::AddColorStop)),
            button("Reset").on_press(message_mapper(GradientEditorMessage::Reset)),
            Space::new(Length::Fill, Length::Shrink),
            button("Cancel").on_press(message_mapper(GradientEditorMessage::Cancel)),
            button("Apply").on_press(message_mapper(GradientEditorMessage::Apply)),
        ]
        .spacing(spacing::MD);

        let content = column![
            title,
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            gradient_type_controls,
            Space::new(Length::Shrink, Length::Fixed(spacing::SM)),
            interpolation_controls,
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            text("Preview:").size(14.0),
            preview,
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            buttons,
        ]
        .spacing(spacing::SM)
        .padding(spacing::LG);

        container(content)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(
                    0.95, 0.95, 0.95,
                ))),
                border: iced::Border {
                    color: iced::Color::BLACK,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
