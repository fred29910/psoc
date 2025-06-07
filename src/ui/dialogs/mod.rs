//! Dialog components for PSOC Image Editor

#[cfg(feature = "gui")]
pub mod about;
#[cfg(feature = "gui")]
pub mod brightness_contrast;
#[cfg(feature = "gui")]
pub mod color_palette;
#[cfg(feature = "gui")]
pub mod color_picker;
#[cfg(feature = "gui")]
pub mod gaussian_blur;
#[cfg(feature = "gui")]
pub mod gradient_editor;
#[cfg(feature = "gui")]
pub mod preferences;

#[cfg(feature = "gui")]
pub use about::{AboutDialog, AboutMessage};
#[cfg(feature = "gui")]
pub use brightness_contrast::{BrightnessContrastDialog, BrightnessContrastMessage};
#[cfg(feature = "gui")]
pub use color_palette::{ColorPalette, ColorPaletteDialog, ColorPaletteMessage};
#[cfg(feature = "gui")]
pub use color_picker::{ColorPickerDialog, ColorPickerMessage};
#[cfg(feature = "gui")]
pub use gaussian_blur::{GaussianBlurDialog, GaussianBlurMessage};
#[cfg(feature = "gui")]
pub use gradient_editor::{GradientEditor, GradientEditorMessage};
#[cfg(feature = "gui")]
pub use preferences::{PreferencesDialog, PreferencesMessage};
