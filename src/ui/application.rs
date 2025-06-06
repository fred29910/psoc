//! Main GUI application using iced framework

#[cfg(feature = "gui")]
use iced::{
    widget::{column, container},
    Element, Length, Settings, Theme, Task,
};
use tracing::{debug, error, info};

use crate::{PsocError, Result};
use super::{
    components,
    icons::Icon,
    theme::{PsocTheme, spacing},
};

/// Main GUI application
#[derive(Debug, Default)]
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
    /// Current image data
    pub current_image: Option<image::DynamicImage>,
    /// Current file path
    pub current_file_path: Option<std::path::PathBuf>,
    /// Current zoom level (1.0 = 100%)
    pub zoom_level: f32,
    /// Canvas pan offset
    pub pan_offset: (f32, f32),
    /// Current tool selection
    pub current_tool: Tool,
    /// Whether the application is in debug mode
    pub debug_mode: bool,
    /// Current theme
    pub theme: PsocTheme,
    /// File manager for I/O operations
    pub file_manager: crate::file_io::FileManager,
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
    /// File selected for opening
    FileSelected(std::path::PathBuf),
    /// Image loaded successfully
    ImageLoaded(image::DynamicImage),
    /// Save the current document
    SaveDocument,
    /// Save as (with file dialog)
    SaveAsDocument,
    /// File selected for saving
    SaveFileSelected(std::path::PathBuf),
    /// Image saved successfully
    ImageSaved,
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
            current_image: None,
            current_file_path: None,
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            current_tool: Tool::default(),
            debug_mode: cfg!(debug_assertions),
            theme: PsocTheme::default(),
            file_manager: crate::file_io::FileManager::new(),
        }
    }
}

impl PsocApp {


    /// Get the current application state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Run the GUI application
    pub fn run() -> Result<()> {
        info!("Starting PSOC GUI application");

        let _settings = Settings::default();

        iced::run(PsocApp::title, PsocApp::update, PsocApp::view)
            .map_err(|e| {
                error!("Failed to run GUI application: {}", e);
                PsocError::gui(format!("GUI application error: {}", e))
            })
    }
}

impl PsocApp {
    fn new() -> (Self, Task<Message>) {
        debug!("Initializing PSOC application");
        (
            Self {
                state: AppState::default(),
                error_message: None,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        let base_title = "PSOC Image Editor";
        if self.state.document_open {
            if let Some(ref path) = self.state.current_file_path {
                let filename = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("Untitled");
                format!("{} - {}", base_title, filename)
            } else {
                format!("{} - Untitled", base_title)
            }
        } else {
            base_title.to_string()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
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
                #[cfg(feature = "gui")]
                {
                    return Task::perform(
                        async {
                            rfd::AsyncFileDialog::new()
                                .add_filter("Image Files", &["png", "jpg", "jpeg"])
                                .pick_file()
                                .await
                        },
                        |file_handle| {
                            if let Some(file) = file_handle {
                                Message::FileSelected(file.path().to_path_buf())
                            } else {
                                Message::Error("No file selected".to_string())
                            }
                        },
                    );
                }
                #[cfg(not(feature = "gui"))]
                {
                    self.error_message = Some("File dialogs require GUI feature".to_string());
                }
            }
            Message::FileSelected(path) => {
                info!("File selected: {}", path.display());
                let file_manager = self.state.file_manager.clone();
                return Task::perform(
                    async move {
                        file_manager.import_image(&path).await
                    },
                    |result| match result {
                        Ok(image) => Message::ImageLoaded(image),
                        Err(e) => Message::Error(format!("Failed to load image: {}", e)),
                    },
                );
            }
            Message::ImageLoaded(image) => {
                info!(
                    width = image.width(),
                    height = image.height(),
                    "Image loaded successfully"
                );
                self.state.current_image = Some(image);
                self.state.document_open = true;
                self.state.zoom_level = 1.0;
                self.state.pan_offset = (0.0, 0.0);
                self.error_message = None;
            }
            Message::SaveDocument => {
                info!("Saving document");
                if let Some(ref image) = self.state.current_image {
                    if let Some(ref path) = self.state.current_file_path {
                        // Save to existing path
                        let file_manager = self.state.file_manager.clone();
                        let image_clone = image.clone();
                        let path_clone = path.clone();
                        return Task::perform(
                            async move {
                                file_manager.export_image(&image_clone, &path_clone).await
                            },
                            |result| match result {
                                Ok(()) => Message::ImageSaved,
                                Err(e) => Message::Error(format!("Failed to save image: {}", e)),
                            },
                        );
                    } else {
                        // No existing path, trigger Save As
                        return self.update(Message::SaveAsDocument);
                    }
                } else {
                    self.error_message = Some("No document to save".to_string());
                }
            }
            Message::SaveAsDocument => {
                info!("Save As document");
                if self.state.current_image.is_some() {
                    #[cfg(feature = "gui")]
                    {
                        return Task::perform(
                            async {
                                rfd::AsyncFileDialog::new()
                                    .add_filter("PNG Files", &["png"])
                                    .add_filter("JPEG Files", &["jpg", "jpeg"])
                                    .save_file()
                                    .await
                            },
                            |file_handle| {
                                if let Some(file) = file_handle {
                                    Message::SaveFileSelected(file.path().to_path_buf())
                                } else {
                                    Message::Error("No save location selected".to_string())
                                }
                            },
                        );
                    }
                    #[cfg(not(feature = "gui"))]
                    {
                        self.error_message = Some("File dialogs require GUI feature".to_string());
                    }
                } else {
                    self.error_message = Some("No document to save".to_string());
                }
            }
            Message::SaveFileSelected(path) => {
                info!("Save file selected: {}", path.display());
                if let Some(ref image) = self.state.current_image {
                    let file_manager = self.state.file_manager.clone();
                    let image_clone = image.clone();
                    let path_clone = path.clone();
                    self.state.current_file_path = Some(path);
                    return Task::perform(
                        async move {
                            file_manager.export_image(&image_clone, &path_clone).await
                        },
                        |result| match result {
                            Ok(()) => Message::ImageSaved,
                            Err(e) => Message::Error(format!("Failed to save image: {}", e)),
                        },
                    );
                } else {
                    self.error_message = Some("No document to save".to_string());
                }
            }
            Message::ImageSaved => {
                info!("Image saved successfully");
                self.error_message = None;
            }
            Message::Exit => {
                info!("Exiting application");
                return iced::exit();
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

        Task::none()
    }

    fn view(&self) -> Element<Message> {
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

    fn theme(&self) -> Theme {
        self.state.theme.to_iced_theme()
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
        components::menu_bar(
            Message::NewDocument,
            Message::OpenDocument,
            Message::SaveDocument,
            Message::SaveAsDocument,
            Message::Exit,
        )
    }

    /// Create the toolbar
    fn toolbar(&self) -> Element<Message> {
        let tools = vec![
            (Icon::Select, Message::ToolChanged(Tool::Select), self.state.current_tool == Tool::Select),
            (Icon::Brush, Message::ToolChanged(Tool::Brush), self.state.current_tool == Tool::Brush),
            (Icon::Eraser, Message::ToolChanged(Tool::Eraser), self.state.current_tool == Tool::Eraser),
            (Icon::Move, Message::ToolChanged(Tool::Move), self.state.current_tool == Tool::Move),
        ];

        components::toolbar(
            tools,
            Message::ZoomIn,
            Message::ZoomOut,
            Message::ZoomReset,
        )
    }

    /// Create the main content area
    fn main_content(&self) -> Element<Message> {
        iced::widget::row![
            self.left_panel(),
            self.canvas_area(),
            self.right_panel(),
        ]
        .spacing(spacing::SM)
        .height(Length::Fill)
        .into()
    }

    /// Create the left panel (tools and layers)
    fn left_panel(&self) -> Element<Message> {
        let tools = vec![
            (Icon::Select, Message::ToolChanged(Tool::Select), self.state.current_tool == Tool::Select),
            (Icon::Brush, Message::ToolChanged(Tool::Brush), self.state.current_tool == Tool::Brush),
            (Icon::Eraser, Message::ToolChanged(Tool::Eraser), self.state.current_tool == Tool::Eraser),
            (Icon::Move, Message::ToolChanged(Tool::Move), self.state.current_tool == Tool::Move),
        ];

        let layers_content = vec![
            components::layer_item(
                "Layer 1".to_string(),
                true,
                true,
                Message::Error("Layer visibility toggle not implemented".to_string()),
                Message::Error("Layer selection not implemented".to_string()),
            ),
            components::layer_item(
                "Background".to_string(),
                true,
                false,
                Message::Error("Layer visibility toggle not implemented".to_string()),
                Message::Error("Layer selection not implemented".to_string()),
            ),
        ];

        column![
            components::side_panel(
                "Tools".to_string(),
                vec![components::tool_palette(tools)],
                250.0
            ),
            components::side_panel(
                "Layers".to_string(),
                layers_content,
                250.0
            ),
        ]
        .spacing(spacing::SM)
        .into()
    }

    /// Create the canvas area
    fn canvas_area(&self) -> Element<Message> {
        if self.state.document_open {
            components::canvas_placeholder(
                self.state.zoom_level,
                self.state.pan_offset,
                &self.state.current_tool.to_string(),
            )
        } else {
            container(
                column![
                    iced::widget::text("No Document Open")
                        .size(24.0)
                        .style(|_theme| iced::widget::text::Style { color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7)) }),
                    iced::widget::text("Click 'New' to create a document")
                        .size(16.0)
                        .style(|_theme| iced::widget::text::Style { color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)) }),
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(spacing::LG)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        }
    }

    /// Create the right panel (properties)
    fn right_panel(&self) -> Element<Message> {
        let properties_content = vec![
            components::section_header("Tool Properties".to_string()),
            components::property_row("Current Tool".to_string(), self.state.current_tool.to_string()),
            components::property_row("Zoom".to_string(), format!("{:.0}%", self.state.zoom_level * 100.0)),

            components::section_header("Document".to_string()),
            components::property_row("Status".to_string(), if self.state.document_open { "Open".to_string() } else { "None".to_string() }),
            components::property_row("Theme".to_string(), match self.state.theme {
                PsocTheme::Dark => "Dark".to_string(),
                PsocTheme::Light => "Light".to_string(),
                PsocTheme::HighContrast => "High Contrast".to_string(),
            }),
        ];

        components::side_panel(
            "Properties".to_string(),
            properties_content,
            250.0
        )
    }

    /// Create the status bar
    fn status_bar(&self) -> Element<Message> {
        let status_text = if let Some(ref error) = self.error_message {
            format!("Error: {}", error)
        } else if self.state.document_open {
            "Ready".to_string()
        } else {
            "Ready - No document open".to_string()
        };

        components::status_bar(status_text, self.state.zoom_level)
    }
}
