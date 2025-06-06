//! Main GUI application using iced framework

#[cfg(feature = "gui")]
use iced::{
    widget::{column, container},
    Element, Length, Settings, Task, Theme,
};
use tracing::{debug, error, info};

use super::{
    canvas::{ImageCanvas, ImageData},
    components,
    dialogs::{AboutDialog, AboutMessage},
    icons::Icon,
    theme::{spacing, PsocTheme},
};
use crate::{
    tools::{ToolManager, ToolType},
    PsocError, Result,
};
use psoc_core::{Document, Layer};

/// Main GUI application
#[derive(Debug, Default)]
pub struct PsocApp {
    /// Current application state
    state: AppState,
    /// Error message to display
    error_message: Option<String>,
    /// About dialog
    about_dialog: AboutDialog,
    /// Image canvas for rendering
    canvas: ImageCanvas,
    /// Tool manager for handling editing tools
    tool_manager: ToolManager,
}



/// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// Whether a document is open
    pub document_open: bool,
    /// Current document
    pub current_document: Option<Document>,
    /// Current image data
    pub current_image: Option<image::DynamicImage>,
    /// Current file path
    pub current_file_path: Option<std::path::PathBuf>,
    /// Current zoom level (1.0 = 100%)
    pub zoom_level: f32,
    /// Canvas pan offset
    pub pan_offset: (f32, f32),
    /// Current tool selection
    pub current_tool: ToolType,
    /// Whether the application is in debug mode
    pub debug_mode: bool,
    /// Current theme
    pub theme: PsocTheme,
    /// File manager for I/O operations
    pub file_manager: crate::file_io::FileManager,
}

// Tool types are now defined in the tools module

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
    ToolChanged(ToolType),
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Reset zoom to 100%
    ZoomReset,
    /// Canvas interaction messages
    Canvas(CanvasMessage),
    /// About dialog messages
    About(AboutMessage),
    /// Show about dialog
    ShowAbout,
    /// Layer-related messages
    Layer(LayerMessage),
    /// Error occurred
    Error(String),
}

/// Layer-specific messages
#[derive(Debug, Clone)]
pub enum LayerMessage {
    /// Add a new empty layer
    AddEmptyLayer,
    /// Add a layer from file
    AddLayerFromFile,
    /// Delete layer at index
    DeleteLayer(usize),
    /// Duplicate layer at index
    DuplicateLayer(usize),
    /// Select layer at index
    SelectLayer(usize),
    /// Toggle layer visibility
    ToggleLayerVisibility(usize),
    /// Change layer opacity
    ChangeLayerOpacity(usize, f32),
    /// Move layer up
    MoveLayerUp(usize),
    /// Move layer down
    MoveLayerDown(usize),
    /// Rename layer
    RenameLayer(usize, String),
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
            current_document: None,
            current_image: None,
            current_file_path: None,
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            current_tool: ToolType::Select,
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

        iced::run(PsocApp::title, PsocApp::update, PsocApp::view).map_err(|e| {
            error!("Failed to run GUI application: {}", e);
            PsocError::gui(format!("GUI application error: {}", e))
        })
    }
}

impl PsocApp {
    #[allow(dead_code)]
    fn new() -> (Self, Task<Message>) {
        debug!("Initializing PSOC application");
        (
            Self {
                state: AppState::default(),
                error_message: None,
                about_dialog: AboutDialog::new(),
                canvas: ImageCanvas::new(),
                tool_manager: ToolManager::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        let base_title = "PSOC Image Editor";
        if self.state.document_open {
            if let Some(ref path) = self.state.current_file_path {
                let filename = path
                    .file_name()
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

                // Create a new document with default dimensions
                let document = Document::new("Untitled".to_string(), 800, 600);
                self.state.current_document = Some(document);
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
                    async move { file_manager.import_image(&path).await },
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

                // Create document from image
                match Document::from_image("Loaded Image".to_string(), &image) {
                    Ok(document) => {
                        // Set document in canvas for proper layer rendering
                        self.canvas.set_document(document.clone());

                        self.state.current_document = Some(document);
                        self.state.current_image = Some(image);
                        self.state.document_open = true;
                        self.state.zoom_level = 1.0;
                        self.state.pan_offset = (0.0, 0.0);

                        // Sync canvas state
                        self.sync_canvas_state();

                        self.error_message = None;
                    }
                    Err(e) => {
                        self.error_message =
                            Some(format!("Failed to create document from image: {}", e));
                    }
                }
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
                            async move { file_manager.export_image(&image_clone, &path_clone).await },
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
                        async move { file_manager.export_image(&image_clone, &path_clone).await },
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

                // Update the tool manager
                if let Err(e) = self.tool_manager.set_active_tool(tool) {
                    self.error_message = Some(format!("Failed to switch tool: {}", e));
                } else {
                    self.error_message = None;
                }
            }
            Message::ZoomIn => {
                let new_zoom = (self.state.zoom_level * 1.2).min(10.0);
                debug!("Zooming in: {} -> {}", self.state.zoom_level, new_zoom);
                self.state.zoom_level = new_zoom;
                self.sync_canvas_state();
            }
            Message::ZoomOut => {
                let new_zoom = (self.state.zoom_level / 1.2).max(0.1);
                debug!("Zooming out: {} -> {}", self.state.zoom_level, new_zoom);
                self.state.zoom_level = new_zoom;
                self.sync_canvas_state();
            }
            Message::ZoomReset => {
                debug!("Resetting zoom to 100%");
                self.state.zoom_level = 1.0;
                self.sync_canvas_state();
            }
            Message::Canvas(canvas_msg) => {
                debug!("Canvas message: {:?}", canvas_msg);
                self.handle_canvas_message(canvas_msg);
            }
            Message::About(about_msg) => {
                debug!("About dialog message: {:?}", about_msg);
                self.about_dialog.update(about_msg);
            }
            Message::ShowAbout => {
                info!("Showing about dialog");
                self.about_dialog.show();
            }
            Message::Layer(layer_msg) => {
                debug!("Layer message: {:?}", layer_msg);
                self.handle_layer_message(layer_msg);
            }
            Message::Error(error) => {
                error!("Application error: {}", error);
                self.error_message = Some(error);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let main_content = column![
            self.menu_bar(),
            self.toolbar(),
            self.main_content(),
            self.status_bar(),
        ]
        .spacing(0);

        let content = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill);

        // Layer the about dialog on top if visible
        if self.about_dialog.visible {
            iced::widget::stack![
                content,
                self.about_dialog.view(Message::About(AboutMessage::Hide))
            ]
            .into()
        } else {
            content.into()
        }
    }

    #[allow(dead_code)]
    fn theme(&self) -> Theme {
        self.state.theme.to_iced_theme()
    }
}

impl PsocApp {
    /// Handle canvas-specific messages
    fn handle_canvas_message(&mut self, message: CanvasMessage) {
        use crate::tools::{ToolEvent, tool_trait::{MouseButton, KeyModifiers}};
        use psoc_core::Point;

        match message {
            CanvasMessage::MouseMoved { x, y } => {
                debug!("Mouse moved on canvas: ({}, {})", x, y);
                let event = ToolEvent::MouseMoved {
                    position: Point::new(x, y),
                    modifiers: KeyModifiers::default(),
                };
                self.handle_tool_event(event);
            }
            CanvasMessage::MousePressed { x, y } => {
                debug!("Mouse pressed on canvas: ({}, {})", x, y);
                let event = ToolEvent::MousePressed {
                    position: Point::new(x, y),
                    button: MouseButton::Left,
                    modifiers: KeyModifiers::default(),
                };
                self.handle_tool_event(event);
            }
            CanvasMessage::MouseReleased { x, y } => {
                debug!("Mouse released on canvas: ({}, {})", x, y);
                let event = ToolEvent::MouseReleased {
                    position: Point::new(x, y),
                    button: MouseButton::Left,
                    modifiers: KeyModifiers::default(),
                };
                self.handle_tool_event(event);
            }
            CanvasMessage::Scrolled { delta_x, delta_y } => {
                debug!("Canvas scrolled: ({}, {})", delta_x, delta_y);
                // Handle panning
                self.state.pan_offset.0 += delta_x;
                self.state.pan_offset.1 += delta_y;
                self.sync_canvas_state();
            }
        }
    }

    /// Synchronize application state with canvas state
    fn sync_canvas_state(&mut self) {
        self.canvas.set_zoom(self.state.zoom_level);
        self.canvas.set_pan_offset(iced::Vector::new(
            self.state.pan_offset.0,
            self.state.pan_offset.1,
        ));
    }

    /// Convert image::DynamicImage to canvas ImageData
    #[allow(dead_code)]
    fn convert_image_to_canvas_data(&self, image: &image::DynamicImage) -> ImageData {
        let rgba_image = image.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let pixels = rgba_image.into_raw();

        ImageData {
            width,
            height,
            pixels,
        }
    }

    /// Create the menu bar
    fn menu_bar(&self) -> Element<Message> {
        components::menu_bar(
            Message::NewDocument,
            Message::OpenDocument,
            Message::SaveDocument,
            Message::SaveAsDocument,
            Message::ShowAbout,
            Message::Exit,
        )
    }

    /// Create the toolbar
    fn toolbar(&self) -> Element<Message> {
        let tools = vec![
            (
                Icon::Select,
                Message::ToolChanged(ToolType::Select),
                self.state.current_tool == ToolType::Select,
            ),
            (
                Icon::Brush,
                Message::ToolChanged(ToolType::Brush),
                self.state.current_tool == ToolType::Brush,
            ),
            (
                Icon::Eraser,
                Message::ToolChanged(ToolType::Eraser),
                self.state.current_tool == ToolType::Eraser,
            ),
            (
                Icon::Move,
                Message::ToolChanged(ToolType::Move),
                self.state.current_tool == ToolType::Move,
            ),
        ];

        components::toolbar(tools, Message::ZoomIn, Message::ZoomOut, Message::ZoomReset)
    }

    /// Create the main content area
    fn main_content(&self) -> Element<Message> {
        iced::widget::row![self.left_panel(), self.canvas_area(), self.right_panel(),]
            .spacing(spacing::SM)
            .height(Length::Fill)
            .into()
    }

    /// Create the left panel (tools and layers)
    fn left_panel(&self) -> Element<Message> {
        let tools = vec![
            (
                Icon::Select,
                Message::ToolChanged(ToolType::Select),
                self.state.current_tool == ToolType::Select,
            ),
            (
                Icon::Brush,
                Message::ToolChanged(ToolType::Brush),
                self.state.current_tool == ToolType::Brush,
            ),
            (
                Icon::Eraser,
                Message::ToolChanged(ToolType::Eraser),
                self.state.current_tool == ToolType::Eraser,
            ),
            (
                Icon::Move,
                Message::ToolChanged(ToolType::Move),
                self.state.current_tool == ToolType::Move,
            ),
        ];

        let layers_content = self.create_layers_content();

        column![
            components::side_panel(
                "Tools".to_string(),
                vec![components::tool_palette(tools)],
                250.0
            ),
            column(layers_content).spacing(0),
        ]
        .spacing(spacing::SM)
        .into()
    }

    /// Create the canvas area
    fn canvas_area(&self) -> Element<Message> {
        if self.state.document_open {
            // Use the actual canvas
            self.canvas.view()
        } else {
            container(
                column![
                    iced::widget::text("No Document Open")
                        .size(24.0)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.7, 0.7, 0.7))
                        }),
                    iced::widget::text("Click 'Open' to load an image")
                        .size(16.0)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5))
                        }),
                ]
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(spacing::LG),
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
            components::property_row(
                "Current Tool".to_string(),
                self.state.current_tool.to_string(),
            ),
            components::property_row(
                "Zoom".to_string(),
                format!("{:.0}%", self.state.zoom_level * 100.0),
            ),
            components::section_header("Document".to_string()),
            components::property_row(
                "Status".to_string(),
                if self.state.document_open {
                    "Open".to_string()
                } else {
                    "None".to_string()
                },
            ),
            components::property_row(
                "Theme".to_string(),
                match self.state.theme {
                    PsocTheme::Dark => "Dark".to_string(),
                    PsocTheme::Light => "Light".to_string(),
                    PsocTheme::HighContrast => "High Contrast".to_string(),
                },
            ),
        ];

        components::side_panel("Properties".to_string(), properties_content, 250.0)
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

    /// Create the layers panel content
    fn create_layers_content(&self) -> Vec<Element<'static, Message>> {
        if let Some(ref document) = self.state.current_document {
            // Create layer data for the panel
            let layers: Vec<(String, bool, bool, Message, Message)> = document
                .layers
                .iter()
                .enumerate()
                .rev() // Display in reverse order (top to bottom in UI)
                .map(|(index, layer)| {
                    let is_selected = document.active_layer_index == Some(index);
                    (
                        layer.name.clone(),
                        layer.visible,
                        is_selected,
                        Message::Layer(LayerMessage::ToggleLayerVisibility(index)),
                        Message::Layer(LayerMessage::SelectLayer(index)),
                    )
                })
                .collect();

            let active_index = document.active_layer_index;
            let layer_count = document.layers.len();

            vec![components::layer_panel(
                layers,
                Message::Layer(LayerMessage::AddEmptyLayer),
                active_index.map(|i| Message::Layer(LayerMessage::DeleteLayer(i))),
                active_index.map(|i| Message::Layer(LayerMessage::DuplicateLayer(i))),
                active_index.and_then(|i| {
                    if i > 0 {
                        Some(Message::Layer(LayerMessage::MoveLayerUp(i)))
                    } else {
                        None
                    }
                }),
                active_index.and_then(|i| {
                    if i < layer_count - 1 {
                        Some(Message::Layer(LayerMessage::MoveLayerDown(i)))
                    } else {
                        None
                    }
                }),
            )]
        } else {
            // No document open - return empty layer panel
            vec![components::layer_panel(
                vec![],
                Message::Error("No document open".to_string()),
                None,
                None,
                None,
                None,
            )]
        }
    }

    /// Handle layer-specific messages
    fn handle_layer_message(&mut self, message: LayerMessage) {
        // Ensure we have a document to work with
        if self.state.current_document.is_none() {
            self.error_message = Some("No document open".to_string());
            return;
        }

        let document = self.state.current_document.as_mut().unwrap();

        match message {
            LayerMessage::AddEmptyLayer => {
                info!("Adding new empty layer");
                let (width, height) = document.dimensions();
                let layer_name = format!("Layer {}", document.layer_count() + 1);
                let layer = Layer::new_pixel(layer_name, width, height);
                document.add_layer(layer);

                // Set the new layer as active
                if let Err(e) = document.set_active_layer(document.layer_count() - 1) {
                    self.error_message = Some(format!("Failed to set active layer: {}", e));
                }

                // Update canvas with new document state
                self.canvas.set_document(document.clone());
            }
            LayerMessage::AddLayerFromFile => {
                info!("Adding layer from file");
                // TODO: Implement file dialog for layer import
                self.error_message = Some("Layer import from file not yet implemented".to_string());
            }
            LayerMessage::DeleteLayer(index) => {
                info!("Deleting layer at index: {}", index);
                match document.remove_layer(index) {
                    Ok(_) => {
                        // If we deleted the last layer, create a new one
                        if document.is_empty() {
                            let (width, height) = document.dimensions();
                            let layer = Layer::new_pixel("Background".to_string(), width, height);
                            document.add_layer(layer);
                            let _ = document.set_active_layer(0);
                        }

                        // Update canvas with new document state
                        self.canvas.set_document(document.clone());
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed to delete layer: {}", e));
                    }
                }
            }
            LayerMessage::DuplicateLayer(index) => {
                info!("Duplicating layer at index: {}", index);
                if let Some(layer) = document.layers.get(index) {
                    let duplicated_layer = layer.duplicate();
                    if let Err(e) = document.insert_layer(index + 1, duplicated_layer) {
                        self.error_message = Some(format!("Failed to duplicate layer: {}", e));
                    } else {
                        // Update canvas with new document state
                        self.canvas.set_document(document.clone());
                    }
                } else {
                    self.error_message = Some("Layer index out of bounds".to_string());
                }
            }
            LayerMessage::SelectLayer(index) => {
                debug!("Selecting layer at index: {}", index);
                if let Err(e) = document.set_active_layer(index) {
                    self.error_message = Some(format!("Failed to select layer: {}", e));
                }
            }
            LayerMessage::ToggleLayerVisibility(index) => {
                debug!("Toggling visibility for layer at index: {}", index);
                if let Some(layer) = document.layers.get_mut(index) {
                    layer.visible = !layer.visible;
                    document.mark_dirty();
                } else {
                    self.error_message = Some("Layer index out of bounds".to_string());
                }
            }
            LayerMessage::ChangeLayerOpacity(index, opacity) => {
                debug!(
                    "Changing opacity for layer at index: {} to {}",
                    index, opacity
                );
                if let Some(layer) = document.layers.get_mut(index) {
                    layer.opacity = opacity.clamp(0.0, 1.0);
                    document.mark_dirty();
                } else {
                    self.error_message = Some("Layer index out of bounds".to_string());
                }
            }
            LayerMessage::MoveLayerUp(index) => {
                debug!("Moving layer up from index: {}", index);
                if index > 0 && index < document.layers.len() {
                    document.layers.swap(index, index - 1);

                    // Update active layer index if necessary
                    if let Some(active_index) = document.active_layer_index {
                        if active_index == index {
                            document.active_layer_index = Some(index - 1);
                        } else if active_index == index - 1 {
                            document.active_layer_index = Some(index);
                        }
                    }

                    document.mark_dirty();
                } else {
                    self.error_message = Some("Cannot move layer up".to_string());
                }
            }
            LayerMessage::MoveLayerDown(index) => {
                debug!("Moving layer down from index: {}", index);
                if index < document.layers.len() - 1 {
                    document.layers.swap(index, index + 1);

                    // Update active layer index if necessary
                    if let Some(active_index) = document.active_layer_index {
                        if active_index == index {
                            document.active_layer_index = Some(index + 1);
                        } else if active_index == index + 1 {
                            document.active_layer_index = Some(index);
                        }
                    }

                    document.mark_dirty();
                } else {
                    self.error_message = Some("Cannot move layer down".to_string());
                }
            }
            LayerMessage::RenameLayer(index, new_name) => {
                debug!("Renaming layer at index: {} to '{}'", index, new_name);
                if let Some(layer) = document.layers.get_mut(index) {
                    layer.name = new_name;
                    document.mark_dirty();
                } else {
                    self.error_message = Some("Layer index out of bounds".to_string());
                }
            }
        }
    }

    /// Handle tool events
    fn handle_tool_event(&mut self, event: crate::tools::ToolEvent) {
        if let Some(ref mut document) = self.state.current_document {
            if let Err(e) = self.tool_manager.handle_event(event, document) {
                self.error_message = Some(format!("Tool error: {}", e));
            }
        }
    }
}
