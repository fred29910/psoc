//! Enhanced transition animation system for smooth UI interactions
//! Provides comprehensive animation support for panels, tools, and UI elements

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::{Color, Point, Size};

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic, ease_out_back, ease_out_quart};

/// Types of UI transition animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionType {
    /// Panel expand/collapse animation
    PanelExpand,
    /// Panel slide in/out animation
    PanelSlide,
    /// Tool switch transition
    ToolSwitch,
    /// Hover state animation
    HoverState,
    /// Loading animation
    Loading,
    /// State change animation
    StateChange,
    /// Fade in/out animation
    Fade,
    /// Scale animation
    Scale,
    /// Bounce animation
    Bounce,
}

/// Direction for directional animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Up,
    Down,
    Left,
    Right,
    In,
    Out,
}

/// Animation state values
#[derive(Debug, Clone)]
pub struct TransitionState {
    /// Position offset
    pub position: Point,
    /// Size scaling
    pub size: Size,
    /// Opacity value (0.0 to 1.0)
    pub opacity: f32,
    /// Scale factor
    pub scale: f32,
    /// Rotation in radians
    pub rotation: f32,
    /// Color value
    pub color: Color,
    /// Custom properties
    pub custom_values: HashMap<String, f32>,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            size: Size::new(0.0, 0.0),
            opacity: 1.0,
            scale: 1.0,
            rotation: 0.0,
            color: Color::WHITE,
            custom_values: HashMap::new(),
        }
    }
}

/// A transition animation instance
#[derive(Debug, Clone)]
pub struct Transition {
    /// Type of transition
    pub transition_type: TransitionType,
    /// Animation direction
    pub direction: AnimationDirection,
    /// Start time
    pub start_time: Instant,
    /// Animation duration
    pub duration: Duration,
    /// Easing function
    pub easing: EasingFunction,
    /// Starting state
    pub start_state: TransitionState,
    /// Target state
    pub target_state: TransitionState,
    /// Whether the animation is active
    pub is_active: bool,
    /// Whether the animation should loop
    pub should_loop: bool,
    /// Loop count (-1 for infinite)
    pub loop_count: i32,
    /// Current loop iteration
    pub current_loop: i32,
}

/// Enhanced transition animation manager
#[derive(Debug)]
pub struct TransitionManager {
    /// Active transitions by ID
    transitions: HashMap<String, Transition>,
    /// Default animation durations by type
    default_durations: HashMap<TransitionType, Duration>,
    /// Default easing functions by type
    default_easings: HashMap<TransitionType, EasingFunction>,
}

impl Default for TransitionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TransitionManager {
    /// Create a new transition manager
    pub fn new() -> Self {
        let mut default_durations = HashMap::new();
        default_durations.insert(TransitionType::PanelExpand, Duration::from_millis(300));
        default_durations.insert(TransitionType::PanelSlide, Duration::from_millis(250));
        default_durations.insert(TransitionType::ToolSwitch, Duration::from_millis(200));
        default_durations.insert(TransitionType::HoverState, Duration::from_millis(150));
        default_durations.insert(TransitionType::Loading, Duration::from_millis(1000));
        default_durations.insert(TransitionType::StateChange, Duration::from_millis(200));
        default_durations.insert(TransitionType::Fade, Duration::from_millis(250));
        default_durations.insert(TransitionType::Scale, Duration::from_millis(200));
        default_durations.insert(TransitionType::Bounce, Duration::from_millis(400));

        let mut default_easings = HashMap::new();
        default_easings.insert(TransitionType::PanelExpand, ease_out_cubic as EasingFunction);
        default_easings.insert(TransitionType::PanelSlide, ease_out_quart as EasingFunction);
        default_easings.insert(TransitionType::ToolSwitch, ease_in_out_cubic as EasingFunction);
        default_easings.insert(TransitionType::HoverState, ease_out_cubic as EasingFunction);
        default_easings.insert(TransitionType::Loading, ease_in_out_cubic as EasingFunction);
        default_easings.insert(TransitionType::StateChange, ease_out_cubic as EasingFunction);
        default_easings.insert(TransitionType::Fade, ease_in_out_cubic as EasingFunction);
        default_easings.insert(TransitionType::Scale, ease_out_back as EasingFunction);
        default_easings.insert(TransitionType::Bounce, ease_out_back as EasingFunction);

        Self {
            transitions: HashMap::new(),
            default_durations,
            default_easings,
        }
    }

    /// Start a panel expand/collapse animation
    pub fn start_panel_expand(&mut self, panel_id: String, is_expanding: bool, target_size: Size) {
        let duration = self.default_durations[&TransitionType::PanelExpand];
        let easing = self.default_easings[&TransitionType::PanelExpand];

        let (start_state, target_state) = if is_expanding {
            (
                TransitionState {
                    size: Size::new(target_size.width, 0.0),
                    opacity: 0.0,
                    scale: 0.95,
                    ..Default::default()
                },
                TransitionState {
                    size: target_size,
                    opacity: 1.0,
                    scale: 1.0,
                    ..Default::default()
                },
            )
        } else {
            (
                TransitionState {
                    size: target_size,
                    opacity: 1.0,
                    scale: 1.0,
                    ..Default::default()
                },
                TransitionState {
                    size: Size::new(target_size.width, 0.0),
                    opacity: 0.0,
                    scale: 0.95,
                    ..Default::default()
                },
            )
        };

        let transition = Transition {
            transition_type: TransitionType::PanelExpand,
            direction: if is_expanding { AnimationDirection::Down } else { AnimationDirection::Up },
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

        self.transitions.insert(panel_id, transition);
    }

    /// Start a panel slide animation
    pub fn start_panel_slide(&mut self, panel_id: String, direction: AnimationDirection, distance: f32) {
        let duration = self.default_durations[&TransitionType::PanelSlide];
        let easing = self.default_easings[&TransitionType::PanelSlide];

        let (start_position, target_position) = match direction {
            AnimationDirection::Left => (Point::new(-distance, 0.0), Point::ORIGIN),
            AnimationDirection::Right => (Point::new(distance, 0.0), Point::ORIGIN),
            AnimationDirection::Up => (Point::new(0.0, -distance), Point::ORIGIN),
            AnimationDirection::Down => (Point::new(0.0, distance), Point::ORIGIN),
            AnimationDirection::In => (Point::ORIGIN, Point::ORIGIN),
            AnimationDirection::Out => (Point::ORIGIN, Point::new(distance, 0.0)),
        };

        let transition = Transition {
            transition_type: TransitionType::PanelSlide,
            direction,
            start_time: Instant::now(),
            duration,
            easing,
            start_state: TransitionState {
                position: start_position,
                opacity: 0.0,
                ..Default::default()
            },
            target_state: TransitionState {
                position: target_position,
                opacity: 1.0,
                ..Default::default()
            },
            is_active: true,
            should_loop: false,
            loop_count: 0,
            current_loop: 0,
        };

        self.transitions.insert(panel_id, transition);
    }

    /// Start a tool switch animation
    pub fn start_tool_switch(&mut self, tool_id: String, is_activating: bool) {
        let duration = self.default_durations[&TransitionType::ToolSwitch];
        let easing = self.default_easings[&TransitionType::ToolSwitch];

        let (start_state, target_state) = if is_activating {
            (
                TransitionState {
                    scale: 0.8,
                    opacity: 0.5,
                    color: Color::from_rgba(0.5, 0.5, 0.5, 1.0),
                    ..Default::default()
                },
                TransitionState {
                    scale: 1.1,
                    opacity: 1.0,
                    color: Color::from_rgba(0.0, 0.75, 1.0, 1.0), // tech-blue
                    ..Default::default()
                },
            )
        } else {
            (
                TransitionState {
                    scale: 1.1,
                    opacity: 1.0,
                    color: Color::from_rgba(0.0, 0.75, 1.0, 1.0),
                    ..Default::default()
                },
                TransitionState {
                    scale: 1.0,
                    opacity: 0.7,
                    color: Color::from_rgba(0.7, 0.7, 0.7, 1.0),
                    ..Default::default()
                },
            )
        };

        let transition = Transition {
            transition_type: TransitionType::ToolSwitch,
            direction: if is_activating { AnimationDirection::In } else { AnimationDirection::Out },
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

        self.transitions.insert(tool_id, transition);
    }

    /// Start a hover state animation
    pub fn start_hover_animation(&mut self, element_id: String, is_entering: bool) {
        let duration = self.default_durations[&TransitionType::HoverState];
        let easing = self.default_easings[&TransitionType::HoverState];

        let (start_state, target_state) = if is_entering {
            (
                TransitionState {
                    scale: 1.0,
                    opacity: 1.0,
                    color: Color::TRANSPARENT,
                    ..Default::default()
                },
                TransitionState {
                    scale: 1.05,
                    opacity: 1.0,
                    color: Color::from_rgba(0.0, 0.75, 1.0, 0.1), // tech-blue with low opacity
                    ..Default::default()
                },
            )
        } else {
            (
                TransitionState {
                    scale: 1.05,
                    opacity: 1.0,
                    color: Color::from_rgba(0.0, 0.75, 1.0, 0.1),
                    ..Default::default()
                },
                TransitionState {
                    scale: 1.0,
                    opacity: 1.0,
                    color: Color::TRANSPARENT,
                    ..Default::default()
                },
            )
        };

        let transition = Transition {
            transition_type: TransitionType::HoverState,
            direction: if is_entering { AnimationDirection::In } else { AnimationDirection::Out },
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

        self.transitions.insert(element_id, transition);
    }

    /// Start a loading animation
    pub fn start_loading_animation(&mut self, element_id: String) {
        let duration = self.default_durations[&TransitionType::Loading];
        let easing = self.default_easings[&TransitionType::Loading];

        let transition = Transition {
            transition_type: TransitionType::Loading,
            direction: AnimationDirection::In,
            start_time: Instant::now(),
            duration,
            easing,
            start_state: TransitionState {
                rotation: 0.0,
                opacity: 0.5,
                ..Default::default()
            },
            target_state: TransitionState {
                rotation: std::f32::consts::PI * 2.0, // Full rotation
                opacity: 1.0,
                ..Default::default()
            },
            is_active: true,
            should_loop: true,
            loop_count: -1, // Infinite loop
            current_loop: 0,
        };

        self.transitions.insert(element_id, transition);
    }

    /// Update all animations and return whether any are still active
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let mut completed_transitions = Vec::new();

        for (id, transition) in &mut self.transitions {
            if !transition.is_active {
                continue;
            }

            let elapsed = now.duration_since(transition.start_time);
            
            if elapsed >= transition.duration {
                if transition.should_loop && (transition.loop_count < 0 || transition.current_loop < transition.loop_count) {
                    // Reset for next loop
                    transition.start_time = now;
                    transition.current_loop += 1;
                } else {
                    // Animation completed
                    transition.is_active = false;
                    completed_transitions.push(id.clone());
                }
            }
        }

        // Remove completed non-looping animations
        for id in completed_transitions {
            self.transitions.remove(&id);
        }

        !self.transitions.is_empty()
    }

    /// Get current animation state for an element
    pub fn get_current_state(&self, element_id: &str) -> Option<TransitionState> {
        self.transitions.get(element_id).and_then(|transition| {
            if !transition.is_active {
                return None;
            }

            let elapsed = Instant::now().duration_since(transition.start_time);
            if elapsed >= transition.duration {
                return Some(transition.target_state.clone());
            }

            let progress = elapsed.as_secs_f32() / transition.duration.as_secs_f32();
            let eased_progress = (transition.easing)(progress);

            Some(self.interpolate_states(&transition.start_state, &transition.target_state, eased_progress))
        })
    }

    /// Interpolate between two transition states
    fn interpolate_states(&self, start: &TransitionState, target: &TransitionState, progress: f32) -> TransitionState {
        let mut custom_values = HashMap::new();
        
        // Interpolate custom values
        for (key, start_val) in &start.custom_values {
            if let Some(target_val) = target.custom_values.get(key) {
                custom_values.insert(key.clone(), start_val + (target_val - start_val) * progress);
            } else {
                custom_values.insert(key.clone(), *start_val);
            }
        }

        TransitionState {
            position: Point::new(
                start.position.x + (target.position.x - start.position.x) * progress,
                start.position.y + (target.position.y - start.position.y) * progress,
            ),
            size: Size::new(
                start.size.width + (target.size.width - start.size.width) * progress,
                start.size.height + (target.size.height - start.size.height) * progress,
            ),
            opacity: start.opacity + (target.opacity - start.opacity) * progress,
            scale: start.scale + (target.scale - start.scale) * progress,
            rotation: start.rotation + (target.rotation - start.rotation) * progress,
            color: Color::from_rgba(
                start.color.r + (target.color.r - start.color.r) * progress,
                start.color.g + (target.color.g - start.color.g) * progress,
                start.color.b + (target.color.b - start.color.b) * progress,
                start.color.a + (target.color.a - start.color.a) * progress,
            ),
            custom_values,
        }
    }

    /// Check if an element has an active animation
    pub fn is_animating(&self, element_id: &str) -> bool {
        self.transitions.get(element_id).map_or(false, |t| t.is_active)
    }

    /// Stop animation for an element
    pub fn stop_animation(&mut self, element_id: &str) {
        self.transitions.remove(element_id);
    }

    /// Stop all animations
    pub fn stop_all_animations(&mut self) {
        self.transitions.clear();
    }

    /// Get the number of active animations
    pub fn active_animation_count(&self) -> usize {
        self.transitions.values().filter(|t| t.is_active).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_transition_manager_creation() {
        let manager = TransitionManager::new();

        // Should have default durations and easings
        assert!(manager.default_durations.len() > 0);
        assert!(manager.default_easings.len() > 0);
        assert_eq!(manager.active_animation_count(), 0);
    }

    #[test]
    fn test_panel_expand_animation() {
        let mut manager = TransitionManager::new();
        let panel_id = "test_panel".to_string();
        let target_size = Size::new(200.0, 300.0);

        // Start expand animation
        manager.start_panel_expand(panel_id.clone(), true, target_size);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(&panel_id));

        // Should have current state
        let state = manager.get_current_state(&panel_id);
        assert!(state.is_some());
    }

    #[test]
    fn test_panel_slide_animation() {
        let mut manager = TransitionManager::new();
        let panel_id = "slide_panel".to_string();

        // Start slide animation
        manager.start_panel_slide(panel_id.clone(), AnimationDirection::Left, 100.0);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(&panel_id));

        // Get current state
        let state = manager.get_current_state(&panel_id).unwrap();
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);
    }

    #[test]
    fn test_tool_switch_animation() {
        let mut manager = TransitionManager::new();
        let tool_id = "test_tool".to_string();

        // Start activation animation
        manager.start_tool_switch(tool_id.clone(), true);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(&tool_id));

        // Get current state
        let state = manager.get_current_state(&tool_id).unwrap();
        assert!(state.scale > 0.0);
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);
    }

    #[test]
    fn test_hover_animation() {
        let mut manager = TransitionManager::new();
        let element_id = "hover_element".to_string();

        // Start hover enter animation
        manager.start_hover_animation(element_id.clone(), true);

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(&element_id));

        // Start hover exit animation
        manager.start_hover_animation(element_id.clone(), false);

        // Should still have one animation (replaced)
        assert_eq!(manager.active_animation_count(), 1);
    }

    #[test]
    fn test_loading_animation() {
        let mut manager = TransitionManager::new();
        let element_id = "loading_element".to_string();

        // Start loading animation
        manager.start_loading_animation(element_id.clone());

        // Should have one active animation
        assert_eq!(manager.active_animation_count(), 1);
        assert!(manager.is_animating(&element_id));

        // Get current state
        let state = manager.get_current_state(&element_id).unwrap();
        assert!(state.rotation >= 0.0);
        assert!(state.opacity >= 0.0 && state.opacity <= 1.0);
    }

    #[test]
    fn test_animation_update() {
        let mut manager = TransitionManager::new();
        let panel_id = "update_test_panel".to_string();

        // Start a short animation
        manager.start_panel_expand(panel_id.clone(), true, Size::new(100.0, 100.0));

        // Should be animating
        assert!(manager.update());
        assert!(manager.is_animating(&panel_id));

        // Wait for animation to complete (in a real scenario, this would be frame-based)
        thread::sleep(std::time::Duration::from_millis(350)); // Longer than default duration

        // Update should return false (no active animations)
        assert!(!manager.update());
        assert!(!manager.is_animating(&panel_id));
    }

    #[test]
    fn test_stop_animations() {
        let mut manager = TransitionManager::new();
        let panel_id = "stop_test_panel".to_string();

        // Start animation
        manager.start_panel_expand(panel_id.clone(), true, Size::new(100.0, 100.0));
        assert!(manager.is_animating(&panel_id));

        // Stop specific animation
        manager.stop_animation(&panel_id);
        assert!(!manager.is_animating(&panel_id));

        // Start multiple animations
        manager.start_panel_expand("panel1".to_string(), true, Size::new(100.0, 100.0));
        manager.start_panel_expand("panel2".to_string(), true, Size::new(100.0, 100.0));
        assert_eq!(manager.active_animation_count(), 2);

        // Stop all animations
        manager.stop_all_animations();
        assert_eq!(manager.active_animation_count(), 0);
    }

    #[test]
    fn test_state_interpolation() {
        let manager = TransitionManager::new();

        let start_state = TransitionState {
            opacity: 0.0,
            scale: 0.5,
            position: Point::new(0.0, 0.0),
            ..Default::default()
        };

        let target_state = TransitionState {
            opacity: 1.0,
            scale: 1.0,
            position: Point::new(100.0, 50.0),
            ..Default::default()
        };

        // Test interpolation at 50%
        let interpolated = manager.interpolate_states(&start_state, &target_state, 0.5);

        assert!((interpolated.opacity - 0.5).abs() < 0.001);
        assert!((interpolated.scale - 0.75).abs() < 0.001);
        assert!((interpolated.position.x - 50.0).abs() < 0.001);
        assert!((interpolated.position.y - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_animation_directions() {
        let mut manager = TransitionManager::new();

        let directions = [
            AnimationDirection::Up,
            AnimationDirection::Down,
            AnimationDirection::Left,
            AnimationDirection::Right,
            AnimationDirection::In,
            AnimationDirection::Out,
        ];

        for (i, direction) in directions.iter().enumerate() {
            let panel_id = format!("direction_test_{}", i);
            manager.start_panel_slide(panel_id.clone(), *direction, 100.0);
            assert!(manager.is_animating(&panel_id));
        }

        assert_eq!(manager.active_animation_count(), directions.len());
    }

    #[test]
    fn test_transition_types() {
        let mut manager = TransitionManager::new();

        // Test all transition types
        manager.start_panel_expand("expand".to_string(), true, Size::new(100.0, 100.0));
        manager.start_panel_slide("slide".to_string(), AnimationDirection::Left, 100.0);
        manager.start_tool_switch("tool".to_string(), true);
        manager.start_hover_animation("hover".to_string(), true);
        manager.start_loading_animation("loading".to_string());

        assert_eq!(manager.active_animation_count(), 5);

        // All should have valid states
        assert!(manager.get_current_state("expand").is_some());
        assert!(manager.get_current_state("slide").is_some());
        assert!(manager.get_current_state("tool").is_some());
        assert!(manager.get_current_state("hover").is_some());
        assert!(manager.get_current_state("loading").is_some());
    }
}
