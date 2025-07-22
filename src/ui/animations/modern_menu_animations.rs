//! Modern menu animation system for enhanced user experience
//! Provides smooth transitions, underline animations, and hover effects

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::{Color, Point};

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic, ease_out_quart};
use crate::ui::components::MenuCategoryId;

/// Types of modern menu animations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModernMenuAnimationType {
    /// Underline sliding animation
    UnderlineSlide,
    /// Menu button hover effect
    ButtonHover,
    /// Dropdown menu appearance
    DropdownReveal,
    /// Menu item highlight
    ItemHighlight,
}

/// Modern menu animation state
#[derive(Debug, Clone)]
pub struct ModernMenuAnimation {
    /// Type of animation
    pub animation_type: ModernMenuAnimationType,
    /// Start time of the animation
    pub start_time: Instant,
    /// Duration of the animation
    pub duration: Duration,
    /// Easing function to use
    pub easing: EasingFunction,
    /// Starting values
    pub start_values: AnimationValues,
    /// Target values
    pub target_values: AnimationValues,
    /// Whether the animation is active
    pub is_active: bool,
}

/// Animation values for different properties
#[derive(Debug, Clone)]
pub struct AnimationValues {
    /// Position (for sliding effects)
    pub position: Point,
    /// Width (for underline animations)
    pub width: f32,
    /// Opacity (for fade effects)
    pub opacity: f32,
    /// Scale (for hover effects)
    pub scale: f32,
    /// Color (for color transitions)
    pub color: Color,
}

impl Default for AnimationValues {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            width: 0.0,
            opacity: 0.0,
            scale: 1.0,
            color: Color::TRANSPARENT,
        }
    }
}

/// Manager for modern menu animations
#[derive(Debug)]
pub struct ModernMenuAnimationManager {
    /// Active animations by menu category
    animations: HashMap<MenuCategoryId, ModernMenuAnimation>,
    /// Underline animation state
    underline_animation: Option<UnderlineAnimation>,
    /// Default animation durations
    default_durations: HashMap<ModernMenuAnimationType, Duration>,
}

/// Underline animation specific state
#[derive(Debug, Clone)]
pub struct UnderlineAnimation {
    /// Current position of the underline
    pub current_position: f32,
    /// Target position of the underline
    pub target_position: f32,
    /// Current width of the underline
    pub current_width: f32,
    /// Target width of the underline
    pub target_width: f32,
    /// Animation start time
    pub start_time: Instant,
    /// Animation duration
    pub duration: Duration,
    /// Easing function
    pub easing: EasingFunction,
    /// Whether the animation is active
    pub is_active: bool,
}

impl Default for ModernMenuAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ModernMenuAnimationManager {
    /// Create a new modern menu animation manager
    pub fn new() -> Self {
        let mut default_durations = HashMap::new();
        default_durations.insert(ModernMenuAnimationType::UnderlineSlide, Duration::from_millis(300));
        default_durations.insert(ModernMenuAnimationType::ButtonHover, Duration::from_millis(200));
        default_durations.insert(ModernMenuAnimationType::DropdownReveal, Duration::from_millis(250));
        default_durations.insert(ModernMenuAnimationType::ItemHighlight, Duration::from_millis(150));

        Self {
            animations: HashMap::new(),
            underline_animation: None,
            default_durations,
        }
    }

    /// Start an underline slide animation
    pub fn start_underline_slide(&mut self, from_position: f32, to_position: f32, from_width: f32, to_width: f32) {
        let duration = self.default_durations[&ModernMenuAnimationType::UnderlineSlide];
        
        self.underline_animation = Some(UnderlineAnimation {
            current_position: from_position,
            target_position: to_position,
            current_width: from_width,
            target_width: to_width,
            start_time: Instant::now(),
            duration,
            easing: ease_out_quart as EasingFunction,
            is_active: true,
        });
    }

    /// Start a button hover animation
    pub fn start_button_hover(&mut self, category_id: MenuCategoryId, is_entering: bool) {
        let duration = self.default_durations[&ModernMenuAnimationType::ButtonHover];
        
        let (start_values, target_values) = if is_entering {
            (
                AnimationValues {
                    scale: 1.0,
                    opacity: 0.0,
                    color: Color::TRANSPARENT,
                    ..Default::default()
                },
                AnimationValues {
                    scale: 1.02,
                    opacity: 0.1,
                    color: Color::from_rgba(0.0, 0.75, 1.0, 0.1), // tech-blue with low opacity
                    ..Default::default()
                },
            )
        } else {
            (
                AnimationValues {
                    scale: 1.02,
                    opacity: 0.1,
                    color: Color::from_rgba(0.0, 0.75, 1.0, 0.1),
                    ..Default::default()
                },
                AnimationValues {
                    scale: 1.0,
                    opacity: 0.0,
                    color: Color::TRANSPARENT,
                    ..Default::default()
                },
            )
        };

        let animation = ModernMenuAnimation {
            animation_type: ModernMenuAnimationType::ButtonHover,
            start_time: Instant::now(),
            duration,
            easing: ease_in_out_cubic as EasingFunction,
            start_values,
            target_values,
            is_active: true,
        };

        self.animations.insert(category_id, animation);
    }

    /// Start a dropdown reveal animation
    pub fn start_dropdown_reveal(&mut self, category_id: MenuCategoryId, is_opening: bool) {
        let duration = self.default_durations[&ModernMenuAnimationType::DropdownReveal];
        
        let (start_values, target_values) = if is_opening {
            (
                AnimationValues {
                    position: Point::new(0.0, -10.0),
                    opacity: 0.0,
                    scale: 0.95,
                    ..Default::default()
                },
                AnimationValues {
                    position: Point::ORIGIN,
                    opacity: 1.0,
                    scale: 1.0,
                    ..Default::default()
                },
            )
        } else {
            (
                AnimationValues {
                    position: Point::ORIGIN,
                    opacity: 1.0,
                    scale: 1.0,
                    ..Default::default()
                },
                AnimationValues {
                    position: Point::new(0.0, -10.0),
                    opacity: 0.0,
                    scale: 0.95,
                    ..Default::default()
                },
            )
        };

        let animation = ModernMenuAnimation {
            animation_type: ModernMenuAnimationType::DropdownReveal,
            start_time: Instant::now(),
            duration,
            easing: ease_out_cubic as EasingFunction,
            start_values,
            target_values,
            is_active: true,
        };

        self.animations.insert(category_id, animation);
    }

    /// Update all animations
    pub fn update(&mut self) -> bool {
        let mut has_active_animations = false;
        let now = Instant::now();

        // Update underline animation
        if let Some(ref mut underline) = self.underline_animation {
            if underline.is_active {
                let elapsed = now.duration_since(underline.start_time);
                if elapsed >= underline.duration {
                    // Animation complete
                    underline.current_position = underline.target_position;
                    underline.current_width = underline.target_width;
                    underline.is_active = false;
                } else {
                    // Interpolate values
                    let progress = elapsed.as_secs_f32() / underline.duration.as_secs_f32();
                    let eased_progress = (underline.easing)(progress);
                    
                    underline.current_position = underline.current_position + 
                        (underline.target_position - underline.current_position) * eased_progress;
                    underline.current_width = underline.current_width + 
                        (underline.target_width - underline.current_width) * eased_progress;
                    
                    has_active_animations = true;
                }
            }
        }

        // Update other animations
        let mut completed_animations = Vec::new();
        for (category_id, animation) in &mut self.animations {
            if animation.is_active {
                let elapsed = now.duration_since(animation.start_time);
                if elapsed >= animation.duration {
                    animation.is_active = false;
                    completed_animations.push(*category_id);
                } else {
                    has_active_animations = true;
                }
            }
        }

        // Remove completed animations
        for category_id in completed_animations {
            self.animations.remove(&category_id);
        }

        has_active_animations
    }

    /// Get current underline state
    pub fn get_underline_state(&self) -> Option<(f32, f32)> {
        self.underline_animation.as_ref().map(|anim| {
            (anim.current_position, anim.current_width)
        })
    }

    /// Get current animation values for a menu category
    pub fn get_animation_values(&self, category_id: MenuCategoryId) -> Option<AnimationValues> {
        self.animations.get(&category_id).and_then(|animation| {
            if !animation.is_active {
                return None;
            }

            let elapsed = Instant::now().duration_since(animation.start_time);
            if elapsed >= animation.duration {
                return Some(animation.target_values.clone());
            }

            let progress = elapsed.as_secs_f32() / animation.duration.as_secs_f32();
            let eased_progress = (animation.easing)(progress);

            Some(self.interpolate_values(&animation.start_values, &animation.target_values, eased_progress))
        })
    }

    /// Interpolate between two animation value sets
    fn interpolate_values(&self, start: &AnimationValues, target: &AnimationValues, progress: f32) -> AnimationValues {
        AnimationValues {
            position: Point::new(
                start.position.x + (target.position.x - start.position.x) * progress,
                start.position.y + (target.position.y - start.position.y) * progress,
            ),
            width: start.width + (target.width - start.width) * progress,
            opacity: start.opacity + (target.opacity - start.opacity) * progress,
            scale: start.scale + (target.scale - start.scale) * progress,
            color: Color::from_rgba(
                start.color.r + (target.color.r - start.color.r) * progress,
                start.color.g + (target.color.g - start.color.g) * progress,
                start.color.b + (target.color.b - start.color.b) * progress,
                start.color.a + (target.color.a - start.color.a) * progress,
            ),
        }
    }

    /// Check if any animations are active
    pub fn has_active_animations(&self) -> bool {
        self.underline_animation.as_ref().map_or(false, |anim| anim.is_active) ||
        self.animations.values().any(|anim| anim.is_active)
    }

    /// Stop all animations for a category
    pub fn stop_animations(&mut self, category_id: MenuCategoryId) {
        self.animations.remove(&category_id);
    }

    /// Stop all animations
    pub fn stop_all_animations(&mut self) {
        self.animations.clear();
        self.underline_animation = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_manager_creation() {
        let manager = ModernMenuAnimationManager::new();
        assert!(!manager.has_active_animations());
        assert_eq!(manager.animations.len(), 0);
        assert!(manager.underline_animation.is_none());
    }

    #[test]
    fn test_underline_animation() {
        let mut manager = ModernMenuAnimationManager::new();
        
        manager.start_underline_slide(0.0, 100.0, 50.0, 80.0);
        assert!(manager.has_active_animations());
        
        let state = manager.get_underline_state();
        assert!(state.is_some());
        let (pos, width) = state.unwrap();
        assert_eq!(pos, 0.0);
        assert_eq!(width, 50.0);
    }

    #[test]
    fn test_button_hover_animation() {
        let mut manager = ModernMenuAnimationManager::new();
        
        manager.start_button_hover(MenuCategoryId::File, true);
        assert!(manager.has_active_animations());
        
        let values = manager.get_animation_values(MenuCategoryId::File);
        assert!(values.is_some());
    }

    #[test]
    fn test_dropdown_reveal_animation() {
        let mut manager = ModernMenuAnimationManager::new();
        
        manager.start_dropdown_reveal(MenuCategoryId::Edit, true);
        assert!(manager.has_active_animations());
        
        let values = manager.get_animation_values(MenuCategoryId::Edit);
        assert!(values.is_some());
    }

    #[test]
    fn test_animation_cleanup() {
        let mut manager = ModernMenuAnimationManager::new();
        
        manager.start_button_hover(MenuCategoryId::File, true);
        manager.start_dropdown_reveal(MenuCategoryId::Edit, true);
        assert!(manager.has_active_animations());
        
        manager.stop_all_animations();
        assert!(!manager.has_active_animations());
    }
}
