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
    /// Elliptical selection
    Ellipse(EllipseSelection),
    /// Freeform selection with path
    Lasso(LassoSelection),
    /// Selection based on pixel mask
    Mask(MaskSelection),
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

    /// Create a new elliptical selection
    pub fn ellipse(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Selection::Ellipse(EllipseSelection::new(
            center_x, center_y, radius_x, radius_y,
        ))
    }

    /// Create an elliptical selection from two points (bounding box)
    pub fn ellipse_from_points(start: Point, end: Point) -> Self {
        let center_x = (start.x + end.x) / 2.0;
        let center_y = (start.y + end.y) / 2.0;
        let radius_x = (end.x - start.x).abs() / 2.0;
        let radius_y = (end.y - start.y).abs() / 2.0;

        Selection::Ellipse(EllipseSelection::new(
            center_x, center_y, radius_x, radius_y,
        ))
    }

    /// Create a new lasso selection from a path
    pub fn lasso(points: Vec<Point>) -> Self {
        Selection::Lasso(LassoSelection::new(points))
    }

    /// Create a new mask selection
    pub fn mask(width: u32, height: u32, mask_data: Vec<u8>) -> Self {
        Selection::Mask(MaskSelection::new(width, height, mask_data))
    }

    /// Check if a point is inside the selection
    pub fn contains_point(&self, point: Point) -> bool {
        match self {
            Selection::None => true, // No selection means everything is selected
            Selection::Rectangle(rect) => rect.contains_point(point),
            Selection::Ellipse(ellipse) => ellipse.contains_point(point),
            Selection::Lasso(lasso) => lasso.contains_point(point),
            Selection::Mask(mask) => mask.contains_point(point),
        }
    }

    /// Get the bounding rectangle of the selection
    pub fn bounds(&self) -> Option<Rect> {
        match self {
            Selection::None => None, // No bounds for "select all"
            Selection::Rectangle(rect) => Some(rect.bounds()),
            Selection::Ellipse(ellipse) => Some(ellipse.bounds()),
            Selection::Lasso(lasso) => Some(lasso.bounds()),
            Selection::Mask(mask) => Some(mask.bounds()),
        }
    }

    /// Check if the selection is empty (has no area)
    pub fn is_empty(&self) -> bool {
        match self {
            Selection::None => false, // "Select all" is not empty
            Selection::Rectangle(rect) => rect.is_empty(),
            Selection::Ellipse(ellipse) => ellipse.is_empty(),
            Selection::Lasso(lasso) => lasso.is_empty(),
            Selection::Mask(mask) => mask.is_empty(),
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
            Selection::Ellipse(ellipse) => ellipse.area(),
            Selection::Lasso(lasso) => lasso.area(),
            Selection::Mask(mask) => mask.area(),
        }
    }

    /// Transform the selection by a given offset
    pub fn translate(&mut self, dx: f32, dy: f32) {
        match self {
            Selection::None => {} // No-op for "select all"
            Selection::Rectangle(rect) => rect.translate(dx, dy),
            Selection::Ellipse(ellipse) => ellipse.translate(dx, dy),
            Selection::Lasso(lasso) => lasso.translate(dx, dy),
            Selection::Mask(mask) => mask.translate(dx, dy),
        }
    }

    /// Scale the selection by a given factor
    pub fn scale(&mut self, factor: f32) {
        match self {
            Selection::None => {} // No-op for "select all"
            Selection::Rectangle(rect) => rect.scale(factor),
            Selection::Ellipse(ellipse) => ellipse.scale(factor),
            Selection::Lasso(lasso) => lasso.scale(factor),
            Selection::Mask(mask) => mask.scale(factor),
        }
    }
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selection::None => write!(f, "Select All"),
            Selection::Rectangle(rect) => write!(f, "Rectangle {}", rect),
            Selection::Ellipse(ellipse) => write!(f, "Ellipse {}", ellipse),
            Selection::Lasso(lasso) => write!(f, "Lasso {}", lasso),
            Selection::Mask(mask) => write!(f, "Mask {}", mask),
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

/// Elliptical selection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EllipseSelection {
    /// Center point of the ellipse
    pub center: Point,
    /// Horizontal radius
    pub radius_x: f32,
    /// Vertical radius
    pub radius_y: f32,
    /// Whether the selection is inverted
    pub inverted: bool,
}

impl EllipseSelection {
    /// Create a new elliptical selection
    pub fn new(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center: Point::new(center_x, center_y),
            radius_x,
            radius_y,
            inverted: false,
        }
    }

    /// Create an inverted elliptical selection
    pub fn new_inverted(center_x: f32, center_y: f32, radius_x: f32, radius_y: f32) -> Self {
        Self {
            center: Point::new(center_x, center_y),
            radius_x,
            radius_y,
            inverted: true,
        }
    }

    /// Check if a point is inside the ellipse
    pub fn contains_point(&self, point: Point) -> bool {
        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;

        // Ellipse equation: (x/a)² + (y/b)² <= 1
        let normalized = (dx * dx) / (self.radius_x * self.radius_x)
            + (dy * dy) / (self.radius_y * self.radius_y);

        let inside_ellipse = normalized <= 1.0;

        if self.inverted {
            !inside_ellipse
        } else {
            inside_ellipse
        }
    }

    /// Get the bounding rectangle
    pub fn bounds(&self) -> Rect {
        Rect::new(
            self.center.x - self.radius_x,
            self.center.y - self.radius_y,
            self.radius_x * 2.0,
            self.radius_y * 2.0,
        )
    }

    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        self.radius_x <= 0.0 || self.radius_y <= 0.0
    }

    /// Get the area of the ellipse
    pub fn area(&self) -> f32 {
        if self.inverted {
            f32::INFINITY
        } else {
            std::f32::consts::PI * self.radius_x * self.radius_y
        }
    }

    /// Translate the ellipse
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.center.x += dx;
        self.center.y += dy;
    }

    /// Scale the ellipse
    pub fn scale(&mut self, factor: f32) {
        self.center.x *= factor;
        self.center.y *= factor;
        self.radius_x *= factor;
        self.radius_y *= factor;
    }
}

impl fmt::Display for EllipseSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inverted {
            write!(
                f,
                "Inverted(center: {:.1}, {:.1}, radii: {:.1}x{:.1})",
                self.center.x, self.center.y, self.radius_x, self.radius_y
            )
        } else {
            write!(
                f,
                "(center: {:.1}, {:.1}, radii: {:.1}x{:.1})",
                self.center.x, self.center.y, self.radius_x, self.radius_y
            )
        }
    }
}

/// Lasso (freeform) selection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LassoSelection {
    /// Path points defining the selection boundary
    pub points: Vec<Point>,
    /// Cached bounding rectangle
    bounds_cache: Option<Rect>,
    /// Whether the selection is inverted
    pub inverted: bool,
}

impl LassoSelection {
    /// Create a new lasso selection
    pub fn new(points: Vec<Point>) -> Self {
        Self {
            points,
            bounds_cache: None,
            inverted: false,
        }
    }

    /// Create an inverted lasso selection
    pub fn new_inverted(points: Vec<Point>) -> Self {
        Self {
            points,
            bounds_cache: None,
            inverted: true,
        }
    }

    /// Add a point to the path
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
        self.bounds_cache = None; // Invalidate cache
    }

    /// Close the path by connecting the last point to the first
    pub fn close_path(&mut self) {
        if self.points.len() > 2 {
            let first_point = self.points[0];
            if let Some(last_point) = self.points.last() {
                if *last_point != first_point {
                    self.points.push(first_point);
                }
            }
        }
    }

    /// Check if a point is inside the lasso selection using ray casting
    pub fn contains_point(&self, point: Point) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = self.points.len() - 1;

        for i in 0..self.points.len() {
            let pi = &self.points[i];
            let pj = &self.points[j];

            if ((pi.y > point.y) != (pj.y > point.y))
                && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
            {
                inside = !inside;
            }
            j = i;
        }

        if self.inverted {
            !inside
        } else {
            inside
        }
    }

    /// Get the bounding rectangle
    pub fn bounds(&self) -> Rect {
        if let Some(cached) = self.bounds_cache {
            return cached;
        }

        if self.points.is_empty() {
            return Rect::new(0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = self.points[0].x;
        let mut max_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut max_y = self.points[0].y;

        for point in &self.points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        self.points.len() < 3
    }

    /// Get the approximate area using the shoelace formula
    pub fn area(&self) -> f32 {
        if self.inverted {
            return f32::INFINITY;
        }

        if self.points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            area += self.points[i].x * self.points[j].y;
            area -= self.points[j].x * self.points[i].y;
        }

        (area / 2.0).abs()
    }

    /// Translate the lasso selection
    pub fn translate(&mut self, dx: f32, dy: f32) {
        for point in &mut self.points {
            point.x += dx;
            point.y += dy;
        }
        self.bounds_cache = None;
    }

    /// Scale the lasso selection
    pub fn scale(&mut self, factor: f32) {
        for point in &mut self.points {
            point.x *= factor;
            point.y *= factor;
        }
        self.bounds_cache = None;
    }
}

impl fmt::Display for LassoSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inverted {
            write!(f, "Inverted({} points)", self.points.len())
        } else {
            write!(f, "({} points)", self.points.len())
        }
    }
}

/// Mask-based selection (for magic wand and similar tools)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaskSelection {
    /// Width of the mask
    pub width: u32,
    /// Height of the mask
    pub height: u32,
    /// Mask data (0 = not selected, 255 = fully selected)
    pub mask_data: Vec<u8>,
    /// Offset of the mask in the image
    pub offset: Point,
    /// Whether the selection is inverted
    pub inverted: bool,
}

impl MaskSelection {
    /// Create a new mask selection
    pub fn new(width: u32, height: u32, mask_data: Vec<u8>) -> Self {
        assert_eq!(mask_data.len(), (width * height) as usize);

        Self {
            width,
            height,
            mask_data,
            offset: Point::new(0.0, 0.0),
            inverted: false,
        }
    }

    /// Create a new mask selection with offset
    pub fn new_with_offset(width: u32, height: u32, mask_data: Vec<u8>, offset: Point) -> Self {
        assert_eq!(mask_data.len(), (width * height) as usize);

        Self {
            width,
            height,
            mask_data,
            offset,
            inverted: false,
        }
    }

    /// Create an inverted mask selection
    pub fn new_inverted(width: u32, height: u32, mask_data: Vec<u8>) -> Self {
        assert_eq!(mask_data.len(), (width * height) as usize);

        Self {
            width,
            height,
            mask_data,
            offset: Point::new(0.0, 0.0),
            inverted: true,
        }
    }

    /// Check if a point is selected in the mask
    pub fn contains_point(&self, point: Point) -> bool {
        let local_x = point.x - self.offset.x;
        let local_y = point.y - self.offset.y;

        if local_x < 0.0
            || local_y < 0.0
            || local_x >= self.width as f32
            || local_y >= self.height as f32
        {
            return self.inverted; // Outside mask bounds
        }

        let x = local_x as u32;
        let y = local_y as u32;
        let index = (y * self.width + x) as usize;

        if index >= self.mask_data.len() {
            return self.inverted;
        }

        let selected = self.mask_data[index] > 127; // Threshold at 50%

        if self.inverted {
            !selected
        } else {
            selected
        }
    }

    /// Get the bounding rectangle
    pub fn bounds(&self) -> Rect {
        Rect::new(
            self.offset.x,
            self.offset.y,
            self.width as f32,
            self.height as f32,
        )
    }

    /// Check if the selection is empty
    pub fn is_empty(&self) -> bool {
        if self.inverted {
            return false; // Inverted mask is never empty
        }

        // Check if any pixel is selected
        self.mask_data.iter().all(|&value| value <= 127)
    }

    /// Get the approximate area by counting selected pixels
    pub fn area(&self) -> f32 {
        if self.inverted {
            return f32::INFINITY;
        }

        let selected_pixels = self.mask_data.iter().filter(|&&value| value > 127).count();

        selected_pixels as f32
    }

    /// Translate the mask selection
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.offset.x += dx;
        self.offset.y += dy;
    }

    /// Scale the mask selection (this is complex and simplified here)
    pub fn scale(&mut self, factor: f32) {
        self.offset.x *= factor;
        self.offset.y *= factor;
        // Note: Actual mask scaling would require resampling the mask data
        // For now, we just scale the offset and dimensions conceptually
    }

    /// Set a pixel in the mask
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if index < self.mask_data.len() {
                self.mask_data[index] = value;
            }
        }
    }

    /// Get a pixel value from the mask
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            if index < self.mask_data.len() {
                return self.mask_data[index];
            }
        }
        0
    }
}

impl fmt::Display for MaskSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inverted {
            write!(f, "Inverted({}x{} mask)", self.width, self.height)
        } else {
            write!(f, "({}x{} mask)", self.width, self.height)
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

    #[test]
    fn test_ellipse_selection_creation() {
        let selection = Selection::ellipse(50.0, 40.0, 30.0, 20.0);

        if let Selection::Ellipse(ellipse) = selection {
            assert_eq!(ellipse.center.x, 50.0);
            assert_eq!(ellipse.center.y, 40.0);
            assert_eq!(ellipse.radius_x, 30.0);
            assert_eq!(ellipse.radius_y, 20.0);
            assert!(!ellipse.inverted);
        } else {
            panic!("Expected Ellipse selection");
        }
    }

    #[test]
    fn test_ellipse_from_points() {
        let start = Point::new(20.0, 30.0);
        let end = Point::new(80.0, 70.0);
        let selection = Selection::ellipse_from_points(start, end);

        if let Selection::Ellipse(ellipse) = selection {
            assert_eq!(ellipse.center.x, 50.0); // (20 + 80) / 2
            assert_eq!(ellipse.center.y, 50.0); // (30 + 70) / 2
            assert_eq!(ellipse.radius_x, 30.0); // |80 - 20| / 2
            assert_eq!(ellipse.radius_y, 20.0); // |70 - 30| / 2
        } else {
            panic!("Expected Ellipse selection");
        }
    }

    #[test]
    fn test_ellipse_contains_point() {
        let ellipse = EllipseSelection::new(50.0, 40.0, 30.0, 20.0);

        // Center point should be inside
        assert!(ellipse.contains_point(Point::new(50.0, 40.0)));

        // Points on the ellipse boundary (approximately)
        assert!(ellipse.contains_point(Point::new(80.0, 40.0))); // Right edge
        assert!(ellipse.contains_point(Point::new(50.0, 60.0))); // Bottom edge

        // Points clearly outside
        assert!(!ellipse.contains_point(Point::new(100.0, 40.0)));
        assert!(!ellipse.contains_point(Point::new(50.0, 100.0)));
    }

    #[test]
    fn test_ellipse_bounds_and_area() {
        let ellipse = EllipseSelection::new(50.0, 40.0, 30.0, 20.0);

        let bounds = ellipse.bounds();
        assert_eq!(bounds.x, 20.0); // center_x - radius_x
        assert_eq!(bounds.y, 20.0); // center_y - radius_y
        assert_eq!(bounds.width, 60.0); // radius_x * 2
        assert_eq!(bounds.height, 40.0); // radius_y * 2

        let expected_area = std::f32::consts::PI * 30.0 * 20.0;
        assert!((ellipse.area() - expected_area).abs() < 0.1);
    }

    #[test]
    fn test_lasso_selection_creation() {
        let points = vec![
            Point::new(10.0, 10.0),
            Point::new(50.0, 10.0),
            Point::new(30.0, 40.0),
        ];
        let selection = Selection::lasso(points.clone());

        if let Selection::Lasso(lasso) = selection {
            assert_eq!(lasso.points, points);
            assert!(!lasso.inverted);
        } else {
            panic!("Expected Lasso selection");
        }
    }

    #[test]
    fn test_lasso_contains_point() {
        let mut lasso = LassoSelection::new(vec![
            Point::new(10.0, 10.0),
            Point::new(50.0, 10.0),
            Point::new(50.0, 50.0),
            Point::new(10.0, 50.0),
        ]);
        lasso.close_path();

        // Point inside the rectangle
        assert!(lasso.contains_point(Point::new(30.0, 30.0)));

        // Point outside
        assert!(!lasso.contains_point(Point::new(5.0, 5.0)));
        assert!(!lasso.contains_point(Point::new(60.0, 60.0)));
    }

    #[test]
    fn test_lasso_bounds() {
        let lasso = LassoSelection::new(vec![
            Point::new(10.0, 20.0),
            Point::new(50.0, 10.0),
            Point::new(40.0, 60.0),
            Point::new(5.0, 30.0),
        ]);

        let bounds = lasso.bounds();
        assert_eq!(bounds.x, 5.0); // min x
        assert_eq!(bounds.y, 10.0); // min y
        assert_eq!(bounds.width, 45.0); // max_x - min_x = 50 - 5
        assert_eq!(bounds.height, 50.0); // max_y - min_y = 60 - 10
    }

    #[test]
    fn test_mask_selection_creation() {
        let mask_data = vec![255; 100]; // 10x10 mask, all selected
        let selection = Selection::mask(10, 10, mask_data.clone());

        if let Selection::Mask(mask) = selection {
            assert_eq!(mask.width, 10);
            assert_eq!(mask.height, 10);
            assert_eq!(mask.mask_data, mask_data);
            assert!(!mask.inverted);
        } else {
            panic!("Expected Mask selection");
        }
    }

    #[test]
    fn test_mask_contains_point() {
        let mut mask_data = vec![0; 100]; // 10x10 mask, initially empty
        mask_data[55] = 255; // Set pixel at (5, 5) to selected

        let mask = MaskSelection::new(10, 10, mask_data);

        // Point at selected pixel
        assert!(mask.contains_point(Point::new(5.0, 5.0)));

        // Point at unselected pixel
        assert!(!mask.contains_point(Point::new(0.0, 0.0)));

        // Point outside mask bounds
        assert!(!mask.contains_point(Point::new(15.0, 15.0)));
    }

    #[test]
    fn test_mask_bounds() {
        let mask_data = vec![255; 100];
        let mask = MaskSelection::new_with_offset(10, 10, mask_data, Point::new(20.0, 30.0));

        let bounds = mask.bounds();
        assert_eq!(bounds.x, 20.0);
        assert_eq!(bounds.y, 30.0);
        assert_eq!(bounds.width, 10.0);
        assert_eq!(bounds.height, 10.0);
    }
}
