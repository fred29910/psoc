//! Dialog components for PSOC Image Editor

#[cfg(feature = "gui")]
pub mod about;

#[cfg(feature = "gui")]
pub use about::{AboutDialog, AboutMessage};
