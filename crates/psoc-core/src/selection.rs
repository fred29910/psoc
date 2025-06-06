//! Selection system for PSOC
//!
//! This module provides selection functionality including rectangular selections,
//! selection masks, and selection operations.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::geometry::{Point, Rect};

/// Selection types supported by PSOC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Selection {
    /// No selection (everything is selected)
    #[default]
    None,
    /// Rectangular selection
    Rectangle(RectangleSelection),
    // Future: Elliptical selection
    // Ellipse(EllipseSelection),
    // Future: Freeform selection with mask
    // Mask(MaskSelection),
}

impl Selection {
    /// Create a new rectangular selection
    pub fn rectangle(x: f32, y: f32, width: f32, height: f32) -> Self {
        Selection::Rectangle(RectangleSelection::new(x, y, width, height))
    }

    /// Create a rectangular selection from two points
    pub fn rectangle_from_points(start: Point, end: Point) -> Self {
        let x = start.x.min(end.x);
        let y = start.y.min(end.y);
        let width = (end.x - start.x).abs();
        let height = (end.y - start.y).abs();

        Selection::Rectangle(RectangleSelection::new(x, y, width, height))
    }

    /// Check if a point is inside the selection
    pub fn contains_point(&self, point: Point) -> bool {
        match self {
            Selection::None => true, // No selection means everything is selected
            Selection::Rectangle(rect) => rect.contains_point(point),
        }
    }

    /// Get the bounding rectangle of the selection
    pub fn bounds(&self) -> Option<Rect> {
        match self {
            Selection::None => None, // No bounds for "select all"
            Selection::Rectangle(rect) => Some(rect.bounds()),
        }
    }

    /// Check if the selection is empty (has no area)
    pub fn is_empty(&self) -> bool {
        match self {
            Selection::None => false, // "Select all" is not empty
            Selection::Rectangle(rect) => rect.is_empty(),
        }
    }

    /// Check if this is a "select all" selection
    pub fn is_select_all(&self) -> bool {
        matches!(self, Selection::None)
    }

    /// Get the area of the selection in pixels
    pub fn area(&self) -> f32 {
        match self {
            Selection::None => f32::INFINITY, // Infinite area for "select all"
            Selection::Rectangle(rect) => rect.area(),
        }
    }

    /// Transform the selection by a given offset
    pub fn translate(&mut self, dx: f32, dy: f32) {
        match self {
            Selection::None => {} // No-op for "select all"
            Selection::Rectangle(rect) => rect.translate(dx, dy),
        }
    }

    /// Scale the selection by a given factor
    pub fn scale(&mut self, factor: f32) {
        match self {
            Selection::None => {} // No-op for "select all"
            Selection::Rectangle(rect) => rect.scale(factor),
        }
    }
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selection::None => write!(f, "Select All"),
            Selection::Rectangle(rect) => write!(f, "Rectangle {}", rect),
        }
    }
}

/// Rectangular selection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RectangleSelection {
    /// Selection rectangle
    pub rect: Rect,
    /// Whether the selection is inverted (everything outside is selected)
    pub inverted: bool,
}

impl RectangleSelection {
    /// Create a new rectangular selection
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            inverted: false,
        }
    }

    /// Create from a rectangle
    pub fn from_rect(rect: Rect) -> Self {
        Self {
            rect,
            inverted: false,
        }
    }

    /// Create an inverted rectangular selection
    pub fn new_inverted(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            inverted: true,
        }
    }

    /// Check if a point is inside the selection
    pub fn contains_point(&self, point: Point) -> bool {
        let inside_rect = self.rect.contains_point(point);
        if self.inverted {
            !inside_rect
        } else {
            inside_rect
        }
    }

    /// Get the bounding rectangle
    pub fn bounds(&self) -> Rect {
        self.rect
    }

    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        self.rect.width <= 0.0 || self.rect.height <= 0.0
    }

    /// Get the area of the selection
    pub fn area(&self) -> f32 {
        if self.inverted {
            // For inverted selections, we can't easily calculate the area
            // without knowing the canvas bounds
            f32::INFINITY
        } else {
            self.rect.width * self.rect.height
        }
    }

    /// Translate the selection
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.rect.x += dx;
        self.rect.y += dy;
    }

    /// Scale the selection
    pub fn scale(&mut self, factor: f32) {
        self.rect.x *= factor;
        self.rect.y *= factor;
        self.rect.width *= factor;
        self.rect.height *= factor;
    }

    /// Get the corners of the selection rectangle
    pub fn corners(&self) -> [Point; 4] {
        [
            Point::new(self.rect.x, self.rect.y), // Top-left
            Point::new(self.rect.x + self.rect.width, self.rect.y), // Top-right
            Point::new(
                self.rect.x + self.rect.width,
                self.rect.y + self.rect.height,
            ), // Bottom-right
            Point::new(self.rect.x, self.rect.y + self.rect.height), // Bottom-left
        ]
    }

    /// Get the center point of the selection
    pub fn center(&self) -> Point {
        Point::new(
            self.rect.x + self.rect.width / 2.0,
            self.rect.y + self.rect.height / 2.0,
        )
    }
}

impl fmt::Display for RectangleSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inverted {
            write!(
                f,
                "Inverted({:.1}, {:.1}, {:.1}x{:.1})",
                self.rect.x, self.rect.y, self.rect.width, self.rect.height
            )
        } else {
            write!(
                f,
                "({:.1}, {:.1}, {:.1}x{:.1})",
                self.rect.x, self.rect.y, self.rect.width, self.rect.height
            )
        }
    }
}

/// Selection operation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SelectionMode {
    /// Replace the current selection
    #[default]
    Replace,
    /// Add to the current selection
    Add,
    /// Subtract from the current selection
    Subtract,
    /// Intersect with the current selection
    Intersect,
}

impl fmt::Display for SelectionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionMode::Replace => write!(f, "Replace"),
            SelectionMode::Add => write!(f, "Add"),
            SelectionMode::Subtract => write!(f, "Subtract"),
            SelectionMode::Intersect => write!(f, "Intersect"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_none() {
        let selection = Selection::None;
        assert!(selection.is_select_all());
        assert!(!selection.is_empty());
        assert!(selection.contains_point(Point::new(100.0, 200.0)));
        assert_eq!(selection.bounds(), None);
        assert_eq!(selection.area(), f32::INFINITY);
    }

    #[test]
    fn test_rectangle_selection_creation() {
        let selection = Selection::rectangle(10.0, 20.0, 100.0, 50.0);

        if let Selection::Rectangle(rect) = selection {
            assert_eq!(rect.rect.x, 10.0);
            assert_eq!(rect.rect.y, 20.0);
            assert_eq!(rect.rect.width, 100.0);
            assert_eq!(rect.rect.height, 50.0);
            assert!(!rect.inverted);
        } else {
            panic!("Expected Rectangle selection");
        }
    }

    #[test]
    fn test_rectangle_from_points() {
        let start = Point::new(50.0, 30.0);
        let end = Point::new(10.0, 80.0);
        let selection = Selection::rectangle_from_points(start, end);

        if let Selection::Rectangle(rect) = selection {
            assert_eq!(rect.rect.x, 10.0); // min x
            assert_eq!(rect.rect.y, 30.0); // min y
            assert_eq!(rect.rect.width, 40.0); // |50 - 10|
            assert_eq!(rect.rect.height, 50.0); // |80 - 30|
        } else {
            panic!("Expected Rectangle selection");
        }
    }

    #[test]
    fn test_rectangle_contains_point() {
        let selection = Selection::rectangle(10.0, 20.0, 100.0, 50.0);

        assert!(selection.contains_point(Point::new(50.0, 40.0))); // Inside
        assert!(selection.contains_point(Point::new(10.0, 20.0))); // Top-left corner
        assert!(selection.contains_point(Point::new(110.0, 70.0))); // Bottom-right corner
        assert!(!selection.contains_point(Point::new(5.0, 40.0))); // Outside left
        assert!(!selection.contains_point(Point::new(50.0, 15.0))); // Outside top
    }

    #[test]
    fn test_rectangle_bounds_and_area() {
        let selection = Selection::rectangle(10.0, 20.0, 100.0, 50.0);

        let bounds = selection.bounds().unwrap();
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 100.0);
        assert_eq!(bounds.height, 50.0);

        assert_eq!(selection.area(), 5000.0); // 100 * 50
    }

    #[test]
    fn test_rectangle_empty() {
        let empty_selection = Selection::rectangle(10.0, 20.0, 0.0, 50.0);
        assert!(empty_selection.is_empty());

        let valid_selection = Selection::rectangle(10.0, 20.0, 100.0, 50.0);
        assert!(!valid_selection.is_empty());
    }

    #[test]
    fn test_rectangle_transform() {
        let mut selection = Selection::rectangle(10.0, 20.0, 100.0, 50.0);

        // Test translation
        selection.translate(5.0, -10.0);
        if let Selection::Rectangle(rect) = &selection {
            assert_eq!(rect.rect.x, 15.0);
            assert_eq!(rect.rect.y, 10.0);
        }

        // Test scaling
        selection.scale(2.0);
        if let Selection::Rectangle(rect) = &selection {
            assert_eq!(rect.rect.x, 30.0);
            assert_eq!(rect.rect.y, 20.0);
            assert_eq!(rect.rect.width, 200.0);
            assert_eq!(rect.rect.height, 100.0);
        }
    }

    #[test]
    fn test_inverted_rectangle() {
        let rect = RectangleSelection::new_inverted(10.0, 20.0, 100.0, 50.0);
        assert!(rect.inverted);

        // Point inside the rectangle should not be selected (inverted)
        assert!(!rect.contains_point(Point::new(50.0, 40.0)));
        // Point outside the rectangle should be selected (inverted)
        assert!(rect.contains_point(Point::new(5.0, 40.0)));
    }

    #[test]
    fn test_selection_mode_display() {
        assert_eq!(format!("{}", SelectionMode::Replace), "Replace");
        assert_eq!(format!("{}", SelectionMode::Add), "Add");
        assert_eq!(format!("{}", SelectionMode::Subtract), "Subtract");
        assert_eq!(format!("{}", SelectionMode::Intersect), "Intersect");
    }

    #[test]
    fn test_rectangle_corners_and_center() {
        let rect = RectangleSelection::new(10.0, 20.0, 100.0, 50.0);

        let corners = rect.corners();
        assert_eq!(corners[0], Point::new(10.0, 20.0)); // Top-left
        assert_eq!(corners[1], Point::new(110.0, 20.0)); // Top-right
        assert_eq!(corners[2], Point::new(110.0, 70.0)); // Bottom-right
        assert_eq!(corners[3], Point::new(10.0, 70.0)); // Bottom-left

        let center = rect.center();
        assert_eq!(center, Point::new(60.0, 45.0));
    }
}
