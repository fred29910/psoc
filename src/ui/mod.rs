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
pub mod fonts;
#[cfg(feature = "gui")]
pub mod theme;
#[cfg(feature = "gui")]
pub mod animations;
#[cfg(feature = "gui")]
pub mod styles;

// Re-export main components
#[cfg(feature = "gui")]
pub use application::{AppState, LayerMessage, Message, PsocApp};
#[cfg(feature = "gui")]
pub use canvas::{ImageCanvas, ImageData};
#[cfg(feature = "gui")]
pub use components::{
    MenuCategory, MenuCategoryId, MenuFactory, MenuItem, MenuMessage, MenuSystem,
    ResponsiveLayoutManager, ResponsiveLayoutMessage, PanelId, PanelState, ScreenSize,
    KeyboardNavigationManager, KbNavMessage, FocusTarget, NavigationAction,
};
#[cfg(feature = "gui")]
pub use dialogs::{
    AboutDialog, BrightnessContrastDialog, ColorPaletteDialog, ColorPickerDialog,
    GaussianBlurDialog,
};
#[cfg(feature = "gui")]
pub use icons::Icon;
#[cfg(feature = "gui")]
pub use fonts::{FontConfig, FontManager, initialize_fonts};
#[cfg(feature = "gui")]
pub use theme::{ButtonStyle, ColorPalette, ContainerStyle, MenuStyle, PsocTheme, VisualStyle};
#[cfg(feature = "gui")]
pub use animations::{MenuAnimationManager, TransitionType};
#[cfg(feature = "gui")]
pub use styles::{VisualEffectStyle, GlassEffect, FrostedGlassStyle, ShadowConfig};
