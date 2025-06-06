//! Color management and color space conversions
//!
//! This module provides color space definitions, conversions, and color management functionality.
//! It supports RGB, HSL, HSV color spaces and provides utilities for color manipulation.

use crate::pixel::{Channel, RgbaPixel, CHANNEL_MAX};
use serde::{Deserialize, Serialize};

/// Color space enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSpace {
    /// sRGB color space
    Srgb,
    /// Adobe RGB color space
    AdobeRgb,
    /// ProPhoto RGB color space
    ProPhotoRgb,
    /// Linear RGB color space
    LinearRgb,
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::Srgb
    }
}

/// HSL (Hue, Saturation, Lightness) color representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HslColor {
    /// Hue in degrees (0-360)
    pub h: f32,
    /// Saturation as percentage (0.0-1.0)
    pub s: f32,
    /// Lightness as percentage (0.0-1.0)
    pub l: f32,
    /// Alpha channel (0.0-1.0)
    pub a: f32,
}

impl HslColor {
    /// Create a new HSL color
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        Self {
            h: h.rem_euclid(360.0),
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create HSL color without alpha (fully opaque)
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::new(h, s, l, 1.0)
    }

    /// Convert to RGBA pixel
    pub fn to_rgba(&self) -> RgbaPixel {
        let (r, g, b) = hsl_to_rgb(self.h, self.s, self.l);
        RgbaPixel::new(
            (r * CHANNEL_MAX as f32) as Channel,
            (g * CHANNEL_MAX as f32) as Channel,
            (b * CHANNEL_MAX as f32) as Channel,
            (self.a * CHANNEL_MAX as f32) as Channel,
        )
    }

    /// Create from RGBA pixel
    pub fn from_rgba(pixel: RgbaPixel) -> Self {
        let r = pixel.r as f32 / CHANNEL_MAX as f32;
        let g = pixel.g as f32 / CHANNEL_MAX as f32;
        let b = pixel.b as f32 / CHANNEL_MAX as f32;
        let a = pixel.a as f32 / CHANNEL_MAX as f32;

        let (h, s, l) = rgb_to_hsl(r, g, b);
        Self::new(h, s, l, a)
    }
}

/// HSV (Hue, Saturation, Value) color representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HsvColor {
    /// Hue in degrees (0-360)
    pub h: f32,
    /// Saturation as percentage (0.0-1.0)
    pub s: f32,
    /// Value as percentage (0.0-1.0)
    pub v: f32,
    /// Alpha channel (0.0-1.0)
    pub a: f32,
}

impl HsvColor {
    /// Create a new HSV color
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self {
            h: h.rem_euclid(360.0),
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        }
    }

    /// Create HSV color without alpha (fully opaque)
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::new(h, s, v, 1.0)
    }

    /// Convert to RGBA pixel
    pub fn to_rgba(&self) -> RgbaPixel {
        let (r, g, b) = hsv_to_rgb(self.h, self.s, self.v);
        RgbaPixel::new(
            (r * CHANNEL_MAX as f32) as Channel,
            (g * CHANNEL_MAX as f32) as Channel,
            (b * CHANNEL_MAX as f32) as Channel,
            (self.a * CHANNEL_MAX as f32) as Channel,
        )
    }

    /// Create from RGBA pixel
    pub fn from_rgba(pixel: RgbaPixel) -> Self {
        let r = pixel.r as f32 / CHANNEL_MAX as f32;
        let g = pixel.g as f32 / CHANNEL_MAX as f32;
        let b = pixel.b as f32 / CHANNEL_MAX as f32;
        let a = pixel.a as f32 / CHANNEL_MAX as f32;

        let (h, s, v) = rgb_to_hsv(r, g, b);
        Self::new(h, s, v, a)
    }
}

/// Color adjustment parameters
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorAdjustment {
    /// Brightness adjustment (-1.0 to 1.0)
    pub brightness: f32,
    /// Contrast adjustment (-1.0 to 1.0)
    pub contrast: f32,
    /// Saturation adjustment (-1.0 to 1.0)
    pub saturation: f32,
    /// Hue shift in degrees (-180.0 to 180.0)
    pub hue_shift: f32,
}

impl Default for ColorAdjustment {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 0.0,
            hue_shift: 0.0,
        }
    }
}

impl ColorAdjustment {
    /// Create a new color adjustment
    pub fn new(brightness: f32, contrast: f32, saturation: f32, hue_shift: f32) -> Self {
        Self {
            brightness: brightness.clamp(-1.0, 1.0),
            contrast: contrast.clamp(-1.0, 1.0),
            saturation: saturation.clamp(-1.0, 1.0),
            hue_shift: hue_shift.clamp(-180.0, 180.0),
        }
    }

    /// Apply adjustment to a pixel
    pub fn apply_to_pixel(&self, pixel: RgbaPixel) -> RgbaPixel {
        if self.is_identity() {
            return pixel;
        }

        let mut hsl = HslColor::from_rgba(pixel);

        // Apply hue shift
        if self.hue_shift != 0.0 {
            hsl.h = (hsl.h + self.hue_shift).rem_euclid(360.0);
        }

        // Apply saturation
        if self.saturation != 0.0 {
            hsl.s = (hsl.s * (1.0 + self.saturation)).clamp(0.0, 1.0);
        }

        let mut result = hsl.to_rgba();

        // Apply brightness
        if self.brightness != 0.0 {
            let brightness_factor = 1.0 + self.brightness;
            result.r =
                ((result.r as f32 * brightness_factor).clamp(0.0, CHANNEL_MAX as f32)) as Channel;
            result.g =
                ((result.g as f32 * brightness_factor).clamp(0.0, CHANNEL_MAX as f32)) as Channel;
            result.b =
                ((result.b as f32 * brightness_factor).clamp(0.0, CHANNEL_MAX as f32)) as Channel;
        }

        // Apply contrast
        if self.contrast != 0.0 {
            let contrast_factor = 1.0 + self.contrast;
            let apply_contrast = |channel: Channel| -> Channel {
                let normalized = channel as f32 / CHANNEL_MAX as f32;
                let adjusted = ((normalized - 0.5) * contrast_factor + 0.5).clamp(0.0, 1.0);
                (adjusted * CHANNEL_MAX as f32) as Channel
            };

            result.r = apply_contrast(result.r);
            result.g = apply_contrast(result.g);
            result.b = apply_contrast(result.b);
        }

        result
    }

    /// Check if this adjustment is identity (no change)
    pub fn is_identity(&self) -> bool {
        self.brightness == 0.0
            && self.contrast == 0.0
            && self.saturation == 0.0
            && self.hue_shift == 0.0
    }
}

/// Convert RGB to HSL
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    // Lightness
    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l); // Achromatic
    }

    // Saturation
    let s = if l > 0.5 {
        delta / (2.0 - max - min)
    } else {
        delta / (max + min)
    };

    // Hue
    let h = if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };

    (h, s, l)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l); // Achromatic
    }

    let hue_to_rgb = |p: f32, q: f32, mut t: f32| -> f32 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    };

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h_norm = h / 360.0;

    let r = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_norm);
    let b = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);

    (r, g, b)
}

/// Convert RGB to HSV
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    // Value
    let v = max;

    if max == 0.0 {
        return (0.0, 0.0, v);
    }

    // Saturation
    let s = delta / max;

    if delta == 0.0 {
        return (0.0, s, v); // Achromatic
    }

    // Hue
    let h = if max == r {
        ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
    } else if max == g {
        ((b - r) / delta + 2.0) * 60.0
    } else {
        ((r - g) / delta + 4.0) * 60.0
    };

    (h, s, v)
}

/// Convert HSV to RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (v, v, v); // Achromatic
    }

    let h_sector = (h / 60.0).floor();
    let h_frac = h / 60.0 - h_sector;

    let p = v * (1.0 - s);
    let q = v * (1.0 - s * h_frac);
    let t = v * (1.0 - s * (1.0 - h_frac));

    match h_sector as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}
