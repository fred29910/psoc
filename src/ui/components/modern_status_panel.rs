//! Modern status panel component
//! Provides real-time status information with modern styling and animations

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Background, Color, Border, Shadow, Vector,
};

use crate::ui::theme::{PsocTheme, spacing};
use crate::ui::styles::glass_container_style;
use crate::ui::theme::GlassIntensity;
use psoc_core::RgbaPixel;

/// Status panel information structure
#[derive(Debug, Clone)]
pub struct ModernStatusInfo {
    /// Current mouse position in image coordinates
    pub mouse_position: Option<(f32, f32)>,
    /// Current pixel color under cursor
    pub pixel_color: Option<RgbaPixel>,
    /// Current zoom level (1.0 = 100%)
    pub zoom_level: f32,
    /// Image/document dimensions
    pub image_size: Option<(u32, u32)>,
    /// Document status (saved, unsaved, etc.)
    pub document_status: String,
    /// Current tool name
    pub current_tool: String,
    /// Memory usage information
    pub memory_usage: Option<String>,
    /// Performance metrics
    pub fps: Option<f32>,
}

impl Default for ModernStatusInfo {
    fn default() -> Self {
        Self {
            mouse_position: None,
            pixel_color: None,
            zoom_level: 1.0,
            image_size: None,
            document_status: "No document".to_string(),
            current_tool: "Select".to_string(),
            memory_usage: None,
            fps: None,
        }
    }
}

/// Create a modern status panel
pub fn modern_status_panel<Message: Clone + 'static>(
    status_info: ModernStatusInfo,
) -> Element<'static, Message> {
    let psoc_theme = PsocTheme::Dark; // TODO: Get from actual theme
    let palette = psoc_theme.palette();

    // Create status sections
    let mut sections = Vec::new();

    // Mouse position and pixel color section
    if let Some((x, y)) = status_info.mouse_position {
        let position_section = create_status_section(
            "Position",
            format!("X: {:.0}, Y: {:.0}", x, y),
            Some(palette.tech_blue),
        );
        sections.push(position_section);

        // Pixel color information
        if let Some(color) = status_info.pixel_color {
            let color_section = create_pixel_color_section(color, &palette);
            sections.push(color_section);
        }
    } else {
        let position_section = create_status_section(
            "Position",
            "Outside canvas".to_string(),
            Some(Color::from_rgba(0.6, 0.6, 0.6, 1.0)),
        );
        sections.push(position_section);
    }

    // Document information section
    let doc_section = create_document_section(&status_info, &palette);
    sections.push(doc_section);

    // Tool information section
    let tool_section = create_status_section(
        "Tool",
        status_info.current_tool,
        Some(palette.tech_blue),
    );
    sections.push(tool_section);

    // Performance section (if available)
    if let Some(fps) = status_info.fps {
        let perf_section = create_status_section(
            "FPS",
            format!("{:.1}", fps),
            Some(if fps >= 30.0 { 
                Color::from_rgba(0.2, 0.8, 0.2, 1.0) 
            } else { 
                Color::from_rgba(0.8, 0.6, 0.2, 1.0) 
            }),
        );
        sections.push(perf_section);
    }

    // Memory usage section (if available)
    if let Some(ref memory) = status_info.memory_usage {
        let memory_section = create_status_section(
            "Memory",
            memory.clone(),
            Some(Color::from_rgba(0.6, 0.8, 0.9, 1.0)),
        );
        sections.push(memory_section);
    }

    // Create the main status bar layout
    let status_content = row(sections)
        .spacing(spacing::MD)
        .align_y(iced::alignment::Vertical::Center);

    // Apply modern container styling
    container(status_content)
        .width(Length::Fill)
        .padding([spacing::SM, spacing::MD])
        .style(move |_theme| {
            let mut style = glass_container_style(GlassIntensity::Light, &psoc_theme);
            style.border = Border {
                color: Color::from_rgba(
                    palette.border.r,
                    palette.border.g,
                    palette.border.b,
                    0.3
                ),
                width: 1.0,
                radius: 0.0.into(), // Status bar typically has no radius
            };
            style.shadow = Shadow {
                color: palette.shadow_color(0.1),
                offset: Vector::new(0.0, -2.0), // Shadow upward
                blur_radius: 8.0,
            };
            style
        })
        .into()
}

/// Create a status section with label and value
fn create_status_section<Message: 'static>(
    label: &str,
    value: String,
    accent_color: Option<Color>,
) -> Element<'static, Message> {
    let label_color = accent_color.unwrap_or(Color::from_rgba(0.8, 0.8, 0.8, 1.0));
    
    row![
        text(format!("{}:", label))
            .size(11.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(label_color),
            }),
        Space::new(Length::Fixed(4.0), Length::Shrink),
        text(value)
            .size(11.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::WHITE),
            })
    ]
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

/// Create a pixel color section with color preview
fn create_pixel_color_section<Message: 'static>(
    color: RgbaPixel,
    palette: &crate::ui::theme::ColorPalette,
) -> Element<'static, Message> {
    let color_preview = container(
        text(" ")
            .size(12.0)
    )
    .width(Length::Fixed(16.0))
    .height(Length::Fixed(16.0))
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(Color::from_rgba(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        ))),
        border: Border {
            color: Color::from_rgba(0.5, 0.5, 0.5, 1.0),
            width: 1.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    });

    let rgb_text = format!("RGB({}, {}, {})", color.r, color.g, color.b);
    let hex_text = format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b);

    row![
        color_preview,
        Space::new(Length::Fixed(6.0), Length::Shrink),
        text(rgb_text)
            .size(11.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::WHITE),
            }),
        Space::new(Length::Fixed(8.0), Length::Shrink),
        text(hex_text)
            .size(11.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0)),
            })
    ]
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

/// Create document information section
fn create_document_section<Message: 'static>(
    status_info: &ModernStatusInfo,
    palette: &crate::ui::theme::ColorPalette,
) -> Element<'static, Message> {
    let size_text = if let Some((w, h)) = status_info.image_size {
        format!("{}×{}", w, h)
    } else {
        "No document".to_string()
    };

    let zoom_text = format!("{:.0}%", status_info.zoom_level * 100.0);

    let status_color = match status_info.document_status.as_str() {
        s if s.contains("Saved") => Color::from_rgba(0.2, 0.8, 0.2, 1.0),
        s if s.contains("Unsaved") => Color::from_rgba(0.8, 0.6, 0.2, 1.0),
        _ => Color::from_rgba(0.6, 0.6, 0.6, 1.0),
    };

    row![
        create_status_section("Size", size_text, Some(palette.tech_blue)),
        Space::new(Length::Fixed(spacing::MD), Length::Shrink),
        create_status_section("Zoom", zoom_text, Some(palette.tech_blue)),
        Space::new(Length::Fixed(spacing::MD), Length::Shrink),
        create_status_section("Status", status_info.document_status.clone(), Some(status_color)),
    ]
    .align_y(iced::alignment::Vertical::Center)
    .into()
}

/// Create an animated status indicator
pub fn animated_status_indicator<Message: 'static>(
    is_active: bool,
    label: String,
) -> Element<'static, Message> {
    let indicator_color = if is_active {
        Color::from_rgba(0.2, 0.8, 0.2, 1.0)
    } else {
        Color::from_rgba(0.6, 0.6, 0.6, 0.5)
    };

    let indicator = container(
        text("●")
            .size(8.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(indicator_color),
            })
    )
    .width(Length::Fixed(12.0))
    .center_x(Length::Fill);

    row![
        indicator,
        text(label)
            .size(11.0)
            .style(move |_theme| iced::widget::text::Style {
                color: Some(if is_active {
                    Color::WHITE
                } else {
                    Color::from_rgba(0.7, 0.7, 0.7, 1.0)
                }),
            })
    ]
    .align_y(iced::alignment::Vertical::Center)
    .spacing(4.0)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_status_info() -> ModernStatusInfo {
        ModernStatusInfo {
            mouse_position: Some((100.0, 200.0)),
            pixel_color: Some(RgbaPixel::new(255, 128, 64, 255)),
            zoom_level: 1.5,
            image_size: Some((1920, 1080)),
            document_status: "Saved".to_string(),
            current_tool: "Brush".to_string(),
            memory_usage: Some("256 MB".to_string()),
            fps: Some(60.0),
        }
    }

    #[test]
    fn test_modern_status_info_creation() {
        let status_info = create_test_status_info();

        assert_eq!(status_info.mouse_position, Some((100.0, 200.0)));
        assert!(status_info.pixel_color.is_some());
        assert_eq!(status_info.zoom_level, 1.5);
        assert_eq!(status_info.image_size, Some((1920, 1080)));
        assert_eq!(status_info.document_status, "Saved");
        assert_eq!(status_info.current_tool, "Brush");
        assert!(status_info.memory_usage.is_some());
        assert_eq!(status_info.fps, Some(60.0));
    }

    #[test]
    fn test_modern_status_info_default() {
        let status_info = ModernStatusInfo::default();

        assert_eq!(status_info.mouse_position, None);
        assert_eq!(status_info.pixel_color, None);
        assert_eq!(status_info.zoom_level, 1.0);
        assert_eq!(status_info.image_size, None);
        assert_eq!(status_info.document_status, "No document");
        assert_eq!(status_info.current_tool, "Select");
        assert_eq!(status_info.memory_usage, None);
        assert_eq!(status_info.fps, None);
    }

    #[test]
    fn test_modern_status_panel_creation() {
        let status_info = create_test_status_info();
        let panel = modern_status_panel::<()>(status_info);

        // Should return an Element
        // This is mainly a compilation test
        let _ = panel;
    }

    #[test]
    fn test_modern_status_panel_no_mouse() {
        let mut status_info = create_test_status_info();
        status_info.mouse_position = None;
        status_info.pixel_color = None;

        let panel = modern_status_panel::<()>(status_info);

        // Should handle missing mouse position gracefully
        let _ = panel;
    }

    #[test]
    fn test_animated_status_indicator() {
        // Test active indicator
        let active_indicator = animated_status_indicator::<()>(true, "Active".to_string());
        let _ = active_indicator;

        // Test inactive indicator
        let inactive_indicator = animated_status_indicator::<()>(false, "Inactive".to_string());
        let _ = inactive_indicator;
    }

    #[test]
    fn test_pixel_color_values() {
        let test_colors = [
            RgbaPixel::new(255, 0, 0, 255),     // Red
            RgbaPixel::new(0, 255, 0, 255),     // Green
            RgbaPixel::new(0, 0, 255, 255),     // Blue
            RgbaPixel::new(128, 128, 128, 128), // Gray with alpha
            RgbaPixel::new(255, 255, 255, 255), // White
            RgbaPixel::new(0, 0, 0, 255),       // Black
        ];

        for color in test_colors {
            let mut status_info = ModernStatusInfo::default();
            status_info.pixel_color = Some(color);
            status_info.mouse_position = Some((50.0, 50.0));

            let panel = modern_status_panel::<()>(status_info);
            // Should handle all color values correctly
            let _ = panel;
        }
    }

    #[test]
    fn test_document_status_variations() {
        let status_variations = [
            "Saved",
            "Unsaved",
            "No document",
            "Loading...",
            "Error",
        ];

        for status in status_variations {
            let mut status_info = ModernStatusInfo::default();
            status_info.document_status = status.to_string();

            let panel = modern_status_panel::<()>(status_info);
            // Should handle all status variations
            let _ = panel;
        }
    }

    #[test]
    fn test_performance_metrics() {
        let fps_values = [30.0, 60.0, 120.0, 15.0, 144.0];

        for fps in fps_values {
            let mut status_info = ModernStatusInfo::default();
            status_info.fps = Some(fps);

            let panel = modern_status_panel::<()>(status_info);
            // Should handle different FPS values
            let _ = panel;
        }
    }

    #[test]
    fn test_zoom_level_display() {
        let zoom_levels = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0, 8.0];

        for zoom in zoom_levels {
            let mut status_info = ModernStatusInfo::default();
            status_info.zoom_level = zoom;

            let panel = modern_status_panel::<()>(status_info);
            // Should handle different zoom levels
            let _ = panel;
        }
    }

    #[test]
    fn test_image_size_variations() {
        let image_sizes = [
            Some((800, 600)),
            Some((1920, 1080)),
            Some((4096, 4096)),
            Some((100, 100)),
            None,
        ];

        for size in image_sizes {
            let mut status_info = ModernStatusInfo::default();
            status_info.image_size = size;

            let panel = modern_status_panel::<()>(status_info);
            // Should handle different image sizes including None
            let _ = panel;
        }
    }
}
