//! User interface module

#[cfg(feature = "gui")]
pub mod application;
#[cfg(feature = "gui")]
pub mod canvas;
#[cfg(feature = "gui")]
pub mod components;
#[cfg(feature = "gui")]
pub mod icons;
#[cfg(feature = "gui")]
pub mod theme;

// Re-export main components
#[cfg(feature = "gui")]
pub use application::{PsocApp, Message, AppState, Tool};
#[cfg(feature = "gui")]
pub use canvas::{ImageCanvas, ImageData};
#[cfg(feature = "gui")]
pub use theme::{PsocTheme, ColorPalette, ButtonStyle, ContainerStyle};
#[cfg(feature = "gui")]
pub use icons::Icon;
