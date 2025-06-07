//! Dialog components for PSOC Image Editor

#[cfg(feature = "gui")]
pub mod about;
#[cfg(feature = "gui")]
pub mod brightness_contrast;

#[cfg(feature = "gui")]
pub use about::{AboutDialog, AboutMessage};
#[cfg(feature = "gui")]
pub use brightness_contrast::{BrightnessContrastDialog, BrightnessContrastMessage};
