//! Advanced styling system for PSOC Image Editor
//! Provides modern visual effects and styling components

pub mod visual_effects;
pub mod glass_effects;
pub mod shadow_system;
pub mod modern_containers;
pub mod gradient_system;

// Re-export main components
pub use visual_effects::{VisualEffectStyle, apply_visual_effects};
pub use glass_effects::{GlassEffect, FrostedGlassStyle};
pub use shadow_system::{ShadowConfig, DropShadow, InnerShadow};
pub use modern_containers::{ModernContainerStyle, ModernContainerConfig, modern_container_style, glass_container_style};
pub use gradient_system::{Gradient, GradientDirection, GradientStop, PsocGradient, GradientUtils};
