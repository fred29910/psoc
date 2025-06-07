//! Icon system for PSOC Image Editor

use iced::{widget::text, Element, Font};

/// Icon font for the application
pub const ICON_FONT: Font = Font::with_name("PSOC Icons");

/// Icon definitions using Unicode characters
#[derive(Debug, Clone, Copy)]
pub enum Icon {
    // File operations
    New,
    Open,
    Save,
    SaveAs,
    Export,
    Import,

    // Edit operations
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Delete,

    // Tools
    Select,
    Move,
    Brush,
    Eraser,
    Transform,
    Eyedropper,
    Bucket,
    Text,
    Shape,
    Gradient,

    // View operations
    ZoomIn,
    ZoomOut,
    ZoomFit,
    ZoomActual,
    Fullscreen,

    // Layers
    Layer,
    LayerAdd,
    LayerDelete,
    LayerDuplicate,
    LayerVisible,
    LayerHidden,
    LayerLock,
    LayerUnlock,

    // Navigation
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,

    // UI elements
    Menu,
    Settings,
    Close,
    Minimize,
    Maximize,
    Help,
    Info,
    Warning,
    Error,
    Success,

    // Adjustments
    Brightness,
    Contrast,
    Saturation,
    Hue,
    Levels,
    Curves,

    // Filters
    Blur,
    Sharpen,
    Noise,
    Distort,
}

impl Icon {
    /// Get a simple text representation for this icon (ASCII-based)
    pub fn unicode(self) -> char {
        match self {
            // File operations (using simple ASCII symbols)
            Icon::New => '+',
            Icon::Open => 'O',
            Icon::Save => 'S',
            Icon::SaveAs => 's',
            Icon::Export => 'E',
            Icon::Import => 'I',

            // Edit operations
            Icon::Undo => '<',
            Icon::Redo => '>',
            Icon::Cut => 'X',
            Icon::Copy => 'C',
            Icon::Paste => 'V',
            Icon::Delete => 'D',

            // Tools
            Icon::Select => '□',
            Icon::Move => '↔',
            Icon::Brush => 'B',
            Icon::Eraser => 'E',
            Icon::Transform => '⟲',
            Icon::Eyedropper => '●',
            Icon::Bucket => 'F',
            Icon::Text => 'T',
            Icon::Shape => '◇',
            Icon::Gradient => '▦',

            // View operations
            Icon::ZoomIn => '+',
            Icon::ZoomOut => '-',
            Icon::ZoomFit => '⊞',
            Icon::ZoomActual => '1',
            Icon::Fullscreen => '□',

            // Layers
            Icon::Layer => 'L',
            Icon::LayerAdd => '+',
            Icon::LayerDelete => '-',
            Icon::LayerDuplicate => '=',
            Icon::LayerVisible => '●',
            Icon::LayerHidden => '○',
            Icon::LayerLock => '🔒',
            Icon::LayerUnlock => '🔓',

            // Navigation
            Icon::ArrowUp => '↑',
            Icon::ArrowDown => '↓',
            Icon::ArrowLeft => '←',
            Icon::ArrowRight => '→',
            Icon::ChevronUp => '^',
            Icon::ChevronDown => 'v',
            Icon::ChevronLeft => '<',
            Icon::ChevronRight => '>',

            // UI elements
            Icon::Menu => '≡',
            Icon::Settings => '⚙',
            Icon::Close => '×',
            Icon::Minimize => '_',
            Icon::Maximize => '□',
            Icon::Help => '?',
            Icon::Info => 'i',
            Icon::Warning => '!',
            Icon::Error => '✗',
            Icon::Success => '✓',

            // Adjustments
            Icon::Brightness => '☀',
            Icon::Contrast => '◐',
            Icon::Saturation => '◈',
            Icon::Hue => '◯',
            Icon::Levels => '▤',
            Icon::Curves => '~',

            // Filters
            Icon::Blur => '◌',
            Icon::Sharpen => '◆',
            Icon::Noise => '▦',
            Icon::Distort => '◉',
        }
    }

    /// Create an icon text element
    pub fn text<Message>(self) -> Element<'static, Message> {
        text(self.unicode()).size(16.0).into()
    }

    /// Create an icon text element with custom size
    pub fn text_sized<Message>(self, size: f32) -> Element<'static, Message> {
        text(self.unicode()).size(size).into()
    }

    /// Create a text-based icon (more reliable than Unicode)
    pub fn text_label<Message>(self) -> Element<'static, Message> {
        text(self.as_str()).size(12.0).into()
    }

    /// Get the icon as a string
    pub fn as_str(self) -> &'static str {
        match self {
            Icon::New => "New",
            Icon::Open => "Open",
            Icon::Save => "Save",
            Icon::SaveAs => "Save As",
            Icon::Export => "Export",
            Icon::Import => "Import",
            Icon::Undo => "Undo",
            Icon::Redo => "Redo",
            Icon::Cut => "Cut",
            Icon::Copy => "Copy",
            Icon::Paste => "Paste",
            Icon::Delete => "Delete",
            Icon::Select => "Select",
            Icon::Move => "Move",
            Icon::Brush => "Brush",
            Icon::Eraser => "Eraser",
            Icon::Transform => "Transform",
            Icon::Eyedropper => "Eyedropper",
            Icon::Bucket => "Bucket Fill",
            Icon::Text => "Text",
            Icon::Shape => "Shape",
            Icon::Gradient => "Gradient",
            Icon::ZoomIn => "Zoom In",
            Icon::ZoomOut => "Zoom Out",
            Icon::ZoomFit => "Zoom to Fit",
            Icon::ZoomActual => "Actual Size",
            Icon::Fullscreen => "Fullscreen",
            Icon::Layer => "Layer",
            Icon::LayerAdd => "Add Layer",
            Icon::LayerDelete => "Delete Layer",
            Icon::LayerDuplicate => "Duplicate Layer",
            Icon::LayerVisible => "Show Layer",
            Icon::LayerHidden => "Hide Layer",
            Icon::LayerLock => "Lock Layer",
            Icon::LayerUnlock => "Unlock Layer",
            Icon::ArrowUp => "Up",
            Icon::ArrowDown => "Down",
            Icon::ArrowLeft => "Left",
            Icon::ArrowRight => "Right",
            Icon::ChevronUp => "Expand Up",
            Icon::ChevronDown => "Expand Down",
            Icon::ChevronLeft => "Expand Left",
            Icon::ChevronRight => "Expand Right",
            Icon::Menu => "Menu",
            Icon::Settings => "Settings",
            Icon::Close => "Close",
            Icon::Minimize => "Minimize",
            Icon::Maximize => "Maximize",
            Icon::Help => "Help",
            Icon::Info => "Information",
            Icon::Warning => "Warning",
            Icon::Error => "Error",
            Icon::Success => "Success",
            Icon::Brightness => "Brightness",
            Icon::Contrast => "Contrast",
            Icon::Saturation => "Saturation",
            Icon::Hue => "Hue",
            Icon::Levels => "Levels",
            Icon::Curves => "Curves",
            Icon::Blur => "Blur",
            Icon::Sharpen => "Sharpen",
            Icon::Noise => "Noise",
            Icon::Distort => "Distort",
        }
    }
}

/// Helper function to create an icon button (text-based for better compatibility)
pub fn icon_button<Message: Clone + 'static>(
    icon: Icon,
    message: Message,
) -> iced::widget::Button<'static, Message> {
    iced::widget::button(text(icon.as_str()).size(14.0)).on_press(message)
}

/// Helper function to create a simple icon button (text-based)
pub fn simple_icon_button<Message: Clone + 'static>(
    icon: Icon,
    message: Message,
) -> iced::widget::Button<'static, Message> {
    iced::widget::button(text(icon.as_str()).size(12.0)).on_press(message)
}

/// Helper function to create a tool button (text-based for better compatibility)
pub fn tool_button<Message: Clone + 'static>(
    icon: Icon,
    message: Message,
    is_active: bool,
) -> iced::widget::Button<'static, Message> {
    let button = iced::widget::button(text(icon.as_str()).size(12.0)).on_press(message);

    // TODO: Add visual styling for active state
    let _ = is_active; // Suppress unused variable warning
    button
}
