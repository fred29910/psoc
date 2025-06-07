//! Shortcut data structures and types
//!
//! This module defines the core data structures for the keyboard shortcuts system,
//! including shortcut keys, modifiers, actions, and the main Shortcut struct.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a keyboard key that can be used in shortcuts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutKey {
    /// Character keys (a-z, 0-9, etc.)
    Character(char),
    /// Function keys (F1-F12)
    Function(u8),
    /// Special keys
    Escape,
    Enter,
    Space,
    Tab,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

/// Keyboard modifiers for shortcuts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ShortcutModifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool, // Windows key on Windows, Cmd key on macOS
}

impl ShortcutModifiers {
    /// Create new empty modifiers
    pub fn new() -> Self {
        Self::default()
    }

    /// Add Ctrl modifier
    pub fn with_ctrl(mut self, ctrl: bool) -> Self {
        self.ctrl = ctrl;
        self
    }

    /// Add Shift modifier
    pub fn with_shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    /// Add Alt modifier
    pub fn with_alt(mut self, alt: bool) -> Self {
        self.alt = alt;
        self
    }

    /// Add Meta modifier
    pub fn with_meta(mut self, meta: bool) -> Self {
        self.meta = meta;
        self
    }

    /// Check if any modifiers are active
    pub fn has_any(&self) -> bool {
        self.ctrl || self.shift || self.alt || self.meta
    }

    /// Check if modifiers match exactly
    pub fn matches(&self, other: &Self) -> bool {
        self.ctrl == other.ctrl
            && self.shift == other.shift
            && self.alt == other.alt
            && self.meta == other.meta
    }
}

impl fmt::Display for ShortcutModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.meta {
            parts.push("Meta");
        }

        write!(f, "{}", parts.join("+"))
    }
}

/// Actions that can be triggered by shortcuts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutAction {
    // File operations
    NewDocument,
    OpenDocument,
    SaveDocument,
    SaveAsDocument,
    ExportDocument,

    // Edit operations
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
    DeselectAll,
    InvertSelection,

    // Tool operations
    SelectTool,
    BrushTool,
    EraserTool,
    MoveTool,
    TransformTool,
    EyedropperTool,

    // View operations
    ZoomIn,
    ZoomOut,
    ZoomReset,
    ZoomFit,
    ToggleRulers,
    ToggleGrid,
    ToggleGuides,

    // Layer operations
    NewLayer,
    DuplicateLayer,
    DeleteLayer,
    MergeDown,
    FlattenImage,

    // Adjustment operations
    BrightnessContrast,
    HueSaturation,
    Levels,
    Curves,

    // Application operations
    ShowPreferences,
    ShowAbout,
    Exit,

    // Custom action with string identifier
    Custom(String),
}

impl fmt::Display for ShortcutAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ShortcutAction::NewDocument => "New Document",
            ShortcutAction::OpenDocument => "Open Document",
            ShortcutAction::SaveDocument => "Save Document",
            ShortcutAction::SaveAsDocument => "Save As...",
            ShortcutAction::ExportDocument => "Export Document",
            ShortcutAction::Undo => "Undo",
            ShortcutAction::Redo => "Redo",
            ShortcutAction::Cut => "Cut",
            ShortcutAction::Copy => "Copy",
            ShortcutAction::Paste => "Paste",
            ShortcutAction::SelectAll => "Select All",
            ShortcutAction::DeselectAll => "Deselect All",
            ShortcutAction::InvertSelection => "Invert Selection",
            ShortcutAction::SelectTool => "Select Tool",
            ShortcutAction::BrushTool => "Brush Tool",
            ShortcutAction::EraserTool => "Eraser Tool",
            ShortcutAction::MoveTool => "Move Tool",
            ShortcutAction::TransformTool => "Transform Tool",
            ShortcutAction::EyedropperTool => "Eyedropper Tool",
            ShortcutAction::ZoomIn => "Zoom In",
            ShortcutAction::ZoomOut => "Zoom Out",
            ShortcutAction::ZoomReset => "Zoom Reset",
            ShortcutAction::ZoomFit => "Zoom Fit",
            ShortcutAction::ToggleRulers => "Toggle Rulers",
            ShortcutAction::ToggleGrid => "Toggle Grid",
            ShortcutAction::ToggleGuides => "Toggle Guides",
            ShortcutAction::NewLayer => "New Layer",
            ShortcutAction::DuplicateLayer => "Duplicate Layer",
            ShortcutAction::DeleteLayer => "Delete Layer",
            ShortcutAction::MergeDown => "Merge Down",
            ShortcutAction::FlattenImage => "Flatten Image",
            ShortcutAction::BrightnessContrast => "Brightness/Contrast",
            ShortcutAction::HueSaturation => "Hue/Saturation",
            ShortcutAction::Levels => "Levels",
            ShortcutAction::Curves => "Curves",
            ShortcutAction::ShowPreferences => "Preferences",
            ShortcutAction::ShowAbout => "About",
            ShortcutAction::Exit => "Exit",
            ShortcutAction::Custom(name) => name,
        };
        write!(f, "{}", name)
    }
}

/// A keyboard shortcut definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Shortcut {
    /// The key that triggers this shortcut
    pub key: ShortcutKey,
    /// Required modifiers
    pub modifiers: ShortcutModifiers,
    /// The action to perform
    pub action: ShortcutAction,
    /// Optional description
    pub description: Option<String>,
    /// Whether this shortcut is enabled
    pub enabled: bool,
}

impl Shortcut {
    /// Create a new shortcut
    pub fn new(key: ShortcutKey, modifiers: ShortcutModifiers, action: ShortcutAction) -> Self {
        Self {
            key,
            modifiers,
            action,
            description: None,
            enabled: true,
        }
    }

    /// Create a new shortcut with description
    pub fn with_description(
        key: ShortcutKey,
        modifiers: ShortcutModifiers,
        action: ShortcutAction,
        description: String,
    ) -> Self {
        Self {
            key,
            modifiers,
            action,
            description: Some(description),
            enabled: true,
        }
    }

    /// Check if this shortcut matches the given key and modifiers
    pub fn matches(&self, key: ShortcutKey, modifiers: &ShortcutModifiers) -> bool {
        self.enabled && self.key == key && self.modifiers.matches(modifiers)
    }

    /// Get a human-readable representation of this shortcut
    pub fn display_string(&self) -> String {
        let key_str = match self.key {
            ShortcutKey::Character(c) => c.to_uppercase().to_string(),
            ShortcutKey::Function(n) => format!("F{}", n),
            ShortcutKey::Escape => "Esc".to_string(),
            ShortcutKey::Enter => "Enter".to_string(),
            ShortcutKey::Space => "Space".to_string(),
            ShortcutKey::Tab => "Tab".to_string(),
            ShortcutKey::Backspace => "Backspace".to_string(),
            ShortcutKey::Delete => "Delete".to_string(),
            ShortcutKey::Insert => "Insert".to_string(),
            ShortcutKey::Home => "Home".to_string(),
            ShortcutKey::End => "End".to_string(),
            ShortcutKey::PageUp => "Page Up".to_string(),
            ShortcutKey::PageDown => "Page Down".to_string(),
            ShortcutKey::ArrowUp => "↑".to_string(),
            ShortcutKey::ArrowDown => "↓".to_string(),
            ShortcutKey::ArrowLeft => "←".to_string(),
            ShortcutKey::ArrowRight => "→".to_string(),
        };

        if self.modifiers.has_any() {
            format!("{}+{}", self.modifiers, key_str)
        } else {
            key_str
        }
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.display_string(), self.action)
    }
}
