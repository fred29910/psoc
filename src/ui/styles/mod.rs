//! Advanced styling system for PSOC Image Editor
//! Provides modern visual effects and styling components

pub mod visual_effects;
pub mod glass_effects;
pub mod shadow_system;
pub mod modern_containers;
pub mod modern_menu_styles;
pub mod modern_canvas_styles;
pub mod gradient_system;

// Re-export main components
pub use visual_effects::{VisualEffectStyle, apply_visual_effects};
pub use glass_effects::{GlassEffect, FrostedGlassStyle};
pub use shadow_system::{ShadowConfig, DropShadow, InnerShadow};
pub use modern_containers::{ModernContainerStyle, ModernContainerConfig, modern_container_style, glass_container_style};
pub use gradient_system::{Gradient, GradientDirection, GradientStop, PsocGradient, GradientUtils};
pub use modern_menu_styles::{
    modern_menu_bar_style, modern_dropdown_style, modern_menu_button_style,
    modern_menu_item_style, menu_separator_style, menu_underline_style,
    MenuTypography,
};
pub use modern_canvas_styles::{
    modern_canvas_area_style, modern_canvas_background_style, modern_placeholder_style,
    modern_zoom_control_style, canvas_document_border_style, canvas_grid_style,
    canvas_ruler_style, canvas_selection_style, canvas_guide_style,
    CanvasStyleConfig,
};
