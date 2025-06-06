//! Application framework module

use crate::{utils::logging::LogConfig, Result};
use tracing::{debug, error, info, instrument, warn};

#[cfg(not(feature = "gui"))]
use crate::PsocError;

/// Main application structure
pub struct Application {
    /// Application configuration
    config: AppConfig,
    /// Whether the application is running
    running: bool,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Debug mode enabled
    pub debug_mode: bool,
    /// Log configuration
    pub log_config: LogConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: crate::NAME.to_string(),
            version: crate::VERSION.to_string(),
            debug_mode: cfg!(debug_assertions),
            log_config: LogConfig::from_env(),
        }
    }
}

impl Application {
    /// Create a new application instance with default configuration
    #[instrument]
    pub fn new() -> Result<Self> {
        Self::with_config(AppConfig::default())
    }

    /// Create a new application instance with custom configuration
    #[instrument(skip(config))]
    pub fn with_config(config: AppConfig) -> Result<Self> {
        // Initialize logging first
        crate::utils::logging::init_logging(config.log_config.clone()).map_err(|e| {
            eprintln!("Failed to initialize logging: {}", e);
            e
        })?;

        info!(
            name = %config.name,
            version = %config.version,
            debug_mode = config.debug_mode,
            "Creating PSOC application"
        );

        let app = Self {
            config,
            running: false,
        };

        debug!("Application instance created successfully");
        Ok(app)
    }

    /// Get the application configuration
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Check if the application is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Initialize the application
    #[instrument(skip(self))]
    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing PSOC application");

        // TODO: Initialize subsystems
        self.initialize_core_systems()?;
        self.initialize_ui_systems()?;
        self.initialize_plugin_systems()?;

        info!("Application initialization completed");
        Ok(())
    }

    /// Initialize core systems
    #[instrument(skip(self))]
    fn initialize_core_systems(&self) -> Result<()> {
        debug!("Initializing core systems");

        // TODO: Initialize document management
        // TODO: Initialize image processing
        // TODO: Initialize file I/O systems

        debug!("Core systems initialized");
        Ok(())
    }

    /// Initialize UI systems
    #[instrument(skip(self))]
    fn initialize_ui_systems(&self) -> Result<()> {
        debug!("Initializing UI systems");

        // TODO: Initialize GUI framework
        // TODO: Initialize themes and styling
        // TODO: Initialize window management

        debug!("UI systems initialized");
        Ok(())
    }

    /// Initialize plugin systems
    #[instrument(skip(self))]
    fn initialize_plugin_systems(&self) -> Result<()> {
        debug!("Initializing plugin systems");

        // TODO: Initialize plugin manager
        // TODO: Load available plugins
        // TODO: Initialize scripting engines

        debug!("Plugin systems initialized");
        Ok(())
    }

    /// Run the application
    #[instrument(skip(self))]
    pub fn run(mut self) -> Result<()> {
        info!(
            name = %self.config.name,
            version = %self.config.version,
            "Starting PSOC Image Editor"
        );

        // Initialize the application
        self.initialize()?;

        // Set running state
        self.running = true;
        info!("Application is now running");

        // Start GUI application
        self.run_gui()?;

        info!("Application shutdown completed");
        Ok(())
    }

    /// Run the GUI application
    #[instrument(skip(self))]
    fn run_gui(&mut self) -> Result<()> {
        info!("Starting GUI application");

        #[cfg(feature = "gui")]
        {
            // Run the iced GUI application
            crate::ui::PsocApp::run().map_err(|e| {
                error!("GUI application failed: {}", e);
                e
            })?;
        }

        #[cfg(not(feature = "gui"))]
        {
            warn!("GUI feature is disabled, cannot run GUI application");
            return Err(PsocError::gui("GUI feature is not enabled".to_string()));
        }

        // Graceful shutdown
        #[cfg(feature = "gui")]
        self.shutdown()?;
        Ok(())
    }

    /// Shutdown the application
    #[instrument(skip(self))]
    fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down application");

        self.running = false;

        // TODO: Cleanup resources
        // TODO: Save application state
        // TODO: Close open documents

        info!("Application shutdown completed");
        Ok(())
    }
}
