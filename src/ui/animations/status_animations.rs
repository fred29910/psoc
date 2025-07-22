//! Status panel animation system
//! Provides smooth transitions for status information updates

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::easing::{EasingFunction, ease_out_cubic, ease_in_out_cubic};
use psoc_core::RgbaPixel;

/// Types of status animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusAnimationType {
    /// Color value change animation
    ColorTransition,
    /// Position value change animation
    PositionTransition,
    /// Zoom level change animation
    ZoomTransition,
    /// Status indicator pulse animation
    StatusPulse,
    /// Value counter animation
    ValueCounter,
}

/// Status animation state
#[derive(Debug, Clone)]
pub struct StatusAnimation {
    /// Type of animation
    pub animation_type: StatusAnimationType,
    /// Start time of the animation
    pub start_time: Instant,
    /// Duration of the animation
    pub duration: Duration,
    /// Easing function to use
    pub easing: EasingFunction,
    /// Starting value
    pub start_value: StatusValue,
    /// Target value
    pub target_value: StatusValue,
}

/// Animated status values
#[derive(Debug, Clone)]
pub enum StatusValue {
    /// RGB color value
    Color(RgbaPixel),
    /// 2D position
    Position(f32, f32),
    /// Single float value
    Float(f32),
    /// Integer value
    Integer(i32),
    /// Boolean state
    Boolean(bool),
}

impl StatusValue {
    /// Interpolate between two status values
    pub fn interpolate(&self, target: &StatusValue, progress: f32) -> StatusValue {
        match (self, target) {
            (StatusValue::Color(start), StatusValue::Color(end)) => {
                StatusValue::Color(RgbaPixel::new(
                    (start.r as f32 + (end.r as f32 - start.r as f32) * progress) as u8,
                    (start.g as f32 + (end.g as f32 - start.g as f32) * progress) as u8,
                    (start.b as f32 + (end.b as f32 - start.b as f32) * progress) as u8,
                    (start.a as f32 + (end.a as f32 - start.a as f32) * progress) as u8,
                ))
            },
            (StatusValue::Position(sx, sy), StatusValue::Position(ex, ey)) => {
                StatusValue::Position(
                    sx + (ex - sx) * progress,
                    sy + (ey - sy) * progress,
                )
            },
            (StatusValue::Float(start), StatusValue::Float(end)) => {
                StatusValue::Float(start + (end - start) * progress)
            },
            (StatusValue::Integer(start), StatusValue::Integer(end)) => {
                StatusValue::Integer(((*start as f32) + ((*end - *start) as f32) * progress) as i32)
            },
            (StatusValue::Boolean(_), StatusValue::Boolean(end)) => {
                // Boolean values don't interpolate smoothly, just switch at 50%
                StatusValue::Boolean(if progress >= 0.5 { *end } else { *self == StatusValue::Boolean(true) })
            },
            _ => target.clone(), // Fallback for mismatched types
        }
    }
}

impl PartialEq for StatusValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StatusValue::Color(a), StatusValue::Color(b)) => a == b,
            (StatusValue::Position(ax, ay), StatusValue::Position(bx, by)) => ax == bx && ay == by,
            (StatusValue::Float(a), StatusValue::Float(b)) => (a - b).abs() < f32::EPSILON,
            (StatusValue::Integer(a), StatusValue::Integer(b)) => a == b,
            (StatusValue::Boolean(a), StatusValue::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

/// Status animation manager
#[derive(Debug)]
pub struct StatusAnimationManager {
    /// Active animations by field name
    animations: HashMap<String, StatusAnimation>,
    /// Current animated values
    current_values: HashMap<String, StatusValue>,
    /// Target values for smooth transitions
    target_values: HashMap<String, StatusValue>,
    /// Default animation duration
    default_duration: Duration,
}

impl Default for StatusAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusAnimationManager {
    /// Create a new status animation manager
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_values: HashMap::new(),
            target_values: HashMap::new(),
            default_duration: Duration::from_millis(200), // 200ms default
        }
    }

    /// Set a target value for animation
    pub fn set_target_value(&mut self, field: String, value: StatusValue) {
        // Check if value has changed
        if let Some(current) = self.target_values.get(&field) {
            if current == &value {
                return; // No change, no animation needed
            }
        }

        // Get current value (either animated or target)
        let start_value = self.get_current_value(&field).unwrap_or(value.clone());
        
        // Create animation
        let animation_type = match value {
            StatusValue::Color(_) => StatusAnimationType::ColorTransition,
            StatusValue::Position(_, _) => StatusAnimationType::PositionTransition,
            StatusValue::Float(_) => StatusAnimationType::ZoomTransition,
            StatusValue::Integer(_) => StatusAnimationType::ValueCounter,
            StatusValue::Boolean(_) => StatusAnimationType::StatusPulse,
        };

        let animation = StatusAnimation {
            animation_type,
            start_time: Instant::now(),
            duration: self.default_duration,
            easing: ease_out_cubic as EasingFunction,
            start_value,
            target_value: value.clone(),
        };

        self.animations.insert(field.clone(), animation);
        self.target_values.insert(field, value);
    }

    /// Get current animated value for a field
    pub fn get_current_value(&self, field: &str) -> Option<StatusValue> {
        if let Some(animation) = self.animations.get(field) {
            let progress = self.get_animation_progress(animation);
            Some(animation.start_value.interpolate(&animation.target_value, progress))
        } else {
            self.target_values.get(field).cloned()
        }
    }

    /// Update all animations and return whether any are still active
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let mut completed_animations = Vec::new();

        for (field, animation) in &self.animations {
            if now.duration_since(animation.start_time) >= animation.duration {
                completed_animations.push(field.clone());
            }
        }

        // Remove completed animations and update current values
        for field in completed_animations {
            if let Some(animation) = self.animations.remove(&field) {
                self.current_values.insert(field, animation.target_value);
            }
        }

        !self.animations.is_empty()
    }

    /// Check if any animations are running
    pub fn has_active_animations(&self) -> bool {
        !self.animations.is_empty()
    }

    /// Get animation progress (0.0 to 1.0)
    fn get_animation_progress(&self, animation: &StatusAnimation) -> f32 {
        let elapsed = Instant::now().duration_since(animation.start_time);
        let progress = elapsed.as_secs_f32() / animation.duration.as_secs_f32();
        let clamped_progress = progress.clamp(0.0, 1.0);
        (animation.easing)(clamped_progress)
    }

    /// Set animation duration for specific animation types
    pub fn set_animation_duration(&mut self, animation_type: StatusAnimationType, duration: Duration) {
        // Update default duration for new animations of this type
        match animation_type {
            StatusAnimationType::ColorTransition => self.default_duration = duration,
            StatusAnimationType::PositionTransition => self.default_duration = duration,
            StatusAnimationType::ZoomTransition => self.default_duration = duration,
            StatusAnimationType::StatusPulse => self.default_duration = duration,
            StatusAnimationType::ValueCounter => self.default_duration = duration,
        }
    }

    /// Create a pulse animation for status indicators
    pub fn create_pulse_animation(&mut self, field: String) {
        let animation = StatusAnimation {
            animation_type: StatusAnimationType::StatusPulse,
            start_time: Instant::now(),
            duration: Duration::from_millis(1000), // 1 second pulse
            easing: ease_in_out_cubic as EasingFunction,
            start_value: StatusValue::Float(1.0),
            target_value: StatusValue::Float(0.3),
        };

        self.animations.insert(field, animation);
    }

    /// Get pulse opacity for status indicators
    pub fn get_pulse_opacity(&self, field: &str) -> f32 {
        if let Some(animation) = self.animations.get(field) {
            if animation.animation_type == StatusAnimationType::StatusPulse {
                let progress = self.get_animation_progress(animation);
                // Create a sine wave pulse effect
                let pulse = (progress * std::f32::consts::PI * 2.0).sin().abs();
                0.3 + pulse * 0.7 // Pulse between 0.3 and 1.0
            } else {
                1.0
            }
        } else {
            1.0
        }
    }

    /// Clear all animations
    pub fn clear_animations(&mut self) {
        self.animations.clear();
    }

    /// Get all active animation field names
    pub fn get_active_fields(&self) -> Vec<String> {
        self.animations.keys().cloned().collect()
    }
}

/// Helper functions for common status value types
impl StatusAnimationManager {
    /// Set mouse position with animation
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.set_target_value("mouse_position".to_string(), StatusValue::Position(x, y));
    }

    /// Set pixel color with animation
    pub fn set_pixel_color(&mut self, color: RgbaPixel) {
        self.set_target_value("pixel_color".to_string(), StatusValue::Color(color));
    }

    /// Set zoom level with animation
    pub fn set_zoom_level(&mut self, zoom: f32) {
        self.set_target_value("zoom_level".to_string(), StatusValue::Float(zoom));
    }

    /// Get animated mouse position
    pub fn get_mouse_position(&self) -> Option<(f32, f32)> {
        if let Some(StatusValue::Position(x, y)) = self.get_current_value("mouse_position") {
            Some((x, y))
        } else {
            None
        }
    }

    /// Get animated pixel color
    pub fn get_pixel_color(&self) -> Option<RgbaPixel> {
        if let Some(StatusValue::Color(color)) = self.get_current_value("pixel_color") {
            Some(color)
        } else {
            None
        }
    }

    /// Get animated zoom level
    pub fn get_zoom_level(&self) -> Option<f32> {
        if let Some(StatusValue::Float(zoom)) = self.get_current_value("zoom_level") {
            Some(zoom)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_value_interpolation() {
        // Test color interpolation
        let start_color = StatusValue::Color(RgbaPixel::new(0, 0, 0, 255));
        let end_color = StatusValue::Color(RgbaPixel::new(255, 255, 255, 255));

        if let StatusValue::Color(interpolated) = start_color.interpolate(&end_color, 0.5) {
            assert_eq!(interpolated.r, 127);
            assert_eq!(interpolated.g, 127);
            assert_eq!(interpolated.b, 127);
            assert_eq!(interpolated.a, 255);
        } else {
            panic!("Expected color interpolation");
        }

        // Test position interpolation
        let start_pos = StatusValue::Position(0.0, 0.0);
        let end_pos = StatusValue::Position(100.0, 200.0);

        if let StatusValue::Position(x, y) = start_pos.interpolate(&end_pos, 0.5) {
            assert_eq!(x, 50.0);
            assert_eq!(y, 100.0);
        } else {
            panic!("Expected position interpolation");
        }

        // Test float interpolation
        let start_float = StatusValue::Float(0.0);
        let end_float = StatusValue::Float(1.0);

        if let StatusValue::Float(value) = start_float.interpolate(&end_float, 0.25) {
            assert_eq!(value, 0.25);
        } else {
            panic!("Expected float interpolation");
        }
    }

    #[test]
    fn test_status_animation_manager_creation() {
        let manager = StatusAnimationManager::new();
        assert!(!manager.has_active_animations());
        assert_eq!(manager.get_active_fields().len(), 0);
    }

    #[test]
    fn test_set_target_value() {
        let mut manager = StatusAnimationManager::new();

        // Set a target value
        let color = RgbaPixel::new(255, 0, 0, 255);
        manager.set_target_value("test_color".to_string(), StatusValue::Color(color));

        assert!(manager.has_active_animations());
        assert_eq!(manager.get_active_fields().len(), 1);

        // Setting the same value should not create a new animation
        manager.set_target_value("test_color".to_string(), StatusValue::Color(color));
        assert_eq!(manager.get_active_fields().len(), 1);
    }

    #[test]
    fn test_mouse_position_animation() {
        let mut manager = StatusAnimationManager::new();

        // Set mouse position
        manager.set_mouse_position(100.0, 200.0);

        assert!(manager.has_active_animations());

        // Get current position (should be animated)
        let pos = manager.get_mouse_position();
        assert!(pos.is_some());
    }

    #[test]
    fn test_pixel_color_animation() {
        let mut manager = StatusAnimationManager::new();

        // Set pixel color
        let color = RgbaPixel::new(128, 64, 192, 255);
        manager.set_pixel_color(color);

        assert!(manager.has_active_animations());

        // Get current color (should be animated)
        let animated_color = manager.get_pixel_color();
        assert!(animated_color.is_some());
    }

    #[test]
    fn test_zoom_level_animation() {
        let mut manager = StatusAnimationManager::new();

        // Set zoom level
        manager.set_zoom_level(2.5);

        assert!(manager.has_active_animations());

        // Get current zoom (should be animated)
        let zoom = manager.get_zoom_level();
        assert!(zoom.is_some());
    }

    #[test]
    fn test_pulse_animation() {
        let mut manager = StatusAnimationManager::new();

        // Create pulse animation
        manager.create_pulse_animation("status_indicator".to_string());

        assert!(manager.has_active_animations());

        // Get pulse opacity
        let opacity = manager.get_pulse_opacity("status_indicator");
        assert!(opacity >= 0.0 && opacity <= 1.0);
    }

    #[test]
    fn test_animation_update() {
        let mut manager = StatusAnimationManager::new();

        // Set a target value to start animation
        manager.set_mouse_position(50.0, 75.0);
        assert!(manager.has_active_animations());

        // Update animations
        let still_active = manager.update();
        // Should still be active for a short time
        assert!(still_active);
    }

    #[test]
    fn test_clear_animations() {
        let mut manager = StatusAnimationManager::new();

        // Set some animations
        manager.set_mouse_position(10.0, 20.0);
        manager.set_zoom_level(1.5);

        assert!(manager.has_active_animations());
        assert_eq!(manager.get_active_fields().len(), 2);

        // Clear all animations
        manager.clear_animations();

        assert!(!manager.has_active_animations());
        assert_eq!(manager.get_active_fields().len(), 0);
    }

    #[test]
    fn test_status_value_equality() {
        let color1 = StatusValue::Color(RgbaPixel::new(255, 0, 0, 255));
        let color2 = StatusValue::Color(RgbaPixel::new(255, 0, 0, 255));
        let color3 = StatusValue::Color(RgbaPixel::new(0, 255, 0, 255));

        assert_eq!(color1, color2);
        assert_ne!(color1, color3);

        let pos1 = StatusValue::Position(10.0, 20.0);
        let pos2 = StatusValue::Position(10.0, 20.0);
        let pos3 = StatusValue::Position(30.0, 40.0);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);

        let float1 = StatusValue::Float(1.5);
        let float2 = StatusValue::Float(1.5);
        let float3 = StatusValue::Float(2.5);

        assert_eq!(float1, float2);
        assert_ne!(float1, float3);
    }
}
