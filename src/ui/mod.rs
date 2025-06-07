//! User interface module

#[cfg(feature = "gui")]
pub mod application;
#[cfg(feature = "gui")]
pub mod canvas;
#[cfg(feature = "gui")]
pub mod components;
#[cfg(feature = "gui")]
pub mod dialogs;
#[cfg(feature = "gui")]
pub mod icons;
#[cfg(feature = "gui")]
pub mod theme;

// Re-export main components
#[cfg(feature = "gui")]
pub use application::{AppState, LayerMessage, Message, PsocApp};
#[cfg(feature = "gui")]
pub use canvas::{ImageCanvas, ImageData};
#[cfg(feature = "gui")]
pub use dialogs::{AboutDialog, BrightnessContrastDialog, GaussianBlurDialog};
#[cfg(feature = "gui")]
pub use icons::Icon;
#[cfg(feature = "gui")]
pub use theme::{ButtonStyle, ColorPalette, ContainerStyle, PsocTheme};
