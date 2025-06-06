//! Main GUI application using iced framework

use iced::{
    widget::{button, column, container, row, text},
    Application, Command, Element, Length, Settings, Theme,
};
use tracing::{debug, error, info, warn};

use crate::{PsocError, Result};

/// Main GUI application
#[derive(Debug)]
pub struct PsocApp {
    /// Current application state
    state: AppState,
    /// Error message to display
    error_message: Option<String>,
}

/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Whether a document is open
    pub document_open: bool,
    /// Current zoom level (1.0 = 100%)
    pub zoom_level: f32,
    /// Canvas pan offset
    pub pan_offset: (f32, f32),
    /// Current tool selection
    pub current_tool: Tool,
    /// Whether the application is in debug mode
    pub debug_mode: bool,
}

/// Available tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Selection tool
    Select,
    /// Brush tool
    Brush,
    /// Eraser tool
    Eraser,
    /// Move tool
    Move,
}

impl Default for Tool {
    fn default() -> Self {
        Self::Select
    }
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tool::Select => write!(f, "Select"),
            Tool::Brush => write!(f, "Brush"),
            Tool::Eraser => write!(f, "Eraser"),
            Tool::Move => write!(f, "Move"),
        }
    }
}

/// Messages that can be sent to the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Create a new document
    NewDocument,
    /// Open an existing document
    OpenDocument,
    /// Save the current document
    SaveDocument,
    /// Exit the application
    Exit,
    /// Change the current tool
    ToolChanged(Tool),
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Reset zoom to 100%
    ZoomReset,
    /// Canvas interaction messages
    Canvas(CanvasMessage),
    /// Error occurred
    Error(String),
}

/// Canvas-specific messages
#[derive(Debug, Clone)]
pub enum CanvasMessage {
    /// Mouse moved on canvas
    MouseMoved { x: f32, y: f32 },
    /// Mouse pressed on canvas
    MousePressed { x: f32, y: f32 },
    /// Mouse released on canvas
    MouseReleased { x: f32, y: f32 },
    /// Canvas scrolled (for zoom/pan)
    Scrolled { delta_x: f32, delta_y: f32 },
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            document_open: false,
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            current_tool: Tool::default(),
            debug_mode: cfg!(debug_assertions),
        }
    }
}

impl PsocApp {
    /// Create a new application instance
    pub fn new() -> Self {
        info!("Creating new PSOC GUI application");
        Self {
            state: AppState::default(),
            error_message: None,
        }
    }

    /// Get the current application state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Run the GUI application
    pub fn run() -> Result<()> {
        info!("Starting PSOC GUI application");

        let settings = Settings {
            id: None,
            window: iced::window::Settings {
                size: (1200, 800),
                min_size: Some((800, 600)),
                max_size: None,
                position: iced::window::Position::Centered,
                resizable: true,
                decorations: true,
                transparent: false,
                visible: true,
                level: iced::window::Level::Normal,
                icon: None,
                platform_specific: Default::default(),
            },
            flags: (),
            default_font: iced::Font::DEFAULT,
            default_text_size: 14.0,
            antialiasing: true,
            exit_on_close_request: true,
        };

        <PsocApp as Application>::run(settings).map_err(|e| {
            error!("Failed to run GUI application: {}", e);
            PsocError::gui(format!("GUI application error: {}", e))
        })
    }
}

impl Application for PsocApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        debug!("Initializing PSOC application");
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        let base_title = "PSOC Image Editor";
        if self.state.document_open {
            format!("{} - Document", base_title)
        } else {
            base_title.to_string()
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        debug!("Processing message: {:?}", message);

        match message {
            Message::NewDocument => {
                info!("Creating new document");
                self.state.document_open = true;
                self.state.zoom_level = 1.0;
                self.state.pan_offset = (0.0, 0.0);
                self.error_message = None;
            }
            Message::OpenDocument => {
                info!("Opening document");
                // TODO: Implement file dialog and document loading
                warn!("Document opening not yet implemented");
                self.error_message = Some("Document opening not yet implemented".to_string());
            }
            Message::SaveDocument => {
                info!("Saving document");
                if self.state.document_open {
                    // TODO: Implement document saving
                    warn!("Document saving not yet implemented");
                    self.error_message = Some("Document saving not yet implemented".to_string());
                } else {
                    self.error_message = Some("No document to save".to_string());
                }
            }
            Message::Exit => {
                info!("Exiting application");
                return iced::window::close();
            }
            Message::ToolChanged(tool) => {
                debug!("Tool changed to: {}", tool);
                self.state.current_tool = tool;
                self.error_message = None;
            }
            Message::ZoomIn => {
                let new_zoom = (self.state.zoom_level * 1.2).min(10.0);
                debug!("Zooming in: {} -> {}", self.state.zoom_level, new_zoom);
                self.state.zoom_level = new_zoom;
            }
            Message::ZoomOut => {
                let new_zoom = (self.state.zoom_level / 1.2).max(0.1);
                debug!("Zooming out: {} -> {}", self.state.zoom_level, new_zoom);
                self.state.zoom_level = new_zoom;
            }
            Message::ZoomReset => {
                debug!("Resetting zoom to 100%");
                self.state.zoom_level = 1.0;
            }
            Message::Canvas(canvas_msg) => {
                debug!("Canvas message: {:?}", canvas_msg);
                self.handle_canvas_message(canvas_msg);
            }
            Message::Error(error) => {
                error!("Application error: {}", error);
                self.error_message = Some(error);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = column![
            self.menu_bar(),
            self.toolbar(),
            self.main_content(),
            self.status_bar(),
        ]
        .spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

impl PsocApp {
    /// Handle canvas-specific messages
    fn handle_canvas_message(&mut self, message: CanvasMessage) {
        match message {
            CanvasMessage::MouseMoved { x, y } => {
                debug!("Mouse moved on canvas: ({}, {})", x, y);
                // TODO: Handle mouse movement for current tool
            }
            CanvasMessage::MousePressed { x, y } => {
                debug!("Mouse pressed on canvas: ({}, {})", x, y);
                // TODO: Handle mouse press for current tool
            }
            CanvasMessage::MouseReleased { x, y } => {
                debug!("Mouse released on canvas: ({}, {})", x, y);
                // TODO: Handle mouse release for current tool
            }
            CanvasMessage::Scrolled { delta_x, delta_y } => {
                debug!("Canvas scrolled: ({}, {})", delta_x, delta_y);
                // Handle panning
                self.state.pan_offset.0 += delta_x;
                self.state.pan_offset.1 += delta_y;
            }
        }
    }

    /// Create the menu bar
    fn menu_bar(&self) -> Element<Message> {
        row![
            button("New").on_press(Message::NewDocument),
            button("Open").on_press(Message::OpenDocument),
            button("Save").on_press(Message::SaveDocument),
            button("Exit").on_press(Message::Exit),
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    /// Create the toolbar
    fn toolbar(&self) -> Element<Message> {
        row![
            button("Select")
                .style(if self.state.current_tool == Tool::Select {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                })
                .on_press(Message::ToolChanged(Tool::Select)),
            button("Brush")
                .style(if self.state.current_tool == Tool::Brush {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                })
                .on_press(Message::ToolChanged(Tool::Brush)),
            button("Eraser")
                .style(if self.state.current_tool == Tool::Eraser {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                })
                .on_press(Message::ToolChanged(Tool::Eraser)),
            button("Move")
                .style(if self.state.current_tool == Tool::Move {
                    iced::theme::Button::Primary
                } else {
                    iced::theme::Button::Secondary
                })
                .on_press(Message::ToolChanged(Tool::Move)),
            text("|").size(20),
            button("Zoom In").on_press(Message::ZoomIn),
            button("Zoom Out").on_press(Message::ZoomOut),
            button("100%").on_press(Message::ZoomReset),
        ]
        .spacing(5)
        .padding(10)
        .into()
    }

    /// Create the main content area
    fn main_content(&self) -> Element<Message> {
        row![
            self.left_panel(),
            self.canvas_area(),
            self.right_panel(),
        ]
        .spacing(5)
        .height(Length::Fill)
        .into()
    }

    /// Create the left panel (tools and layers)
    fn left_panel(&self) -> Element<Message> {
        column![
            text("Tools").size(16),
            text(format!("Current: {}", self.state.current_tool)),
            text(""),
            text("Layers").size(16),
            text("Layer 1"),
            text("Background"),
        ]
        .spacing(5)
        .padding(10)
        .width(Length::Fixed(200.0))
        .into()
    }

    /// Create the canvas area
    fn canvas_area(&self) -> Element<Message> {
        if self.state.document_open {
            // For now, show a placeholder canvas area
            container(
                column![
                    text("Canvas Area").size(20),
                    text(format!("Zoom: {:.0}%", self.state.zoom_level * 100.0)),
                    text(format!("Pan: ({:.1}, {:.1})", self.state.pan_offset.0, self.state.pan_offset.1)),
                    text("Tool: ").size(14),
                    text(format!("{}", self.state.current_tool)).size(14),
                    text(""),
                    text("Canvas implementation in progress...").size(12),
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Box)
            .into()
        } else {
            // Show placeholder when no document is open
            container(
                column![
                    text("No Document Open").size(24),
                    text("Click 'New' to create a document").size(16),
                ]
                .align_items(iced::Alignment::Center)
                .spacing(10)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Box)
            .into()
        }
    }

    /// Create the right panel (properties)
    fn right_panel(&self) -> Element<Message> {
        column![
            text("Properties").size(16),
            text(format!("Tool: {}", self.state.current_tool)),
            text(""),
            text("Document").size(16),
            text(if self.state.document_open { "Open" } else { "None" }),
        ]
        .spacing(5)
        .padding(10)
        .width(Length::Fixed(200.0))
        .into()
    }

    /// Create the status bar
    fn status_bar(&self) -> Element<Message> {
        let status_text = if let Some(ref error) = self.error_message {
            format!("Error: {}", error)
        } else if self.state.document_open {
            format!("Ready - Zoom: {:.0}%", self.state.zoom_level * 100.0)
        } else {
            "Ready - No document open".to_string()
        };

        container(text(status_text))
            .padding(5)
            .width(Length::Fill)
            .style(iced::theme::Container::Box)
            .into()
    }
}
