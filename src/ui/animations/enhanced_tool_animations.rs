//! Enhanced tool animation system with modern transitions
//! Provides smooth tool switching, hover effects, and state changes

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::{Color, Point, Size};

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic, ease_out_back, ease_out_quart};
use crate::tools::ToolType;

/// Enhanced tool animation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnhancedToolAnimationType {
    /// Tool activation with glow effect
    ActivationGlow,
    /// Tool deactivation fade
    DeactivationFade,
    /// Hover state animation
    HoverEffect,
    /// Tool switch transition
    SwitchTransition,
    /// Loading state animation
    LoadingSpinner,
    /// Error state animation
    ErrorShake,
    /// Success state animation
    SuccessPulse,
}

/// Tool animation state
#[derive(Debug, Clone)]
pub struct ToolAnimationState {
    /// Scale factor
    pub scale: f32,
    /// Opacity
    pub opacity: f32,
    /// Glow intensity
    pub glow_intensity: f32,
    /// Glow color
    pub glow_color: Color,
    /// Position offset
    pub position_offset: Point,
    /// Rotation angle
    pub rotation: f32,
    /// Background color
    pub background_color: Color,
    /// Border color
    pub border_color: Color,
    /// Shadow intensity
    pub shadow_intensity: f32,
}

impl Default for ToolAnimationState {
    fn default() -> Self {
        Self {
            scale: 1.0,
            opacity: 1.0,
            glow_intensity: 0.0,
            glow_color: Color::TRANSPARENT,
            position_offset: Point::ORIGIN,
            rotation: 0.0,
            background_color: Color::TRANSPARENT,
            border_color: Color::TRANSPARENT,
            shadow_intensity: 0.0,
        }
    }
}

/// Enhanced tool animation instance
#[derive(Debug, Clone)]
pub struct EnhancedToolAnimation {
    /// Animation type
    pub animation_type: EnhancedToolAnimationType,
    /// Tool type
    pub tool_type: ToolType,
    /// Start time
    pub start_time: Instant,
    /// Duration
    pub duration: Duration,
    /// Easing function
    pub easing: EasingFunction,
    /// Start state
    pub start_state: ToolAnimationState,
    /// Target state
    pub target_state: ToolAnimationState,
    /// Whether animation is active
    pub is_active: bool,
    /// Whether animation should loop
    pub should_loop: bool,
    /// Loop count (-1 for infinite)
    pub loop_count: i32,
    /// Current loop iteration
    pub current_loop: i32,
}

/// Enhanced tool animation manager
#[derive(Debug)]
pub struct EnhancedToolAnimationManager {
    /// Active animations by tool type
    animations: HashMap<ToolType, EnhancedToolAnimation>,
    /// Currently active tool
    active_tool: Option<ToolType>,
    /// Default durations by animation type
    default_durations: HashMap<EnhancedToolAnimationType, Duration>,
    /// Default easing functions by animation type
    default_easings: HashMap<EnhancedToolAnimationType, EasingFunction>,
    /// Whether animations are enabled
    animations_enabled: bool,
}

impl Default for EnhancedToolAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EnhancedToolAnimationManager {
    /// Create a new enhanced tool animation manager
    pub fn new() -> Self {
        let mut default_durations = HashMap::new();
        default_durations.insert(EnhancedToolAnimationType::ActivationGlow, Duration::from_millis(300));
        default_durations.insert(EnhancedToolAnimationType::DeactivationFade, Duration::from_millis(200));
        default_durations.insert(EnhancedToolAnimationType::HoverEffect, Duration::from_millis(150));
        default_durations.insert(EnhancedToolAnimationType::SwitchTransition, Duration::from_millis(250));
        default_durations.insert(EnhancedToolAnimationType::LoadingSpinner, Duration::from_millis(1000));
        default_durations.insert(EnhancedToolAnimationType::ErrorShake, Duration::from_millis(400));
        default_durations.insert(EnhancedToolAnimationType::SuccessPulse, Duration::from_millis(600));

        let mut default_easings = HashMap::new();
        default_easings.insert(EnhancedToolAnimationType::ActivationGlow, ease_out_back as EasingFunction);
        default_easings.insert(EnhancedToolAnimationType::DeactivationFade, ease_in_out_cubic as EasingFunction);
        default_easings.insert(EnhancedToolAnimationType::HoverEffect, ease_out_cubic as EasingFunction);
        default_easings.insert(EnhancedToolAnimationType::SwitchTransition, ease_out_quart as EasingFunction);
        default_easings.insert(EnhancedToolAnimationType::LoadingSpinner, ease_in_out_cubic as EasingFunction);
        default_easings.insert(EnhancedToolAnimationType::ErrorShake, ease_out_cubic as EasingFunction);
        default_easings.insert(EnhancedToolAnimationType::SuccessPulse, ease_out_back as EasingFunction);

        Self {
            animations: HashMap::new(),
            active_tool: None,
            default_durations,
            default_easings,
            animations_enabled: true,
        }
    }

    /// Start tool activation animation
    pub fn start_tool_activation(&mut self, tool_type: ToolType) {
        if !self.animations_enabled {
            self.active_tool = Some(tool_type);
            return;
        }

        // Deactivate previous tool if any
        if let Some(prev_tool) = self.active_tool {
            if prev_tool != tool_type {
                self.start_tool_deactivation(prev_tool);
            }
        }

        let duration = self.default_durations[&EnhancedToolAnimationType::ActivationGlow];
        let easing = self.default_easings[&EnhancedToolAnimationType::ActivationGlow];

        let animation = EnhancedToolAnimation {
            animation_type: EnhancedToolAnimationType::ActivationGlow,
            tool_type,
            start_time: Instant::now(),
            duration,
            easing,
            start_state: ToolAnimationState {
                scale: 0.9,
                opacity: 0.7,
                glow_intensity: 0.0,
                glow_color: Color::TRANSPARENT,
                background_color: Color::from_rgba(0.5, 0.5, 0.5, 0.3),
                border_color: Color::from_rgba(0.7, 0.7, 0.7, 0.5),
                ..Default::default()
            },
            target_state: ToolAnimationState {
                scale: 1.1,
                opacity: 1.0,
                glow_intensity: 1.0,
                glow_color: Color::from_rgba(0.0, 0.75, 1.0, 0.8), // tech-blue glow
                background_color: Color::from_rgba(0.0, 0.75, 1.0, 0.2),
                border_color: Color::from_rgba(0.0, 0.75, 1.0, 0.8),
                shadow_intensity: 0.3,
                ..Default::default()
            },
            is_active: true,
            should_loop: false,
            loop_count: 0,
            current_loop: 0,
        };

        self.animations.insert(tool_type, animation);
        self.active_tool = Some(tool_type);
    }

    /// Start tool deactivation animation
    pub fn start_tool_deactivation(&mut self, tool_type: ToolType) {
        if !self.animations_enabled {
            if self.active_tool == Some(tool_type) {
                self.active_tool = None;
            }
            return;
        }

        let duration = self.default_durations[&EnhancedToolAnimationType::DeactivationFade];
        let easing = self.default_easings[&EnhancedToolAnimationType::DeactivationFade];

        let animation = EnhancedToolAnimation {
            animation_type: EnhancedToolAnimationType::DeactivationFade,
            tool_type,
            start_time: Instant::now(),
            duration,
            easing,
            start_state: ToolAnimationState {
                scale: 1.1,
                opacity: 1.0,
                glow_intensity: 1.0,
                glow_color: Color::from_rgba(0.0, 0.75, 1.0, 0.8),
                background_color: Color::from_rgba(0.0, 0.75, 1.0, 0.2),
                border_color: Color::from_rgba(0.0, 0.75, 1.0, 0.8),
                shadow_intensity: 0.3,
                ..Default::default()
            },
            target_state: ToolAnimationState {
                scale: 1.0,
                opacity: 0.7,
                glow_intensity: 0.0,
                glow_color: Color::TRANSPARENT,
                background_color: Color::TRANSPARENT,
                border_color: Color::from_rgba(0.7, 0.7, 0.7, 0.3),
                shadow_intensity: 0.0,
                ..Default::default()
            },
            is_active: true,
            should_loop: false,
            loop_count: 0,
            current_loop: 0,
        };

        self.animations.insert(tool_type, animation);
        
        if self.active_tool == Some(tool_type) {
            self.active_tool = None;
        }
    }

    /// Start hover effect animation
    pub fn start_hover_effect(&mut self, tool_type: ToolType, is_entering: bool) {
        if !self.animations_enabled {
            return;
        }

        let duration = self.default_durations[&EnhancedToolAnimationType::HoverEffect];
        let easing = self.default_easings[&EnhancedToolAnimationType::HoverEffect];

        let (start_state, target_state) = if is_entering {
            (
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 0.7,
                    glow_intensity: 0.0,
                    background_color: Color::TRANSPARENT,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.05,
                    opacity: 1.0,
                    glow_intensity: 0.3,
                    glow_color: Color::from_rgba(0.0, 0.75, 1.0, 0.4),
                    background_color: Color::from_rgba(0.0, 0.75, 1.0, 0.1),
                    border_color: Color::from_rgba(0.0, 0.75, 1.0, 0.4),
                    shadow_intensity: 0.1,
                    ..Default::default()
                },
            )
        } else {
            (
                ToolAnimationState {
                    scale: 1.05,
                    opacity: 1.0,
                    glow_intensity: 0.3,
                    glow_color: Color::from_rgba(0.0, 0.75, 1.0, 0.4),
                    background_color: Color::from_rgba(0.0, 0.75, 1.0, 0.1),
                    border_color: Color::from_rgba(0.0, 0.75, 1.0, 0.4),
                    shadow_intensity: 0.1,
                    ..Default::default()
                },
                ToolAnimationState {
                    scale: 1.0,
                    opacity: 0.7,
                    glow_intensity: 0.0,
                    background_color: Color::TRANSPARENT,
                    border_color: Color::TRANSPARENT,
                    shadow_intensity: 0.0,
                    ..Default::default()
                },
            )
        };

        let animation = EnhancedToolAnimation {
            animation_type: EnhancedToolAnimationType::HoverEffect,
            tool_type,
            start_time: Instant::now(),
            duration,
            easing,
            start_state,
            target_state,
            is_active: true,
            should_loop: false,
            loop_count: 0,
            current_loop: 0,
        };

        self.animations.insert(tool_type, animation);
    }

    /// Start loading animation
    pub fn start_loading_animation(&mut self, tool_type: ToolType) {
        if !self.animations_enabled {
            return;
        }

        let duration = self.default_durations[&EnhancedToolAnimationType::LoadingSpinner];
        let easing = self.default_easings[&EnhancedToolAnimationType::LoadingSpinner];

        let animation = EnhancedToolAnimation {
            animation_type: EnhancedToolAnimationType::LoadingSpinner,
            tool_type,
            start_time: Instant::now(),
            duration,
            easing,
            start_state: ToolAnimationState {
                rotation: 0.0,
                opacity: 0.5,
                ..Default::default()
            },
            target_state: ToolAnimationState {
                rotation: std::f32::consts::PI * 2.0, // Full rotation
                opacity: 1.0,
                ..Default::default()
            },
            is_active: true,
            should_loop: true,
            loop_count: -1, // Infinite loop
            current_loop: 0,
        };

        self.animations.insert(tool_type, animation);
    }

    /// Update all animations and return whether any are still active
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let mut completed_animations = Vec::new();

        for (tool_type, animation) in &mut self.animations {
            if !animation.is_active {
                continue;
            }

            let elapsed = now.duration_since(animation.start_time);
            
            if elapsed >= animation.duration {
                if animation.should_loop && (animation.loop_count < 0 || animation.current_loop < animation.loop_count) {
                    // Reset for next loop
                    animation.start_time = now;
                    animation.current_loop += 1;
                } else {
                    // Animation completed
                    animation.is_active = false;
                    completed_animations.push(*tool_type);
                }
            }
        }

        // Remove completed non-looping animations
        for tool_type in completed_animations {
            self.animations.remove(&tool_type);
        }

        !self.animations.is_empty()
    }

    /// Get current animation state for a tool
    pub fn get_current_state(&self, tool_type: ToolType) -> Option<ToolAnimationState> {
        self.animations.get(&tool_type).and_then(|animation| {
            if !animation.is_active {
                return None;
            }

            let elapsed = Instant::now().duration_since(animation.start_time);
            if elapsed >= animation.duration {
                return Some(animation.target_state.clone());
            }

            let progress = elapsed.as_secs_f32() / animation.duration.as_secs_f32();
            let eased_progress = (animation.easing)(progress);

            Some(self.interpolate_states(&animation.start_state, &animation.target_state, eased_progress))
        })
    }

    /// Interpolate between two tool animation states
    fn interpolate_states(&self, start: &ToolAnimationState, target: &ToolAnimationState, progress: f32) -> ToolAnimationState {
        ToolAnimationState {
            scale: start.scale + (target.scale - start.scale) * progress,
            opacity: start.opacity + (target.opacity - start.opacity) * progress,
            glow_intensity: start.glow_intensity + (target.glow_intensity - start.glow_intensity) * progress,
            glow_color: Color::from_rgba(
                start.glow_color.r + (target.glow_color.r - start.glow_color.r) * progress,
                start.glow_color.g + (target.glow_color.g - start.glow_color.g) * progress,
                start.glow_color.b + (target.glow_color.b - start.glow_color.b) * progress,
                start.glow_color.a + (target.glow_color.a - start.glow_color.a) * progress,
            ),
            position_offset: Point::new(
                start.position_offset.x + (target.position_offset.x - start.position_offset.x) * progress,
                start.position_offset.y + (target.position_offset.y - start.position_offset.y) * progress,
            ),
            rotation: start.rotation + (target.rotation - start.rotation) * progress,
            background_color: Color::from_rgba(
                start.background_color.r + (target.background_color.r - start.background_color.r) * progress,
                start.background_color.g + (target.background_color.g - start.background_color.g) * progress,
                start.background_color.b + (target.background_color.b - start.background_color.b) * progress,
                start.background_color.a + (target.background_color.a - start.background_color.a) * progress,
            ),
            border_color: Color::from_rgba(
                start.border_color.r + (target.border_color.r - start.border_color.r) * progress,
                start.border_color.g + (target.border_color.g - start.border_color.g) * progress,
                start.border_color.b + (target.border_color.b - start.border_color.b) * progress,
                start.border_color.a + (target.border_color.a - start.border_color.a) * progress,
            ),
            shadow_intensity: start.shadow_intensity + (target.shadow_intensity - start.shadow_intensity) * progress,
        }
    }

    /// Check if a tool has an active animation
    pub fn is_animating(&self, tool_type: ToolType) -> bool {
        self.animations.get(&tool_type).map_or(false, |a| a.is_active)
    }

    /// Stop animation for a tool
    pub fn stop_animation(&mut self, tool_type: ToolType) {
        self.animations.remove(&tool_type);
    }

    /// Stop all animations
    pub fn stop_all_animations(&mut self) {
        self.animations.clear();
    }

    /// Get the currently active tool
    pub fn get_active_tool(&self) -> Option<ToolType> {
        self.active_tool
    }

    /// Enable or disable animations
    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.animations_enabled = enabled;
        if !enabled {
            self.stop_all_animations();
        }
    }

    /// Get the number of active animations
    pub fn active_animation_count(&self) -> usize {
        self.animations.values().filter(|a| a.is_active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ToolType;

    #[test]
    fn test_enhanced_tool_animation_manager_creation() {
        let manager = EnhancedToolAnimationManager::new();

        // Should have default durations and easings
        assert!(manager.default_durations.len() > 0);
        assert!(manager.default_easings.len() > 0);
        assert_eq!(manager.active_animation_count(), 0);
        assert!(manager.animations_enabled);
        assert_eq!(manager.get_active_tool(), None);
    }

    #[test]
    fn test_tool_activation_animation() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Select;

        // Start activation animation
        manager.start_tool_activation(tool_type);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(tool_type));
        assert_eq!(manager.get_active_tool(), Some(tool_type));

        // Should have current state
        let state = manager.get_current_state(tool_type);
        assert!(state.is_some());

        let state = state.unwrap();
        assert!(state.scale > 0.0);
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);
        assert!(state.glow_intensity >= 0.0 && state.glow_intensity <= 1.0);
    }

    #[test]
    fn test_tool_deactivation_animation() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Brush;

        // First activate
        manager.start_tool_activation(tool_type);
        assert_eq!(manager.get_active_tool(), Some(tool_type));

        // Then deactivate
        manager.start_tool_deactivation(tool_type);
        assert_eq!(manager.get_active_tool(), None);
        assert!(manager.is_animating(tool_type));

        // Should have deactivation state
        let state = manager.get_current_state(tool_type).unwrap();
        assert!(state.scale >= 0.0);
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);
    }

    #[test]
    fn test_tool_switch_animation() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool1 = ToolType::Select;
        let tool2 = ToolType::Brush;

        // Activate first tool
        manager.start_tool_activation(tool1);
        assert_eq!(manager.get_active_tool(), Some(tool1));

        // Switch to second tool
        manager.start_tool_activation(tool2);
        assert_eq!(manager.get_active_tool(), Some(tool2));

        // Should have animations for both tools
        assert!(manager.is_animating(tool1) || manager.is_animating(tool2));
    }

    #[test]
    fn test_hover_effect_animation() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Eraser;

        // Start hover enter animation
        manager.start_hover_effect(tool_type, true);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(tool_type));

        // Get current state
        let state = manager.get_current_state(tool_type).unwrap();
        assert!(state.scale >= 1.0);
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);

        // Start hover exit animation
        manager.start_hover_effect(tool_type, false);

        // Should still have one animation (replaced)
        assert_eq!(manager.active_animation_count(), 1);
    }

    #[test]
    fn test_loading_animation() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Move;

        // Start loading animation
        manager.start_loading_animation(tool_type);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(tool_type));

        // Get current state
        let state = manager.get_current_state(tool_type).unwrap();
        assert!(state.rotation >= 0.0);
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);
    }

    #[test]
    fn test_animation_update() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Select;

        // Start animation
        manager.start_tool_activation(tool_type);

        // Should be animating
        assert!(manager.update());
        assert!(manager.is_animating(tool_type));

        // Multiple updates should work
        for _ in 0..10 {
            manager.update();
        }

        assert!(manager.is_animating(tool_type));
    }

    #[test]
    fn test_stop_animations() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Brush;

        // Start animation
        manager.start_tool_activation(tool_type);
        assert!(manager.is_animating(tool_type));

        // Stop specific animation
        manager.stop_animation(tool_type);
        assert!(!manager.is_animating(tool_type));

        // Start multiple animations
        manager.start_tool_activation(ToolType::Select);
        manager.start_tool_activation(ToolType::Brush);
        manager.start_tool_activation(ToolType::Eraser);
        assert!(manager.active_animation_count() > 0);

        // Stop all animations
        manager.stop_all_animations();
        assert_eq!(manager.active_animation_count(), 0);
    }

    #[test]
    fn test_animations_enabled_toggle() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Select;

        // Disable animations
        manager.set_animations_enabled(false);

        // Start activation (should not create animation)
        manager.start_tool_activation(tool_type);
        assert_eq!(manager.active_animation_count(), 0);
        assert_eq!(manager.get_active_tool(), Some(tool_type)); // Tool should still be active

        // Re-enable animations
        manager.set_animations_enabled(true);

        // Start activation (should create animation)
        manager.start_tool_activation(ToolType::Brush);
        assert!(manager.active_animation_count() > 0);
    }

    #[test]
    fn test_state_interpolation() {
        let manager = EnhancedToolAnimationManager::new();

        let start_state = ToolAnimationState {
            scale: 0.5,
            opacity: 0.0,
            glow_intensity: 0.0,
            rotation: 0.0,
            ..Default::default()
        };

        let target_state = ToolAnimationState {
            scale: 1.0,
            opacity: 1.0,
            glow_intensity: 1.0,
            rotation: std::f32::consts::PI,
            ..Default::default()
        };

        // Test interpolation at 50%
        let interpolated = manager.interpolate_states(&start_state, &target_state, 0.5);

        assert!((interpolated.scale - 0.75).abs() < 0.001);
        assert!((interpolated.opacity - 0.5).abs() < 0.001);
        assert!((interpolated.glow_intensity - 0.5).abs() < 0.001);
        assert!((interpolated.rotation - std::f32::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_types() {
        let mut manager = EnhancedToolAnimationManager::new();
        let tool_type = ToolType::Select;

        // Test all animation types
        manager.start_tool_activation(tool_type);
        let activation_state = manager.get_current_state(tool_type);
        assert!(activation_state.is_some());

        manager.start_tool_deactivation(tool_type);
        let deactivation_state = manager.get_current_state(tool_type);
        assert!(deactivation_state.is_some());

        manager.start_hover_effect(tool_type, true);
        let hover_state = manager.get_current_state(tool_type);
        assert!(hover_state.is_some());

        manager.start_loading_animation(tool_type);
        let loading_state = manager.get_current_state(tool_type);
        assert!(loading_state.is_some());

        // All states should be valid
        assert!(activation_state.unwrap().scale > 0.0);
        assert!(deactivation_state.unwrap().scale > 0.0);
        assert!(hover_state.unwrap().scale > 0.0);
        assert!(loading_state.unwrap().scale >= 0.0);
    }

    #[test]
    fn test_color_interpolation() {
        let manager = EnhancedToolAnimationManager::new();

        let start_state = ToolAnimationState {
            glow_color: Color::from_rgba(1.0, 0.0, 0.0, 1.0), // Red
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0), // Transparent black
            ..Default::default()
        };

        let target_state = ToolAnimationState {
            glow_color: Color::from_rgba(0.0, 1.0, 0.0, 1.0), // Green
            background_color: Color::from_rgba(1.0, 1.0, 1.0, 1.0), // White
            ..Default::default()
        };

        // Test color interpolation at 50%
        let interpolated = manager.interpolate_states(&start_state, &target_state, 0.5);

        // Should be halfway between red and green
        assert!((interpolated.glow_color.r - 0.5).abs() < 0.001);
        assert!((interpolated.glow_color.g - 0.5).abs() < 0.001);
        assert!((interpolated.glow_color.b - 0.0).abs() < 0.001);

        // Should be halfway between transparent black and white
        assert!((interpolated.background_color.r - 0.5).abs() < 0.001);
        assert!((interpolated.background_color.g - 0.5).abs() < 0.001);
        assert!((interpolated.background_color.b - 0.5).abs() < 0.001);
        assert!((interpolated.background_color.a - 0.5).abs() < 0.001);
    }
}
