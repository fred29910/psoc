//! Interactive 2D canvas component for image editing

use iced::{
    mouse::Cursor,
    widget::canvas::{self, Canvas, Event, Frame, Geometry, Path, Stroke},
    Color, Element, Length, Point, Rectangle, Size, Vector,
};
use tracing::{debug, trace};

use crate::ui::application::{CanvasMessage, Message};

/// Interactive canvas for image editing
#[derive(Debug)]
pub struct ImageCanvas {
    /// Canvas state
    state: CanvasState,
    /// Current image data
    image_data: Option<ImageData>,
}

/// Canvas state
#[derive(Debug, Default)]
pub struct CanvasState {
    /// Current zoom level
    pub zoom: f32,
    /// Pan offset
    pub pan_offset: Vector,
    /// Whether the canvas is being dragged
    pub is_dragging: bool,
    /// Last mouse position for drag calculations
    pub last_mouse_pos: Option<Point>,
    /// Canvas bounds
    pub bounds: Rectangle,
}

/// Image data for rendering
#[derive(Debug, Clone)]
pub struct ImageData {
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Image pixels (RGBA format)
    pub pixels: Vec<u8>,
}

impl ImageData {
    /// Create a new ImageData from raw RGBA pixels
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Get the image as an iced::widget::image::Handle
    pub fn to_image_handle(&self) -> iced::widget::image::Handle {
        iced::widget::image::Handle::from_rgba(self.width, self.height, self.pixels.clone())
    }
}

impl ImageCanvas {
    /// Create a new canvas
    pub fn new() -> Self {
        Self {
            state: CanvasState {
                zoom: 1.0,
                pan_offset: Vector::new(0.0, 0.0),
                is_dragging: false,
                last_mouse_pos: None,
                bounds: Rectangle::new(Point::new(0.0, 0.0), Size::new(0.0, 0.0)),
            },
            image_data: None,
        }
    }

    /// Set the image data to display
    pub fn set_image(&mut self, image_data: ImageData) {
        debug!(
            "Setting canvas image: {}x{}",
            image_data.width, image_data.height
        );
        self.image_data = Some(image_data);
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        debug!("Clearing canvas");
        self.image_data = None;
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.state.zoom = zoom.clamp(0.1, 10.0);
        debug!("Canvas zoom set to: {:.2}", self.state.zoom);
    }

    /// Set pan offset
    pub fn set_pan_offset(&mut self, offset: Vector) {
        self.state.pan_offset = offset;
        debug!(
            "Canvas pan offset set to: ({:.2}, {:.2})",
            offset.x, offset.y
        );
    }

    /// Get the current zoom level
    pub fn zoom(&self) -> f32 {
        self.state.zoom
    }

    /// Get the current pan offset
    pub fn pan_offset(&self) -> Vector {
        self.state.pan_offset
    }

    /// Update canvas bounds
    pub fn set_bounds(&mut self, bounds: Rectangle) {
        self.state.bounds = bounds;
    }

    /// Get canvas bounds
    pub fn bounds(&self) -> Rectangle {
        self.state.bounds
    }

    /// Convert screen coordinates to canvas coordinates
    pub fn screen_to_canvas(&self, screen_point: Point) -> Point {
        let canvas_center = Point::new(
            self.state.bounds.width / 2.0,
            self.state.bounds.height / 2.0,
        );

        Point::new(
            (screen_point.x - canvas_center.x - self.state.pan_offset.x) / self.state.zoom,
            (screen_point.y - canvas_center.y - self.state.pan_offset.y) / self.state.zoom,
        )
    }

    /// Convert canvas coordinates to screen coordinates
    pub fn canvas_to_screen(&self, canvas_point: Point) -> Point {
        let canvas_center = Point::new(
            self.state.bounds.width / 2.0,
            self.state.bounds.height / 2.0,
        );

        Point::new(
            canvas_point.x * self.state.zoom + canvas_center.x + self.state.pan_offset.x,
            canvas_point.y * self.state.zoom + canvas_center.y + self.state.pan_offset.y,
        )
    }

    /// Create the canvas widget
    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<Message> for ImageCanvas {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<Message>) {
        use iced::mouse;

        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(position) = cursor.position_in(bounds) {
                            trace!("Mouse pressed at: {:?}", position);
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::Canvas(CanvasMessage::MousePressed {
                                    x: position.x,
                                    y: position.y,
                                })),
                            );
                        }
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if let Some(position) = cursor.position_in(bounds) {
                            trace!("Mouse released at: {:?}", position);
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::Canvas(CanvasMessage::MouseReleased {
                                    x: position.x,
                                    y: position.y,
                                })),
                            );
                        }
                    }
                    mouse::Event::CursorMoved { .. } => {
                        if let Some(position) = cursor.position_in(bounds) {
                            return (
                                canvas::event::Status::Captured,
                                Some(Message::Canvas(CanvasMessage::MouseMoved {
                                    x: position.x,
                                    y: position.y,
                                })),
                            );
                        }
                    }
                    mouse::Event::WheelScrolled { delta } => {
                        match delta {
                            mouse::ScrollDelta::Lines { x, y } => {
                                trace!("Wheel scrolled: lines ({}, {})", x, y);
                                return (
                                    canvas::event::Status::Captured,
                                    Some(Message::Canvas(CanvasMessage::Scrolled {
                                        delta_x: x * 20.0, // Scale for panning
                                        delta_y: y * 20.0,
                                    })),
                                );
                            }
                            mouse::ScrollDelta::Pixels { x, y } => {
                                trace!("Wheel scrolled: pixels ({}, {})", x, y);
                                return (
                                    canvas::event::Status::Captured,
                                    Some(Message::Canvas(CanvasMessage::Scrolled {
                                        delta_x: x,
                                        delta_y: y,
                                    })),
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::Keyboard(keyboard_event) => {
                trace!("Keyboard event: {:?}", keyboard_event);
                // Handle keyboard events for shortcuts
            }
            _ => {}
        }

        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw background
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb(0.2, 0.2, 0.2));

        // Draw grid
        self.draw_grid(&mut frame, bounds);

        // Draw image if available
        if let Some(ref image_data) = self.image_data {
            self.draw_image(&mut frame, bounds, image_data);
        } else {
            // Draw placeholder
            self.draw_placeholder(&mut frame, bounds);
        }

        // Draw canvas border
        frame.stroke(
            &Path::rectangle(Point::ORIGIN, bounds.size()),
            Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
        );

        vec![frame.into_geometry()]
    }
}

impl ImageCanvas {
    /// Draw the grid background
    fn draw_grid(&self, frame: &mut Frame, bounds: Rectangle) {
        let grid_size = 20.0 * self.state.zoom;
        let grid_color = Color::from_rgba(1.0, 1.0, 1.0, 0.1);

        if grid_size > 5.0 {
            // Draw vertical lines
            let mut x = (self.state.pan_offset.x % grid_size) - grid_size;
            while x < bounds.width {
                if x >= 0.0 {
                    frame.stroke(
                        &Path::line(Point::new(x, 0.0), Point::new(x, bounds.height)),
                        Stroke::default().with_width(0.5).with_color(grid_color),
                    );
                }
                x += grid_size;
            }

            // Draw horizontal lines
            let mut y = (self.state.pan_offset.y % grid_size) - grid_size;
            while y < bounds.height {
                if y >= 0.0 {
                    frame.stroke(
                        &Path::line(Point::new(0.0, y), Point::new(bounds.width, y)),
                        Stroke::default().with_width(0.5).with_color(grid_color),
                    );
                }
                y += grid_size;
            }
        }
    }

    /// Draw the image data
    fn draw_image(&self, frame: &mut Frame, bounds: Rectangle, image_data: &ImageData) {
        // Calculate image position and size
        let image_width = image_data.width as f32 * self.state.zoom;
        let image_height = image_data.height as f32 * self.state.zoom;

        let image_x = (bounds.width - image_width) / 2.0 + self.state.pan_offset.x;
        let image_y = (bounds.height - image_height) / 2.0 + self.state.pan_offset.y;

        // For now, draw a placeholder with image info since iced canvas doesn't directly support image rendering
        // In a real implementation, we would need to use a different approach or render pixel by pixel

        // Draw image background
        frame.fill_rectangle(
            Point::new(image_x, image_y),
            Size::new(image_width, image_height),
            Color::from_rgb(0.9, 0.9, 0.9),
        );

        // Draw image border
        frame.stroke(
            &Path::rectangle(
                Point::new(image_x, image_y),
                Size::new(image_width, image_height),
            ),
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.3, 0.3, 0.3)),
        );

        // Draw a pattern to indicate this is an image
        let pattern_size = 10.0 * self.state.zoom.min(1.0);
        let pattern_color = Color::from_rgba(0.7, 0.7, 0.7, 0.5);

        let mut x = image_x;
        while x < image_x + image_width {
            let mut y = image_y;
            while y < image_y + image_height {
                if ((x - image_x) / pattern_size) as i32 % 2
                    == ((y - image_y) / pattern_size) as i32 % 2
                {
                    frame.fill_rectangle(
                        Point::new(x, y),
                        Size::new(
                            pattern_size.min(image_x + image_width - x),
                            pattern_size.min(image_y + image_height - y),
                        ),
                        pattern_color,
                    );
                }
                y += pattern_size;
            }
            x += pattern_size;
        }

        // Draw image dimensions text (simplified)
        let center_x = image_x + image_width / 2.0;
        let center_y = image_y + image_height / 2.0;

        // Draw a small indicator at the center
        frame.fill_rectangle(
            Point::new(center_x - 2.0, center_y - 2.0),
            Size::new(4.0, 4.0),
            Color::from_rgb(1.0, 0.0, 0.0),
        );
    }

    /// Draw placeholder when no image is loaded
    fn draw_placeholder(&self, frame: &mut Frame, bounds: Rectangle) {
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let size = 200.0;

        // Draw placeholder rectangle
        frame.stroke(
            &Path::rectangle(
                Point::new(center.x - size / 2.0, center.y - size / 2.0),
                Size::new(size, size),
            ),
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
        );

        // Draw diagonal lines
        frame.stroke(
            &Path::line(
                Point::new(center.x - size / 2.0, center.y - size / 2.0),
                Point::new(center.x + size / 2.0, center.y + size / 2.0),
            ),
            Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
        );

        frame.stroke(
            &Path::line(
                Point::new(center.x + size / 2.0, center.y - size / 2.0),
                Point::new(center.x - size / 2.0, center.y + size / 2.0),
            ),
            Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
        );
    }
}

impl Default for ImageCanvas {
    fn default() -> Self {
        Self::new()
    }
}
