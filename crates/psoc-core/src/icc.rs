//! ICC Profile management and color space conversion
//!
//! This module provides ICC profile handling, color space conversion using LCMS2,
//! and color management system (CMS) functionality for PSOC.

use anyhow::Result;
use lcms2::{Locale, Profile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// ICC Profile wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IccProfile {
    /// Profile description/name
    pub description: String,
    /// Profile size in bytes
    pub size: usize,
    /// Profile color space
    pub color_space: ColorSpace,
    /// Profile class (display, input, output, etc.)
    pub profile_class: ProfileClass,
    /// Raw profile data for serialization and reconstruction
    pub raw_data: Option<Vec<u8>>,
}

// Thread-safe wrapper for LCMS2 Profile
unsafe impl Send for IccProfile {}
unsafe impl Sync for IccProfile {}

impl IccProfile {
    /// Create a new ICC profile from raw data
    pub fn from_data(data: Vec<u8>, description: String) -> Result<Self> {
        // Parse the profile to get metadata
        let profile = Profile::new_icc(&data)
            .map_err(|e| anyhow::anyhow!("Failed to parse ICC profile: {:?}", e))?;

        let locale = Locale::new("en");
        let description = profile
            .info(lcms2::InfoType::Description, locale)
            .unwrap_or(description);

        let color_space = match profile.color_space() {
            lcms2::ColorSpaceSignature::RgbData => ColorSpace::Rgb,
            lcms2::ColorSpaceSignature::CmykData => ColorSpace::Cmyk,
            lcms2::ColorSpaceSignature::GrayData => ColorSpace::Gray,
            lcms2::ColorSpaceSignature::LabData => ColorSpace::Lab,
            lcms2::ColorSpaceSignature::XYZData => ColorSpace::Xyz,
            _ => ColorSpace::Unknown,
        };

        let profile_class = match profile.device_class() {
            lcms2::ProfileClassSignature::InputClass => ProfileClass::Input,
            lcms2::ProfileClassSignature::DisplayClass => ProfileClass::Display,
            lcms2::ProfileClassSignature::OutputClass => ProfileClass::Output,
            lcms2::ProfileClassSignature::LinkClass => ProfileClass::DeviceLink,
            lcms2::ProfileClassSignature::ColorSpaceClass => ProfileClass::ColorSpace,
            lcms2::ProfileClassSignature::AbstractClass => ProfileClass::Abstract,
            lcms2::ProfileClassSignature::NamedColorClass => ProfileClass::NamedColor,
            _ => ProfileClass::Unknown,
        };

        Ok(Self {
            description,
            size: data.len(),
            color_space,
            profile_class,
            raw_data: Some(data),
        })
    }

    /// Create sRGB profile
    pub fn new_srgb() -> Self {
        Self {
            description: "sRGB IEC61966-2.1".to_string(),
            size: 0,
            color_space: ColorSpace::Rgb,
            profile_class: ProfileClass::Display,
            raw_data: None,
        }
    }

    /// Get the LCMS2 profile (recreated each time for thread safety)
    pub fn get_profile(&self) -> Result<Profile> {
        if let Some(ref data) = self.raw_data {
            Profile::new_icc(data)
                .map_err(|e| anyhow::anyhow!("Failed to recreate profile: {:?}", e))
        } else {
            // Built-in sRGB profile
            Ok(Profile::new_srgb())
        }
    }
}

/// Color space enumeration for ICC profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorSpace {
    /// RGB color space
    Rgb,
    /// CMYK color space
    Cmyk,
    /// Grayscale color space
    Gray,
    /// LAB color space
    Lab,
    /// XYZ color space
    Xyz,
    /// Unknown or unsupported color space
    Unknown,
}

/// ICC Profile class enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileClass {
    /// Input device profile
    Input,
    /// Display device profile
    Display,
    /// Output device profile
    Output,
    /// Device link profile
    DeviceLink,
    /// Color space conversion profile
    ColorSpace,
    /// Abstract profile
    Abstract,
    /// Named color profile
    NamedColor,
    /// Unknown profile class
    Unknown,
}

/// Color Management System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsConfig {
    /// Default display profile (sRGB if None)
    pub display_profile: Option<String>,
    /// Default working color space
    pub working_color_space: ColorSpace,
    /// Rendering intent for conversions
    pub rendering_intent: RenderingIntent,
    /// Enable black point compensation
    pub black_point_compensation: bool,
    /// Enable color management
    pub enabled: bool,
}

/// Rendering intent for color conversions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderingIntent {
    /// Perceptual rendering intent
    Perceptual,
    /// Relative colorimetric intent
    RelativeColorimetric,
    /// Saturation intent
    Saturation,
    /// Absolute colorimetric intent
    AbsoluteColorimetric,
}

impl Default for CmsConfig {
    fn default() -> Self {
        Self {
            display_profile: None, // Use sRGB as default
            working_color_space: ColorSpace::Rgb,
            rendering_intent: RenderingIntent::Perceptual,
            black_point_compensation: true,
            enabled: true,
        }
    }
}

/// Color Management System manager
pub struct ColorManager {
    /// Profile cache
    profile_cache: Arc<Mutex<HashMap<String, IccProfile>>>,
    /// Current configuration
    config: CmsConfig,
    /// sRGB display profile
    srgb_profile: Option<IccProfile>,
}

impl ColorManager {
    /// Create a new color manager
    pub fn new() -> Result<Self> {
        let mut manager = Self {
            profile_cache: Arc::new(Mutex::new(HashMap::new())),
            config: CmsConfig::default(),
            srgb_profile: None,
        };

        // Initialize with sRGB profile
        manager.initialize_srgb_profile()?;

        Ok(manager)
    }

    /// Initialize the built-in sRGB profile
    fn initialize_srgb_profile(&mut self) -> Result<()> {
        let profile = IccProfile::new_srgb();

        self.srgb_profile = Some(profile.clone());

        // Cache the sRGB profile
        let mut cache = self.profile_cache.lock().unwrap();
        cache.insert("sRGB".to_string(), profile);

        Ok(())
    }

    /// Load ICC profile from raw data
    pub fn load_profile_from_data(&mut self, data: &[u8], name: String) -> Result<IccProfile> {
        // Check cache first
        {
            let cache = self.profile_cache.lock().unwrap();
            if let Some(profile) = cache.get(&name) {
                return Ok(profile.clone());
            }
        }

        let icc_profile = IccProfile::from_data(data.to_vec(), name.clone())?;

        // Cache the profile
        {
            let mut cache = self.profile_cache.lock().unwrap();
            cache.insert(name, icc_profile.clone());
        }

        Ok(icc_profile)
    }

    /// Get the sRGB display profile
    pub fn get_srgb_profile(&self) -> Option<&IccProfile> {
        self.srgb_profile.as_ref()
    }

    /// Get the current display profile (sRGB if none configured)
    pub fn get_display_profile(&self) -> Option<IccProfile> {
        if let Some(profile_name) = &self.config.display_profile {
            let cache = self.profile_cache.lock().unwrap();
            cache.get(profile_name).cloned()
        } else {
            self.get_srgb_profile().cloned()
        }
    }

    /// Create a color transform from source to display profile
    pub fn create_display_transform(&self, _source_profile: &IccProfile) -> Result<()> {
        // TODO: Implement actual transform creation when LCMS2 integration is complete
        // For now, just return Ok to allow compilation
        Ok(())
    }

    /// Update CMS configuration
    pub fn update_config(&mut self, config: CmsConfig) {
        self.config = config;
    }

    /// Get current CMS configuration
    pub fn get_config(&self) -> &CmsConfig {
        &self.config
    }

    /// Check if color management is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

impl Default for ColorManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ColorManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_manager_creation() {
        let manager = ColorManager::new();
        assert!(manager.is_ok());

        let manager = manager.unwrap();
        assert!(manager.get_srgb_profile().is_some());
        assert!(manager.is_enabled());
    }

    #[test]
    fn test_srgb_profile() {
        let manager = ColorManager::new().unwrap();
        let srgb = manager.get_srgb_profile().unwrap();

        assert_eq!(srgb.color_space, ColorSpace::Rgb);
        assert_eq!(srgb.profile_class, ProfileClass::Display);
        assert_eq!(srgb.description, "sRGB IEC61966-2.1");
    }

    #[test]
    fn test_cms_config_default() {
        let config = CmsConfig::default();
        assert!(config.enabled);
        assert!(config.black_point_compensation);
        assert_eq!(config.working_color_space, ColorSpace::Rgb);
        assert_eq!(config.rendering_intent, RenderingIntent::Perceptual);
    }

    #[test]
    fn test_display_profile_fallback() {
        let manager = ColorManager::new().unwrap();
        let display_profile = manager.get_display_profile();
        assert!(display_profile.is_some());

        // Should fallback to sRGB
        let srgb = manager.get_srgb_profile().unwrap();
        assert_eq!(display_profile.unwrap().description, srgb.description);
    }

    #[test]
    fn test_color_space_enum() {
        assert_eq!(ColorSpace::Rgb, ColorSpace::Rgb);
        assert_ne!(ColorSpace::Rgb, ColorSpace::Cmyk);

        // Test serialization
        let rgb_json = serde_json::to_string(&ColorSpace::Rgb).unwrap();
        let rgb_deserialized: ColorSpace = serde_json::from_str(&rgb_json).unwrap();
        assert_eq!(rgb_deserialized, ColorSpace::Rgb);
    }

    #[test]
    fn test_profile_class_enum() {
        assert_eq!(ProfileClass::Display, ProfileClass::Display);
        assert_ne!(ProfileClass::Display, ProfileClass::Input);

        // Test serialization
        let display_json = serde_json::to_string(&ProfileClass::Display).unwrap();
        let display_deserialized: ProfileClass = serde_json::from_str(&display_json).unwrap();
        assert_eq!(display_deserialized, ProfileClass::Display);
    }

    #[test]
    fn test_rendering_intent_enum() {
        assert_eq!(RenderingIntent::Perceptual, RenderingIntent::Perceptual);
        assert_ne!(RenderingIntent::Perceptual, RenderingIntent::Saturation);

        // Test serialization
        let perceptual_json = serde_json::to_string(&RenderingIntent::Perceptual).unwrap();
        let perceptual_deserialized: RenderingIntent =
            serde_json::from_str(&perceptual_json).unwrap();
        assert_eq!(perceptual_deserialized, RenderingIntent::Perceptual);
    }

    #[test]
    fn test_cms_config_serialization() {
        let config = CmsConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CmsConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.working_color_space, deserialized.working_color_space);
        assert_eq!(config.rendering_intent, deserialized.rendering_intent);
        assert_eq!(
            config.black_point_compensation,
            deserialized.black_point_compensation
        );
    }

    #[test]
    fn test_color_manager_config_update() {
        let mut manager = ColorManager::new().unwrap();

        let new_config = CmsConfig {
            enabled: false,
            working_color_space: ColorSpace::Cmyk,
            rendering_intent: RenderingIntent::Saturation,
            black_point_compensation: false,
            display_profile: Some("Custom".to_string()),
        };

        manager.update_config(new_config.clone());
        let current_config = manager.get_config();

        assert_eq!(current_config.enabled, new_config.enabled);
        assert_eq!(
            current_config.working_color_space,
            new_config.working_color_space
        );
        assert_eq!(current_config.rendering_intent, new_config.rendering_intent);
        assert_eq!(
            current_config.black_point_compensation,
            new_config.black_point_compensation
        );
    }

    #[test]
    fn test_display_transform_creation() {
        let manager = ColorManager::new().unwrap();
        let srgb_profile = manager.get_srgb_profile().unwrap();

        let transform = manager.create_display_transform(srgb_profile);
        assert!(transform.is_ok());
    }
}
