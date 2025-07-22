//! Toolbar animation system for smooth tool switching
//! Provides animated transitions when switching between tools

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::Color;

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic, ease_out_back};
use crate::tools::ToolType;

/// Types of tool transition animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolTransitionType {
    /// Tool button scaling with glow effect
    ScaleGlow,
    /// Tool button sliding with color transition
    SlideColor,
    /// Tool button pulsing effect
    Pulse,
    /// Tool button bouncing effect
    Bounce,
}

/// Tool transition state
#[derive(Debug, Clone)]
pub struct ToolTransition {
    /// Type of transition
    pub transition_type: ToolTransitionType,
    /// Start time of the transition
    pub start_time: Instant,
    /// Duration of the transition
    pub duration: Duration,
    /// Whether this is activating or deactivating
    pub is_activating: bool,
    /// Easing function to use
    pub easing: EasingFunction,
    /// Starting state
    pub start_state: ToolAnimationState,
    /// Target state
    pub target_state: ToolAnimationState,
}

/// Animation state values for tools
#[derive(Debug, Clone)]
pub struct ToolAnimationState {
    /// Scale factor (1.0 = normal size)
    pub scale: f32,
    /// Opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
    /// Glow intensity (0.0 = no glow, 1.0 = full glow)
    pub glow_intensity: f32,
    /// Background color
    pub background_color: Color,
    /// Border width
    pub border_width: f32,
    /// Rotation angle in radians
    pub rotation: f32,
}

impl Default for ToolAnimationState {
    fn default() -> Self {
        Self {
            scale: 1.0,
            opacity: 1.0,
            glow_intensity: 0.0,
            background_color: Color::TRANSPARENT,
            border_width: 0.0,
            rotation: 0.0,
        }
    }
}

/// Tool animation manager
#[derive(Debug)]
pub struct ToolAnimationManager {
    /// Active transitions for each tool
    transitions: HashMap<ToolType, ToolTransition>,
    /// Currently active tool
    active_tool: Option<ToolType>,
    /// Default transition duration
    default_duration: Duration,
    /// Default transition type
    default_transition: ToolTransitionType,
}

impl Default for ToolAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolAnimationManager {
    /// Create a new tool animation manager
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            active_tool: None,
            default_duration: Duration::from_millis(300), // 300ms default
            default_transition: ToolTransitionType::ScaleGlow,
        }
    }

    /// Start tool activation animation
    pub fn start_tool_activation(&mut self, tool_type: ToolType) {
        // Deactivate previous tool if any
        if let Some(prev_tool) = self.active_tool {
            if prev_tool != tool_type {
                self.start_tool_deactivation(prev_tool);
            }
        }

        // Start activation animation for new tool
        let transition = self.create_activation_transition(tool_type);
        self.transitions.insert(tool_type, transition);
        self.active_tool = Some(tool_type);
    }

    /// Start tool deactivation animation
    pub fn start_tool_deactivation(&mut self, tool_type: ToolType) {
        let transition = self.create_deactivation_transition(tool_type);
        self.transitions.insert(tool_type, transition);
        
        if self.active_tool == Some(tool_type) {
            self.active_tool = None;
        }
    }

    /// Update all animations and return whether any are still active
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let mut completed = Vec::new();

        for (tool_type, transition) in &self.transitions {
            if now.duration_since(transition.start_time) >= transition.duration {
                completed.push(*tool_type);
            }
        }

        // Remove completed transitions
        for tool_type in completed {
            self.transitions.remove(&tool_type);
        }

        !self.transitions.is_empty()
    }

    /// Get current animation state for a tool
    pub fn get_current_state(&self, tool_type: ToolType) -> Option<ToolAnimationState> {
        self.transitions.get(&tool_type).map(|transition| {
            let progress = self.get_transition_progress(transition);
            self.interpolate_state(transition, progress)
        })
    }

    /// Check if a tool is currently active
    pub fn is_tool_active(&self, tool_type: ToolType) -> bool {
        self.active_tool == Some(tool_type)
    }

    /// Check if any animations are running
    pub fn has_active_animations(&self) -> bool {
        !self.transitions.is_empty()
    }

    /// Get transition progress (0.0 to 1.0)
    fn get_transition_progress(&self, transition: &ToolTransition) -> f32 {
        let elapsed = Instant::now().duration_since(transition.start_time);
        let progress = elapsed.as_secs_f32() / transition.duration.as_secs_f32();
        progress.clamp(0.0, 1.0)
    }

    /// Interpolate between start and target states
    fn interpolate_state(&self, transition: &ToolTransition, progress: f32) -> ToolAnimationState {
        let eased_progress = (transition.easing)(progress);
        let start = &transition.start_state;
        let target = &transition.target_state;

        ToolAnimationState {
            scale: start.scale + (target.scale - start.scale) * eased_progress,
            opacity: start.opacity + (target.opacity - start.opacity) * eased_progress,
            glow_intensity: start.glow_intensity + (target.glow_intensity - start.glow_intensity) * eased_progress,
            background_color: Color {
                r: start.background_color.r + (target.background_color.r - start.background_color.r) * eased_progress,
                g: start.background_color.g + (target.background_color.g - start.background_color.g) * eased_progress,
                b: start.background_color.b + (target.background_color.b - start.background_color.b) * eased_progress,
                a: start.background_color.a + (target.background_color.a - start.background_color.a) * eased_progress,
            },
            border_width: start.border_width + (target.border_width - start.border_width) * eased_progress,
            rotation: start.rotation + (target.rotation - start.rotation) * eased_progress,
        }
    }

    /// Create activation transition
    fn create_activation_transition(&self, _tool_type: ToolType) -> ToolTransition {
        let (start_state, target_state, easing) = match self.default_transition {
            ToolTransitionType::ScaleGlow => (
                ToolAnimationState {
                    scale: 0.8,
                    opacity: 0.7,
                    glow_intensity: 0.0,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    ..Default::default()
                },
                ease_out_back as EasingFunction,
            ),
            ToolTransitionType::SlideColor => (
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 0.5,
                    glow_intensity: 0.0,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    ..Default::default()
                },
                ease_out_cubic as EasingFunction,
            ),
            ToolTransitionType::Pulse => (
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 0.0,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.1,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            ToolTransitionType::Bounce => (
                ToolAnimationState {
                    scale: 0.9,
                    opacity: 0.8,
                    glow_intensity: 0.0,
                    rotation: -0.1,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    rotation: 0.0,
                    ..Default::default()
                },
                ease_out_back as EasingFunction,
            ),
        };

        ToolTransition {
            transition_type: self.default_transition,
            start_time: Instant::now(),
            duration: self.default_duration,
            is_activating: true,
            easing,
            start_state,
            target_state,
        }
    }

    /// Create deactivation transition
    fn create_deactivation_transition(&self, _tool_type: ToolType) -> ToolTransition {
        let (start_state, target_state, easing) = match self.default_transition {
            ToolTransitionType::ScaleGlow => (
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 0.7,
                    glow_intensity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            ToolTransitionType::SlideColor => (
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 0.5,
                    glow_intensity: 0.0,
                    ..Default::default()
                },
                ease_out_cubic as EasingFunction,
            ),
            ToolTransitionType::Pulse | ToolTransitionType::Bounce => (
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 1.0,
                    glow_intensity: 1.0,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 0.8,
                    glow_intensity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
        };

        ToolTransition {
            transition_type: self.default_transition,
            start_time: Instant::now(),
            duration: Duration::from_millis(200), // Faster deactivation
            is_activating: false,
            easing,
            start_state,
            target_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ToolType;

    #[test]
    fn test_tool_animation_manager_creation() {
        let manager = ToolAnimationManager::new();
        assert!(!manager.has_active_animations());
        assert_eq!(manager.active_tool, None);
    }

    #[test]
    fn test_tool_activation_animation() {
        let mut manager = ToolAnimationManager::new();

        // Start activation animation
        manager.start_tool_activation(ToolType::Brush);

        assert!(manager.has_active_animations());
        assert_eq!(manager.active_tool, Some(ToolType::Brush));
        assert!(manager.is_tool_active(ToolType::Brush));

        // Should have animation state
        let state = manager.get_current_state(ToolType::Brush);
        assert!(state.is_some());
    }

    #[test]
    fn test_tool_deactivation_animation() {
        let mut manager = ToolAnimationManager::new();

        // First activate a tool
        manager.start_tool_activation(ToolType::Brush);

        // Then deactivate it
        manager.start_tool_deactivation(ToolType::Brush);

        assert!(manager.has_active_animations());
        assert!(!manager.is_tool_active(ToolType::Brush));
    }

    #[test]
    fn test_tool_switching_animation() {
        let mut manager = ToolAnimationManager::new();

        // Activate first tool
        manager.start_tool_activation(ToolType::Brush);
        assert!(manager.is_tool_active(ToolType::Brush));

        // Switch to second tool
        manager.start_tool_activation(ToolType::Eraser);
        assert!(manager.is_tool_active(ToolType::Eraser));
        assert!(!manager.is_tool_active(ToolType::Brush));

        // Should have animations for both tools
        assert!(manager.has_active_animations());
    }

    #[test]
    fn test_animation_state_interpolation() {
        let manager = ToolAnimationManager::new();

        let start_state = ToolAnimationState {
            scale: 0.8,
            opacity: 0.5,
            glow_intensity: 0.0,
            ..Default::default()
        };

        let target_state = ToolAnimationState {
            scale: 1.0,
            opacity: 1.0,
            glow_intensity: 1.0,
            ..Default::default()
        };

        let transition = ToolTransition {
            transition_type: ToolTransitionType::ScaleGlow,
            start_time: std::time::Instant::now(),
            duration: std::time::Duration::from_millis(300),
            is_activating: true,
            easing: crate::ui::animations::easing::ease_out_cubic,
            start_state,
            target_state,
        };

        // Test interpolation at 50% progress
        let interpolated = manager.interpolate_state(&transition, 0.5);

        // Values should be between start and target
        assert!(interpolated.scale > 0.8 && interpolated.scale < 1.0);
        assert!(interpolated.opacity > 0.5 && interpolated.opacity < 1.0);
        assert!(interpolated.glow_intensity > 0.0 && interpolated.glow_intensity < 1.0);
    }

    #[test]
    fn test_animation_update() {
        let mut manager = ToolAnimationManager::new();

        // Start an animation
        manager.start_tool_activation(ToolType::Brush);
        assert!(manager.has_active_animations());

        // Simulate time passing (animations should still be active for a short time)
        std::thread::sleep(std::time::Duration::from_millis(10));
        let still_active = manager.update();
        assert!(still_active);

        // After a longer time, animations should complete
        // Note: In real tests, we might want to mock time instead of sleeping
    }
}
