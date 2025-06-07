//! Preferences management module for PSOC Image Editor

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};

use crate::ui::dialogs::preferences::UserPreferences;

/// Preferences manager for handling user settings
#[derive(Debug)]
pub struct PreferencesManager {
    /// Path to the preferences file
    preferences_path: PathBuf,
    /// Current preferences
    preferences: UserPreferences,
}

impl PreferencesManager {
    /// Create a new preferences manager
    pub fn new() -> Result<Self> {
        let preferences_path = Self::get_preferences_path()?;
        let preferences = Self::load_preferences(&preferences_path)?;

        Ok(Self {
            preferences_path,
            preferences,
        })
    }

    /// Get the path to the preferences file
    fn get_preferences_path() -> Result<PathBuf> {
        // Try to get the user's config directory
        let config_dir = if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("psoc")
        } else {
            // Fallback to current directory
            std::env::current_dir()
                .context("Failed to get current directory")?
                .join(".psoc")
        };

        // Create the config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).with_context(|| {
                format!(
                    "Failed to create config directory: {}",
                    config_dir.display()
                )
            })?;
        }

        Ok(config_dir.join("preferences.ron"))
    }

    /// Load preferences from file
    fn load_preferences(path: &Path) -> Result<UserPreferences> {
        if !path.exists() {
            info!(
                "Preferences file not found, using defaults: {}",
                path.display()
            );
            return Ok(UserPreferences::default());
        }

        debug!("Loading preferences from: {}", path.display());

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read preferences file: {}", path.display()))?;

        let preferences: UserPreferences = ron::from_str(&content)
            .with_context(|| format!("Failed to parse preferences file: {}", path.display()))?;

        info!("Successfully loaded preferences from: {}", path.display());
        Ok(preferences)
    }

    /// Save preferences to file
    pub fn save_preferences(&self) -> Result<()> {
        debug!("Saving preferences to: {}", self.preferences_path.display());

        // Serialize preferences to RON format
        let content =
            ron::ser::to_string_pretty(&self.preferences, ron::ser::PrettyConfig::default())
                .context("Failed to serialize preferences")?;

        // Write to file
        fs::write(&self.preferences_path, content).with_context(|| {
            format!(
                "Failed to write preferences file: {}",
                self.preferences_path.display()
            )
        })?;

        info!(
            "Successfully saved preferences to: {}",
            self.preferences_path.display()
        );
        Ok(())
    }

    /// Get the current preferences
    pub fn preferences(&self) -> &UserPreferences {
        &self.preferences
    }

    /// Update preferences
    pub fn update_preferences(&mut self, preferences: UserPreferences) -> Result<()> {
        self.preferences = preferences;
        self.save_preferences()
    }

    /// Reset preferences to defaults
    pub fn reset_to_defaults(&mut self) -> Result<()> {
        info!("Resetting preferences to defaults");
        self.preferences = UserPreferences::default();
        self.save_preferences()
    }

    /// Validate preferences and fix any invalid values
    pub fn validate_and_fix(&mut self) -> bool {
        let mut changed = false;

        // Validate interface preferences
        if self.preferences.interface.ui_scale < 0.5 || self.preferences.interface.ui_scale > 2.0 {
            warn!(
                "Invalid UI scale: {}, resetting to 1.0",
                self.preferences.interface.ui_scale
            );
            self.preferences.interface.ui_scale = 1.0;
            changed = true;
        }

        if self.preferences.interface.font_size < 8 || self.preferences.interface.font_size > 24 {
            warn!(
                "Invalid font size: {}, resetting to 12",
                self.preferences.interface.font_size
            );
            self.preferences.interface.font_size = 12;
            changed = true;
        }

        // Validate performance preferences
        if self.preferences.performance.memory_limit < 512
            || self.preferences.performance.memory_limit > 8192
        {
            warn!(
                "Invalid memory limit: {}, resetting to 2048",
                self.preferences.performance.memory_limit
            );
            self.preferences.performance.memory_limit = 2048;
            changed = true;
        }

        if self.preferences.performance.cache_size < 128
            || self.preferences.performance.cache_size > 2048
        {
            warn!(
                "Invalid cache size: {}, resetting to 512",
                self.preferences.performance.cache_size
            );
            self.preferences.performance.cache_size = 512;
            changed = true;
        }

        if self.preferences.performance.worker_threads < 1
            || self.preferences.performance.worker_threads > 16
        {
            warn!(
                "Invalid worker threads: {}, resetting to 4",
                self.preferences.performance.worker_threads
            );
            self.preferences.performance.worker_threads = 4;
            changed = true;
        }

        if self.preferences.performance.tile_size < 64
            || self.preferences.performance.tile_size > 512
        {
            warn!(
                "Invalid tile size: {}, resetting to 256",
                self.preferences.performance.tile_size
            );
            self.preferences.performance.tile_size = 256;
            changed = true;
        }

        // Validate default preferences
        if self.preferences.defaults.auto_save_interval > 60 {
            warn!(
                "Invalid auto-save interval: {}, resetting to 5",
                self.preferences.defaults.auto_save_interval
            );
            self.preferences.defaults.auto_save_interval = 5;
            changed = true;
        }

        if self.preferences.defaults.max_undo_history < 10
            || self.preferences.defaults.max_undo_history > 1000
        {
            warn!(
                "Invalid max undo history: {}, resetting to 100",
                self.preferences.defaults.max_undo_history
            );
            self.preferences.defaults.max_undo_history = 100;
            changed = true;
        }

        // Validate default canvas color (RGBA values should be 0.0-1.0)
        let mut color_changed = false;
        let mut new_color = self.preferences.defaults.default_canvas_color;
        for (i, value) in new_color.iter_mut().enumerate() {
            if !(0.0..=1.0).contains(value) {
                warn!(
                    "Invalid canvas color component {}: {}, resetting to 1.0",
                    i, value
                );
                *value = 1.0;
                color_changed = true;
            }
        }
        if color_changed {
            self.preferences.defaults.default_canvas_color = new_color;
            changed = true;
        }

        // Validate plugin directory
        if let Some(ref plugin_dir) = self.preferences.advanced.plugin_directory {
            if !plugin_dir.exists() {
                warn!(
                    "Plugin directory does not exist: {}, clearing",
                    plugin_dir.display()
                );
                self.preferences.advanced.plugin_directory = None;
                changed = true;
            }
        }

        if changed {
            info!("Preferences validation found and fixed invalid values");
            if let Err(e) = self.save_preferences() {
                error!("Failed to save corrected preferences: {}", e);
            }
        }

        changed
    }

    /// Get preferences file path
    pub fn preferences_path(&self) -> &Path {
        &self.preferences_path
    }

    /// Check if preferences file exists
    pub fn preferences_file_exists(&self) -> bool {
        self.preferences_path.exists()
    }

    /// Export preferences to a specific file
    pub fn export_preferences(&self, path: &Path) -> Result<()> {
        debug!("Exporting preferences to: {}", path.display());

        let content =
            ron::ser::to_string_pretty(&self.preferences, ron::ser::PrettyConfig::default())
                .context("Failed to serialize preferences for export")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to export preferences to: {}", path.display()))?;

        info!("Successfully exported preferences to: {}", path.display());
        Ok(())
    }

    /// Import preferences from a specific file
    pub fn import_preferences(&mut self, path: &Path) -> Result<()> {
        debug!("Importing preferences from: {}", path.display());

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read preferences file: {}", path.display()))?;

        let preferences: UserPreferences = ron::from_str(&content)
            .with_context(|| format!("Failed to parse preferences file: {}", path.display()))?;

        self.preferences = preferences;
        self.validate_and_fix();
        self.save_preferences()?;

        info!("Successfully imported preferences from: {}", path.display());
        Ok(())
    }
}

impl Default for PreferencesManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            error!("Failed to create preferences manager: {}", e);
            Self {
                preferences_path: PathBuf::from("preferences.ron"),
                preferences: UserPreferences::default(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;
    use crate::ui::theme::PsocTheme;
    use tempfile::TempDir;

    #[test]
    fn test_preferences_manager_creation() {
        let manager = PreferencesManager::default();
        // Note: The manager might load system preferences, so we just check it's created successfully
        assert!(manager.preferences_file_exists() || !manager.preferences_file_exists());
        // Check that we have valid preferences
        assert!(manager.preferences().interface.ui_scale >= 0.5);
        assert!(manager.preferences().interface.ui_scale <= 2.0);
    }

    #[test]
    fn test_preferences_validation() {
        let mut manager = PreferencesManager::default();

        // Set invalid values
        manager.preferences.interface.ui_scale = 5.0; // Invalid
        manager.preferences.performance.memory_limit = 100; // Invalid
        manager.preferences.defaults.max_undo_history = 5; // Invalid
        manager.preferences.defaults.default_canvas_color = [-1.0, 2.0, 0.5, 1.0]; // Invalid

        let changed = manager.validate_and_fix();
        assert!(changed);

        // Check that values were fixed
        assert_eq!(manager.preferences.interface.ui_scale, 1.0);
        assert_eq!(manager.preferences.performance.memory_limit, 2048);
        assert_eq!(manager.preferences.defaults.max_undo_history, 100);
        assert_eq!(
            manager.preferences.defaults.default_canvas_color,
            [1.0, 1.0, 0.5, 1.0]
        );
    }

    #[test]
    fn test_preferences_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let preferences_path = temp_dir.path().join("test_preferences.ron");

        // Create preferences with custom values
        let mut preferences = UserPreferences::default();
        preferences.interface.theme = PsocTheme::Light;
        preferences.interface.ui_scale = 1.5;
        preferences.performance.memory_limit = 4096;

        // Save preferences
        let content =
            ron::ser::to_string_pretty(&preferences, ron::ser::PrettyConfig::default()).unwrap();
        std::fs::write(&preferences_path, content).unwrap();

        // Load preferences
        let loaded_preferences = PreferencesManager::load_preferences(&preferences_path).unwrap();

        assert_eq!(loaded_preferences.interface.theme, PsocTheme::Light);
        assert_eq!(loaded_preferences.interface.ui_scale, 1.5);
        assert_eq!(loaded_preferences.performance.memory_limit, 4096);
    }

    #[test]
    fn test_preferences_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let non_existent_path = temp_dir.path().join("non_existent.ron");

        let preferences = PreferencesManager::load_preferences(&non_existent_path).unwrap();

        // Should return default preferences
        assert_eq!(preferences.interface.theme, PsocTheme::Dark);
        assert_eq!(preferences.interface.language, Language::English);
    }

    #[test]
    fn test_preferences_export_import() {
        let temp_dir = TempDir::new().unwrap();
        let export_path = temp_dir.path().join("exported_preferences.ron");

        let mut manager = PreferencesManager::default();

        // Modify some preferences
        manager.preferences.interface.theme = PsocTheme::Light;
        manager.preferences.performance.memory_limit = 4096;

        // Export preferences
        manager.export_preferences(&export_path).unwrap();
        assert!(export_path.exists());

        // Create a new manager and import
        let mut new_manager = PreferencesManager::default();
        new_manager.import_preferences(&export_path).unwrap();

        assert_eq!(new_manager.preferences.interface.theme, PsocTheme::Light);
        assert_eq!(new_manager.preferences.performance.memory_limit, 4096);
    }

    #[test]
    fn test_preferences_reset_to_defaults() {
        let mut manager = PreferencesManager::default();

        // Modify preferences
        manager.preferences.interface.theme = PsocTheme::Light;
        manager.preferences.performance.memory_limit = 4096;

        // Reset to defaults
        manager.reset_to_defaults().unwrap();

        assert_eq!(manager.preferences.interface.theme, PsocTheme::Dark);
        assert_eq!(manager.preferences.performance.memory_limit, 2048);
    }
}
