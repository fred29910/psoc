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
    dialogs::{
        AboutDialog, AboutMessage, BrightnessContrastDialog, BrightnessContrastMessage,
        GaussianBlurDialog, GaussianBlurMessage,
    },
    icons::Icon,
    theme::{spacing, PsocTheme},
};

use crate::{
    tools::{
        tool_trait::{ToolOption, ToolOptionType, ToolOptionValue},
        ToolManager, ToolType,
    },
    PsocError, Result,
};

use psoc_core::{Command, Document, Layer};

/// Main GUI application
#[derive(Debug, Default)]
pub struct PsocApp {
    /// Current application state
    state: AppState,
    /// Error message to display
    error_message: Option<String>,
    /// About dialog
    about_dialog: AboutDialog,
    /// Brightness/Contrast adjustment dialog
    brightness_contrast_dialog: BrightnessContrastDialog,
    /// Gaussian Blur filter dialog
    gaussian_blur_dialog: GaussianBlurDialog,
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
    /// Tool option changed
    ToolOption(ToolOptionMessage),
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
    /// Brightness/Contrast dialog messages
    BrightnessContrast(BrightnessContrastMessage),
    /// Gaussian Blur dialog messages
    GaussianBlur(GaussianBlurMessage),
    /// Layer-related messages
    Layer(LayerMessage),
    /// Undo the last operation
    Undo,
    /// Redo the last undone operation
    Redo,
    /// Adjustment-related messages
    Adjustment(AdjustmentMessage),
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

/// Tool option-specific messages
#[derive(Debug, Clone)]
pub enum ToolOptionMessage {
    /// Set a tool option value
    SetOption {
        name: String,
        value: ToolOptionValue,
    },
    /// Reset tool options to defaults
    ResetOptions,
}

/// Adjustment-specific messages
#[derive(Debug, Clone)]
pub enum AdjustmentMessage {
    /// Apply brightness adjustment
    ApplyBrightness(f32),
    /// Apply contrast adjustment
    ApplyContrast(f32),
    /// Show brightness/contrast dialog
    ShowBrightnessContrast,
    /// Apply HSL adjustment
    ApplyHsl {
        hue: f32,
        saturation: f32,
        lightness: f32,
    },
    /// Show HSL dialog
    ShowHsl,
    /// Apply grayscale adjustment
    ApplyGrayscale { method: String, opacity: f32 },
    /// Show grayscale dialog
    ShowGrayscale,
    /// Apply color balance adjustment
    ApplyColorBalance {
        shadows_cyan_red: f32,
        shadows_magenta_green: f32,
        shadows_yellow_blue: f32,
        midtones_cyan_red: f32,
        midtones_magenta_green: f32,
        midtones_yellow_blue: f32,
        highlights_cyan_red: f32,
        highlights_magenta_green: f32,
        highlights_yellow_blue: f32,
    },
    /// Show color balance dialog
    ShowColorBalance,
    /// Apply curves adjustment
    ApplyCurves {
        rgb_curve_points: Vec<(f32, f32)>,
        red_curve_points: Vec<(f32, f32)>,
        green_curve_points: Vec<(f32, f32)>,
        blue_curve_points: Vec<(f32, f32)>,
        use_individual_curves: bool,
    },
    /// Show curves dialog
    ShowCurves,
    /// Apply levels adjustment
    ApplyLevels {
        input_black: u8,
        input_white: u8,
        gamma: f32,
        output_black: u8,
        output_white: u8,
        per_channel: bool,
    },
    /// Show levels dialog
    ShowLevels,
    /// Apply Gaussian blur filter
    ApplyGaussianBlur { radius: f32, quality: f32 },
    /// Show Gaussian blur dialog
    ShowGaussianBlur,
    /// Apply motion blur filter
    ApplyMotionBlur { distance: f32, angle: f32 },
    /// Show motion blur dialog
    ShowMotionBlur,
    /// Apply unsharp mask filter
    ApplyUnsharpMask {
        amount: f32,
        radius: f32,
        threshold: u8,
    },
    /// Show unsharp mask dialog
    ShowUnsharpMask,
    /// Apply sharpen filter
    ApplySharpen { strength: f32 },
    /// Show sharpen dialog
    ShowSharpen,
    /// Apply add noise filter
    ApplyAddNoise {
        noise_type: String,
        amount: f32,
        monochromatic: bool,
        seed: u32,
    },
    /// Show add noise dialog
    ShowAddNoise,
    /// Apply reduce noise filter
    ApplyReduceNoise { strength: u8, preserve_details: f32 },
    /// Show reduce noise dialog
    ShowReduceNoise,
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
                brightness_contrast_dialog: BrightnessContrastDialog::new(),
                gaussian_blur_dialog: GaussianBlurDialog::new(),
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
            Message::ToolOption(tool_option_msg) => {
                debug!("Tool option message: {:?}", tool_option_msg);
                self.handle_tool_option_message(tool_option_msg);
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
            Message::BrightnessContrast(bc_msg) => {
                debug!("Brightness/Contrast dialog message: {:?}", bc_msg);
                self.handle_brightness_contrast_message(bc_msg);
            }
            Message::GaussianBlur(gb_msg) => {
                debug!("Gaussian Blur dialog message: {:?}", gb_msg);
                self.handle_gaussian_blur_message(gb_msg);
            }
            Message::Layer(layer_msg) => {
                debug!("Layer message: {:?}", layer_msg);
                self.handle_layer_message(layer_msg);
            }
            Message::Undo => {
                debug!("Undo requested");
                if let Some(ref mut document) = self.state.current_document {
                    match document.undo() {
                        Ok(true) => {
                            info!("Undo operation successful");
                            // Update canvas with the modified document
                            self.canvas.set_document(document.clone());
                            self.sync_canvas_state();
                            self.error_message = None;
                        }
                        Ok(false) => {
                            debug!("No operations to undo");
                            self.error_message = Some("Nothing to undo".to_string());
                        }
                        Err(e) => {
                            error!("Undo operation failed: {}", e);
                            self.error_message = Some(format!("Undo failed: {}", e));
                        }
                    }
                } else {
                    self.error_message = Some("No document open".to_string());
                }
            }
            Message::Redo => {
                debug!("Redo requested");
                if let Some(ref mut document) = self.state.current_document {
                    match document.redo() {
                        Ok(true) => {
                            info!("Redo operation successful");
                            // Update canvas with the modified document
                            self.canvas.set_document(document.clone());
                            self.sync_canvas_state();
                            self.error_message = None;
                        }
                        Ok(false) => {
                            debug!("No operations to redo");
                            self.error_message = Some("Nothing to redo".to_string());
                        }
                        Err(e) => {
                            error!("Redo operation failed: {}", e);
                            self.error_message = Some(format!("Redo failed: {}", e));
                        }
                    }
                } else {
                    self.error_message = Some("No document open".to_string());
                }
            }
            Message::Adjustment(adj_msg) => {
                debug!("Adjustment message: {:?}", adj_msg);
                self.handle_adjustment_message(adj_msg);
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

        // Layer dialogs on top if visible
        let mut layers = vec![content.into()];

        if self.about_dialog.visible {
            layers.push(self.about_dialog.view(Message::About(AboutMessage::Hide)));
        }

        if self.brightness_contrast_dialog.visible {
            layers.push(
                self.brightness_contrast_dialog
                    .view(Message::BrightnessContrast),
            );
        }

        if self.gaussian_blur_dialog.visible {
            layers.push(self.gaussian_blur_dialog.view(Message::GaussianBlur));
        }

        if layers.len() > 1 {
            iced::widget::stack(layers).into()
        } else {
            layers.into_iter().next().unwrap()
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
        use crate::tools::{
            tool_trait::{KeyModifiers, MouseButton},
            ToolEvent,
        };
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
            Message::Undo,
            Message::Redo,
            Message::Adjustment(AdjustmentMessage::ShowBrightnessContrast),
            Message::Adjustment(AdjustmentMessage::ShowHsl),
            Message::Adjustment(AdjustmentMessage::ShowGrayscale),
            Message::Adjustment(AdjustmentMessage::ShowColorBalance),
            Message::Adjustment(AdjustmentMessage::ShowCurves),
            Message::Adjustment(AdjustmentMessage::ShowLevels),
            Message::Adjustment(AdjustmentMessage::ShowGaussianBlur),
            Message::Adjustment(AdjustmentMessage::ShowUnsharpMask),
            Message::Adjustment(AdjustmentMessage::ShowAddNoise),
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
            (
                Icon::Transform,
                Message::ToolChanged(ToolType::Transform),
                self.state.current_tool == ToolType::Transform,
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
            (
                Icon::Transform,
                Message::ToolChanged(ToolType::Transform),
                self.state.current_tool == ToolType::Transform,
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

    /// Create the right panel (properties and tool options)
    fn right_panel(&self) -> Element<Message> {
        let content = vec![
            self.create_tool_options_panel(),
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
                "Zoom".to_string(),
                format!("{:.0}%", self.state.zoom_level * 100.0),
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

        components::side_panel("Properties".to_string(), content, 250.0)
    }

    /// Create the tool options panel
    fn create_tool_options_panel(&self) -> Element<'static, Message> {
        use components::ToolOptionControl;

        let tool_name = match self.state.current_tool {
            ToolType::Select => "Selection",
            ToolType::Brush => "Brush",
            ToolType::Eraser => "Eraser",
            ToolType::Move => "Move",
            ToolType::Transform => "Transform",
        };

        let options = self.tool_manager.get_active_tool_options();
        let mut controls = Vec::new();

        for option in options {
            let control = match option.option_type {
                ToolOptionType::Float { min, max } => {
                    if let Some(ToolOptionValue::Float(value)) =
                        self.tool_manager.get_tool_option(&option.name)
                    {
                        ToolOptionControl::FloatSlider {
                            label: option.display_name,
                            value,
                            min,
                            max,
                            step: (max - min) / 100.0,
                            on_change: {
                                let name = option.name.clone();
                                Box::new(move |v| {
                                    Message::ToolOption(ToolOptionMessage::SetOption {
                                        name: name.clone(),
                                        value: ToolOptionValue::Float(v),
                                    })
                                })
                            },
                        }
                    } else {
                        continue;
                    }
                }
                ToolOptionType::Int { min, max } => {
                    if let Some(ToolOptionValue::Int(value)) =
                        self.tool_manager.get_tool_option(&option.name)
                    {
                        ToolOptionControl::IntSlider {
                            label: option.display_name,
                            value,
                            min,
                            max,
                            on_change: {
                                let name = option.name.clone();
                                Box::new(move |v| {
                                    Message::ToolOption(ToolOptionMessage::SetOption {
                                        name: name.clone(),
                                        value: ToolOptionValue::Int(v),
                                    })
                                })
                            },
                        }
                    } else {
                        continue;
                    }
                }
                ToolOptionType::Color => {
                    if let Some(ToolOptionValue::Color(value)) =
                        self.tool_manager.get_tool_option(&option.name)
                    {
                        ToolOptionControl::ColorPicker {
                            label: option.display_name,
                            value,
                            on_change: {
                                let name = option.name.clone();
                                Box::new(move |v| {
                                    Message::ToolOption(ToolOptionMessage::SetOption {
                                        name: name.clone(),
                                        value: ToolOptionValue::Color(v),
                                    })
                                })
                            },
                        }
                    } else {
                        continue;
                    }
                }
                ToolOptionType::Bool => {
                    if let Some(ToolOptionValue::Bool(value)) =
                        self.tool_manager.get_tool_option(&option.name)
                    {
                        ToolOptionControl::Checkbox {
                            label: option.display_name,
                            value,
                            on_change: {
                                let name = option.name.clone();
                                Box::new(move |v| {
                                    Message::ToolOption(ToolOptionMessage::SetOption {
                                        name: name.clone(),
                                        value: ToolOptionValue::Bool(v),
                                    })
                                })
                            },
                        }
                    } else {
                        continue;
                    }
                }
                ToolOptionType::String => {
                    if let Some(ToolOptionValue::String(value)) =
                        self.tool_manager.get_tool_option(&option.name)
                    {
                        ToolOptionControl::TextInput {
                            label: option.display_name,
                            value,
                            placeholder: option.description,
                            on_change: {
                                let name = option.name.clone();
                                Box::new(move |v| {
                                    Message::ToolOption(ToolOptionMessage::SetOption {
                                        name: name.clone(),
                                        value: ToolOptionValue::String(v),
                                    })
                                })
                            },
                        }
                    } else {
                        continue;
                    }
                }
                ToolOptionType::Enum(ref enum_options) => {
                    if let Some(ToolOptionValue::String(value)) =
                        self.tool_manager.get_tool_option(&option.name)
                    {
                        ToolOptionControl::Dropdown {
                            label: option.display_name,
                            options: enum_options.clone(),
                            selected: value,
                            on_change: {
                                let name = option.name.clone();
                                Box::new(move |v| {
                                    Message::ToolOption(ToolOptionMessage::SetOption {
                                        name: name.clone(),
                                        value: ToolOptionValue::String(v),
                                    })
                                })
                            },
                        }
                    } else {
                        continue;
                    }
                }
            };
            controls.push(control);
        }

        components::tool_options_panel(tool_name.to_string(), controls)
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

    /// Handle adjustment messages
    fn handle_adjustment_message(&mut self, message: AdjustmentMessage) {
        match message {
            AdjustmentMessage::ApplyBrightness(brightness) => {
                info!("Applying brightness adjustment: {}", brightness);
                self.apply_brightness_adjustment(brightness);
            }
            AdjustmentMessage::ApplyContrast(contrast) => {
                info!("Applying contrast adjustment: {}", contrast);
                self.apply_contrast_adjustment(contrast);
            }
            AdjustmentMessage::ShowBrightnessContrast => {
                info!("Showing brightness/contrast dialog");
                self.brightness_contrast_dialog.show();
            }
            AdjustmentMessage::ApplyHsl {
                hue,
                saturation,
                lightness,
            } => {
                info!(
                    "Applying HSL adjustment: h={}, s={}, l={}",
                    hue, saturation, lightness
                );
                self.apply_hsl_adjustment(hue, saturation, lightness);
            }
            AdjustmentMessage::ShowHsl => {
                info!("Showing HSL dialog");
                // TODO: Implement HSL dialog
                self.error_message = Some("HSL dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyGrayscale { method, opacity } => {
                info!(
                    "Applying grayscale adjustment: method={}, opacity={}",
                    method, opacity
                );
                self.apply_grayscale_adjustment(method, opacity);
            }
            AdjustmentMessage::ShowGrayscale => {
                info!("Showing grayscale dialog");
                // TODO: Implement grayscale dialog
                self.error_message = Some("Grayscale dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyColorBalance {
                shadows_cyan_red,
                shadows_magenta_green,
                shadows_yellow_blue,
                midtones_cyan_red,
                midtones_magenta_green,
                midtones_yellow_blue,
                highlights_cyan_red,
                highlights_magenta_green,
                highlights_yellow_blue,
            } => {
                info!("Applying color balance adjustment");
                self.apply_color_balance_adjustment(
                    shadows_cyan_red,
                    shadows_magenta_green,
                    shadows_yellow_blue,
                    midtones_cyan_red,
                    midtones_magenta_green,
                    midtones_yellow_blue,
                    highlights_cyan_red,
                    highlights_magenta_green,
                    highlights_yellow_blue,
                );
            }
            AdjustmentMessage::ShowColorBalance => {
                info!("Showing color balance dialog");
                // TODO: Implement color balance dialog
                self.error_message = Some("Color balance dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyCurves {
                rgb_curve_points,
                red_curve_points,
                green_curve_points,
                blue_curve_points,
                use_individual_curves,
            } => {
                info!("Applying curves adjustment");
                self.apply_curves_adjustment(
                    rgb_curve_points,
                    red_curve_points,
                    green_curve_points,
                    blue_curve_points,
                    use_individual_curves,
                );
            }
            AdjustmentMessage::ShowCurves => {
                info!("Showing curves dialog");
                // TODO: Implement curves dialog
                self.error_message = Some("Curves dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyLevels {
                input_black,
                input_white,
                gamma,
                output_black,
                output_white,
                per_channel,
            } => {
                info!("Applying levels adjustment");
                self.apply_levels_adjustment(
                    input_black,
                    input_white,
                    gamma,
                    output_black,
                    output_white,
                    per_channel,
                );
            }
            AdjustmentMessage::ShowLevels => {
                info!("Showing levels dialog");
                // TODO: Implement levels dialog
                self.error_message = Some("Levels dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyGaussianBlur { radius, quality } => {
                info!(
                    "Applying Gaussian blur: radius={}, quality={}",
                    radius, quality
                );
                self.apply_gaussian_blur_filter(radius, quality);
            }
            AdjustmentMessage::ShowGaussianBlur => {
                info!("Showing Gaussian blur dialog");
                self.gaussian_blur_dialog.show();
            }
            AdjustmentMessage::ApplyMotionBlur { distance, angle } => {
                info!(
                    "Applying motion blur: distance={}, angle={}",
                    distance, angle
                );
                self.apply_motion_blur_filter(distance, angle);
            }
            AdjustmentMessage::ShowMotionBlur => {
                info!("Showing motion blur dialog");
                // TODO: Implement motion blur dialog
                self.error_message = Some("Motion blur dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyUnsharpMask {
                amount,
                radius,
                threshold,
            } => {
                info!(
                    "Applying unsharp mask: amount={}, radius={}, threshold={}",
                    amount, radius, threshold
                );
                self.apply_unsharp_mask_filter(amount, radius, threshold);
            }
            AdjustmentMessage::ShowUnsharpMask => {
                info!("Showing unsharp mask dialog");
                // TODO: Implement unsharp mask dialog
                self.error_message = Some("Unsharp mask dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplySharpen { strength } => {
                info!("Applying sharpen filter: strength={}", strength);
                self.apply_sharpen_filter(strength);
            }
            AdjustmentMessage::ShowSharpen => {
                info!("Showing sharpen dialog");
                // TODO: Implement sharpen dialog
                self.error_message = Some("Sharpen dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyAddNoise {
                noise_type,
                amount,
                monochromatic,
                seed,
            } => {
                info!(
                    "Applying add noise filter: type={}, amount={}, mono={}, seed={}",
                    noise_type, amount, monochromatic, seed
                );
                self.apply_add_noise_filter(noise_type, amount, monochromatic, seed);
            }
            AdjustmentMessage::ShowAddNoise => {
                info!("Showing add noise dialog");
                // TODO: Implement add noise dialog
                self.error_message = Some("Add noise dialog not yet implemented".to_string());
            }
            AdjustmentMessage::ApplyReduceNoise {
                strength,
                preserve_details,
            } => {
                info!(
                    "Applying reduce noise filter: strength={}, preserve={}",
                    strength, preserve_details
                );
                self.apply_reduce_noise_filter(strength, preserve_details);
            }
            AdjustmentMessage::ShowReduceNoise => {
                info!("Showing reduce noise dialog");
                // TODO: Implement reduce noise dialog
                self.error_message = Some("Reduce noise dialog not yet implemented".to_string());
            }
        }
    }

    /// Apply brightness adjustment to the current document
    fn apply_brightness_adjustment(&mut self, brightness: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({ "brightness": brightness });
                let application = AdjustmentApplication::new(
                    "brightness".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply brightness: {}", e));
                } else {
                    // Update canvas with the modified document
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply contrast adjustment to the current document
    fn apply_contrast_adjustment(&mut self, contrast: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({ "contrast": contrast });
                let application = AdjustmentApplication::new(
                    "contrast".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply contrast: {}", e));
                } else {
                    // Update canvas with the modified document
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply HSL adjustment to the current document
    fn apply_hsl_adjustment(&mut self, hue: f32, saturation: f32, lightness: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "hue": hue,
                    "saturation": saturation,
                    "lightness": lightness
                });
                let application = AdjustmentApplication::new(
                    "hsl".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply HSL adjustment: {}", e));
                } else {
                    // Update canvas with the modified document
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply grayscale adjustment to the current document
    fn apply_grayscale_adjustment(&mut self, method: String, opacity: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "method": method,
                    "opacity": opacity
                });
                let application = AdjustmentApplication::new(
                    "grayscale".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message =
                        Some(format!("Failed to apply grayscale adjustment: {}", e));
                } else {
                    // Update canvas with the modified document
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply color balance adjustment to the current document
    #[allow(clippy::too_many_arguments)]
    fn apply_color_balance_adjustment(
        &mut self,
        shadows_cyan_red: f32,
        shadows_magenta_green: f32,
        shadows_yellow_blue: f32,
        midtones_cyan_red: f32,
        midtones_magenta_green: f32,
        midtones_yellow_blue: f32,
        highlights_cyan_red: f32,
        highlights_magenta_green: f32,
        highlights_yellow_blue: f32,
    ) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "shadows_cyan_red": shadows_cyan_red,
                    "shadows_magenta_green": shadows_magenta_green,
                    "shadows_yellow_blue": shadows_yellow_blue,
                    "midtones_cyan_red": midtones_cyan_red,
                    "midtones_magenta_green": midtones_magenta_green,
                    "midtones_yellow_blue": midtones_yellow_blue,
                    "highlights_cyan_red": highlights_cyan_red,
                    "highlights_magenta_green": highlights_magenta_green,
                    "highlights_yellow_blue": highlights_yellow_blue,
                    "preserve_luminosity": true
                });
                let application = AdjustmentApplication::new(
                    "color_balance".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message =
                        Some(format!("Failed to apply color balance adjustment: {}", e));
                } else {
                    // Update canvas with the modified document
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply curves adjustment to the current document
    #[allow(clippy::too_many_arguments)]
    fn apply_curves_adjustment(
        &mut self,
        rgb_curve_points: Vec<(f32, f32)>,
        red_curve_points: Vec<(f32, f32)>,
        green_curve_points: Vec<(f32, f32)>,
        blue_curve_points: Vec<(f32, f32)>,
        use_individual_curves: bool,
    ) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "rgb_curve": {
                        "points": rgb_curve_points.iter().map(|(input, output)| {
                            serde_json::json!({"input": input, "output": output})
                        }).collect::<Vec<_>>()
                    },
                    "red_curve": {
                        "points": red_curve_points.iter().map(|(input, output)| {
                            serde_json::json!({"input": input, "output": output})
                        }).collect::<Vec<_>>()
                    },
                    "green_curve": {
                        "points": green_curve_points.iter().map(|(input, output)| {
                            serde_json::json!({"input": input, "output": output})
                        }).collect::<Vec<_>>()
                    },
                    "blue_curve": {
                        "points": blue_curve_points.iter().map(|(input, output)| {
                            serde_json::json!({"input": input, "output": output})
                        }).collect::<Vec<_>>()
                    },
                    "use_individual_curves": use_individual_curves
                });
                let application = AdjustmentApplication::new(
                    "curves".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply curves adjustment: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply levels adjustment to the current document
    #[allow(clippy::too_many_arguments)]
    fn apply_levels_adjustment(
        &mut self,
        input_black: u8,
        input_white: u8,
        gamma: f32,
        output_black: u8,
        output_white: u8,
        per_channel: bool,
    ) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "input_black": input_black,
                    "input_white": input_white,
                    "gamma": gamma,
                    "output_black": output_black,
                    "output_white": output_white,
                    "per_channel": per_channel
                });
                let application = AdjustmentApplication::new(
                    "levels".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply levels adjustment: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply Gaussian blur filter to the current document
    fn apply_gaussian_blur_filter(&mut self, radius: f32, quality: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "radius": radius,
                    "quality": quality
                });
                let application = AdjustmentApplication::new(
                    "gaussian_blur".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply Gaussian blur: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply motion blur filter to the current document
    fn apply_motion_blur_filter(&mut self, distance: f32, angle: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "distance": distance,
                    "angle": angle
                });
                let application = AdjustmentApplication::new(
                    "motion_blur".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply motion blur: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply unsharp mask filter to the current document
    fn apply_unsharp_mask_filter(&mut self, amount: f32, radius: f32, threshold: u8) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "amount": amount,
                    "radius": radius,
                    "threshold": threshold
                });
                let application = AdjustmentApplication::new(
                    "unsharp_mask".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply unsharp mask: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply sharpen filter to the current document
    fn apply_sharpen_filter(&mut self, strength: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "strength": strength
                });
                let application = AdjustmentApplication::new(
                    "sharpen".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply sharpen filter: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply add noise filter to the current document
    fn apply_add_noise_filter(
        &mut self,
        noise_type: String,
        amount: f32,
        monochromatic: bool,
        seed: u32,
    ) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "noise_type": noise_type,
                    "amount": amount,
                    "monochromatic": monochromatic,
                    "seed": seed
                });
                let application = AdjustmentApplication::new(
                    "add_noise".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message = Some(format!("Failed to apply add noise filter: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
        }
    }

    /// Apply reduce noise filter to the current document
    fn apply_reduce_noise_filter(&mut self, strength: u8, preserve_details: f32) {
        use crate::commands::ApplyAdjustmentCommand;
        use psoc_core::adjustment::{AdjustmentApplication, AdjustmentScope};

        if let Some(ref mut document) = self.state.current_document {
            if let Some(active_layer_index) = document.active_layer_index {
                let params = serde_json::json!({
                    "strength": strength,
                    "preserve_details": preserve_details
                });
                let application = AdjustmentApplication::new(
                    "reduce_noise".to_string(),
                    params,
                    AdjustmentScope::EntireLayer,
                    active_layer_index,
                );

                let command = ApplyAdjustmentCommand::new(application);
                if let Err(e) = command.execute(document) {
                    self.error_message =
                        Some(format!("Failed to apply reduce noise filter: {}", e));
                } else {
                    self.canvas.set_document(document.clone());
                    self.sync_canvas_state();
                    self.error_message = None;
                }
            } else {
                self.error_message = Some("No active layer".to_string());
            }
        } else {
            self.error_message = Some("No document open".to_string());
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

    /// Handle tool option messages
    fn handle_tool_option_message(&mut self, message: ToolOptionMessage) {
        match message {
            ToolOptionMessage::SetOption { name, value } => {
                debug!("Setting tool option: {} = {:?}", name, value);
                if let Err(e) = self.tool_manager.set_tool_option(&name, value) {
                    self.error_message = Some(format!("Failed to set tool option: {}", e));
                } else {
                    self.error_message = None;
                }
            }
            ToolOptionMessage::ResetOptions => {
                debug!("Resetting tool options to defaults");
                if let Err(e) = self.tool_manager.reset_tool_options() {
                    self.error_message = Some(format!("Failed to reset tool options: {}", e));
                } else {
                    self.error_message = None;
                }
            }
        }
    }

    /// Handle brightness/contrast dialog messages
    fn handle_brightness_contrast_message(&mut self, message: BrightnessContrastMessage) {
        match message {
            BrightnessContrastMessage::Show => {
                self.brightness_contrast_dialog.show();
            }
            BrightnessContrastMessage::Hide => {
                self.brightness_contrast_dialog.hide();
            }
            BrightnessContrastMessage::BrightnessChanged(value) => {
                self.brightness_contrast_dialog.set_brightness(value);
                if self.brightness_contrast_dialog.is_preview_enabled() {
                    self.apply_brightness_preview(value);
                }
            }
            BrightnessContrastMessage::ContrastChanged(value) => {
                self.brightness_contrast_dialog.set_contrast(value);
                if self.brightness_contrast_dialog.is_preview_enabled() {
                    self.apply_contrast_preview(value);
                }
            }
            BrightnessContrastMessage::BrightnessTextChanged(text) => {
                self.brightness_contrast_dialog
                    .update_brightness_from_text(text);
                if self.brightness_contrast_dialog.is_preview_enabled() {
                    self.apply_brightness_preview(self.brightness_contrast_dialog.brightness());
                }
            }
            BrightnessContrastMessage::ContrastTextChanged(text) => {
                self.brightness_contrast_dialog
                    .update_contrast_from_text(text);
                if self.brightness_contrast_dialog.is_preview_enabled() {
                    self.apply_contrast_preview(self.brightness_contrast_dialog.contrast());
                }
            }
            BrightnessContrastMessage::TogglePreview => {
                self.brightness_contrast_dialog.toggle_preview();
                if self.brightness_contrast_dialog.is_preview_enabled() {
                    // Apply current values as preview
                    self.apply_brightness_preview(self.brightness_contrast_dialog.brightness());
                    self.apply_contrast_preview(self.brightness_contrast_dialog.contrast());
                } else {
                    // Remove preview by resetting to original
                    self.reset_preview();
                }
            }
            BrightnessContrastMessage::Reset => {
                self.brightness_contrast_dialog.reset_values();
                if self.brightness_contrast_dialog.is_preview_enabled() {
                    self.reset_preview();
                }
            }
            BrightnessContrastMessage::Apply => {
                let brightness = self.brightness_contrast_dialog.brightness();
                let contrast = self.brightness_contrast_dialog.contrast();

                // Apply both adjustments permanently
                if brightness != 0.0 {
                    self.apply_brightness_adjustment(brightness);
                }
                if contrast != 0.0 {
                    self.apply_contrast_adjustment(contrast);
                }

                self.brightness_contrast_dialog
                    .update(BrightnessContrastMessage::Apply);
                self.brightness_contrast_dialog.hide();
            }
            BrightnessContrastMessage::Cancel => {
                self.reset_preview();
                self.brightness_contrast_dialog
                    .update(BrightnessContrastMessage::Cancel);
            }
        }
    }

    /// Apply brightness adjustment as preview (temporary)
    fn apply_brightness_preview(&mut self, _brightness: f32) {
        // TODO: Implement preview functionality
        // This would apply the adjustment temporarily without modifying the document
        debug!("Brightness preview: {}", _brightness);
    }

    /// Apply contrast adjustment as preview (temporary)
    fn apply_contrast_preview(&mut self, _contrast: f32) {
        // TODO: Implement preview functionality
        // This would apply the adjustment temporarily without modifying the document
        debug!("Contrast preview: {}", _contrast);
    }

    /// Reset preview to original state
    fn reset_preview(&mut self) {
        // TODO: Implement preview reset functionality
        // This would restore the original image state
        debug!("Resetting preview");
    }

    /// Handle Gaussian blur dialog messages
    fn handle_gaussian_blur_message(&mut self, message: GaussianBlurMessage) {
        match message {
            GaussianBlurMessage::Show => {
                self.gaussian_blur_dialog.show();
            }
            GaussianBlurMessage::Hide => {
                self.gaussian_blur_dialog.hide();
            }
            GaussianBlurMessage::RadiusChanged(value) => {
                self.gaussian_blur_dialog.set_radius(value);
                if self.gaussian_blur_dialog.preview_enabled() {
                    self.apply_gaussian_blur_preview(value, self.gaussian_blur_dialog.quality());
                }
            }
            GaussianBlurMessage::QualityChanged(value) => {
                self.gaussian_blur_dialog.set_quality(value);
                if self.gaussian_blur_dialog.preview_enabled() {
                    self.apply_gaussian_blur_preview(self.gaussian_blur_dialog.radius(), value);
                }
            }
            GaussianBlurMessage::RadiusTextChanged(text) => {
                self.gaussian_blur_dialog.set_radius_text(text);
                if self.gaussian_blur_dialog.preview_enabled() {
                    self.apply_gaussian_blur_preview(
                        self.gaussian_blur_dialog.radius(),
                        self.gaussian_blur_dialog.quality(),
                    );
                }
            }
            GaussianBlurMessage::QualityTextChanged(text) => {
                self.gaussian_blur_dialog.set_quality_text(text);
                if self.gaussian_blur_dialog.preview_enabled() {
                    self.apply_gaussian_blur_preview(
                        self.gaussian_blur_dialog.radius(),
                        self.gaussian_blur_dialog.quality(),
                    );
                }
            }
            GaussianBlurMessage::TogglePreview => {
                self.gaussian_blur_dialog.toggle_preview();
                if self.gaussian_blur_dialog.preview_enabled() {
                    // Apply current values as preview
                    self.apply_gaussian_blur_preview(
                        self.gaussian_blur_dialog.radius(),
                        self.gaussian_blur_dialog.quality(),
                    );
                } else {
                    // Remove preview by resetting to original
                    self.reset_preview();
                }
            }
            GaussianBlurMessage::Reset => {
                self.gaussian_blur_dialog.reset();
                if self.gaussian_blur_dialog.preview_enabled() {
                    self.reset_preview();
                }
            }
            GaussianBlurMessage::Apply => {
                let radius = self.gaussian_blur_dialog.radius();
                let quality = self.gaussian_blur_dialog.quality();

                // Apply Gaussian blur filter permanently
                self.apply_gaussian_blur_filter(radius, quality);

                self.gaussian_blur_dialog.update(GaussianBlurMessage::Apply);
                self.gaussian_blur_dialog.hide();
            }
            GaussianBlurMessage::Cancel => {
                self.reset_preview();
                self.gaussian_blur_dialog
                    .update(GaussianBlurMessage::Cancel);
            }
        }
    }

    /// Apply Gaussian blur as preview (temporary)
    fn apply_gaussian_blur_preview(&mut self, _radius: f32, _quality: f32) {
        // TODO: Implement preview functionality
        // This would apply the filter temporarily without modifying the document
        debug!(
            "Gaussian blur preview: radius={}, quality={}",
            _radius, _quality
        );
    }
}
