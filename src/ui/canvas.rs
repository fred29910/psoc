//! Interactive 2D canvas component for image editing

use iced::{
    mouse::Cursor,
    widget::canvas::{self, Canvas, Event, Frame, Geometry, Path, Stroke, Text},
    Color, Element, Font, Length, Point, Rectangle, Size, Vector,
};
use tracing::{debug, trace};

use crate::core::Document;
use crate::rendering::AppRenderer;
use crate::ui::application::{CanvasMessage, Message};

/// Interactive canvas for image editing
#[derive(Debug)]
pub struct ImageCanvas {
    /// Canvas state
    state: CanvasState,
    /// Current image data (legacy)
    image_data: Option<ImageData>,
    /// Current document for rendering
    document: Option<Document>,
    /// Renderer for document composition
    renderer: AppRenderer,
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
    /// Whether rulers are visible
    pub show_rulers: bool,
    /// Whether grid is visible
    pub show_grid: bool,
    /// Whether guides are visible
    pub show_guides: bool,
    /// Grid size in pixels
    pub grid_size: f32,
    /// Ruler size in pixels
    pub ruler_size: f32,
    /// Horizontal guides (y-coordinates in image space)
    pub horizontal_guides: Vec<f32>,
    /// Vertical guides (x-coordinates in image space)
    pub vertical_guides: Vec<f32>,
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
                show_rulers: true,
                show_grid: false,
                show_guides: true,
                grid_size: 20.0,
                ruler_size: 20.0,
                horizontal_guides: Vec::new(),
                vertical_guides: Vec::new(),
            },
            image_data: None,
            document: None,
            renderer: AppRenderer::new(),
        }
    }

    /// Set the image data to display (legacy method)
    pub fn set_image(&mut self, image_data: ImageData) {
        debug!(
            "Setting canvas image: {}x{}",
            image_data.width, image_data.height
        );
        self.image_data = Some(image_data);
    }

    /// Set the document to display
    pub fn set_document(&mut self, document: Document) {
        debug!(
            "Setting canvas document: {}x{} with {} layers",
            document.size.width,
            document.size.height,
            document.layers.len()
        );
        self.document = Some(document);
        // Clear legacy image data when using document
        self.image_data = None;
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        debug!("Clearing canvas");
        self.image_data = None;
        self.document = None;
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.state.zoom = zoom.clamp(0.1, 10.0);
        debug!("Canvas zoom set to: {:.2}", self.state.zoom);
    }

    /// Toggle ruler visibility
    pub fn toggle_rulers(&mut self) {
        self.state.show_rulers = !self.state.show_rulers;
        debug!("Rulers visibility: {}", self.state.show_rulers);
    }

    /// Set ruler visibility
    pub fn set_rulers_visible(&mut self, visible: bool) {
        self.state.show_rulers = visible;
        debug!("Rulers visibility set to: {}", visible);
    }

    /// Toggle grid visibility
    pub fn toggle_grid(&mut self) {
        self.state.show_grid = !self.state.show_grid;
        debug!("Grid visibility: {}", self.state.show_grid);
    }

    /// Set grid visibility
    pub fn set_grid_visible(&mut self, visible: bool) {
        self.state.show_grid = visible;
        debug!("Grid visibility set to: {}", visible);
    }

    /// Set grid size
    pub fn set_grid_size(&mut self, size: f32) {
        self.state.grid_size = size.clamp(5.0, 100.0);
        debug!("Grid size set to: {:.1}", self.state.grid_size);
    }

    /// Toggle guides visibility
    pub fn toggle_guides(&mut self) {
        self.state.show_guides = !self.state.show_guides;
        debug!("Guides visibility: {}", self.state.show_guides);
    }

    /// Set guides visibility
    pub fn set_guides_visible(&mut self, visible: bool) {
        self.state.show_guides = visible;
        debug!("Guides visibility set to: {}", visible);
    }

    /// Add horizontal guide at y position (in image coordinates)
    pub fn add_horizontal_guide(&mut self, y: f32) {
        self.state.horizontal_guides.push(y);
        debug!("Added horizontal guide at y: {:.1}", y);
    }

    /// Add vertical guide at x position (in image coordinates)
    pub fn add_vertical_guide(&mut self, x: f32) {
        self.state.vertical_guides.push(x);
        debug!("Added vertical guide at x: {:.1}", x);
    }

    /// Remove horizontal guide at index
    pub fn remove_horizontal_guide(&mut self, index: usize) {
        if index < self.state.horizontal_guides.len() {
            let y = self.state.horizontal_guides.remove(index);
            debug!("Removed horizontal guide at y: {:.1}", y);
        }
    }

    /// Remove vertical guide at index
    pub fn remove_vertical_guide(&mut self, index: usize) {
        if index < self.state.vertical_guides.len() {
            let x = self.state.vertical_guides.remove(index);
            debug!("Removed vertical guide at x: {:.1}", x);
        }
    }

    /// Clear all guides
    pub fn clear_guides(&mut self) {
        self.state.horizontal_guides.clear();
        self.state.vertical_guides.clear();
        debug!("Cleared all guides");
    }

    /// Get canvas state (for testing)
    pub fn state(&self) -> &CanvasState {
        &self.state
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

        // Calculate content area (excluding rulers)
        let content_bounds = if self.state.show_rulers {
            Rectangle::new(
                Point::new(self.state.ruler_size, self.state.ruler_size),
                Size::new(
                    bounds.width - self.state.ruler_size,
                    bounds.height - self.state.ruler_size,
                ),
            )
        } else {
            bounds
        };

        // Draw background
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::from_rgb(0.2, 0.2, 0.2));

        // Draw rulers first (behind content)
        if self.state.show_rulers {
            self.draw_rulers(&mut frame, bounds, content_bounds);
        }

        // Draw grid in content area
        if self.state.show_grid {
            self.draw_grid(&mut frame, content_bounds);
        }

        // Draw document or image if available
        if let Some(ref document) = self.document {
            self.draw_document(&mut frame, content_bounds, document);
            // Draw selection overlay
            self.draw_selection(&mut frame, content_bounds, document);
            // Draw guides
            if self.state.show_guides {
                self.draw_guides(&mut frame, content_bounds, document);
            }
        } else if let Some(ref image_data) = self.image_data {
            self.draw_image(&mut frame, content_bounds, image_data);
        } else {
            // Draw placeholder
            self.draw_placeholder(&mut frame, content_bounds);
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
        let grid_size = self.state.grid_size * self.state.zoom;
        let grid_color = Color::from_rgba(1.0, 1.0, 1.0, 0.15);

        if grid_size > 5.0 {
            // Draw vertical lines
            let mut x = (self.state.pan_offset.x % grid_size) - grid_size + bounds.x;
            while x < bounds.x + bounds.width {
                if x >= bounds.x {
                    frame.stroke(
                        &Path::line(
                            Point::new(x, bounds.y),
                            Point::new(x, bounds.y + bounds.height),
                        ),
                        Stroke::default().with_width(0.5).with_color(grid_color),
                    );
                }
                x += grid_size;
            }

            // Draw horizontal lines
            let mut y = (self.state.pan_offset.y % grid_size) - grid_size + bounds.y;
            while y < bounds.y + bounds.height {
                if y >= bounds.y {
                    frame.stroke(
                        &Path::line(
                            Point::new(bounds.x, y),
                            Point::new(bounds.x + bounds.width, y),
                        ),
                        Stroke::default().with_width(0.5).with_color(grid_color),
                    );
                }
                y += grid_size;
            }
        }
    }

    /// Draw rulers on the edges of the canvas
    fn draw_rulers(&self, frame: &mut Frame, canvas_bounds: Rectangle, content_bounds: Rectangle) {
        let ruler_color = Color::from_rgb(0.3, 0.3, 0.3);
        let ruler_text_color = Color::from_rgb(0.9, 0.9, 0.9);
        let tick_color = Color::from_rgb(0.6, 0.6, 0.6);

        // Draw ruler backgrounds
        // Top ruler
        frame.fill_rectangle(
            Point::new(0.0, 0.0),
            Size::new(canvas_bounds.width, self.state.ruler_size),
            ruler_color,
        );

        // Left ruler
        frame.fill_rectangle(
            Point::new(0.0, 0.0),
            Size::new(self.state.ruler_size, canvas_bounds.height),
            ruler_color,
        );

        // Draw ruler ticks and labels
        self.draw_horizontal_ruler(frame, content_bounds, ruler_text_color, tick_color);
        self.draw_vertical_ruler(frame, content_bounds, ruler_text_color, tick_color);
    }

    /// Draw horizontal ruler (top)
    fn draw_horizontal_ruler(
        &self,
        frame: &mut Frame,
        content_bounds: Rectangle,
        text_color: Color,
        tick_color: Color,
    ) {
        let tick_spacing = 50.0 * self.state.zoom; // 50 pixels in image space
        let minor_tick_spacing = 10.0 * self.state.zoom; // 10 pixels in image space

        if tick_spacing > 5.0 {
            // Calculate starting position
            let start_x = content_bounds.x + self.state.pan_offset.x;
            let mut x = start_x - (start_x % tick_spacing);

            while x < content_bounds.x + content_bounds.width {
                if x >= content_bounds.x {
                    // Calculate image coordinate
                    let image_x =
                        (x - content_bounds.x - self.state.pan_offset.x) / self.state.zoom;

                    // Draw major tick
                    frame.stroke(
                        &Path::line(
                            Point::new(x, self.state.ruler_size - 8.0),
                            Point::new(x, self.state.ruler_size),
                        ),
                        Stroke::default().with_width(1.0).with_color(tick_color),
                    );

                    // Draw label
                    if image_x >= 0.0 {
                        let label = format!("{:.0}", image_x);
                        frame.fill_text(Text {
                            content: label,
                            position: Point::new(x + 2.0, 2.0),
                            color: text_color,
                            size: iced::Pixels(10.0),
                            font: Font::default(),
                            horizontal_alignment: iced::alignment::Horizontal::Left,
                            vertical_alignment: iced::alignment::Vertical::Top,
                            line_height: iced::widget::text::LineHeight::default(),
                            shaping: iced::widget::text::Shaping::default(),
                        });
                    }
                }

                // Draw minor ticks
                if minor_tick_spacing > 2.0 {
                    for i in 1..5 {
                        let minor_x = x + i as f32 * minor_tick_spacing;
                        if minor_x >= content_bounds.x
                            && minor_x < content_bounds.x + content_bounds.width
                        {
                            frame.stroke(
                                &Path::line(
                                    Point::new(minor_x, self.state.ruler_size - 4.0),
                                    Point::new(minor_x, self.state.ruler_size),
                                ),
                                Stroke::default().with_width(0.5).with_color(tick_color),
                            );
                        }
                    }
                }

                x += tick_spacing;
            }
        }
    }

    /// Draw vertical ruler (left)
    fn draw_vertical_ruler(
        &self,
        frame: &mut Frame,
        content_bounds: Rectangle,
        text_color: Color,
        tick_color: Color,
    ) {
        let tick_spacing = 50.0 * self.state.zoom; // 50 pixels in image space
        let minor_tick_spacing = 10.0 * self.state.zoom; // 10 pixels in image space

        if tick_spacing > 5.0 {
            // Calculate starting position
            let start_y = content_bounds.y + self.state.pan_offset.y;
            let mut y = start_y - (start_y % tick_spacing);

            while y < content_bounds.y + content_bounds.height {
                if y >= content_bounds.y {
                    // Calculate image coordinate
                    let image_y =
                        (y - content_bounds.y - self.state.pan_offset.y) / self.state.zoom;

                    // Draw major tick
                    frame.stroke(
                        &Path::line(
                            Point::new(self.state.ruler_size - 8.0, y),
                            Point::new(self.state.ruler_size, y),
                        ),
                        Stroke::default().with_width(1.0).with_color(tick_color),
                    );

                    // Draw label (rotated text is complex in iced, so we'll skip for now)
                    if image_y >= 0.0 {
                        let label = format!("{:.0}", image_y);
                        frame.fill_text(Text {
                            content: label,
                            position: Point::new(2.0, y - 6.0),
                            color: text_color,
                            size: iced::Pixels(10.0),
                            font: Font::default(),
                            horizontal_alignment: iced::alignment::Horizontal::Left,
                            vertical_alignment: iced::alignment::Vertical::Top,
                            line_height: iced::widget::text::LineHeight::default(),
                            shaping: iced::widget::text::Shaping::default(),
                        });
                    }
                }

                // Draw minor ticks
                if minor_tick_spacing > 2.0 {
                    for i in 1..5 {
                        let minor_y = y + i as f32 * minor_tick_spacing;
                        if minor_y >= content_bounds.y
                            && minor_y < content_bounds.y + content_bounds.height
                        {
                            frame.stroke(
                                &Path::line(
                                    Point::new(self.state.ruler_size - 4.0, minor_y),
                                    Point::new(self.state.ruler_size, minor_y),
                                ),
                                Stroke::default().with_width(0.5).with_color(tick_color),
                            );
                        }
                    }
                }

                y += tick_spacing;
            }
        }
    }

    /// Draw guides (reference lines)
    fn draw_guides(&self, frame: &mut Frame, bounds: Rectangle, document: &Document) {
        let guide_color = Color::from_rgba(0.0, 1.0, 1.0, 0.8); // Cyan color
        let guide_stroke = Stroke::default().with_width(1.0).with_color(guide_color);

        // Calculate document position and size
        let doc_width = document.size.width * self.state.zoom;
        let doc_height = document.size.height * self.state.zoom;
        let doc_x = (bounds.width - doc_width) / 2.0 + self.state.pan_offset.x + bounds.x;
        let doc_y = (bounds.height - doc_height) / 2.0 + self.state.pan_offset.y + bounds.y;

        // Draw horizontal guides
        for &guide_y in &self.state.horizontal_guides {
            let canvas_y = doc_y + guide_y * self.state.zoom;
            if canvas_y >= bounds.y && canvas_y <= bounds.y + bounds.height {
                frame.stroke(
                    &Path::line(
                        Point::new(bounds.x, canvas_y),
                        Point::new(bounds.x + bounds.width, canvas_y),
                    ),
                    guide_stroke,
                );
            }
        }

        // Draw vertical guides
        for &guide_x in &self.state.vertical_guides {
            let canvas_x = doc_x + guide_x * self.state.zoom;
            if canvas_x >= bounds.x && canvas_x <= bounds.x + bounds.width {
                frame.stroke(
                    &Path::line(
                        Point::new(canvas_x, bounds.y),
                        Point::new(canvas_x, bounds.y + bounds.height),
                    ),
                    guide_stroke,
                );
            }
        }
    }

    /// Draw the document with proper layer composition
    fn draw_document(&self, frame: &mut Frame, bounds: Rectangle, document: &Document) {
        // Calculate document position and size
        let doc_width = document.size.width * self.state.zoom;
        let doc_height = document.size.height * self.state.zoom;

        let doc_x = (bounds.width - doc_width) / 2.0 + self.state.pan_offset.x;
        let doc_y = (bounds.height - doc_height) / 2.0 + self.state.pan_offset.y;

        // Render the document to pixel data
        match self.renderer.render_for_display(document) {
            Ok(pixel_data) => {
                // Convert pixel data to image data for rendering
                let (width, height) = pixel_data.dimensions();
                let mut pixels = Vec::with_capacity((width * height * 4) as usize);

                for y in 0..height {
                    for x in 0..width {
                        if let Some(pixel) = pixel_data.get_pixel(x, y) {
                            pixels.push(pixel.r);
                            pixels.push(pixel.g);
                            pixels.push(pixel.b);
                            pixels.push(pixel.a);
                        } else {
                            // Transparent pixel
                            pixels.extend_from_slice(&[0, 0, 0, 0]);
                        }
                    }
                }

                let image_data = ImageData {
                    width,
                    height,
                    pixels,
                };

                // Draw the rendered image
                self.draw_rendered_image(
                    frame,
                    bounds,
                    &image_data,
                    doc_x,
                    doc_y,
                    doc_width,
                    doc_height,
                );
            }
            Err(e) => {
                debug!("Failed to render document: {}", e);
                // Fall back to placeholder
                self.draw_document_placeholder(
                    frame, bounds, document, doc_x, doc_y, doc_width, doc_height,
                );
            }
        }
    }

    /// Draw the image data (legacy method)
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

    /// Draw rendered image data with proper positioning
    #[allow(clippy::too_many_arguments)]
    fn draw_rendered_image(
        &self,
        frame: &mut Frame,
        _bounds: Rectangle,
        image_data: &ImageData,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        // For now, we'll use a simplified approach since iced canvas doesn't directly support image rendering
        // In a production implementation, we would need to:
        // 1. Convert the pixel data to a texture
        // 2. Use a custom renderer or image widget
        // 3. Or render pixel by pixel (very slow)

        // Draw image background
        frame.fill_rectangle(
            Point::new(x, y),
            Size::new(width, height),
            Color::from_rgb(0.95, 0.95, 0.95),
        );

        // Draw image border
        frame.stroke(
            &Path::rectangle(Point::new(x, y), Size::new(width, height)),
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.2, 0.2, 0.2)),
        );

        // Sample some pixels to show the image content
        let sample_size = 8.0 * self.state.zoom.clamp(0.1, 1.0);
        if sample_size >= 2.0 {
            let samples_x = (width / sample_size) as u32;
            let samples_y = (height / sample_size) as u32;

            for sy in 0..samples_y {
                for sx in 0..samples_x {
                    let pixel_x =
                        (sx * image_data.width / samples_x.max(1)).min(image_data.width - 1);
                    let pixel_y =
                        (sy * image_data.height / samples_y.max(1)).min(image_data.height - 1);

                    let pixel_index = ((pixel_y * image_data.width + pixel_x) * 4) as usize;
                    if pixel_index + 3 < image_data.pixels.len() {
                        let r = image_data.pixels[pixel_index] as f32 / 255.0;
                        let g = image_data.pixels[pixel_index + 1] as f32 / 255.0;
                        let b = image_data.pixels[pixel_index + 2] as f32 / 255.0;
                        let a = image_data.pixels[pixel_index + 3] as f32 / 255.0;

                        let sample_x = x + sx as f32 * sample_size;
                        let sample_y = y + sy as f32 * sample_size;

                        frame.fill_rectangle(
                            Point::new(sample_x, sample_y),
                            Size::new(sample_size, sample_size),
                            Color::from_rgba(r, g, b, a),
                        );
                    }
                }
            }
        }

        // Draw center indicator
        let center_x = x + width / 2.0;
        let center_y = y + height / 2.0;
        frame.fill_rectangle(
            Point::new(center_x - 2.0, center_y - 2.0),
            Size::new(4.0, 4.0),
            Color::from_rgb(1.0, 0.0, 0.0),
        );
    }

    /// Draw document placeholder when rendering fails
    #[allow(clippy::too_many_arguments)]
    fn draw_document_placeholder(
        &self,
        frame: &mut Frame,
        _bounds: Rectangle,
        document: &Document,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        // Draw document background
        frame.fill_rectangle(
            Point::new(x, y),
            Size::new(width, height),
            Color::from_rgb(0.8, 0.8, 0.9),
        );

        // Draw document border
        frame.stroke(
            &Path::rectangle(Point::new(x, y), Size::new(width, height)),
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.3, 0.3, 0.5)),
        );

        // Draw layer indicators
        let layer_height = (height / document.layers.len().max(1) as f32).min(20.0);
        for (i, layer) in document.layers.iter().enumerate() {
            let layer_y = y + i as f32 * layer_height;
            let layer_color = if layer.visible {
                Color::from_rgba(0.6, 0.8, 0.6, 0.7)
            } else {
                Color::from_rgba(0.8, 0.6, 0.6, 0.7)
            };

            frame.fill_rectangle(
                Point::new(x + 5.0, layer_y),
                Size::new(width - 10.0, layer_height - 2.0),
                layer_color,
            );
        }

        // Draw center indicator
        let center_x = x + width / 2.0;
        let center_y = y + height / 2.0;
        frame.fill_rectangle(
            Point::new(center_x - 3.0, center_y - 3.0),
            Size::new(6.0, 6.0),
            Color::from_rgb(0.0, 0.5, 1.0),
        );
    }

    /// Draw selection overlay
    fn draw_selection(&self, frame: &mut Frame, bounds: Rectangle, document: &Document) {
        // Only draw selection if there's an active selection
        if document.has_selection() {
            if let Some(selection_bounds) = document.selection_bounds() {
                // Transform selection coordinates to canvas coordinates
                let doc_width = document.size.width * self.state.zoom;
                let doc_height = document.size.height * self.state.zoom;

                let doc_x = (bounds.width - doc_width) / 2.0 + self.state.pan_offset.x;
                let doc_y = (bounds.height - doc_height) / 2.0 + self.state.pan_offset.y;

                // Convert selection bounds to canvas coordinates
                let sel_x = doc_x + selection_bounds.x * self.state.zoom;
                let sel_y = doc_y + selection_bounds.y * self.state.zoom;
                let sel_width = selection_bounds.width * self.state.zoom;
                let sel_height = selection_bounds.height * self.state.zoom;

                // Draw selection border with marching ants effect
                self.draw_marching_ants(frame, sel_x, sel_y, sel_width, sel_height);

                // Draw selection handles at corners
                self.draw_selection_handles(frame, sel_x, sel_y, sel_width, sel_height);
            }
        }
    }

    /// Draw marching ants selection border
    fn draw_marching_ants(&self, frame: &mut Frame, x: f32, y: f32, width: f32, height: f32) {
        let stroke_width = 1.0;
        let _dash_length = 8.0;

        // Create selection rectangle path
        let selection_rect = Path::rectangle(Point::new(x, y), Size::new(width, height));

        // Draw outer border (white)
        frame.stroke(
            &selection_rect,
            Stroke::default()
                .with_width(stroke_width + 2.0)
                .with_color(Color::WHITE),
        );

        // Draw inner border (black)
        // Note: iced canvas doesn't support line dash in current version
        frame.stroke(
            &selection_rect,
            Stroke::default()
                .with_width(stroke_width)
                .with_color(Color::BLACK),
        );
    }

    /// Draw selection handles at corners and edges
    fn draw_selection_handles(&self, frame: &mut Frame, x: f32, y: f32, width: f32, height: f32) {
        let handle_size = 6.0;
        let half_handle = handle_size / 2.0;

        // Define handle positions
        let handles = [
            // Corners
            (x - half_handle, y - half_handle),         // Top-left
            (x + width - half_handle, y - half_handle), // Top-right
            (x + width - half_handle, y + height - half_handle), // Bottom-right
            (x - half_handle, y + height - half_handle), // Bottom-left
            // Edges
            (x + width / 2.0 - half_handle, y - half_handle), // Top-center
            (x + width - half_handle, y + height / 2.0 - half_handle), // Right-center
            (x + width / 2.0 - half_handle, y + height - half_handle), // Bottom-center
            (x - half_handle, y + height / 2.0 - half_handle), // Left-center
        ];

        for (handle_x, handle_y) in handles.iter() {
            // Draw handle background (white)
            frame.fill_rectangle(
                Point::new(*handle_x, *handle_y),
                Size::new(handle_size, handle_size),
                Color::WHITE,
            );

            // Draw handle border (black)
            frame.stroke(
                &Path::rectangle(
                    Point::new(*handle_x, *handle_y),
                    Size::new(handle_size, handle_size),
                ),
                Stroke::default().with_width(1.0).with_color(Color::BLACK),
            );
        }
    }
}

impl Default for ImageCanvas {
    fn default() -> Self {
        Self::new()
    }
}
