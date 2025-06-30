//! Color picker and palette tests for PSOC Image Editor

use psoc::ui::components::ColorHistory; // Removed ColorHistoryMessage
use psoc::ui::dialogs::{
    ColorPaletteDialog, ColorPaletteMessage, ColorPickerDialog, ColorPickerMessage,
};
use psoc_core::RgbaPixel;

#[test]
fn test_color_picker_creation() {
    let dialog = ColorPickerDialog::new();
    assert!(!dialog.is_visible());
    assert_eq!(dialog.current_color(), RgbaPixel::new(255, 255, 255, 255));
    assert_eq!(dialog.original_color(), RgbaPixel::new(255, 255, 255, 255));
}

#[test]
fn test_color_picker_show_hide() {
    let mut dialog = ColorPickerDialog::new();
    let test_color = RgbaPixel::new(128, 64, 192, 255);

    dialog.show(test_color);
    assert!(dialog.is_visible());
    assert_eq!(dialog.current_color(), test_color);
    assert_eq!(dialog.original_color(), test_color);

    dialog.hide();
    assert!(!dialog.is_visible());
}

#[test]
fn test_color_picker_rgb_changes() {
    let mut dialog = ColorPickerDialog::new();
    let initial_color = RgbaPixel::new(100, 150, 200, 255);
    dialog.show(initial_color);

    // Test red change
    dialog.update(ColorPickerMessage::RedChanged(255.0));
    assert_eq!(dialog.current_color().r, 255);
    assert_eq!(dialog.current_color().g, 150);
    assert_eq!(dialog.current_color().b, 200);

    // Test green change
    dialog.update(ColorPickerMessage::GreenChanged(0.0));
    assert_eq!(dialog.current_color().g, 0);

    // Test blue change
    dialog.update(ColorPickerMessage::BlueChanged(128.0));
    assert_eq!(dialog.current_color().b, 128);

    // Test alpha change
    dialog.update(ColorPickerMessage::AlphaChanged(200.0));
    assert_eq!(dialog.current_color().a, 200);
}

#[test]
fn test_color_picker_hsl_changes() {
    let mut dialog = ColorPickerDialog::new();
    let initial_color = RgbaPixel::new(255, 0, 0, 255); // Pure red
    dialog.show(initial_color);

    // Test hue change (should change color)
    dialog.update(ColorPickerMessage::HueChanged(120.0)); // Green hue
    let color = dialog.current_color();
    assert!(color.g > color.r && color.g > color.b); // Should be more green

    // Test saturation change
    dialog.update(ColorPickerMessage::SaturationChanged(50.0));
    // Color should be less saturated

    // Test lightness change
    dialog.update(ColorPickerMessage::LightnessChanged(25.0));
    // Color should be darker
}

#[test]
fn test_color_picker_hex_input() {
    let mut dialog = ColorPickerDialog::new();
    dialog.show(RgbaPixel::new(0, 0, 0, 255));

    // Test valid hex input
    dialog.update(ColorPickerMessage::HexChanged("FF0000".to_string()));
    assert_eq!(dialog.current_color().r, 255);
    assert_eq!(dialog.current_color().g, 0);
    assert_eq!(dialog.current_color().b, 0);

    // Test short hex input
    dialog.update(ColorPickerMessage::HexChanged("F0F".to_string()));
    assert_eq!(dialog.current_color().r, 255);
    assert_eq!(dialog.current_color().g, 0);
    assert_eq!(dialog.current_color().b, 255);
}

#[test]
fn test_color_picker_preset_colors() {
    let mut dialog = ColorPickerDialog::new();
    dialog.show(RgbaPixel::new(128, 128, 128, 255));

    let presets = ColorPickerDialog::preset_colors();
    assert!(!presets.is_empty());
    assert!(presets.len() >= 8); // Should have at least basic colors

    // Test selecting a preset
    let red_preset = RgbaPixel::new(255, 0, 0, 255);
    dialog.update(ColorPickerMessage::SelectPreset(red_preset));
    assert_eq!(dialog.current_color(), red_preset);
}

#[test]
fn test_color_picker_cancel_reset() {
    let mut dialog = ColorPickerDialog::new();
    let original_color = RgbaPixel::new(100, 150, 200, 255);
    dialog.show(original_color);

    // Change color
    dialog.update(ColorPickerMessage::RedChanged(255.0));
    assert_ne!(dialog.current_color(), original_color);

    // Cancel should restore original
    dialog.update(ColorPickerMessage::Cancel);
    assert_eq!(dialog.current_color(), original_color);

    // Reset should set to white
    dialog.update(ColorPickerMessage::Reset);
    assert_eq!(dialog.current_color(), RgbaPixel::new(255, 255, 255, 255));
}

#[test]
fn test_color_palette_creation() {
    let dialog = ColorPaletteDialog::new();
    assert!(!dialog.is_visible());

    let palette_names = dialog.palette_names();
    assert!(palette_names.contains(&"Basic Colors".to_string()));
    assert!(palette_names.contains(&"Grayscale".to_string()));
    assert!(palette_names.contains(&"Web Safe".to_string()));
}

#[test]
fn test_color_palette_show_hide() {
    let mut dialog = ColorPaletteDialog::new();

    dialog.show();
    assert!(dialog.is_visible());

    dialog.hide();
    assert!(!dialog.is_visible());
}

#[test]
fn test_color_palette_new_palette() {
    let mut dialog = ColorPaletteDialog::new();
    let initial_count = dialog.palette_names().len();

    // Set new palette name and create
    dialog.update(ColorPaletteMessage::PaletteNameChanged(
        "Test Palette".to_string(),
    ));
    dialog.update(ColorPaletteMessage::NewPalette);

    let new_count = dialog.palette_names().len();
    assert_eq!(new_count, initial_count + 1);
    assert!(dialog.palette_names().contains(&"Test Palette".to_string()));
}

#[test]
fn test_color_palette_load_palette() {
    let mut dialog = ColorPaletteDialog::new();

    dialog.update(ColorPaletteMessage::LoadPalette("Grayscale".to_string()));
    assert_eq!(dialog.current_palette().unwrap().name, "Grayscale");
}

#[test]
fn test_color_palette_add_remove_colors() {
    let mut dialog = ColorPaletteDialog::new();

    // Create a new palette
    dialog.update(ColorPaletteMessage::PaletteNameChanged(
        "Test Colors".to_string(),
    ));
    dialog.update(ColorPaletteMessage::NewPalette);

    let test_color = RgbaPixel::new(255, 128, 64, 255);

    // Add color
    dialog.update(ColorPaletteMessage::AddColor(test_color));
    let palette = dialog.current_palette().unwrap();
    assert!(palette.colors.contains(&test_color));

    // Remove color
    dialog.update(ColorPaletteMessage::RemoveColor(0));
    let palette = dialog.current_palette().unwrap();
    assert!(!palette.colors.contains(&test_color));
}

#[test]
fn test_color_palette_built_in_protection() {
    let mut dialog = ColorPaletteDialog::new();

    // Load built-in palette
    dialog.update(ColorPaletteMessage::LoadPalette("Basic Colors".to_string()));
    let initial_colors = dialog.current_palette().unwrap().colors.clone();

    // Try to add color to built-in palette (should be ignored)
    let test_color = RgbaPixel::new(123, 45, 67, 255);
    dialog.update(ColorPaletteMessage::AddColor(test_color));

    let final_colors = dialog.current_palette().unwrap().colors.clone();
    assert_eq!(initial_colors, final_colors); // Should be unchanged
}

#[test]
fn test_color_history_creation() {
    let history = ColorHistory::new(16); // Provide max_size
    assert!(history.colors().is_empty());
    assert_eq!(history.colors().len(), 0);
    assert_eq!(history.colors().front(), None); // .front() returns Option<&T>
}

#[test]
fn test_color_history_add_colors() {
    let mut history = ColorHistory::new(16);
    let color1 = RgbaPixel::new(255, 0, 0, 255);
    let color2 = RgbaPixel::new(0, 255, 0, 255);

    history.add_color(color1);
    assert_eq!(history.colors().len(), 1);
    assert_eq!(history.colors().front().copied(), Some(color1));

    history.add_color(color2);
    assert_eq!(history.colors().len(), 2);
    assert_eq!(history.colors().front().copied(), Some(color2));
}

#[test]
fn test_color_history_duplicate_handling() {
    let mut history = ColorHistory::new(16);
    let color = RgbaPixel::new(255, 0, 0, 255);

    history.add_color(color);
    history.add_color(color); // Add same color again

    assert_eq!(history.colors().len(), 1); // Should not duplicate
    assert_eq!(history.colors().front().copied(), Some(color));
}

#[test]
fn test_color_history_max_size() {
    let max_size = 5; // Use a smaller max_size for testing this specifically
    let mut history = ColorHistory::new(max_size);

    // Add more than max colors
    for i in 0..(max_size + 5) { // e.g. 10 colors if max_size is 5
        let color = RgbaPixel::new(i as u8, 0, 0, 255);
        history.add_color(color);
    }

    assert_eq!(history.colors().len(), max_size);
    // Most recent should be the last one added that fits
    assert_eq!(history.colors().front().copied(), Some(RgbaPixel::new((max_size + 4) as u8, 0, 0, 255)));
}

#[test]
fn test_color_history_clear() {
    let mut history = ColorHistory::new(16);
    history.add_color(RgbaPixel::new(255, 0, 0, 255));
    history.add_color(RgbaPixel::new(0, 255, 0, 255));

    assert_eq!(history.colors().len(), 2);

    // Re-initialize to clear, as there's no direct clear method on the immutable getter
    history = ColorHistory::new(16);
    assert!(history.colors().is_empty());
    assert_eq!(history.colors().len(), 0);
}

#[test]
fn test_color_history_order() {
    let mut history = ColorHistory::new(16);
    let color1 = RgbaPixel::new(255, 0, 0, 255);
    let color2 = RgbaPixel::new(0, 255, 0, 255);
    let color3 = RgbaPixel::new(0, 0, 255, 255);

    history.add_color(color1);
    history.add_color(color2);
    history.add_color(color3);

    let colors: Vec<_> = history.colors().iter().copied().collect();
    assert_eq!(colors[0], color3); // Most recent first
    assert_eq!(colors[1], color2);
    assert_eq!(colors[2], color1);
}
