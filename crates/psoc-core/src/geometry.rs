//! Geometric calculations and utilities
//!
//! This module provides geometric primitives, transformations, and calculations
//! for image editing operations.

use glam::{Affine2, Vec2};
use serde::{Deserialize, Serialize};

/// 2D point representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Create a new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Origin point (0, 0)
    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Distance to another point
    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Translate by offset
    pub fn translate(&self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    /// Convert to Vec2
    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Create from Vec2
    pub fn from_vec2(v: Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Point> for (f32, f32) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

/// 2D size representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    /// Create a new size
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Square size
    pub fn square(size: f32) -> Self {
        Self::new(size, size)
    }

    /// Area of the size
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Aspect ratio (width / height)
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0.0 {
            f32::INFINITY
        } else {
            self.width / self.height
        }
    }

    /// Scale by factor
    pub fn scale(&self, factor: f32) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }

    /// Scale by different factors for width and height
    pub fn scale_xy(&self, x_factor: f32, y_factor: f32) -> Self {
        Self::new(self.width * x_factor, self.height * y_factor)
    }
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self::new(width, height)
    }
}

/// Rectangle representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create rectangle from position and size
    pub fn from_pos_size(pos: Point, size: Size) -> Self {
        Self::new(pos.x, pos.y, size.width, size.height)
    }

    /// Create rectangle from two points
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p2.x - p1.x).abs();
        let height = (p2.y - p1.y).abs();
        Self::new(x, y, width, height)
    }

    /// Get position (top-left corner)
    pub fn position(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Get size
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    /// Get center point
    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Get top-left corner
    pub fn top_left(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Get top-right corner
    pub fn top_right(&self) -> Point {
        Point::new(self.x + self.width, self.y)
    }

    /// Get bottom-left corner
    pub fn bottom_left(&self) -> Point {
        Point::new(self.x, self.y + self.height)
    }

    /// Get bottom-right corner
    pub fn bottom_right(&self) -> Point {
        Point::new(self.x + self.width, self.y + self.height)
    }

    /// Check if point is inside rectangle
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }

    /// Check if rectangle intersects with another rectangle
    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.x + self.width < other.x
            || other.x + other.width < self.x
            || self.y + self.height < other.y
            || other.y + other.height < self.y)
    }

    /// Get intersection with another rectangle
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }

        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);

        Some(Rect::new(x, y, right - x, bottom - y))
    }

    /// Get union with another rectangle
    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = (self.x + self.width).max(other.x + other.width);
        let bottom = (self.y + self.height).max(other.y + other.height);

        Rect::new(x, y, right - x, bottom - y)
    }

    /// Translate rectangle
    pub fn translate(&self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    /// Scale rectangle from center
    pub fn scale(&self, factor: f32) -> Self {
        let center = self.center();
        let new_width = self.width * factor;
        let new_height = self.height * factor;
        let new_x = center.x - new_width / 2.0;
        let new_y = center.y - new_height / 2.0;
        Self::new(new_x, new_y, new_width, new_height)
    }
}

/// 2D transformation matrix
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    /// Internal affine transformation matrix
    matrix: Affine2,
}

impl Transform {
    /// Identity transformation
    pub fn identity() -> Self {
        Self {
            matrix: Affine2::IDENTITY,
        }
    }

    /// Translation transformation
    pub fn translation(dx: f32, dy: f32) -> Self {
        Self {
            matrix: Affine2::from_translation(Vec2::new(dx, dy)),
        }
    }

    /// Rotation transformation (angle in radians)
    pub fn rotation(angle: f32) -> Self {
        Self {
            matrix: Affine2::from_angle(angle),
        }
    }

    /// Scale transformation
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            matrix: Affine2::from_scale(Vec2::new(sx, sy)),
        }
    }

    /// Uniform scale transformation
    pub fn uniform_scale(scale: f32) -> Self {
        Self::scale(scale, scale)
    }

    /// Apply transformation to a point
    pub fn transform_point(&self, point: Point) -> Point {
        let transformed = self.matrix.transform_point2(point.to_vec2());
        Point::from_vec2(transformed)
    }

    /// Apply transformation to a rectangle
    pub fn transform_rect(&self, rect: Rect) -> Rect {
        let corners = [
            rect.top_left(),
            rect.top_right(),
            rect.bottom_left(),
            rect.bottom_right(),
        ];

        let transformed_corners: Vec<Point> = corners
            .iter()
            .map(|&corner| self.transform_point(corner))
            .collect();

        let min_x = transformed_corners
            .iter()
            .map(|p| p.x)
            .fold(f32::INFINITY, f32::min);
        let max_x = transformed_corners
            .iter()
            .map(|p| p.x)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = transformed_corners
            .iter()
            .map(|p| p.y)
            .fold(f32::INFINITY, f32::min);
        let max_y = transformed_corners
            .iter()
            .map(|p| p.y)
            .fold(f32::NEG_INFINITY, f32::max);

        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Combine with another transformation
    pub fn then(&self, other: &Transform) -> Self {
        Self {
            matrix: other.matrix * self.matrix,
        }
    }

    /// Get inverse transformation
    pub fn inverse(&self) -> Self {
        Self {
            matrix: self.matrix.inverse(),
        }
    }

    /// Get the transformation matrix as a 2x3 array
    pub fn to_array(self) -> [[f32; 2]; 3] {
        self.matrix.to_cols_array_2d()
    }

    /// Create from 2x3 array
    pub fn from_array(array: [[f32; 2]; 3]) -> Self {
        Self {
            matrix: Affine2::from_cols_array_2d(&array),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Serialize for Transform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_array().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Transform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let array = <[[f32; 2]; 3]>::deserialize(deserializer)?;
        Ok(Self::from_array(array))
    }
}
