//! Easing functions for smooth animations
//! Provides various easing curves for natural motion

/// Easing function type
pub type EasingFunction = fn(f32) -> f32;

/// Linear interpolation (no easing)
pub fn linear(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Ease in cubic - slow start, accelerating
pub fn ease_in_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t
}

/// Ease out cubic - fast start, decelerating
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Ease in-out cubic - slow start and end, fast middle
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Ease out back - slight overshoot for bouncy effect
pub fn ease_out_back(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

/// Ease out elastic - elastic bounce effect
pub fn ease_out_elastic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        let c4 = (2.0 * std::f32::consts::PI) / 3.0;
        2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
    }
}

/// Ease out quart - smooth deceleration
pub fn ease_out_quart(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(4)
}

/// Ease in-out quart - smooth acceleration and deceleration
pub fn ease_in_out_quart(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

/// Apply easing function to a value between start and end
pub fn interpolate(start: f32, end: f32, t: f32, easing: EasingFunction) -> f32 {
    let eased_t = easing(t);
    start + (end - start) * eased_t
}

/// Apply easing to color interpolation
pub fn interpolate_color(
    start: iced::Color,
    end: iced::Color,
    t: f32,
    easing: EasingFunction,
) -> iced::Color {
    let eased_t = easing(t);
    iced::Color {
        r: start.r + (end.r - start.r) * eased_t,
        g: start.g + (end.g - start.g) * eased_t,
        b: start.b + (end.b - start.b) * eased_t,
        a: start.a + (end.a - start.a) * eased_t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_easing() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(0.5), 0.5);
        assert_eq!(linear(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_cubic() {
        assert_eq!(ease_in_cubic(0.0), 0.0);
        assert_eq!(ease_in_cubic(1.0), 1.0);
        assert!(ease_in_cubic(0.5) < 0.5); // Should be slower at start
    }

    #[test]
    fn test_ease_out_cubic() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
        assert_eq!(ease_out_cubic(1.0), 1.0);
        assert!(ease_out_cubic(0.5) > 0.5); // Should be faster at start
    }

    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(0.0, 10.0, 0.0, linear), 0.0);
        assert_eq!(interpolate(0.0, 10.0, 1.0, linear), 10.0);
        assert_eq!(interpolate(0.0, 10.0, 0.5, linear), 5.0);
    }
}
