//! Color palette management dialog for PSOC Image Editor

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Element, Length,
};
use psoc_core::RgbaPixel;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ui::theme::{spacing, PsocTheme};

/// Color palette dialog messages
#[derive(Debug, Clone)]
pub enum ColorPaletteMessage {
    /// Create a new palette
    NewPalette,
    /// Load a palette
    LoadPalette(String),
    /// Save current palette
    SavePalette,
    /// Delete a palette
    DeletePalette(String),
    /// Rename a palette
    RenamePalette(String, String),
    /// Add color to current palette
    AddColor(RgbaPixel),
    /// Remove color from current palette
    RemoveColor(usize),
    /// Select a color from palette
    SelectColor(RgbaPixel),
    /// Edit a color in palette
    EditColor(usize, RgbaPixel),
    /// Palette name changed
    PaletteNameChanged(String),
    /// Import palette from file
    ImportPalette,
    /// Export palette to file
    ExportPalette,
    /// Close dialog
    Close,
}

/// A color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// Palette name
    pub name: String,
    /// Colors in the palette
    pub colors: Vec<RgbaPixel>,
    /// Whether this is a built-in palette
    pub built_in: bool,
}

impl ColorPalette {
    /// Create a new empty palette
    pub fn new(name: String) -> Self {
        Self {
            name,
            colors: Vec::new(),
            built_in: false,
        }
    }

    /// Create a built-in palette
    pub fn built_in(name: String, colors: Vec<RgbaPixel>) -> Self {
        Self {
            name,
            colors,
            built_in: true,
        }
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, color: RgbaPixel) {
        if !self.colors.contains(&color) {
            self.colors.push(color);
        }
    }

    /// Remove a color from the palette
    pub fn remove_color(&mut self, index: usize) {
        if index < self.colors.len() {
            self.colors.remove(index);
        }
    }

    /// Edit a color in the palette
    pub fn edit_color(&mut self, index: usize, color: RgbaPixel) {
        if index < self.colors.len() {
            self.colors[index] = color;
        }
    }
}

/// Color palette dialog state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPaletteDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Available palettes
    pub palettes: HashMap<String, ColorPalette>,
    /// Currently selected palette
    pub current_palette: Option<String>,
    /// New palette name input
    pub new_palette_name: String,
    /// Selected color index for editing
    pub selected_color_index: Option<usize>,
}

impl Default for ColorPaletteDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorPaletteDialog {
    /// Create a new color palette dialog
    pub fn new() -> Self {
        let mut palettes = HashMap::new();

        // Add default palettes
        palettes.insert(
            "Basic Colors".to_string(),
            ColorPalette::built_in(
                "Basic Colors".to_string(),
                vec![
                    RgbaPixel::new(0, 0, 0, 255),       // Black
                    RgbaPixel::new(255, 255, 255, 255), // White
                    RgbaPixel::new(255, 0, 0, 255),     // Red
                    RgbaPixel::new(0, 255, 0, 255),     // Green
                    RgbaPixel::new(0, 0, 255, 255),     // Blue
                    RgbaPixel::new(255, 255, 0, 255),   // Yellow
                    RgbaPixel::new(255, 0, 255, 255),   // Magenta
                    RgbaPixel::new(0, 255, 255, 255),   // Cyan
                ],
            ),
        );

        palettes.insert(
            "Grayscale".to_string(),
            ColorPalette::built_in(
                "Grayscale".to_string(),
                vec![
                    RgbaPixel::new(0, 0, 0, 255),       // Black
                    RgbaPixel::new(64, 64, 64, 255),    // Dark Gray
                    RgbaPixel::new(128, 128, 128, 255), // Gray
                    RgbaPixel::new(192, 192, 192, 255), // Light Gray
                    RgbaPixel::new(255, 255, 255, 255), // White
                ],
            ),
        );

        palettes.insert(
            "Web Safe".to_string(),
            ColorPalette::built_in("Web Safe".to_string(), Self::generate_web_safe_colors()),
        );

        Self {
            visible: false,
            palettes,
            current_palette: Some("Basic Colors".to_string()),
            new_palette_name: String::new(),
            selected_color_index: None,
        }
    }

    /// Generate web safe colors (216 colors)
    fn generate_web_safe_colors() -> Vec<RgbaPixel> {
        let mut colors = Vec::new();
        let values = [0, 51, 102, 153, 204, 255];

        for &r in &values {
            for &g in &values {
                for &b in &values {
                    colors.push(RgbaPixel::new(r, g, b, 255));
                }
            }
        }

        colors
    }

    /// Show the dialog
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Check if the dialog is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the current palette
    pub fn current_palette(&self) -> Option<&ColorPalette> {
        self.current_palette
            .as_ref()
            .and_then(|name| self.palettes.get(name))
    }

    /// Get the current palette (mutable)
    pub fn current_palette_mut(&mut self) -> Option<&mut ColorPalette> {
        self.current_palette
            .as_ref()
            .and_then(|name| self.palettes.get_mut(name))
    }

    /// Get all palette names
    pub fn palette_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.palettes.keys().cloned().collect();
        names.sort();
        names
    }

    /// Handle color palette messages
    pub fn update(&mut self, message: ColorPaletteMessage) {
        match message {
            ColorPaletteMessage::NewPalette => {
                if !self.new_palette_name.is_empty() {
                    let name = self.new_palette_name.clone();
                    if !self.palettes.contains_key(&name) {
                        self.palettes
                            .insert(name.clone(), ColorPalette::new(name.clone()));
                        self.current_palette = Some(name);
                        self.new_palette_name.clear();
                    }
                }
            }
            ColorPaletteMessage::LoadPalette(name) => {
                if self.palettes.contains_key(&name) {
                    self.current_palette = Some(name);
                }
            }
            ColorPaletteMessage::SavePalette => {
                // Save current palette to file (implementation depends on file system)
                // For now, just keep in memory
            }
            ColorPaletteMessage::DeletePalette(name) => {
                if let Some(palette) = self.palettes.get(&name) {
                    if !palette.built_in {
                        self.palettes.remove(&name);
                        if self.current_palette.as_ref() == Some(&name) {
                            self.current_palette = self.palettes.keys().next().cloned();
                        }
                    }
                }
            }
            ColorPaletteMessage::RenamePalette(old_name, new_name) => {
                if let Some(mut palette) = self.palettes.remove(&old_name) {
                    if !palette.built_in && !self.palettes.contains_key(&new_name) {
                        palette.name = new_name.clone();
                        self.palettes.insert(new_name.clone(), palette);
                        if self.current_palette.as_ref() == Some(&old_name) {
                            self.current_palette = Some(new_name);
                        }
                    } else {
                        // Restore if rename failed
                        self.palettes.insert(old_name, palette);
                    }
                }
            }
            ColorPaletteMessage::AddColor(color) => {
                if let Some(palette) = self.current_palette_mut() {
                    if !palette.built_in {
                        palette.add_color(color);
                    }
                }
            }
            ColorPaletteMessage::RemoveColor(index) => {
                if let Some(palette) = self.current_palette_mut() {
                    if !palette.built_in {
                        palette.remove_color(index);
                    }
                }
            }
            ColorPaletteMessage::SelectColor(_color) => {
                // Color selection will be handled by parent
            }
            ColorPaletteMessage::EditColor(index, color) => {
                if let Some(palette) = self.current_palette_mut() {
                    if !palette.built_in {
                        palette.edit_color(index, color);
                    }
                }
            }
            ColorPaletteMessage::PaletteNameChanged(name) => {
                self.new_palette_name = name;
            }
            ColorPaletteMessage::ImportPalette => {
                // Import palette from file (implementation depends on file system)
            }
            ColorPaletteMessage::ExportPalette => {
                // Export current palette to file (implementation depends on file system)
            }
            ColorPaletteMessage::Close => {
                self.hide();
            }
        }
    }

    /// Create the color palette dialog view
    pub fn view<'a, F>(&'a self, message_mapper: F) -> Element<'a, ColorPaletteMessage>
    where
        F: Fn(ColorPaletteMessage) -> ColorPaletteMessage + Copy + 'a,
    {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        // Palette selection
        let palette_names = self.palette_names();
        let mut palette_buttons = row![].spacing(spacing::SM);
        for name in &palette_names {
            let is_current = self.current_palette.as_ref() == Some(name);
            let button_style = if is_current {
                button::primary
            } else {
                button::secondary
            };

            palette_buttons = palette_buttons.push(
                button(text(name.clone()).size(12.0))
                    .on_press(message_mapper(ColorPaletteMessage::LoadPalette(
                        name.clone(),
                    )))
                    .style(button_style),
            );
        }

        // New palette creation
        let new_palette_row = row![
            text_input("New palette name", &self.new_palette_name)
                .on_input(move |text| message_mapper(ColorPaletteMessage::PaletteNameChanged(text)))
                .width(Length::FillPortion(3))
                .size(12.0),
            button("Create")
                .on_press(message_mapper(ColorPaletteMessage::NewPalette))
                .style(button::primary),
        ]
        .spacing(spacing::SM)
        .align_y(iced::alignment::Vertical::Center);

        // Current palette colors
        let colors_content = if let Some(palette) = self.current_palette() {
            let mut color_grid = column![].spacing(spacing::XS);

            // Display colors in a simple grid
            let mut color_buttons: Vec<Element<'_, ColorPaletteMessage>> = Vec::new();
            for &color in &palette.colors {
                let color_button = button(Space::new(Length::Fixed(32.0), Length::Fixed(32.0)))
                    .on_press(message_mapper(ColorPaletteMessage::SelectColor(color)))
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
                color_buttons.push(color_button.into());
            }

            // Create rows of 8 colors each
            let mut current_row = row![].spacing(spacing::XS);
            let mut count = 0;

            for button_element in color_buttons {
                current_row = current_row.push(button_element);
                count += 1;

                if count == 8 {
                    color_grid = color_grid.push(current_row);
                    current_row = row![].spacing(spacing::XS);
                    count = 0;
                }
            }

            // Add remaining buttons if any
            if count > 0 {
                color_grid = color_grid.push(current_row);
            }

            let palette_info = row![
                text(format!("Palette: {}", palette.name)).size(14.0),
                Space::new(Length::Fill, Length::Shrink),
                text(format!("{} colors", palette.colors.len())).size(12.0),
            ]
            .align_y(iced::alignment::Vertical::Center);

            column![
                palette_info,
                Space::new(Length::Shrink, Length::Fixed(spacing::SM)),
                scrollable(color_grid).height(Length::Fixed(200.0)),
            ]
            .spacing(spacing::SM)
        } else {
            column![text("No palette selected").size(14.0)]
        };

        // Action buttons
        let action_buttons = row![
            button("Import")
                .on_press(message_mapper(ColorPaletteMessage::ImportPalette))
                .style(button::secondary),
            button("Export")
                .on_press(message_mapper(ColorPaletteMessage::ExportPalette))
                .style(button::secondary),
            Space::new(Length::Fill, Length::Shrink),
            button("Close")
                .on_press(message_mapper(ColorPaletteMessage::Close))
                .style(button::secondary),
        ]
        .spacing(spacing::MD);

        let content = column![
            text("Color Palettes").size(18.0),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            scrollable(palette_buttons).width(Length::Fill),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            new_palette_row,
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            colors_content,
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            action_buttons,
        ]
        .spacing(spacing::SM)
        .padding(spacing::LG);

        container(content)
            .padding(spacing::LG)
            .width(Length::Fixed(600.0))
            .height(Length::Fixed(500.0))
            .into()
    }
}
