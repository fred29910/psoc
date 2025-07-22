//! Animation system for PSOC Image Editor
//! Provides smooth transitions and visual effects

pub mod menu_animations;
pub mod modern_menu_animations;
pub mod toolbar_animations;
pub mod tool_options_animations;
pub mod status_animations;
pub mod transitions;
pub mod enhanced_tool_animations;
pub mod easing;

// Re-export main components
pub use menu_animations::{MenuAnimationManager, MenuTransition, TransitionType};
pub use modern_menu_animations::{ModernMenuAnimationManager, ModernMenuAnimation, ModernMenuAnimationType, AnimationValues};
pub use toolbar_animations::{ToolAnimationManager, ToolTransition, ToolTransitionType, ToolAnimationState};
pub use tool_options_animations::{ToolOptionAnimationManager, ToolOptionAnimation, ToolOptionAnimationType, ToolOptionAnimationState};
pub use status_animations::{StatusAnimationManager, StatusAnimation, StatusAnimationType, StatusValue};
pub use transitions::{TransitionManager, Transition, TransitionType as UITransitionType, TransitionState, AnimationDirection};
pub use enhanced_tool_animations::{EnhancedToolAnimationManager, EnhancedToolAnimation, EnhancedToolAnimationType, ToolAnimationState as EnhancedToolAnimationState};
pub use easing::{EasingFunction, ease_in_out_cubic, ease_out_cubic, ease_in_cubic, ease_out_quart};
