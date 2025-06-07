//! Layer data structures and operations
//!
//! This module defines the layer system for the PSOC image editor, including
//! layer types, blend modes, and layer operations.

use crate::geometry::{Point, Rect, Transform};
use crate::pixel::{PixelData, RgbaPixel};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Layer blend modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    /// Normal blending (default)
    Normal,
    /// Multiply blending
    Multiply,
    /// Screen blending
    Screen,
    /// Overlay blending
    Overlay,
    /// Soft light blending
    SoftLight,
    /// Hard light blending
    HardLight,
    /// Color dodge blending
    ColorDodge,
    /// Color burn blending
    ColorBurn,
    /// Darken blending
    Darken,
    /// Lighten blending
    Lighten,
    /// Difference blending
    Difference,
    /// Exclusion blending
    Exclusion,
    /// Hue blending
    Hue,
    /// Saturation blending
    Saturation,
    /// Color blending
    Color,
    /// Luminosity blending
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl BlendMode {
    /// Get all available blend modes
    pub fn all() -> Vec<BlendMode> {
        vec![
            BlendMode::Normal,
            BlendMode::Multiply,
            BlendMode::Screen,
            BlendMode::Overlay,
            BlendMode::SoftLight,
            BlendMode::HardLight,
            BlendMode::ColorDodge,
            BlendMode::ColorBurn,
            BlendMode::Darken,
            BlendMode::Lighten,
            BlendMode::Difference,
            BlendMode::Exclusion,
            BlendMode::Hue,
            BlendMode::Saturation,
            BlendMode::Color,
            BlendMode::Luminosity,
        ]
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::HardLight => "Hard Light",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }

    /// Apply blend mode to two pixels
    pub fn blend(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        if opacity <= 0.0 {
            return base;
        }

        // Only skip blending for Normal mode with full opacity and alpha
        if *self == BlendMode::Normal && opacity >= 1.0 && overlay.a == 255 {
            return overlay;
        }

        // Apply the specific blend mode
        match self {
            BlendMode::Normal => self.blend_normal(base, overlay, opacity),
            BlendMode::Multiply => self.blend_multiply(base, overlay, opacity),
            BlendMode::Screen => self.blend_screen(base, overlay, opacity),
            BlendMode::Overlay => self.blend_overlay(base, overlay, opacity),
            BlendMode::SoftLight => self.blend_soft_light(base, overlay, opacity),
            BlendMode::HardLight => self.blend_hard_light(base, overlay, opacity),
            BlendMode::ColorDodge => self.blend_color_dodge(base, overlay, opacity),
            BlendMode::ColorBurn => self.blend_color_burn(base, overlay, opacity),
            BlendMode::Darken => self.blend_darken(base, overlay, opacity),
            BlendMode::Lighten => self.blend_lighten(base, overlay, opacity),
            BlendMode::Difference => self.blend_difference(base, overlay, opacity),
            BlendMode::Exclusion => self.blend_exclusion(base, overlay, opacity),
            BlendMode::Hue => self.blend_hue(base, overlay, opacity),
            BlendMode::Saturation => self.blend_saturation(base, overlay, opacity),
            BlendMode::Color => self.blend_color(base, overlay, opacity),
            BlendMode::Luminosity => self.blend_luminosity(base, overlay, opacity),
        }
    }

    /// Normal blending implementation
    fn blend_normal(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let overlay_alpha = (overlay.a as f32 / 255.0) * opacity;
        let base_alpha = base.a as f32 / 255.0;

        if overlay_alpha <= 0.0 {
            return base;
        }

        let result_alpha = overlay_alpha + base_alpha * (1.0 - overlay_alpha);

        if result_alpha <= 0.0 {
            return RgbaPixel::transparent();
        }

        let blend_factor = overlay_alpha / result_alpha;

        let r = (overlay.r as f32 * blend_factor + base.r as f32 * (1.0 - blend_factor)) as u8;
        let g = (overlay.g as f32 * blend_factor + base.g as f32 * (1.0 - blend_factor)) as u8;
        let b = (overlay.b as f32 * blend_factor + base.b as f32 * (1.0 - blend_factor)) as u8;
        let a = (result_alpha * 255.0) as u8;

        RgbaPixel::new(r, g, b, a)
    }

    /// Multiply blending implementation
    fn blend_multiply(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blended_r = (base.r as f32 * overlay.r as f32 / 255.0) as u8;
        let blended_g = (base.g as f32 * overlay.g as f32 / 255.0) as u8;
        let blended_b = (base.b as f32 * overlay.b as f32 / 255.0) as u8;
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, base.a);

        // Apply opacity blending if needed
        if opacity >= 1.0 {
            blended
        } else {
            BlendMode::Normal.blend_normal(base, blended, opacity)
        }
    }

    /// Screen blending implementation
    fn blend_screen(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blended_r =
            (255.0 - (255.0 - base.r as f32) * (255.0 - overlay.r as f32) / 255.0) as u8;
        let blended_g =
            (255.0 - (255.0 - base.g as f32) * (255.0 - overlay.g as f32) / 255.0) as u8;
        let blended_b =
            (255.0 - (255.0 - base.b as f32) * (255.0 - overlay.b as f32) / 255.0) as u8;
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Overlay blending implementation
    fn blend_overlay(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blend_channel = |base: u8, overlay: u8| -> u8 {
            let base_f = base as f32 / 255.0;
            let overlay_f = overlay as f32 / 255.0;

            let result = if base_f < 0.5 {
                2.0 * base_f * overlay_f
            } else {
                1.0 - 2.0 * (1.0 - base_f) * (1.0 - overlay_f)
            };

            (result * 255.0).clamp(0.0, 255.0) as u8
        };

        let blended_r = blend_channel(base.r, overlay.r);
        let blended_g = blend_channel(base.g, overlay.g);
        let blended_b = blend_channel(base.b, overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        self.blend_normal(base, blended, opacity)
    }

    /// Soft light blending implementation
    fn blend_soft_light(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blend_channel = |base: u8, overlay: u8| -> u8 {
            let base_f = base as f32 / 255.0;
            let overlay_f = overlay as f32 / 255.0;

            let result = if overlay_f < 0.5 {
                2.0 * base_f * overlay_f + base_f * base_f * (1.0 - 2.0 * overlay_f)
            } else {
                2.0 * base_f * (1.0 - overlay_f) + base_f.sqrt() * (2.0 * overlay_f - 1.0)
            };

            (result * 255.0).clamp(0.0, 255.0) as u8
        };

        let blended_r = blend_channel(base.r, overlay.r);
        let blended_g = blend_channel(base.g, overlay.g);
        let blended_b = blend_channel(base.b, overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        self.blend_normal(base, blended, opacity)
    }

    /// Hard light blending implementation
    fn blend_hard_light(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blend_channel = |base: u8, overlay: u8| -> u8 {
            let base_f = base as f32 / 255.0;
            let overlay_f = overlay as f32 / 255.0;

            let result = if overlay_f < 0.5 {
                2.0 * base_f * overlay_f
            } else {
                1.0 - 2.0 * (1.0 - base_f) * (1.0 - overlay_f)
            };

            (result * 255.0).clamp(0.0, 255.0) as u8
        };

        let blended_r = blend_channel(base.r, overlay.r);
        let blended_g = blend_channel(base.g, overlay.g);
        let blended_b = blend_channel(base.b, overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        self.blend_normal(base, blended, opacity)
    }

    /// Color dodge blending implementation
    fn blend_color_dodge(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blend_channel = |base: u8, overlay: u8| -> u8 {
            let base_f = base as f32 / 255.0;
            let overlay_f = overlay as f32 / 255.0;

            let result = if overlay_f >= 1.0 {
                1.0
            } else {
                (base_f / (1.0 - overlay_f)).min(1.0)
            };

            (result * 255.0) as u8
        };

        let blended_r = blend_channel(base.r, overlay.r);
        let blended_g = blend_channel(base.g, overlay.g);
        let blended_b = blend_channel(base.b, overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        self.blend_normal(base, blended, opacity)
    }

    /// Color burn blending implementation
    fn blend_color_burn(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blend_channel = |base: u8, overlay: u8| -> u8 {
            let base_f = base as f32 / 255.0;
            let overlay_f = overlay as f32 / 255.0;

            let result = if overlay_f <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - base_f) / overlay_f).min(1.0)
            };

            (result * 255.0) as u8
        };

        let blended_r = blend_channel(base.r, overlay.r);
        let blended_g = blend_channel(base.g, overlay.g);
        let blended_b = blend_channel(base.b, overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        self.blend_normal(base, blended, opacity)
    }

    /// Darken blending implementation
    fn blend_darken(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blended_r = base.r.min(overlay.r);
        let blended_g = base.g.min(overlay.g);
        let blended_b = base.b.min(overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        // For darken/lighten modes, we need to apply normal blending with the result
        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Lighten blending implementation
    fn blend_lighten(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blended_r = base.r.max(overlay.r);
        let blended_g = base.g.max(overlay.g);
        let blended_b = base.b.max(overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        // For darken/lighten modes, we need to apply normal blending with the result
        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Difference blending implementation
    fn blend_difference(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blended_r = (base.r as i16 - overlay.r as i16).unsigned_abs() as u8;
        let blended_g = (base.g as i16 - overlay.g as i16).unsigned_abs() as u8;
        let blended_b = (base.b as i16 - overlay.b as i16).unsigned_abs() as u8;
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Exclusion blending implementation
    fn blend_exclusion(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        let blend_channel = |base: u8, overlay: u8| -> u8 {
            let base_f = base as f32 / 255.0;
            let overlay_f = overlay as f32 / 255.0;
            let result = base_f + overlay_f - 2.0 * base_f * overlay_f;
            (result * 255.0).clamp(0.0, 255.0) as u8
        };

        let blended_r = blend_channel(base.r, overlay.r);
        let blended_g = blend_channel(base.g, overlay.g);
        let blended_b = blend_channel(base.b, overlay.b);
        let blended = RgbaPixel::new(blended_r, blended_g, blended_b, overlay.a);

        self.blend_normal(base, blended, opacity)
    }

    /// Hue blending implementation
    fn blend_hue(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        use crate::color::HslColor;

        // If base and overlay are the same, return base to avoid conversion artifacts
        if base == overlay {
            return BlendMode::Normal.blend_normal(base, overlay, opacity);
        }

        let base_hsl = HslColor::from_rgba(base);
        let overlay_hsl = HslColor::from_rgba(overlay);

        let blended_hsl = HslColor::new(overlay_hsl.h, base_hsl.s, base_hsl.l, overlay_hsl.a);

        let blended = blended_hsl.to_rgba();
        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Saturation blending implementation
    fn blend_saturation(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        use crate::color::HslColor;

        // If base and overlay are the same, return base to avoid conversion artifacts
        if base == overlay {
            return BlendMode::Normal.blend_normal(base, overlay, opacity);
        }

        let base_hsl = HslColor::from_rgba(base);
        let overlay_hsl = HslColor::from_rgba(overlay);

        let blended_hsl = HslColor::new(base_hsl.h, overlay_hsl.s, base_hsl.l, overlay_hsl.a);

        let blended = blended_hsl.to_rgba();
        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Color blending implementation
    fn blend_color(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        use crate::color::HslColor;

        // If base and overlay are the same, return base to avoid conversion artifacts
        if base == overlay {
            return BlendMode::Normal.blend_normal(base, overlay, opacity);
        }

        let base_hsl = HslColor::from_rgba(base);
        let overlay_hsl = HslColor::from_rgba(overlay);

        let blended_hsl = HslColor::new(overlay_hsl.h, overlay_hsl.s, base_hsl.l, overlay_hsl.a);

        let blended = blended_hsl.to_rgba();
        BlendMode::Normal.blend_normal(base, blended, opacity)
    }

    /// Luminosity blending implementation
    fn blend_luminosity(&self, base: RgbaPixel, overlay: RgbaPixel, opacity: f32) -> RgbaPixel {
        use crate::color::HslColor;

        // If base and overlay are the same, return base to avoid conversion artifacts
        if base == overlay {
            return BlendMode::Normal.blend_normal(base, overlay, opacity);
        }

        let base_hsl = HslColor::from_rgba(base);
        let overlay_hsl = HslColor::from_rgba(overlay);

        let blended_hsl = HslColor::new(base_hsl.h, base_hsl.s, overlay_hsl.l, overlay_hsl.a);

        let blended = blended_hsl.to_rgba();
        BlendMode::Normal.blend_normal(base, blended, opacity)
    }
}

/// Layer type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayerType {
    /// Regular pixel layer
    Pixel,
    /// Text layer
    Text {
        content: String,
        font_family: String,
        font_size: f32,
        color: RgbaPixel,
    },
    /// Shape layer
    Shape {
        shape_type: String,
        fill_color: Option<RgbaPixel>,
        stroke_color: Option<RgbaPixel>,
        stroke_width: f32,
    },
    /// Adjustment layer
    Adjustment {
        adjustment_type: String,
        parameters: std::collections::HashMap<String, f32>,
    },
}

impl Default for LayerType {
    fn default() -> Self {
        Self::Pixel
    }
}

/// Layer data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Unique identifier
    pub id: Uuid,
    /// Layer name
    pub name: String,
    /// Layer type
    pub layer_type: LayerType,
    /// Pixel data (for pixel layers)
    pub pixel_data: Option<PixelData>,
    /// Layer visibility
    pub visible: bool,
    /// Layer opacity (0.0 to 1.0)
    pub opacity: f32,
    /// Blend mode
    pub blend_mode: BlendMode,
    /// Layer position offset
    pub offset: Point,
    /// Layer transformation
    pub transform: Transform,
    /// Layer bounds
    pub bounds: Rect,
    /// Whether layer is locked
    pub locked: bool,
    /// Layer mask (optional)
    pub mask: Option<PixelData>,
}

impl Layer {
    /// Create a new empty pixel layer
    pub fn new_pixel(name: String, width: u32, height: u32) -> Self {
        let id = Uuid::new_v4();
        let pixel_data = PixelData::new_rgba(width, height);
        let bounds = Rect::new(0.0, 0.0, width as f32, height as f32);

        Self {
            id,
            name,
            layer_type: LayerType::Pixel,
            pixel_data: Some(pixel_data),
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            offset: Point::origin(),
            transform: Transform::identity(),
            bounds,
            locked: false,
            mask: None,
        }
    }

    /// Create a new text layer
    pub fn new_text(
        name: String,
        content: String,
        font_family: String,
        font_size: f32,
        color: RgbaPixel,
        position: Point,
    ) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            name,
            layer_type: LayerType::Text {
                content,
                font_family,
                font_size,
                color,
            },
            pixel_data: None,
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            offset: position,
            transform: Transform::identity(),
            bounds: Rect::new(position.x, position.y, 100.0, font_size), // Placeholder bounds
            locked: false,
            mask: None,
        }
    }

    /// Create a new adjustment layer
    pub fn new_adjustment(
        name: String,
        adjustment_type: String,
        parameters: std::collections::HashMap<String, f32>,
    ) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            name,
            layer_type: LayerType::Adjustment {
                adjustment_type,
                parameters,
            },
            pixel_data: None,
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            offset: Point::origin(),
            transform: Transform::identity(),
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0), // Adjustment layers have no bounds
            locked: false,
            mask: None,
        }
    }

    /// Get layer dimensions
    pub fn dimensions(&self) -> Option<(u32, u32)> {
        self.pixel_data.as_ref().map(|data| data.dimensions())
    }

    /// Check if layer has pixel data
    pub fn has_pixel_data(&self) -> bool {
        self.pixel_data.is_some()
    }

    /// Get pixel at coordinates (relative to layer)
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        self.pixel_data.as_ref()?.get_pixel(x, y)
    }

    /// Set pixel at coordinates (relative to layer)
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: RgbaPixel) -> Result<()> {
        if let Some(ref mut pixel_data) = self.pixel_data {
            pixel_data.set_pixel(x, y, pixel)
        } else {
            Err(anyhow::anyhow!("Layer has no pixel data"))
        }
    }

    /// Fill layer with color
    pub fn fill(&mut self, color: RgbaPixel) {
        if let Some(ref mut pixel_data) = self.pixel_data {
            pixel_data.fill(color);
        }
    }

    /// Clear layer (fill with transparent)
    pub fn clear(&mut self) {
        self.fill(RgbaPixel::transparent());
    }

    /// Duplicate layer
    pub fn duplicate(&self) -> Self {
        let mut duplicate = self.clone();
        duplicate.id = Uuid::new_v4();
        duplicate.name = format!("{} copy", self.name);
        duplicate
    }

    /// Move layer by offset
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.offset = self.offset.translate(dx, dy);
        self.bounds = self.bounds.translate(dx, dy);
    }

    /// Set layer position
    pub fn set_position(&mut self, position: Point) {
        let dx = position.x - self.offset.x;
        let dy = position.y - self.offset.y;
        self.move_by(dx, dy);
    }

    /// Get current transformation
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Set transformation
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    /// Apply transformation to layer
    pub fn apply_transform(&mut self, transform: Transform) {
        self.transform = self.transform.then(&transform);
        self.bounds = transform.transform_rect(self.bounds);
    }

    /// Reset transformation
    pub fn reset_transform(&mut self) {
        self.transform = Transform::identity();
    }

    /// Get effective opacity (considering parent groups, etc.)
    pub fn effective_opacity(&self) -> f32 {
        // For now, just return the layer's opacity
        // TODO: Consider parent group opacity when layer groups are implemented
        self.opacity
    }

    /// Check if layer is effectively visible
    pub fn is_effectively_visible(&self) -> bool {
        self.visible && self.effective_opacity() > 0.0
    }

    /// Check if layer has a mask
    pub fn has_mask(&self) -> bool {
        self.mask.is_some()
    }

    /// Get mask dimensions
    pub fn mask_dimensions(&self) -> Option<(u32, u32)> {
        self.mask.as_ref().map(|mask| mask.dimensions())
    }

    /// Create a new mask for this layer
    pub fn create_mask(&mut self, width: u32, height: u32) -> Result<()> {
        let mut mask = PixelData::new_grayscale(width, height);
        // Initialize mask to fully opaque (white)
        mask.fill(RgbaPixel::white());
        self.mask = Some(mask);
        Ok(())
    }

    /// Remove the mask from this layer
    pub fn remove_mask(&mut self) {
        self.mask = None;
    }

    /// Get mask pixel at coordinates
    pub fn get_mask_pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        self.mask.as_ref()?.get_pixel(x, y)
    }

    /// Set mask pixel at coordinates
    pub fn set_mask_pixel(&mut self, x: u32, y: u32, pixel: RgbaPixel) -> Result<()> {
        if let Some(ref mut mask) = self.mask {
            mask.set_pixel(x, y, pixel)
        } else {
            Err(anyhow::anyhow!("Layer has no mask"))
        }
    }

    /// Fill mask with color
    pub fn fill_mask(&mut self, color: RgbaPixel) -> Result<()> {
        if let Some(ref mut mask) = self.mask {
            mask.fill(color);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Layer has no mask"))
        }
    }

    /// Clear mask (fill with black - fully transparent)
    pub fn clear_mask(&mut self) -> Result<()> {
        self.fill_mask(RgbaPixel::black())
    }

    /// Invert mask
    pub fn invert_mask(&mut self) -> Result<()> {
        if let Some(ref mut mask) = self.mask {
            let (width, height) = mask.dimensions();
            for y in 0..height {
                for x in 0..width {
                    if let Some(pixel) = mask.get_pixel(x, y) {
                        let inverted =
                            RgbaPixel::new(255 - pixel.r, 255 - pixel.g, 255 - pixel.b, pixel.a);
                        mask.set_pixel(x, y, inverted)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Layer has no mask"))
        }
    }

    /// Apply mask to get effective pixel opacity
    pub fn get_masked_pixel(&self, x: u32, y: u32) -> Option<RgbaPixel> {
        let mut pixel = self.get_pixel(x, y)?;

        if let Some(mask_pixel) = self.get_mask_pixel(x, y) {
            // Use the red channel of the mask as the mask value (grayscale)
            let mask_value = mask_pixel.r as f32 / 255.0;
            let new_alpha = (pixel.a as f32 * mask_value) as u8;
            pixel.a = new_alpha;
        }

        Some(pixel)
    }

    /// Get layer bounds in document coordinates
    pub fn document_bounds(&self) -> Rect {
        self.transform
            .transform_rect(self.bounds.translate(self.offset.x, self.offset.y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_creation() {
        let layer = Layer::new_pixel("Test Layer".to_string(), 100, 50);

        assert_eq!(layer.name, "Test Layer");
        assert!(layer.visible);
        assert_eq!(layer.opacity, 1.0);
        assert_eq!(layer.blend_mode, BlendMode::Normal);
        assert!(layer.has_pixel_data());

        let (width, height) = layer.dimensions().unwrap();
        assert_eq!(width, 100);
        assert_eq!(height, 50);
    }

    #[test]
    fn test_text_layer_creation() {
        let layer = Layer::new_text(
            "Text Layer".to_string(),
            "Hello World".to_string(),
            "Arial".to_string(),
            24.0,
            RgbaPixel::black(),
            Point::new(10.0, 20.0),
        );

        assert_eq!(layer.name, "Text Layer");
        assert!(matches!(layer.layer_type, LayerType::Text { .. }));
        assert!(!layer.has_pixel_data());
    }

    #[test]
    fn test_layer_pixel_operations() {
        let mut layer = Layer::new_pixel("Test".to_string(), 10, 10);
        let test_color = RgbaPixel::new(255, 128, 64, 200);

        layer.set_pixel(5, 5, test_color).unwrap();
        let retrieved = layer.get_pixel(5, 5).unwrap();

        assert_eq!(retrieved, test_color);
    }

    #[test]
    fn test_layer_fill() {
        let mut layer = Layer::new_pixel("Test".to_string(), 5, 5);
        let fill_color = RgbaPixel::new(100, 150, 200, 255);

        layer.fill(fill_color);

        // Check a few pixels
        assert_eq!(layer.get_pixel(0, 0).unwrap(), fill_color);
        assert_eq!(layer.get_pixel(2, 3).unwrap(), fill_color);
        assert_eq!(layer.get_pixel(4, 4).unwrap(), fill_color);
    }

    #[test]
    fn test_blend_mode_normal() {
        let base = RgbaPixel::new(100, 100, 100, 255);
        let overlay = RgbaPixel::new(200, 200, 200, 128);

        let result = BlendMode::Normal.blend(base, overlay, 1.0);

        // Result should be between base and overlay
        assert!(result.r > base.r && result.r < overlay.r);
        assert!(result.g > base.g && result.g < overlay.g);
        assert!(result.b > base.b && result.b < overlay.b);
    }

    #[test]
    fn test_layer_duplication() {
        let original = Layer::new_pixel("Original".to_string(), 10, 10);
        let duplicate = original.duplicate();

        assert_ne!(original.id, duplicate.id);
        assert_eq!(duplicate.name, "Original copy");
        assert_eq!(original.dimensions(), duplicate.dimensions());
    }

    #[test]
    fn test_layer_visibility_and_opacity() {
        let mut layer = Layer::new_pixel("Test".to_string(), 10, 10);

        // Test default visibility and opacity
        assert!(layer.visible);
        assert_eq!(layer.opacity, 1.0);
        assert!(layer.is_effectively_visible());

        // Test visibility toggle
        layer.visible = false;
        assert!(!layer.is_effectively_visible());

        layer.visible = true;
        layer.opacity = 0.0;
        assert!(!layer.is_effectively_visible());

        layer.opacity = 0.5;
        assert!(layer.is_effectively_visible());
        assert_eq!(layer.effective_opacity(), 0.5);
    }

    #[test]
    fn test_blend_mode_assignment() {
        let mut layer = Layer::new_pixel("Test".to_string(), 10, 10);

        // Test default blend mode
        assert_eq!(layer.blend_mode, BlendMode::Normal);

        // Test blend mode assignment
        layer.blend_mode = BlendMode::Multiply;
        assert_eq!(layer.blend_mode, BlendMode::Multiply);

        layer.blend_mode = BlendMode::Screen;
        assert_eq!(layer.blend_mode, BlendMode::Screen);
    }

    #[test]
    fn test_layer_pixel_data_requirement() {
        // Pixel layer should have pixel data
        let pixel_layer = Layer::new_pixel("Pixel".to_string(), 10, 10);
        assert!(pixel_layer.has_pixel_data());
        assert!(pixel_layer.pixel_data.is_some());

        // Text layer should not have pixel data initially
        let text_layer = Layer::new_text(
            "Text".to_string(),
            "Hello".to_string(),
            "Arial".to_string(),
            12.0,
            RgbaPixel::black(),
            Point::origin(),
        );
        assert!(!text_layer.has_pixel_data());
        assert!(text_layer.pixel_data.is_none());
    }

    #[test]
    fn test_blend_mode_names() {
        assert_eq!(BlendMode::Normal.name(), "Normal");
        assert_eq!(BlendMode::Multiply.name(), "Multiply");
        assert_eq!(BlendMode::Screen.name(), "Screen");
        assert_eq!(BlendMode::Overlay.name(), "Overlay");
    }

    #[test]
    fn test_blend_mode_all() {
        let all_modes = BlendMode::all();
        assert!(!all_modes.is_empty());
        assert!(all_modes.contains(&BlendMode::Normal));
        assert!(all_modes.contains(&BlendMode::Multiply));
        assert!(all_modes.contains(&BlendMode::Screen));
    }

    #[test]
    fn test_blend_mode_multiply() {
        let base = RgbaPixel::new(200, 200, 200, 255);
        let overlay = RgbaPixel::new(128, 128, 128, 255);

        let result = BlendMode::Multiply.blend(base, overlay, 1.0);

        // Multiply should darken the image
        // 200 * 128 / 255 = ~100, which is darker than 200
        assert!(result.r < base.r);
        assert!(result.g < base.g);
        assert!(result.b < base.b);
    }

    #[test]
    fn test_blend_mode_screen() {
        let base = RgbaPixel::new(128, 128, 128, 255);
        let overlay = RgbaPixel::new(128, 128, 128, 255);

        let result = BlendMode::Screen.blend(base, overlay, 1.0);

        // Screen should lighten the image
        assert!(result.r >= base.r);
        assert!(result.g >= base.g);
        assert!(result.b >= base.b);
    }

    #[test]
    fn test_blend_mode_darken() {
        let base = RgbaPixel::new(200, 100, 150, 255);
        let overlay = RgbaPixel::new(100, 200, 100, 255);

        let result = BlendMode::Darken.blend(base, overlay, 1.0);

        // Darken should pick the darker color for each channel
        // Expected: min(200,100)=100, min(100,200)=100, min(150,100)=100
        assert!(result.r < base.r); // Should be darker than base.r (200), closer to 100
        assert!(result.g <= base.g + 5); // Should be close to base.g (100), allowing for blending effects
        assert!(result.b < base.b); // Should be darker than base.b (150), closer to 100

        // The result should be different from the base (since blending occurred)
        assert_ne!(result, base);
    }

    #[test]
    fn test_blend_mode_lighten() {
        let base = RgbaPixel::new(200, 100, 150, 255);
        let overlay = RgbaPixel::new(100, 200, 100, 255);

        let result = BlendMode::Lighten.blend(base, overlay, 1.0);

        // Lighten should pick the lighter color for each channel (after normal blending)
        // The result should be closer to the lighter values
        assert!(result.r >= base.r || result.r >= overlay.r); // Should be at least as light as one of them
        assert!(result.g > base.g); // Should be lighter than base.g (100)
        assert!(result.b >= base.b || result.b >= overlay.b); // Should be at least as light as one of them
    }

    #[test]
    fn test_blend_mode_difference() {
        let base = RgbaPixel::new(200, 100, 150, 255);
        let overlay = RgbaPixel::new(50, 200, 100, 128); // Use semi-transparent overlay

        let result = BlendMode::Difference.blend(base, overlay, 1.0);

        // Difference should compute absolute difference (after normal blending)
        // The result should be different from both base and overlay
        assert_ne!(result, base);
        assert_ne!(result, overlay);
        // The result should reflect the difference operation
        assert!(result.r != base.r);
    }

    #[test]
    fn test_blend_mode_opacity() {
        let base = RgbaPixel::new(100, 100, 100, 255);
        let overlay = RgbaPixel::new(200, 200, 200, 255);

        // Test with 50% opacity
        let result = BlendMode::Normal.blend(base, overlay, 0.5);

        // Result should be between base and overlay
        assert!(result.r > base.r && result.r < overlay.r);
        assert!(result.g > base.g && result.g < overlay.g);
        assert!(result.b > base.b && result.b < overlay.b);

        // Test with 0% opacity (should return base)
        let result_zero = BlendMode::Normal.blend(base, overlay, 0.0);
        assert_eq!(result_zero, base);
    }

    #[test]
    fn test_blend_mode_hsl_modes() {
        let base = RgbaPixel::new(255, 128, 64, 255); // Orange
        let overlay = RgbaPixel::new(64, 128, 255, 128); // Semi-transparent Blue

        // Test hue blending (should take hue from overlay, saturation/lightness from base)
        let hue_result = BlendMode::Hue.blend(base, overlay, 1.0);

        // Test saturation blending (should take saturation from overlay, hue/lightness from base)
        let sat_result = BlendMode::Saturation.blend(base, overlay, 1.0);

        // Test color blending (should take hue/saturation from overlay, lightness from base)
        let color_result = BlendMode::Color.blend(base, overlay, 1.0);

        // Test luminosity blending (should take lightness from overlay, hue/saturation from base)
        let lum_result = BlendMode::Luminosity.blend(base, overlay, 1.0);

        // Results should be different from base (since we're blending with semi-transparent overlay)
        assert_ne!(hue_result, base);
        assert_ne!(sat_result, base);
        assert_ne!(color_result, base);
        assert_ne!(lum_result, base);

        // At least some results should be different from each other (different blend modes)
        // Note: Some HSL blend modes might produce similar results with certain color combinations
        let results_different = hue_result != sat_result
            || hue_result != color_result
            || hue_result != lum_result
            || sat_result != color_result
            || sat_result != lum_result
            || color_result != lum_result;
        assert!(
            results_different,
            "HSL blend modes should produce at least some different results"
        );
    }

    #[test]
    fn test_layer_mask_creation() {
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 50);

        assert!(!layer.has_mask());
        assert!(layer.mask_dimensions().is_none());

        // Create a mask
        layer.create_mask(100, 50).unwrap();

        assert!(layer.has_mask());
        assert_eq!(layer.mask_dimensions(), Some((100, 50)));
    }

    #[test]
    fn test_layer_mask_removal() {
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 100, 50);

        // Create and then remove mask
        layer.create_mask(100, 50).unwrap();
        assert!(layer.has_mask());

        layer.remove_mask();
        assert!(!layer.has_mask());
        assert!(layer.mask_dimensions().is_none());
    }

    #[test]
    fn test_layer_mask_pixel_operations() {
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);
        layer.create_mask(10, 10).unwrap();

        // Test setting and getting mask pixels
        let white_pixel = RgbaPixel::white();
        let black_pixel = RgbaPixel::black();

        layer.set_mask_pixel(5, 5, black_pixel).unwrap();
        assert_eq!(layer.get_mask_pixel(5, 5), Some(black_pixel));

        // Test mask fill
        layer.fill_mask(white_pixel).unwrap();
        assert_eq!(layer.get_mask_pixel(5, 5), Some(white_pixel));

        // Test mask clear
        layer.clear_mask().unwrap();
        assert_eq!(layer.get_mask_pixel(5, 5), Some(black_pixel));
    }

    #[test]
    fn test_layer_mask_inversion() {
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);
        layer.create_mask(10, 10).unwrap();

        // Fill with white, then invert to black
        layer.fill_mask(RgbaPixel::white()).unwrap();
        layer.invert_mask().unwrap();

        let pixel = layer.get_mask_pixel(5, 5).unwrap();
        assert_eq!(pixel.r, 0); // Should be black after inversion
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn test_masked_pixel_retrieval() {
        let mut layer = Layer::new_pixel("Test Layer".to_string(), 10, 10);

        // Set a red pixel
        let red_pixel = RgbaPixel::new(255, 0, 0, 255);
        layer.set_pixel(5, 5, red_pixel).unwrap();

        // Without mask, should get full opacity
        assert_eq!(layer.get_masked_pixel(5, 5), Some(red_pixel));

        // Create mask and set to half opacity (gray)
        layer.create_mask(10, 10).unwrap();
        let gray_pixel = RgbaPixel::new(128, 128, 128, 255);
        layer.set_mask_pixel(5, 5, gray_pixel).unwrap();

        // Should get pixel with reduced alpha
        let masked_pixel = layer.get_masked_pixel(5, 5).unwrap();
        assert_eq!(masked_pixel.r, 255); // Color unchanged
        assert_eq!(masked_pixel.g, 0);
        assert_eq!(masked_pixel.b, 0);
        assert!(masked_pixel.a < 255); // Alpha reduced by mask
    }
}
