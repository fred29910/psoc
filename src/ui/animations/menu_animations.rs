//! Advanced menu animation system for PSOC Image Editor
//! Provides smooth transitions and visual effects for menu interactions

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::{Color, Point, Size};

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic, ease_out_back};
use crate::ui::components::MenuCategoryId;

/// Types of menu transitions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionType {
    /// Menu sliding down from top
    SlideDown,
    /// Menu fading in/out
    Fade,
    /// Menu scaling from center
    Scale,
    /// Menu sliding with bounce effect
    BounceDown,
}

/// Menu transition state
#[derive(Debug, Clone)]
pub struct MenuTransition {
    /// Type of transition
    pub transition_type: TransitionType,
    /// Start time of the transition
    pub start_time: Instant,
    /// Duration of the transition
    pub duration: Duration,
    /// Whether this is opening or closing
    pub is_opening: bool,
    /// Easing function to use
    pub easing: EasingFunction,
    /// Starting position/state
    pub start_state: TransitionState,
    /// Target position/state
    pub target_state: TransitionState,
}

/// State values for transitions
#[derive(Debug, Clone)]
pub struct TransitionState {
    /// Position offset
    pub position: Point,
    /// Scale factor (1.0 = normal size)
    pub scale: f32,
    /// Opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
    /// Background color
    pub background_color: Color,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            scale: 1.0,
            opacity: 1.0,
            background_color: Color::TRANSPARENT,
        }
    }
}

/// Menu animation manager
#[derive(Debug)]
pub struct MenuAnimationManager {
    /// Active transitions for each menu
    transitions: HashMap<MenuCategoryId, MenuTransition>,
    /// Default transition duration
    default_duration: Duration,
    /// Default transition type
    default_transition: TransitionType,
}

impl Default for MenuAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuAnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            default_duration: Duration::from_millis(250), // 250ms default
            default_transition: TransitionType::SlideDown,
        }
    }

    /// Start opening animation for a menu
    pub fn start_open_animation(&mut self, menu_id: MenuCategoryId, position: Point) {
        let transition = self.create_open_transition(menu_id, position);
        self.transitions.insert(menu_id, transition);
    }

    /// Start closing animation for a menu
    pub fn start_close_animation(&mut self, menu_id: MenuCategoryId) {
        if let Some(current) = self.transitions.get(&menu_id) {
            let mut transition = self.create_close_transition(menu_id);
            // If currently opening, start from current state
            if current.is_opening {
                let progress = self.get_transition_progress(current);
                transition.start_state = self.interpolate_state(current, progress);
            }
            self.transitions.insert(menu_id, transition);
        } else {
            let transition = self.create_close_transition(menu_id);
            self.transitions.insert(menu_id, transition);
        }
    }

    /// Update all animations and return whether any are still active
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let mut completed = Vec::new();

        for (menu_id, transition) in &self.transitions {
            if now.duration_since(transition.start_time) >= transition.duration {
                completed.push(*menu_id);
            }
        }

        // Remove completed transitions
        for menu_id in completed {
            self.transitions.remove(&menu_id);
        }

        !self.transitions.is_empty()
    }

    /// Get current animation state for a menu
    pub fn get_current_state(&self, menu_id: MenuCategoryId) -> Option<TransitionState> {
        self.transitions.get(&menu_id).map(|transition| {
            let progress = self.get_transition_progress(transition);
            self.interpolate_state(transition, progress)
        })
    }

    /// Check if a menu is currently animating
    pub fn is_animating(&self, menu_id: MenuCategoryId) -> bool {
        self.transitions.contains_key(&menu_id)
    }

    /// Get the progress of a transition (0.0 to 1.0)
    fn get_transition_progress(&self, transition: &MenuTransition) -> f32 {
        let elapsed = Instant::now().duration_since(transition.start_time);
        let progress = elapsed.as_secs_f32() / transition.duration.as_secs_f32();
        progress.clamp(0.0, 1.0)
    }

    /// Interpolate between start and target states
    fn interpolate_state(&self, transition: &MenuTransition, progress: f32) -> TransitionState {
        let eased_progress = (transition.easing)(progress);
        
        TransitionState {
            position: Point::new(
                transition.start_state.position.x + 
                (transition.target_state.position.x - transition.start_state.position.x) * eased_progress,
                transition.start_state.position.y + 
                (transition.target_state.position.y - transition.start_state.position.y) * eased_progress,
            ),
            scale: transition.start_state.scale + 
                (transition.target_state.scale - transition.start_state.scale) * eased_progress,
            opacity: transition.start_state.opacity + 
                (transition.target_state.opacity - transition.start_state.opacity) * eased_progress,
            background_color: Color {
                r: transition.start_state.background_color.r + 
                   (transition.target_state.background_color.r - transition.start_state.background_color.r) * eased_progress,
                g: transition.start_state.background_color.g + 
                   (transition.target_state.background_color.g - transition.start_state.background_color.g) * eased_progress,
                b: transition.start_state.background_color.b + 
                   (transition.target_state.background_color.b - transition.start_state.background_color.b) * eased_progress,
                a: transition.start_state.background_color.a + 
                   (transition.target_state.background_color.a - transition.start_state.background_color.a) * eased_progress,
            },
        }
    }

    /// Create opening transition
    fn create_open_transition(&self, _menu_id: MenuCategoryId, position: Point) -> MenuTransition {
        let (start_state, target_state, easing) = match self.default_transition {
            TransitionType::SlideDown => (
                TransitionState {
                    position: Point::new(position.x, position.y - 20.0),
                    opacity: 0.0,
                    ..Default::default()
                },
                TransitionState {
                    position,
                    opacity: 1.0,
                    ..Default::default()
                },
                ease_out_cubic as EasingFunction,
            ),
            TransitionType::Fade => (
                TransitionState {
                    position,
                    opacity: 0.0,
                    ..Default::default()
                },
                TransitionState {
                    position,
                    opacity: 1.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            TransitionType::Scale => (
                TransitionState {
                    position,
                    scale: 0.8,
                    opacity: 0.0,
                    ..Default::default()
                },
                TransitionState {
                    position,
                    scale: 1.0,
                    opacity: 1.0,
                    ..Default::default()
                },
                ease_out_back as EasingFunction,
            ),
            TransitionType::BounceDown => (
                TransitionState {
                    position: Point::new(position.x, position.y - 30.0),
                    opacity: 0.0,
                    ..Default::default()
                },
                TransitionState {
                    position,
                    opacity: 1.0,
                    ..Default::default()
                },
                ease_out_back as EasingFunction,
            ),
        };

        MenuTransition {
            transition_type: self.default_transition,
            start_time: Instant::now(),
            duration: self.default_duration,
            is_opening: true,
            easing,
            start_state,
            target_state,
        }
    }

    /// Create closing transition
    fn create_close_transition(&self, _menu_id: MenuCategoryId) -> MenuTransition {
        let (start_state, target_state, easing) = match self.default_transition {
            TransitionType::SlideDown => (
                TransitionState {
                    opacity: 1.0,
                    ..Default::default()
                },
                TransitionState {
                    position: Point::new(0.0, -10.0),
                    opacity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            TransitionType::Fade => (
                TransitionState {
                    opacity: 1.0,
                    ..Default::default()
                },
                TransitionState {
                    opacity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            TransitionType::Scale => (
                TransitionState {
                    scale: 1.0,
                    opacity: 1.0,
                    ..Default::default()
                },
                TransitionState {
                    scale: 0.8,
                    opacity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
            TransitionType::BounceDown => (
                TransitionState {
                    opacity: 1.0,
                    ..Default::default()
                },
                TransitionState {
                    position: Point::new(0.0, -15.0),
                    opacity: 0.0,
                    ..Default::default()
                },
                ease_in_out_cubic as EasingFunction,
            ),
        };

        MenuTransition {
            transition_type: self.default_transition,
            start_time: Instant::now(),
            duration: Duration::from_millis(200), // Slightly faster for closing
            is_opening: false,
            easing,
            start_state,
            target_state,
        }
    }

    /// Set default transition type
    pub fn set_default_transition(&mut self, transition_type: TransitionType) {
        self.default_transition = transition_type;
    }

    /// Set default duration
    pub fn set_default_duration(&mut self, duration: Duration) {
        self.default_duration = duration;
    }
}
