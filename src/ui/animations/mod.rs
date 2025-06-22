//! Animation system for PSOC Image Editor
//! Provides smooth transitions and visual effects

pub mod menu_animations;
pub mod easing;

// Re-export main components
pub use menu_animations::{MenuAnimationManager, MenuTransition, TransitionType};
pub use easing::{EasingFunction, ease_in_out_cubic, ease_out_cubic, ease_in_cubic};
