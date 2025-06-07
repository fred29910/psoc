//! Preferences/Settings dialog for PSOC Image Editor

use iced::{
    widget::{
        button, checkbox, column, container, horizontal_rule, pick_list, row, scrollable, slider,
        text, text_input, Space,
    },
    Alignment, Element, Length,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::i18n::{Language, LocalizationManager};
use crate::ui::theme::{spacing, PsocTheme};

/// User preferences structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    /// Interface preferences
    pub interface: InterfacePreferences,
    /// Performance preferences
    pub performance: PerformancePreferences,
    /// Default behavior preferences
    pub defaults: DefaultPreferences,
    /// Advanced preferences
    pub advanced: AdvancedPreferences,
}

/// Interface-related preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfacePreferences {
    /// Current theme
    pub theme: PsocTheme,
    /// Current language
    pub language: Language,
    /// UI scale factor (0.5 to 2.0)
    pub ui_scale: f32,
    /// Font size (8 to 24)
    pub font_size: u16,
    /// Show tooltips
    pub show_tooltips: bool,
    /// Show rulers by default
    pub show_rulers: bool,
    /// Show grid by default
    pub show_grid: bool,
    /// Show status bar
    pub show_status_bar: bool,
}

/// Performance-related preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePreferences {
    /// Memory limit in MB (512 to 8192)
    pub memory_limit: u32,
    /// Cache size in MB (128 to 2048)
    pub cache_size: u32,
    /// Number of worker threads (1 to 16)
    pub worker_threads: u8,
    /// Enable GPU acceleration
    pub gpu_acceleration: bool,
    /// Enable multi-threading for rendering
    pub multithreaded_rendering: bool,
    /// Tile size for rendering (64 to 512)
    pub tile_size: u16,
}

/// Default behavior preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPreferences {
    /// Default tool on startup
    pub default_tool: String,
    /// Auto-save interval in minutes (0 = disabled)
    pub auto_save_interval: u16,
    /// Maximum undo history entries (10 to 1000)
    pub max_undo_history: u16,
    /// Default image format for new documents
    pub default_image_format: String,
    /// Default canvas color
    pub default_canvas_color: [f32; 4], // RGBA
    /// Confirm before closing unsaved documents
    pub confirm_close_unsaved: bool,
    /// Remember window size and position
    pub remember_window_state: bool,
}

/// Advanced preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedPreferences {
    /// Enable debug mode
    pub debug_mode: bool,
    /// Log level
    pub log_level: String,
    /// Enable experimental features
    pub experimental_features: bool,
    /// Plugin directory
    pub plugin_directory: Option<PathBuf>,
    /// Enable crash reporting
    pub crash_reporting: bool,
    /// Enable telemetry
    pub telemetry: bool,
}

impl Default for InterfacePreferences {
    fn default() -> Self {
        Self {
            theme: PsocTheme::Dark,
            language: Language::English,
            ui_scale: 1.0,
            font_size: 12,
            show_tooltips: true,
            show_rulers: true,
            show_grid: false,
            show_status_bar: true,
        }
    }
}

impl Default for PerformancePreferences {
    fn default() -> Self {
        Self {
            memory_limit: 2048,
            cache_size: 512,
            worker_threads: num_cpus::get().min(8) as u8,
            gpu_acceleration: true,
            multithreaded_rendering: true,
            tile_size: 256,
        }
    }
}

impl Default for DefaultPreferences {
    fn default() -> Self {
        Self {
            default_tool: "Select".to_string(),
            auto_save_interval: 5,
            max_undo_history: 100,
            default_image_format: "PNG".to_string(),
            default_canvas_color: [1.0, 1.0, 1.0, 1.0], // White
            confirm_close_unsaved: true,
            remember_window_state: true,
        }
    }
}

impl Default for AdvancedPreferences {
    fn default() -> Self {
        Self {
            debug_mode: cfg!(debug_assertions),
            log_level: "Info".to_string(),
            experimental_features: false,
            plugin_directory: None,
            crash_reporting: true,
            telemetry: false,
        }
    }
}

/// Preferences category for the dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferencesCategory {
    /// Interface settings
    Interface,
    /// Performance settings
    Performance,
    /// Default behavior settings
    Defaults,
    /// Advanced settings
    Advanced,
}

impl PreferencesCategory {
    /// Get all available categories
    pub fn all() -> Vec<Self> {
        vec![
            Self::Interface,
            Self::Performance,
            Self::Defaults,
            Self::Advanced,
        ]
    }

    /// Get the display name for this category
    pub fn display_name(self, localization: &LocalizationManager) -> String {
        match self {
            Self::Interface => localization.translate("preferences-category-interface"),
            Self::Performance => localization.translate("preferences-category-performance"),
            Self::Defaults => localization.translate("preferences-category-defaults"),
            Self::Advanced => localization.translate("preferences-category-advanced"),
        }
    }
}

/// Messages for the preferences dialog
#[derive(Debug, Clone)]
pub enum PreferencesMessage {
    /// Show the preferences dialog
    Show,
    /// Hide the preferences dialog
    Hide,
    /// Change the selected category
    CategoryChanged(PreferencesCategory),
    /// Interface preference changes
    InterfaceChanged(InterfaceMessage),
    /// Performance preference changes
    PerformanceChanged(PerformanceMessage),
    /// Default preference changes
    DefaultsChanged(DefaultsMessage),
    /// Advanced preference changes
    AdvancedChanged(AdvancedMessage),
    /// Apply changes
    Apply,
    /// Reset to defaults
    Reset,
    /// Cancel changes
    Cancel,
}

/// Interface preference messages
#[derive(Debug, Clone)]
pub enum InterfaceMessage {
    /// Theme changed
    ThemeChanged(PsocTheme),
    /// Language changed
    LanguageChanged(Language),
    /// UI scale changed
    UiScaleChanged(f32),
    /// Font size changed
    FontSizeChanged(u16),
    /// Show tooltips toggled
    ShowTooltipsToggled(bool),
    /// Show rulers toggled
    ShowRulersToggled(bool),
    /// Show grid toggled
    ShowGridToggled(bool),
    /// Show status bar toggled
    ShowStatusBarToggled(bool),
}

/// Performance preference messages
#[derive(Debug, Clone)]
pub enum PerformanceMessage {
    /// Memory limit changed
    MemoryLimitChanged(u32),
    /// Cache size changed
    CacheSizeChanged(u32),
    /// Worker threads changed
    WorkerThreadsChanged(u8),
    /// GPU acceleration toggled
    GpuAccelerationToggled(bool),
    /// Multithreaded rendering toggled
    MultithreadedRenderingToggled(bool),
    /// Tile size changed
    TileSizeChanged(u16),
}

/// Default preference messages
#[derive(Debug, Clone)]
pub enum DefaultsMessage {
    /// Default tool changed
    DefaultToolChanged(String),
    /// Auto-save interval changed
    AutoSaveIntervalChanged(u16),
    /// Max undo history changed
    MaxUndoHistoryChanged(u16),
    /// Default image format changed
    DefaultImageFormatChanged(String),
    /// Default canvas color changed
    DefaultCanvasColorChanged([f32; 4]),
    /// Confirm close unsaved toggled
    ConfirmCloseUnsavedToggled(bool),
    /// Remember window state toggled
    RememberWindowStateToggled(bool),
}

/// Advanced preference messages
#[derive(Debug, Clone)]
pub enum AdvancedMessage {
    /// Debug mode toggled
    DebugModeToggled(bool),
    /// Log level changed
    LogLevelChanged(String),
    /// Experimental features toggled
    ExperimentalFeaturesToggled(bool),
    /// Plugin directory changed
    PluginDirectoryChanged(Option<PathBuf>),
    /// Crash reporting toggled
    CrashReportingToggled(bool),
    /// Telemetry toggled
    TelemetryToggled(bool),
}

/// Preferences dialog component
#[derive(Debug, Clone)]
pub struct PreferencesDialog {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Current preferences
    pub preferences: UserPreferences,
    /// Original preferences (for cancel functionality)
    pub original_preferences: UserPreferences,
    /// Currently selected category
    pub selected_category: PreferencesCategory,
    /// Whether changes have been made
    pub has_changes: bool,
}

impl Default for PreferencesDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl PreferencesDialog {
    /// Create a new preferences dialog
    pub fn new() -> Self {
        let preferences = UserPreferences::default();
        Self {
            visible: false,
            preferences: preferences.clone(),
            original_preferences: preferences,
            selected_category: PreferencesCategory::Interface,
            has_changes: false,
        }
    }

    /// Show the dialog with current preferences
    pub fn show(&mut self, preferences: UserPreferences) {
        self.preferences = preferences.clone();
        self.original_preferences = preferences;
        self.visible = true;
        self.has_changes = false;
    }

    /// Hide the dialog
    pub fn hide(&mut self) {
        self.visible = false;
        self.has_changes = false;
    }

    /// Update the dialog with a message
    pub fn update(&mut self, message: PreferencesMessage) {
        match message {
            PreferencesMessage::Show => {
                self.visible = true;
            }
            PreferencesMessage::Hide => {
                self.hide();
            }
            PreferencesMessage::CategoryChanged(category) => {
                self.selected_category = category;
            }
            PreferencesMessage::InterfaceChanged(interface_msg) => {
                self.update_interface(interface_msg);
                self.has_changes = true;
            }
            PreferencesMessage::PerformanceChanged(performance_msg) => {
                self.update_performance(performance_msg);
                self.has_changes = true;
            }
            PreferencesMessage::DefaultsChanged(defaults_msg) => {
                self.update_defaults(defaults_msg);
                self.has_changes = true;
            }
            PreferencesMessage::AdvancedChanged(advanced_msg) => {
                self.update_advanced(advanced_msg);
                self.has_changes = true;
            }
            PreferencesMessage::Apply => {
                self.original_preferences = self.preferences.clone();
                self.has_changes = false;
            }
            PreferencesMessage::Reset => {
                self.preferences = UserPreferences::default();
                self.has_changes = true;
            }
            PreferencesMessage::Cancel => {
                self.preferences = self.original_preferences.clone();
                self.hide();
            }
        }
    }

    /// Update interface preferences
    fn update_interface(&mut self, message: InterfaceMessage) {
        match message {
            InterfaceMessage::ThemeChanged(theme) => {
                self.preferences.interface.theme = theme;
            }
            InterfaceMessage::LanguageChanged(language) => {
                self.preferences.interface.language = language;
            }
            InterfaceMessage::UiScaleChanged(scale) => {
                self.preferences.interface.ui_scale = scale.clamp(0.5, 2.0);
            }
            InterfaceMessage::FontSizeChanged(size) => {
                self.preferences.interface.font_size = size.clamp(8, 24);
            }
            InterfaceMessage::ShowTooltipsToggled(show) => {
                self.preferences.interface.show_tooltips = show;
            }
            InterfaceMessage::ShowRulersToggled(show) => {
                self.preferences.interface.show_rulers = show;
            }
            InterfaceMessage::ShowGridToggled(show) => {
                self.preferences.interface.show_grid = show;
            }
            InterfaceMessage::ShowStatusBarToggled(show) => {
                self.preferences.interface.show_status_bar = show;
            }
        }
    }

    /// Update performance preferences
    fn update_performance(&mut self, message: PerformanceMessage) {
        match message {
            PerformanceMessage::MemoryLimitChanged(limit) => {
                self.preferences.performance.memory_limit = limit.clamp(512, 8192);
            }
            PerformanceMessage::CacheSizeChanged(size) => {
                self.preferences.performance.cache_size = size.clamp(128, 2048);
            }
            PerformanceMessage::WorkerThreadsChanged(threads) => {
                self.preferences.performance.worker_threads = threads.clamp(1, 16);
            }
            PerformanceMessage::GpuAccelerationToggled(enabled) => {
                self.preferences.performance.gpu_acceleration = enabled;
            }
            PerformanceMessage::MultithreadedRenderingToggled(enabled) => {
                self.preferences.performance.multithreaded_rendering = enabled;
            }
            PerformanceMessage::TileSizeChanged(size) => {
                self.preferences.performance.tile_size = size.clamp(64, 512);
            }
        }
    }

    /// Update default preferences
    fn update_defaults(&mut self, message: DefaultsMessage) {
        match message {
            DefaultsMessage::DefaultToolChanged(tool) => {
                self.preferences.defaults.default_tool = tool;
            }
            DefaultsMessage::AutoSaveIntervalChanged(interval) => {
                self.preferences.defaults.auto_save_interval = interval.clamp(0, 60);
            }
            DefaultsMessage::MaxUndoHistoryChanged(max) => {
                self.preferences.defaults.max_undo_history = max.clamp(10, 1000);
            }
            DefaultsMessage::DefaultImageFormatChanged(format) => {
                self.preferences.defaults.default_image_format = format;
            }
            DefaultsMessage::DefaultCanvasColorChanged(color) => {
                self.preferences.defaults.default_canvas_color = color;
            }
            DefaultsMessage::ConfirmCloseUnsavedToggled(confirm) => {
                self.preferences.defaults.confirm_close_unsaved = confirm;
            }
            DefaultsMessage::RememberWindowStateToggled(remember) => {
                self.preferences.defaults.remember_window_state = remember;
            }
        }
    }

    /// Update advanced preferences
    fn update_advanced(&mut self, message: AdvancedMessage) {
        match message {
            AdvancedMessage::DebugModeToggled(debug) => {
                self.preferences.advanced.debug_mode = debug;
            }
            AdvancedMessage::LogLevelChanged(level) => {
                self.preferences.advanced.log_level = level;
            }
            AdvancedMessage::ExperimentalFeaturesToggled(experimental) => {
                self.preferences.advanced.experimental_features = experimental;
            }
            AdvancedMessage::PluginDirectoryChanged(dir) => {
                self.preferences.advanced.plugin_directory = dir;
            }
            AdvancedMessage::CrashReportingToggled(reporting) => {
                self.preferences.advanced.crash_reporting = reporting;
            }
            AdvancedMessage::TelemetryToggled(telemetry) => {
                self.preferences.advanced.telemetry = telemetry;
            }
        }
    }

    /// Create the preferences dialog view
    pub fn view<'a>(
        &'a self,
        localization: &'a LocalizationManager,
    ) -> Element<'a, PreferencesMessage> {
        if !self.visible {
            return Space::new(Length::Shrink, Length::Shrink).into();
        }

        let title = text(localization.translate("preferences-title"))
            .size(20)
            .style(text::primary);

        // Category list (left panel)
        let mut category_list = column![].spacing(spacing::SM);
        for category in PreferencesCategory::all() {
            let is_selected = category == self.selected_category;
            let button_style = if is_selected {
                button::primary
            } else {
                button::secondary
            };

            category_list = category_list.push(
                button(text(category.display_name(localization)).size(14))
                    .on_press(PreferencesMessage::CategoryChanged(category))
                    .style(button_style)
                    .width(Length::Fill),
            );
        }

        let category_panel = container(
            column![
                text(localization.translate("preferences-categories"))
                    .size(16)
                    .style(text::primary),
                Space::new(Length::Shrink, Length::Fixed(spacing::SM)),
                category_list
            ]
            .spacing(spacing::SM),
        )
        .padding(spacing::MD)
        .width(Length::Fixed(200.0));

        // Settings panel (right panel)
        let settings_panel = container(
            scrollable(self.create_settings_panel(localization)).height(Length::Fixed(400.0)),
        )
        .padding(spacing::MD)
        .width(Length::Fill);

        // Button row
        let button_row = row![
            button(text(localization.translate("preferences-reset")).size(14))
                .on_press(PreferencesMessage::Reset)
                .style(button::secondary),
            Space::new(Length::Fill, Length::Shrink),
            button(text(localization.translate("preferences-cancel")).size(14))
                .on_press(PreferencesMessage::Cancel)
                .style(button::secondary),
            button(text(localization.translate("preferences-apply")).size(14))
                .on_press(PreferencesMessage::Apply)
                .style(if self.has_changes {
                    button::primary
                } else {
                    button::secondary
                })
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center);

        // Main dialog content
        let content = column![
            title,
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            horizontal_rule(1),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            row![category_panel, settings_panel].spacing(spacing::MD),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            horizontal_rule(1),
            Space::new(Length::Shrink, Length::Fixed(spacing::SM)),
            button_row
        ]
        .spacing(spacing::SM)
        .padding(spacing::LG)
        .max_width(800);

        container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// Create the settings panel for the selected category
    fn create_settings_panel<'a>(
        &'a self,
        localization: &'a LocalizationManager,
    ) -> Element<'a, PreferencesMessage> {
        match self.selected_category {
            PreferencesCategory::Interface => self.create_interface_panel(localization),
            PreferencesCategory::Performance => self.create_performance_panel(localization),
            PreferencesCategory::Defaults => self.create_defaults_panel(localization),
            PreferencesCategory::Advanced => self.create_advanced_panel(localization),
        }
    }

    /// Create the interface settings panel
    fn create_interface_panel<'a>(
        &'a self,
        localization: &'a LocalizationManager,
    ) -> Element<'a, PreferencesMessage> {
        let interface = &self.preferences.interface;

        // Theme selection
        let theme_options = vec![PsocTheme::Dark, PsocTheme::Light, PsocTheme::HighContrast];
        let theme_picker = pick_list(theme_options, Some(interface.theme), |theme| {
            PreferencesMessage::InterfaceChanged(InterfaceMessage::ThemeChanged(theme))
        });

        // Language selection
        let language_options = vec![Language::English, Language::ChineseSimplified];
        let language_picker = pick_list(language_options, Some(interface.language), |language| {
            PreferencesMessage::InterfaceChanged(InterfaceMessage::LanguageChanged(language))
        });

        // UI Scale slider
        let ui_scale_slider = slider(0.5..=2.0, interface.ui_scale, |scale| {
            PreferencesMessage::InterfaceChanged(InterfaceMessage::UiScaleChanged(scale))
        });

        // Font size input
        let font_size_input = text_input(
            &localization.translate("preferences-font-size-placeholder"),
            &interface.font_size.to_string(),
        )
        .on_input(|input| {
            if let Ok(size) = input.parse::<u16>() {
                PreferencesMessage::InterfaceChanged(InterfaceMessage::FontSizeChanged(size))
            } else {
                PreferencesMessage::InterfaceChanged(InterfaceMessage::FontSizeChanged(
                    interface.font_size,
                ))
            }
        });

        column![
            text(localization.translate("preferences-interface-title"))
                .size(18)
                .style(text::primary),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Theme
            row![
                text(localization.translate("preferences-theme")).width(Length::Fixed(150.0)),
                theme_picker
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Language
            row![
                text(localization.translate("preferences-language")).width(Length::Fixed(150.0)),
                language_picker
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // UI Scale
            row![
                text(localization.translate("preferences-ui-scale")).width(Length::Fixed(150.0)),
                ui_scale_slider,
                text(format!("{:.1}x", interface.ui_scale)).width(Length::Fixed(50.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Font Size
            row![
                text(localization.translate("preferences-font-size")).width(Length::Fixed(150.0)),
                font_size_input.width(Length::Fixed(80.0)),
                text("px").width(Length::Fixed(30.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Checkboxes
            checkbox(
                localization.translate("preferences-show-tooltips"),
                interface.show_tooltips
            ),
            checkbox(
                localization.translate("preferences-show-rulers"),
                interface.show_rulers
            ),
            checkbox(
                localization.translate("preferences-show-grid"),
                interface.show_grid
            ),
            checkbox(
                localization.translate("preferences-show-status-bar"),
                interface.show_status_bar
            ),
        ]
        .spacing(spacing::SM)
        .into()
    }

    /// Create the performance settings panel
    fn create_performance_panel<'a>(
        &'a self,
        localization: &'a LocalizationManager,
    ) -> Element<'a, PreferencesMessage> {
        let performance = &self.preferences.performance;

        // Memory limit slider
        let memory_slider = slider(512.0..=8192.0, performance.memory_limit as f32, |limit| {
            PreferencesMessage::PerformanceChanged(PerformanceMessage::MemoryLimitChanged(
                limit as u32,
            ))
        });

        // Cache size slider
        let cache_slider = slider(128.0..=2048.0, performance.cache_size as f32, |size| {
            PreferencesMessage::PerformanceChanged(PerformanceMessage::CacheSizeChanged(
                size as u32,
            ))
        });

        // Worker threads slider
        let threads_slider = slider(1.0..=16.0, performance.worker_threads as f32, |threads| {
            PreferencesMessage::PerformanceChanged(PerformanceMessage::WorkerThreadsChanged(
                threads as u8,
            ))
        });

        // Tile size slider
        let tile_size_slider = slider(64.0..=512.0, performance.tile_size as f32, |size| {
            PreferencesMessage::PerformanceChanged(PerformanceMessage::TileSizeChanged(size as u16))
        });

        column![
            text(localization.translate("preferences-performance-title"))
                .size(18)
                .style(text::primary),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Memory Limit
            row![
                text(localization.translate("preferences-memory-limit"))
                    .width(Length::Fixed(150.0)),
                memory_slider,
                text(format!("{} MB", performance.memory_limit)).width(Length::Fixed(80.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Cache Size
            row![
                text(localization.translate("preferences-cache-size")).width(Length::Fixed(150.0)),
                cache_slider,
                text(format!("{} MB", performance.cache_size)).width(Length::Fixed(80.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Worker Threads
            row![
                text(localization.translate("preferences-worker-threads"))
                    .width(Length::Fixed(150.0)),
                threads_slider,
                text(format!("{}", performance.worker_threads)).width(Length::Fixed(80.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Tile Size
            row![
                text(localization.translate("preferences-tile-size")).width(Length::Fixed(150.0)),
                tile_size_slider,
                text(format!("{} px", performance.tile_size)).width(Length::Fixed(80.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Checkboxes
            checkbox(
                localization.translate("preferences-gpu-acceleration"),
                performance.gpu_acceleration
            ),
            checkbox(
                localization.translate("preferences-multithreaded-rendering"),
                performance.multithreaded_rendering
            ),
        ]
        .spacing(spacing::SM)
        .into()
    }

    /// Create the defaults settings panel
    fn create_defaults_panel<'a>(
        &'a self,
        localization: &'a LocalizationManager,
    ) -> Element<'a, PreferencesMessage> {
        let defaults = &self.preferences.defaults;

        // Default tool selection
        let tool_options = vec![
            "Select".to_string(),
            "Brush".to_string(),
            "Eraser".to_string(),
            "Move".to_string(),
            "Transform".to_string(),
        ];
        let tool_picker = pick_list(tool_options, Some(defaults.default_tool.clone()), |tool| {
            PreferencesMessage::DefaultsChanged(DefaultsMessage::DefaultToolChanged(tool))
        });

        // Image format selection
        let format_options = vec!["PNG".to_string(), "JPEG".to_string(), "TIFF".to_string()];
        let format_picker = pick_list(
            format_options,
            Some(defaults.default_image_format.clone()),
            |format| {
                PreferencesMessage::DefaultsChanged(DefaultsMessage::DefaultImageFormatChanged(
                    format,
                ))
            },
        );

        // Auto-save interval slider
        let auto_save_slider = slider(0.0..=60.0, defaults.auto_save_interval as f32, |interval| {
            PreferencesMessage::DefaultsChanged(DefaultsMessage::AutoSaveIntervalChanged(
                interval as u16,
            ))
        });

        // Max undo history slider
        let undo_slider = slider(10.0..=1000.0, defaults.max_undo_history as f32, |max| {
            PreferencesMessage::DefaultsChanged(DefaultsMessage::MaxUndoHistoryChanged(max as u16))
        });

        column![
            text(localization.translate("preferences-defaults-title"))
                .size(18)
                .style(text::primary),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Default Tool
            row![
                text(localization.translate("preferences-default-tool"))
                    .width(Length::Fixed(150.0)),
                tool_picker
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Default Image Format
            row![
                text(localization.translate("preferences-default-format"))
                    .width(Length::Fixed(150.0)),
                format_picker
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Auto-save Interval
            row![
                text(localization.translate("preferences-auto-save")).width(Length::Fixed(150.0)),
                auto_save_slider,
                text(if defaults.auto_save_interval == 0 {
                    localization.translate("preferences-disabled")
                } else {
                    format!("{} min", defaults.auto_save_interval)
                })
                .width(Length::Fixed(80.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Max Undo History
            row![
                text(localization.translate("preferences-max-undo")).width(Length::Fixed(150.0)),
                undo_slider,
                text(format!("{}", defaults.max_undo_history)).width(Length::Fixed(80.0))
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Checkboxes
            checkbox(
                localization.translate("preferences-confirm-close"),
                defaults.confirm_close_unsaved
            ),
            checkbox(
                localization.translate("preferences-remember-window"),
                defaults.remember_window_state
            ),
        ]
        .spacing(spacing::SM)
        .into()
    }

    /// Create the advanced settings panel
    fn create_advanced_panel<'a>(
        &'a self,
        localization: &'a LocalizationManager,
    ) -> Element<'a, PreferencesMessage> {
        let advanced = &self.preferences.advanced;

        // Log level selection
        let log_levels = vec![
            "Error".to_string(),
            "Warn".to_string(),
            "Info".to_string(),
            "Debug".to_string(),
            "Trace".to_string(),
        ];
        let log_level_picker = pick_list(log_levels, Some(advanced.log_level.clone()), |level| {
            PreferencesMessage::AdvancedChanged(AdvancedMessage::LogLevelChanged(level))
        });

        // Plugin directory input
        let plugin_dir_text = advanced
            .plugin_directory
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| localization.translate("preferences-no-plugin-dir"));

        let plugin_dir_input = text_input(
            &localization.translate("preferences-plugin-dir-placeholder"),
            &plugin_dir_text,
        )
        .on_input(|input| {
            let path = if input.is_empty() {
                None
            } else {
                Some(PathBuf::from(input))
            };
            PreferencesMessage::AdvancedChanged(AdvancedMessage::PluginDirectoryChanged(path))
        });

        column![
            text(localization.translate("preferences-advanced-title"))
                .size(18)
                .style(text::primary),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Log Level
            row![
                text(localization.translate("preferences-log-level")).width(Length::Fixed(150.0)),
                log_level_picker
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            // Plugin Directory
            row![
                text(localization.translate("preferences-plugin-dir")).width(Length::Fixed(150.0)),
                plugin_dir_input
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            Space::new(Length::Shrink, Length::Fixed(spacing::MD)),
            // Checkboxes
            checkbox(
                localization.translate("preferences-debug-mode"),
                advanced.debug_mode
            ),
            checkbox(
                localization.translate("preferences-experimental"),
                advanced.experimental_features
            ),
            checkbox(
                localization.translate("preferences-crash-reporting"),
                advanced.crash_reporting
            ),
            checkbox(
                localization.translate("preferences-telemetry"),
                advanced.telemetry
            ),
        ]
        .spacing(spacing::SM)
        .into()
    }

    /// Get the current preferences
    pub fn preferences(&self) -> &UserPreferences {
        &self.preferences
    }

    /// Check if there are unsaved changes
    pub fn has_changes(&self) -> bool {
        self.has_changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_dialog_creation() {
        let dialog = PreferencesDialog::new();
        assert!(!dialog.visible);
        assert!(!dialog.has_changes);
        assert_eq!(dialog.selected_category, PreferencesCategory::Interface);
    }

    #[test]
    fn test_preferences_dialog_show_hide() {
        let mut dialog = PreferencesDialog::new();
        let preferences = UserPreferences::default();

        dialog.show(preferences.clone());
        assert!(dialog.visible);
        assert!(!dialog.has_changes);
        assert_eq!(
            dialog.preferences.interface.theme,
            preferences.interface.theme
        );

        dialog.hide();
        assert!(!dialog.visible);
        assert!(!dialog.has_changes);
    }

    #[test]
    fn test_preferences_category_display_names() {
        let localization = LocalizationManager::new();

        let categories = PreferencesCategory::all();
        assert_eq!(categories.len(), 4);

        for category in categories {
            let display_name = category.display_name(&localization);
            assert!(!display_name.is_empty());
        }
    }

    #[test]
    fn test_user_preferences_defaults() {
        let preferences = UserPreferences::default();

        // Test interface defaults
        assert_eq!(preferences.interface.theme, PsocTheme::Dark);
        assert_eq!(preferences.interface.language, Language::English);
        assert_eq!(preferences.interface.ui_scale, 1.0);
        assert_eq!(preferences.interface.font_size, 12);
        assert!(preferences.interface.show_tooltips);
        assert!(preferences.interface.show_rulers);
        assert!(!preferences.interface.show_grid);
        assert!(preferences.interface.show_status_bar);

        // Test performance defaults
        assert_eq!(preferences.performance.memory_limit, 2048);
        assert_eq!(preferences.performance.cache_size, 512);
        assert!(preferences.performance.gpu_acceleration);
        assert!(preferences.performance.multithreaded_rendering);
        assert_eq!(preferences.performance.tile_size, 256);

        // Test defaults
        assert_eq!(preferences.defaults.default_tool, "Select");
        assert_eq!(preferences.defaults.auto_save_interval, 5);
        assert_eq!(preferences.defaults.max_undo_history, 100);
        assert_eq!(preferences.defaults.default_image_format, "PNG");
        assert!(preferences.defaults.confirm_close_unsaved);
        assert!(preferences.defaults.remember_window_state);

        // Test advanced defaults
        assert_eq!(preferences.advanced.debug_mode, cfg!(debug_assertions));
        assert_eq!(preferences.advanced.log_level, "Info");
        assert!(!preferences.advanced.experimental_features);
        assert!(preferences.advanced.crash_reporting);
        assert!(!preferences.advanced.telemetry);
    }

    #[test]
    fn test_preferences_dialog_update() {
        let mut dialog = PreferencesDialog::new();

        // Test category change
        dialog.update(PreferencesMessage::CategoryChanged(
            PreferencesCategory::Performance,
        ));
        assert_eq!(dialog.selected_category, PreferencesCategory::Performance);

        // Test interface changes
        dialog.update(PreferencesMessage::InterfaceChanged(
            InterfaceMessage::ThemeChanged(PsocTheme::Light),
        ));
        assert!(dialog.has_changes);
        assert_eq!(dialog.preferences.interface.theme, PsocTheme::Light);

        // Test apply
        dialog.update(PreferencesMessage::Apply);
        assert!(!dialog.has_changes);

        // Test reset
        dialog.update(PreferencesMessage::Reset);
        assert!(dialog.has_changes);
        assert_eq!(dialog.preferences.interface.theme, PsocTheme::Dark); // Back to default
    }

    #[test]
    fn test_interface_preferences_validation() {
        let mut dialog = PreferencesDialog::new();

        // Test UI scale clamping
        dialog.update_interface(InterfaceMessage::UiScaleChanged(3.0));
        assert_eq!(dialog.preferences.interface.ui_scale, 2.0); // Clamped to max

        dialog.update_interface(InterfaceMessage::UiScaleChanged(0.1));
        assert_eq!(dialog.preferences.interface.ui_scale, 0.5); // Clamped to min

        // Test font size clamping
        dialog.update_interface(InterfaceMessage::FontSizeChanged(50));
        assert_eq!(dialog.preferences.interface.font_size, 24); // Clamped to max

        dialog.update_interface(InterfaceMessage::FontSizeChanged(5));
        assert_eq!(dialog.preferences.interface.font_size, 8); // Clamped to min
    }

    #[test]
    fn test_performance_preferences_validation() {
        let mut dialog = PreferencesDialog::new();

        // Test memory limit clamping
        dialog.update_performance(PerformanceMessage::MemoryLimitChanged(10000));
        assert_eq!(dialog.preferences.performance.memory_limit, 8192); // Clamped to max

        dialog.update_performance(PerformanceMessage::MemoryLimitChanged(100));
        assert_eq!(dialog.preferences.performance.memory_limit, 512); // Clamped to min

        // Test worker threads clamping
        dialog.update_performance(PerformanceMessage::WorkerThreadsChanged(20));
        assert_eq!(dialog.preferences.performance.worker_threads, 16); // Clamped to max

        dialog.update_performance(PerformanceMessage::WorkerThreadsChanged(0));
        assert_eq!(dialog.preferences.performance.worker_threads, 1); // Clamped to min
    }

    #[test]
    fn test_defaults_preferences_validation() {
        let mut dialog = PreferencesDialog::new();

        // Test auto-save interval clamping
        dialog.update_defaults(DefaultsMessage::AutoSaveIntervalChanged(100));
        assert_eq!(dialog.preferences.defaults.auto_save_interval, 60); // Clamped to max

        // Test max undo history clamping
        dialog.update_defaults(DefaultsMessage::MaxUndoHistoryChanged(2000));
        assert_eq!(dialog.preferences.defaults.max_undo_history, 1000); // Clamped to max

        dialog.update_defaults(DefaultsMessage::MaxUndoHistoryChanged(5));
        assert_eq!(dialog.preferences.defaults.max_undo_history, 10); // Clamped to min
    }
}
