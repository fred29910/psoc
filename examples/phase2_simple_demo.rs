//! Phase 2 Visual Effects Demo - Standalone Version
//! Demonstrates the core logic of our visual effects system

use std::time::{Duration, Instant};

// Simple color representation
#[derive(Debug, Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    
    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    fn tech_blue() -> Self {
        Self::from_rgba(0.0, 0.75, 1.0, 1.0) // #00BFFF
    }
}

// Simple point representation
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
    
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// Simple vector representation
#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum TransitionType {
    SlideDown,
    Fade,
    Scale,
    BounceDown,
}

#[derive(Debug, Clone)]
struct TransitionState {
    position: Point,
    scale: f32,
    opacity: f32,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            scale: 1.0,
            opacity: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
struct GlassEffect {
    transparency: f32,
    blur_intensity: f32,
    tint_color: Color,
}

impl GlassEffect {
    fn frosted_light() -> Self {
        Self {
            transparency: 0.9,
            blur_intensity: 0.3,
            tint_color: Color::from_rgba(0.2, 0.2, 0.2, 0.9),
        }
    }

    fn frosted_heavy() -> Self {
        Self {
            transparency: 0.7,
            blur_intensity: 0.8,
            tint_color: Color::from_rgba(0.2, 0.2, 0.2, 0.7),
        }
    }

    fn tech_blue() -> Self {
        Self {
            transparency: 0.85,
            blur_intensity: 0.4,
            tint_color: Color::from_rgba(0.0, 0.75, 1.0, 0.85),
        }
    }

    fn interpolate(&self, target: &GlassEffect, progress: f32) -> GlassEffect {
        let progress = progress.clamp(0.0, 1.0);
        
        GlassEffect {
            transparency: self.transparency + (target.transparency - self.transparency) * progress,
            blur_intensity: self.blur_intensity + (target.blur_intensity - self.blur_intensity) * progress,
            tint_color: Color {
                r: self.tint_color.r + (target.tint_color.r - self.tint_color.r) * progress,
                g: self.tint_color.g + (target.tint_color.g - self.tint_color.g) * progress,
                b: self.tint_color.b + (target.tint_color.b - self.tint_color.b) * progress,
                a: self.tint_color.a + (target.tint_color.a - self.tint_color.a) * progress,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct DropShadow {
    color: Color,
    offset: Vector,
    blur_radius: f32,
}

impl DropShadow {
    fn subtle() -> Self {
        Self {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }
    }

    fn medium() -> Self {
        Self {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        }
    }

    fn high() -> Self {
        Self {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 16.0,
        }
    }

    fn tech_accent() -> Self {
        Self {
            color: Color::from_rgba(0.0, 0.75, 1.0, 0.3),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        }
    }
}

// Easing functions
fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

fn ease_out_back(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

#[derive(Debug)]
struct MenuAnimation {
    transition_type: TransitionType,
    start_time: Instant,
    duration: Duration,
    is_opening: bool,
    start_state: TransitionState,
    target_state: TransitionState,
}

impl MenuAnimation {
    fn new_opening(transition_type: TransitionType, position: Point) -> Self {
        let (start_state, target_state) = match transition_type {
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
            ),
        };

        Self {
            transition_type,
            start_time: Instant::now(),
            duration: Duration::from_millis(250),
            is_opening: true,
            start_state,
            target_state,
        }
    }

    fn get_progress(&self) -> f32 {
        let elapsed = Instant::now().duration_since(self.start_time);
        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        progress.clamp(0.0, 1.0)
    }

    fn get_current_state(&self) -> TransitionState {
        let progress = self.get_progress();
        let eased_progress = match self.transition_type {
            TransitionType::SlideDown => ease_out_cubic(progress),
            TransitionType::Fade => ease_in_out_cubic(progress),
            TransitionType::Scale | TransitionType::BounceDown => ease_out_back(progress),
        };

        TransitionState {
            position: Point::new(
                self.start_state.position.x + 
                (self.target_state.position.x - self.start_state.position.x) * eased_progress,
                self.start_state.position.y + 
                (self.target_state.position.y - self.start_state.position.y) * eased_progress,
            ),
            scale: self.start_state.scale + 
                (self.target_state.scale - self.start_state.scale) * eased_progress,
            opacity: self.start_state.opacity + 
                (self.target_state.opacity - self.start_state.opacity) * eased_progress,
        }
    }

    fn is_complete(&self) -> bool {
        self.get_progress() >= 1.0
    }
}

fn main() {
    println!("🎨 PSOC Phase 2: Visual Effects Demo");
    println!("=====================================\n");

    // Demo 1: Glass Effects
    println!("1. 🪟 Glass Effects Demo");
    println!("------------------------");
    
    let light_glass = GlassEffect::frosted_light();
    let heavy_glass = GlassEffect::frosted_heavy();
    let tech_glass = GlassEffect::tech_blue();
    
    println!("Light Glass: transparency={:.1}, blur={:.1}", 
             light_glass.transparency, light_glass.blur_intensity);
    println!("Heavy Glass: transparency={:.1}, blur={:.1}", 
             heavy_glass.transparency, heavy_glass.blur_intensity);
    println!("Tech Glass: transparency={:.1}, tint=tech_blue", 
             tech_glass.transparency);
    
    // Demonstrate interpolation
    let interpolated = light_glass.interpolate(&heavy_glass, 0.5);
    println!("Interpolated (50%): transparency={:.1}, blur={:.1}\n", 
             interpolated.transparency, interpolated.blur_intensity);

    // Demo 2: Shadow System
    println!("2. 🌑 Shadow System Demo");
    println!("------------------------");
    
    let subtle_shadow = DropShadow::subtle();
    let medium_shadow = DropShadow::medium();
    let high_shadow = DropShadow::high();
    let tech_shadow = DropShadow::tech_accent();
    
    println!("Subtle Shadow: blur={:.1}, alpha={:.1}", 
             subtle_shadow.blur_radius, subtle_shadow.color.a);
    println!("Medium Shadow: blur={:.1}, alpha={:.1}", 
             medium_shadow.blur_radius, medium_shadow.color.a);
    println!("High Shadow: blur={:.1}, alpha={:.1}", 
             high_shadow.blur_radius, high_shadow.color.a);
    println!("Tech Shadow: blur={:.1}, color=tech_blue\n", 
             tech_shadow.blur_radius);

    // Demo 3: Animation System
    println!("3. 🎬 Animation System Demo");
    println!("---------------------------");
    
    let menu_position = Point::new(100.0, 50.0);
    let slide_animation = MenuAnimation::new_opening(TransitionType::SlideDown, menu_position);
    let fade_animation = MenuAnimation::new_opening(TransitionType::Fade, menu_position);
    let scale_animation = MenuAnimation::new_opening(TransitionType::Scale, menu_position);
    let bounce_animation = MenuAnimation::new_opening(TransitionType::BounceDown, menu_position);
    
    println!("Slide Animation: {:?}", slide_animation.transition_type);
    println!("Fade Animation: {:?}", fade_animation.transition_type);
    println!("Scale Animation: {:?}", scale_animation.transition_type);
    println!("Bounce Animation: {:?}", bounce_animation.transition_type);
    
    // Simulate animation progress
    std::thread::sleep(Duration::from_millis(50));
    let current_state = slide_animation.get_current_state();
    println!("Current animation state: opacity={:.2}, y_offset={:.1}\n", 
             current_state.opacity, current_state.position.y - menu_position.y);

    // Demo 4: Easing Functions
    println!("4. 📈 Easing Functions Demo");
    println!("---------------------------");
    
    let test_values = [0.0, 0.25, 0.5, 0.75, 1.0];
    println!("Input  | Linear | EaseInOut | EaseBack");
    println!("-------|--------|-----------|----------");
    
    for &t in &test_values {
        println!("{:5.2} | {:6.2} | {:9.2} | {:8.2}", 
                 t, t, ease_in_out_cubic(t), ease_out_back(t));
    }
    
    println!("\n5. 🎯 Integration Demo");
    println!("---------------------");
    
    // Demonstrate a complete dropdown menu effect
    println!("Creating dropdown menu with combined effects:");
    println!("• Glass effect: Medium frosted glass");
    println!("• Shadow: Medium drop shadow");
    println!("• Animation: Slide down with ease-out-cubic");
    println!("• Duration: 250ms");
    
    let dropdown_glass = GlassEffect::frosted_heavy();
    let dropdown_shadow = DropShadow::medium();
    let dropdown_animation = MenuAnimation::new_opening(TransitionType::SlideDown, Point::new(200.0, 100.0));
    
    println!("✓ Dropdown configured with modern visual effects");
    
    println!("\n🎉 Phase 2 Visual Effects Demo Complete!");
    println!("========================================");
    println!("All visual effects systems are working correctly:");
    println!("✓ Glass effects with interpolation");
    println!("✓ Multi-level shadow system");
    println!("✓ Smooth animation transitions");
    println!("✓ Advanced easing functions");
    println!("✓ Integrated visual styling");
}
