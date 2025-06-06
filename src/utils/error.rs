//! Error handling for PSOC

use thiserror::Error;

/// Main error type for PSOC application
#[derive(Error, Debug)]
pub enum PsocError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    #[error("GUI error: {0}")]
    Gui(String),

    #[error("File format error: {0}")]
    FileFormat(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
