//! Tool options panel animation system
//! Provides smooth transitions for tool option panel visibility and content changes

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::Color;

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic, ease_out_back};
use crate::tools::ToolType;

/// Types of tool options panel animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolOptionAnimationType {
    /// Panel sliding in/out from the side
    SlideInOut,
    /// Panel fading in/out
    FadeInOut,
    /// Panel expanding/collapsing
    ExpandCollapse,
    /// Option controls morphing between different tool options
    OptionMorph,
}

/// Tool options panel animation state
#[derive(Debug, Clone)]
pub struct ToolOptionAnimation {
    /// Type of animation
    pub animation_type: ToolOptionAnimationType,
    /// Start time of the animation
    pub start_time: Instant,
    /// Duration of the animation
    pub duration: Duration,
    /// Whether this is showing or hiding
    pub is_showing: bool,
    /// Easing function to use
    pub easing: EasingFunction,
    /// Starting state
    pub start_state: ToolOptionAnimationState,
    /// Target state
    pub target_state: ToolOptionAnimationState,
}

/// Animation state values for tool options panel
#[derive(Debug, Clone)]
pub struct ToolOptionAnimationState {
    /// Panel opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
    /// Panel width factor (0.0 = collapsed, 1.0 = full width)
    pub width_factor: f32,
    /// Panel height factor (0.0 = collapsed, 1.0 = full height)
    pub height_factor: f32,
    /// Horizontal offset for sliding animations
    pub offset_x: f32,
    /// Vertical offset for sliding animations
    pub offset_y: f32,
    /// Scale factor for expand/collapse animations
    pub scale: f32,
    /// Background blur intensity
    pub blur_intensity: f32,
}

impl Default for ToolOptionAnimationState {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            width_factor: 1.0,
            height_factor: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            blur_intensity: 0.0,
        }
    }
}

/// Tool options animation manager
#[derive(Debug)]
pub struct ToolOptionAnimationManager {
    /// Current panel animation
    panel_animation: Option<ToolOptionAnimation>,
    /// Option control animations for smooth transitions between tools
    option_animations: HashMap<String, ToolOptionAnimation>,
    /// Currently visible tool
    current_tool: Option<ToolType>,
    /// Panel visibility state
    is_panel_visible: bool,
    /// Default animation duration
    default_duration: Duration,
    /// Default animation type
    default_animation_type: ToolOptionAnimationType,
}

impl Default for ToolOptionAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolOptionAnimationManager {
    /// Create a new tool options animation manager
    pub fn new() -> Self {
        Self {
            panel_animation: None,
            option_animations: HashMap::new(),
            current_tool: None,
            is_panel_visible: false,
            default_duration: Duration::from_millis(250), // 250ms default
            default_animation_type: ToolOptionAnimationType::SlideInOut,
        }
    }

    /// Show the tool options panel with animation
    pub fn show_panel(&mut self, tool_type: ToolType) {
        if self.current_tool != Some(tool_type) {
            // Tool changed, animate option morphing
            self.animate_tool_change(tool_type);
        }

        if !self.is_panel_visible {
            // Panel is hidden, animate showing
            self.animate_panel_show();
        }

        self.current_tool = Some(tool_type);
        self.is_panel_visible = true;
    }

    /// Hide the tool options panel with animation
    pub fn hide_panel(&mut self) {
        if self.is_panel_visible {
            self.animate_panel_hide();
        }
        self.is_panel_visible = false;
    }

    /// Update all animations and return whether any are still active
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let mut has_active_animations = false;

        // Update panel animation
        if let Some(ref animation) = self.panel_animation {
            if now.duration_since(animation.start_time) >= animation.duration {
                self.panel_animation = None;
            } else {
                has_active_animations = true;
            }
        }

        // Update option animations
        let mut completed_options = Vec::new();
        for (option_id, animation) in &self.option_animations {
            if now.duration_since(animation.start_time) >= animation.duration {
                completed_options.push(option_id.clone());
            } else {
                has_active_animations = true;
            }
        }

        // Remove completed option animations
        for option_id in completed_options {
            self.option_animations.remove(&option_id);
        }

        has_active_animations
    }

    /// Get current panel animation state
    pub fn get_panel_state(&self) -> Option<ToolOptionAnimationState> {
        self.panel_animation.as_ref().map(|animation| {
            let progress = self.get_animation_progress(animation);
            self.interpolate_state(animation, progress)
        })
    }

    /// Get current option animation state for a specific option
    pub fn get_option_state(&self, option_id: &str) -> Option<ToolOptionAnimationState> {
        self.option_animations.get(option_id).map(|animation| {
            let progress = self.get_animation_progress(animation);
            self.interpolate_state(animation, progress)
        })
    }

    /// Check if panel is currently visible (including during animations)
    pub fn is_panel_visible(&self) -> bool {
        self.is_panel_visible || self.panel_animation.is_some()
    }

    /// Check if any animations are running
    pub fn has_active_animations(&self) -> bool {
        self.panel_animation.is_some() || !self.option_animations.is_empty()
    }

    /// Get current tool
    pub fn current_tool(&self) -> Option<ToolType> {
        self.current_tool
    }

    /// Animate panel showing
    fn animate_panel_show(&mut self) {
        let (start_state, target_state, easing) = match self.default_animation_type {
            ToolOptionAnimationType::SlideInOut => (
                ToolOptionAnimationState {
                    opacity: 0.0,
                    width_factor: 1.0,
                    height_factor: 1.0,
                    offset_x: -300.0, // Slide in from left
                    offset_y: 0.0,
                    scale: 1.0,
                    blur_intensity: 0.0,
                },
                ToolOptionAnimationState::default(),
                ease_out_back as EasingFunction,
            ),
            ToolOptionAnimationType::FadeInOut => (
                ToolOptionAnimationState {
                    opacity: 0.0,
                    ..Default::default()
                },
                ToolOptionAnimationState::default(),
                ease_out_cubic as EasingFunction,
            ),
            ToolOptionAnimationType::ExpandCollapse => (
                ToolOptionAnimationState {
                    opacity: 0.0,
                    width_factor: 0.0,
                    height_factor: 0.0,
                    scale: 0.8,
                    ..Default::default()
                },
                ToolOptionAnimationState::default(),
                ease_out_back as EasingFunction,
            ),
            ToolOptionAnimationType::OptionMorph => (
                ToolOptionAnimationState {
                    opacity: 0.8,
                    scale: 0.95,
                    blur_intensity: 1.0,
                    ..Default::default()
                },
                ToolOptionAnimationState::default(),
                ease_in_out_cubic as EasingFunction,
            ),
        };

        self.panel_animation = Some(ToolOptionAnimation {
            animation_type: self.default_animation_type,
            start_time: Instant::now(),
            duration: self.default_duration,
            is_showing: true,
            easing,
            start_state,
            target_state,
        });
    }

    /// Animate panel hiding
    fn animate_panel_hide(&mut self) {
        let (start_state, target_state, easing) = match self.default_animation_type {
            ToolOptionAnimationType::SlideInOut => (
                ToolOptionAnimationState::default(),
                ToolOptionAnimationState {
                    opacity: 0.0,
                    width_factor: 1.0,
                    height_factor: 1.0,
                    offset_x: -300.0, // Slide out to left
                    offset_y: 0.0,
                    scale: 1.0,
                    blur_intensity: 0.0,
                },
                ease_in_out_cubic as EasingFunction,
            ),
            ToolOptionAnimationType::FadeInOut => (
                ToolOptionAnimationState::default(),
                ToolOptionAnimationState {
                    opacity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            ToolOptionAnimationType::ExpandCollapse => (
                ToolOptionAnimationState::default(),
                ToolOptionAnimationState {
                    opacity: 0.0,
                    width_factor: 0.0,
                    height_factor: 0.0,
                    scale: 0.8,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            ToolOptionAnimationType::OptionMorph => (
                ToolOptionAnimationState::default(),
                ToolOptionAnimationState {
                    opacity: 0.0,
                    scale: 0.9,
                    blur_intensity: 2.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
        };

        self.panel_animation = Some(ToolOptionAnimation {
            animation_type: self.default_animation_type,
            start_time: Instant::now(),
            duration: Duration::from_millis(200), // Faster hiding
            is_showing: false,
            easing,
            start_state,
            target_state,
        });
    }

    /// Animate tool change (option morphing)
    fn animate_tool_change(&mut self, _new_tool: ToolType) {
        // Create morphing animation for option controls
        let morph_animation = ToolOptionAnimation {
            animation_type: ToolOptionAnimationType::OptionMorph,
            start_time: Instant::now(),
            duration: Duration::from_millis(300),
            is_showing: true,
            easing: ease_in_out_cubic as EasingFunction,
            start_state: ToolOptionAnimationState {
                opacity: 1.0,
                scale: 1.0,
                blur_intensity: 0.0,
                ..Default::default()
            },
            target_state: ToolOptionAnimationState {
                opacity: 1.0,
                scale: 1.0,
                blur_intensity: 0.0,
                ..Default::default()
            },
        };

        self.option_animations.insert("tool_change".to_string(), morph_animation);
    }

    /// Get animation progress (0.0 to 1.0)
    fn get_animation_progress(&self, animation: &ToolOptionAnimation) -> f32 {
        let elapsed = Instant::now().duration_since(animation.start_time);
        let progress = elapsed.as_secs_f32() / animation.duration.as_secs_f32();
        progress.clamp(0.0, 1.0)
    }

    /// Interpolate between start and target states
    fn interpolate_state(&self, animation: &ToolOptionAnimation, progress: f32) -> ToolOptionAnimationState {
        let eased_progress = (animation.easing)(progress);
        let start = &animation.start_state;
        let target = &animation.target_state;

        ToolOptionAnimationState {
            opacity: start.opacity + (target.opacity - start.opacity) * eased_progress,
            width_factor: start.width_factor + (target.width_factor - start.width_factor) * eased_progress,
            height_factor: start.height_factor + (target.height_factor - start.height_factor) * eased_progress,
            offset_x: start.offset_x + (target.offset_x - start.offset_x) * eased_progress,
            offset_y: start.offset_y + (target.offset_y - start.offset_y) * eased_progress,
            scale: start.scale + (target.scale - start.scale) * eased_progress,
            blur_intensity: start.blur_intensity + (target.blur_intensity - start.blur_intensity) * eased_progress,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ToolType;

    #[test]
    fn test_tool_option_animation_manager_creation() {
        let manager = ToolOptionAnimationManager::new();
        assert!(!manager.has_active_animations());
        assert!(!manager.is_panel_visible());
        assert_eq!(manager.current_tool(), None);
    }

    #[test]
    fn test_show_panel_animation() {
        let mut manager = ToolOptionAnimationManager::new();

        // Show panel for brush tool
        manager.show_panel(ToolType::Brush);

        assert!(manager.is_panel_visible());
        assert_eq!(manager.current_tool(), Some(ToolType::Brush));
        assert!(manager.has_active_animations());

        // Should have panel animation state
        let state = manager.get_panel_state();
        assert!(state.is_some());
    }

    #[test]
    fn test_hide_panel_animation() {
        let mut manager = ToolOptionAnimationManager::new();

        // First show panel
        manager.show_panel(ToolType::Brush);
        assert!(manager.is_panel_visible());

        // Then hide it
        manager.hide_panel();
        // Panel should be marked as not visible, but animation might still be running
        assert_eq!(manager.is_panel_visible, false);
        assert!(manager.has_active_animations()); // Animation should be running
    }

    #[test]
    fn test_tool_change_animation() {
        let mut manager = ToolOptionAnimationManager::new();

        // Show panel for first tool
        manager.show_panel(ToolType::Brush);
        assert_eq!(manager.current_tool(), Some(ToolType::Brush));

        // Switch to second tool
        manager.show_panel(ToolType::Eraser);
        assert_eq!(manager.current_tool(), Some(ToolType::Eraser));

        // Should have animations for tool change
        assert!(manager.has_active_animations());
    }

    #[test]
    fn test_animation_state_interpolation() {
        let start_state = ToolOptionAnimationState {
            opacity: 0.0,
            width_factor: 0.0,
            height_factor: 0.0,
            offset_x: -300.0,
            offset_y: 0.0,
            scale: 0.8,
            blur_intensity: 0.0,
        };

        let target_state = ToolOptionAnimationState::default();

        let animation = ToolOptionAnimation {
            animation_type: ToolOptionAnimationType::SlideInOut,
            start_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(300),
            is_showing: true,
            easing: crate::ui::animations::easing::ease_out_cubic,
            start_state,
            target_state,
        };

        let manager = ToolOptionAnimationManager::new();

        // Test interpolation at 50% progress
        let interpolated = manager.interpolate_state(&animation, 0.5);

        // Values should be between start and target
        assert!(interpolated.opacity > 0.0 && interpolated.opacity < 1.0);
        assert!(interpolated.width_factor > 0.0 && interpolated.width_factor < 1.0);
        assert!(interpolated.offset_x > -300.0 && interpolated.offset_x < 0.0);
    }

    #[test]
    fn test_animation_update() {
        let mut manager = ToolOptionAnimationManager::new();

        // Start an animation
        manager.show_panel(ToolType::Brush);
        assert!(manager.has_active_animations());

        // Simulate time passing (animations should still be active for a short time)
        std::thread::sleep(std::time::Duration::from_millis(10));
        let still_active = manager.update();
        assert!(still_active);

        // After a longer time, animations should complete
        // Note: In real tests, we might want to mock time instead of sleeping
    }

    #[test]
    fn test_different_animation_types() {
        let animation_types = [
            ToolOptionAnimationType::SlideInOut,
            ToolOptionAnimationType::FadeInOut,
            ToolOptionAnimationType::ExpandCollapse,
            ToolOptionAnimationType::OptionMorph,
        ];

        for animation_type in animation_types {
            let mut manager = ToolOptionAnimationManager::new();
            manager.default_animation_type = animation_type;

            // Show panel should create animation of the specified type
            manager.show_panel(ToolType::Brush);
            assert!(manager.has_active_animations());

            let state = manager.get_panel_state();
            assert!(state.is_some());
        }
    }

    #[test]
    fn test_animation_state_default() {
        let state = ToolOptionAnimationState::default();
        assert_eq!(state.opacity, 1.0);
        assert_eq!(state.width_factor, 1.0);
        assert_eq!(state.height_factor, 1.0);
        assert_eq!(state.offset_x, 0.0);
        assert_eq!(state.offset_y, 0.0);
        assert_eq!(state.scale, 1.0);
        assert_eq!(state.blur_intensity, 0.0);
    }
}
