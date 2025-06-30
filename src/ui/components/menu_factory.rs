//! Menu factory for creating predefined menu structures
//! Implements the 10 Office-style menu categories

use super::menu_system::{MenuCategory, MenuCategoryId, MenuItem};
use crate::ui::application::Message;
use crate::ui::icons::Icon;
use crate::i18n::t;

/// Factory for creating standard menu categories
pub struct MenuFactory;

impl MenuFactory {
    /// Create all standard menu categories
    pub fn create_all_menus() -> Vec<MenuCategory<Message>> {
        vec![
            Self::create_file_menu(),
            Self::create_edit_menu(),
            Self::create_image_menu(),
            Self::create_layer_menu(),
            Self::create_text_menu(),
            Self::create_select_menu(),
            Self::create_filter_menu(),
            Self::create_view_menu(),
            Self::create_window_menu(),
            Self::create_help_menu(),
        ]
    }

    /// Create File menu (文件)
    pub fn create_file_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("file-new", &t("menu-file-new"), Some(Message::NewDocument))
                .with_icon(Icon::New)
                .with_shortcut("shortcut-new"), // Changed to key
            MenuItem::new("file-open", &t("menu-file-open"), Some(Message::OpenDocument))
                .with_icon(Icon::Open)
                .with_shortcut("shortcut-open"), // Changed to key
            MenuItem::separator(),
            MenuItem::new("file-save", &t("menu-file-save"), Some(Message::SaveDocument))
                .with_icon(Icon::Save)
                .with_shortcut("shortcut-save"), // Changed to key
            MenuItem::new("file-save-as", &t("menu-file-save-as"), Some(Message::SaveAsDocument))
                .with_icon(Icon::SaveAs)
                .with_shortcut("shortcut-save-as"), // Changed to key
            MenuItem::new("file-export", &t("menu-file-export"), Some(Message::SaveAsDocument))
                .with_icon(Icon::Export)
                .with_shortcut("shortcut-export"), // Changed to key
            MenuItem::new("file-import", &t("menu-file-import"), Some(Message::OpenDocument))
                .with_icon(Icon::Import)
                .with_shortcut("shortcut-import"), // Changed to key
            MenuItem::separator(),
            MenuItem::new("file-recent", &t("menu-file-recent"), Some(Message::NewDocument)), // Placeholder action, no shortcut for now
            MenuItem::separator(),
            MenuItem::new("file-exit", &t("menu-file-exit"), Some(Message::Exit))
                .with_shortcut("shortcut-exit"), // Changed to key
        ];

        MenuCategory::new(MenuCategoryId::File, &t("menu-file"), items)
    }

    /// Create Edit menu (编辑)
    pub fn create_edit_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("edit-undo", &t("menu-edit-undo"), Some(Message::Undo))
                .with_icon(Icon::Undo)
                .with_shortcut("shortcut-undo"), // Changed to key
            MenuItem::new("edit-redo", &t("menu-edit-redo"), Some(Message::Redo))
                .with_icon(Icon::Redo)
                .with_shortcut("shortcut-redo"), // Changed to key
            MenuItem::separator(),
            MenuItem::new("edit-cut", &t("menu-edit-cut"), Some(Message::NewDocument)) // Placeholder
                .with_icon(Icon::Cut)
                .with_shortcut("shortcut-cut"), // Changed to key
            MenuItem::new("edit-copy", &t("menu-edit-copy"), Some(Message::NewDocument)) // Placeholder
                .with_icon(Icon::Copy)
                .with_shortcut("shortcut-copy"), // Changed to key
            MenuItem::new("edit-paste", &t("menu-edit-paste"), Some(Message::NewDocument)) // Placeholder
                .with_icon(Icon::Paste)
                .with_shortcut("shortcut-paste"), // Changed to key
            MenuItem::new("edit-delete", &t("menu-edit-delete"), Some(Message::NewDocument)) // Placeholder
                .with_icon(Icon::Delete)
                .with_shortcut("shortcut-delete"), // Changed to key
            MenuItem::separator(),
            MenuItem::new("edit-select-all", &t("menu-edit-select-all"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-select-all"), // Changed to key
            MenuItem::new("edit-deselect", &t("menu-edit-deselect"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-deselect"), // Changed to key
            MenuItem::separator(),
            MenuItem::new("edit-preferences", &t("menu-edit-preferences"), Some(Message::ShowPreferences))
                .with_icon(Icon::Settings)
                .with_shortcut("shortcut-preferences"), // Changed to key
        ];

        MenuCategory::new(MenuCategoryId::Edit, &t("menu-edit"), items)
    }

    /// Create Image menu (图像)
    pub fn create_image_menu() -> MenuCategory<Message> {
        use crate::ui::application::AdjustmentMessage;

        let items = vec![
            MenuItem::new(
                "image-brightness-contrast",
                &t("menu-image-brightness-contrast"),
                Some(Message::Adjustment(AdjustmentMessage::ShowBrightnessContrast)),
            ),
            MenuItem::new(
                "image-hsl",
                &t("menu-image-hsl"),
                Some(Message::Adjustment(AdjustmentMessage::ShowHsl)),
            ),
            MenuItem::new(
                "image-color-balance",
                &t("menu-image-color-balance"),
                Some(Message::Adjustment(AdjustmentMessage::ShowColorBalance)),
            ),
            MenuItem::new(
                "image-curves",
                &t("menu-image-curves"),
                Some(Message::Adjustment(AdjustmentMessage::ShowCurves)),
            ),
            MenuItem::new(
                "image-levels",
                &t("menu-image-levels"),
                Some(Message::Adjustment(AdjustmentMessage::ShowLevels)),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "image-grayscale",
                &t("menu-image-grayscale"),
                Some(Message::Adjustment(AdjustmentMessage::ShowGrayscale)),
            ),
        ];

        MenuCategory::new(MenuCategoryId::Image, &t("menu-image"), items)
    }

    /// Create Layer menu (图层)
    pub fn create_layer_menu() -> MenuCategory<Message> {
        use crate::ui::application::LayerMessage;

        let items = vec![
            MenuItem::new(
                "layer-add-empty",
                &t("menu-layer-add-empty"),
                Some(Message::Layer(LayerMessage::AddEmptyLayer)),
            )
            .with_shortcut("shortcut-new-layer"), // Changed to key
            MenuItem::new(
                "layer-add-from-file",
                &t("menu-layer-add-from-file"),
                Some(Message::Layer(LayerMessage::AddLayerFromFile)),
            ), // No standard shortcut, or add if available
            MenuItem::separator(),
            MenuItem::new(
                "layer-duplicate",
                &t("menu-layer-duplicate"),
                Some(Message::Layer(LayerMessage::DuplicateLayer(0))), // Placeholder index
            )
            .with_shortcut("shortcut-duplicate-layer"), // Changed to key
            MenuItem::new(
                "layer-delete",
                &t("menu-layer-delete"),
                Some(Message::Layer(LayerMessage::DeleteLayer(0))), // Placeholder index
            )
            .with_shortcut("shortcut-delete-layer"), // Changed to key (might be same as edit-delete)
        ];

        MenuCategory::new(MenuCategoryId::Layer, &t("menu-layer"), items)
    }

    /// Create Text menu (文字)
    pub fn create_text_menu() -> MenuCategory<Message> {
        use crate::tools::ToolType;

        let items = vec![
            MenuItem::new(
                "text-tool",
                &t("menu-text-tool"),
                Some(Message::ToolChanged(ToolType::Text)),
            )
            .with_icon(Icon::Text)
            .with_shortcut("shortcut-text-tool"), // Changed to key
        ];

        MenuCategory::new(MenuCategoryId::Text, &t("menu-text"), items)
    }

    /// Create Select menu (选择)
    pub fn create_select_menu() -> MenuCategory<Message> {
        use crate::tools::ToolType;

        let items = vec![
            MenuItem::new(
                "select-rectangle",
                &t("menu-select-rectangle"),
                Some(Message::ToolChanged(ToolType::Select)),
            )
            .with_icon(Icon::Select) // Assuming Icon::Select is for rectangle
            .with_shortcut("shortcut-select-tool"), // Changed to key (M)
            MenuItem::new(
                "select-ellipse",
                &t("menu-select-ellipse"),
                Some(Message::ToolChanged(ToolType::EllipseSelect)),
            ), // No standard shortcut, or add if available
            MenuItem::new(
                "select-lasso",
                &t("menu-select-lasso"),
                Some(Message::ToolChanged(ToolType::LassoSelect)),
            )
            .with_shortcut("shortcut-lasso-tool"), // Changed to key (L)
            MenuItem::new(
                "select-magic-wand",
                &t("menu-select-magic-wand"),
                Some(Message::ToolChanged(ToolType::MagicWand)),
            )
            .with_shortcut("shortcut-magic-wand-tool"), // Changed to key (W)
            MenuItem::separator(),
            MenuItem::new("select-all", &t("menu-select-all"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-select-all"), // Re-uses edit-select-all key
            MenuItem::new("select-deselect", &t("menu-select-deselect"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-deselect"), // Re-uses edit-deselect key
            MenuItem::new("select-invert", &t("menu-select-invert"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-invert-selection"), // Changed to key
        ];

        MenuCategory::new(MenuCategoryId::Select, &t("menu-select"), items)
    }

    /// Create Filter menu (滤镜)
    pub fn create_filter_menu() -> MenuCategory<Message> {
        use crate::ui::application::AdjustmentMessage;

        let items = vec![
            MenuItem::new(
                "filter-gaussian-blur",
                &t("menu-filter-gaussian-blur"),
                Some(Message::Adjustment(AdjustmentMessage::ShowGaussianBlur)),
            ),
            MenuItem::new(
                "filter-motion-blur",
                &t("menu-filter-motion-blur"),
                Some(Message::Adjustment(AdjustmentMessage::ShowMotionBlur)),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "filter-unsharp-mask",
                &t("menu-filter-unsharp-mask"),
                Some(Message::Adjustment(AdjustmentMessage::ShowUnsharpMask)),
            ),
            MenuItem::new(
                "filter-sharpen",
                &t("menu-filter-sharpen"),
                Some(Message::Adjustment(AdjustmentMessage::ShowSharpen)),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "filter-add-noise",
                &t("menu-filter-add-noise"),
                Some(Message::Adjustment(AdjustmentMessage::ShowAddNoise)),
            ),
            MenuItem::new(
                "filter-reduce-noise",
                &t("menu-filter-reduce-noise"),
                Some(Message::Adjustment(AdjustmentMessage::ShowReduceNoise)),
            ),
        ];

        MenuCategory::new(MenuCategoryId::Filter, &t("menu-filter"), items)
    }

    /// Create View menu (视图)
    pub fn create_view_menu() -> MenuCategory<Message> {
        use crate::ui::application::ViewMessage;

        let items = vec![
            MenuItem::new("view-zoom-in", &t("menu-view-zoom-in"), Some(Message::ZoomIn))
                .with_shortcut("shortcut-zoom-in"), // Changed to key
            MenuItem::new("view-zoom-out", &t("menu-view-zoom-out"), Some(Message::ZoomOut))
                .with_shortcut("shortcut-zoom-out"), // Changed to key
            MenuItem::new("view-zoom-reset", &t("menu-view-zoom-reset"), Some(Message::ZoomReset))
                .with_shortcut("shortcut-zoom-reset"), // Changed to key
            MenuItem::new("view-zoom-fit", &t("menu-view-zoom-fit"), Some(Message::ZoomReset)) // Placeholder action
                .with_shortcut("shortcut-zoom-fit"), // Changed to key
            MenuItem::separator(),
            MenuItem::new(
                "view-rulers",
                &t("menu-view-rulers"),
                Some(Message::View(ViewMessage::ToggleRulers)),
            )
            .with_shortcut("shortcut-rulers"), // Changed to key
            MenuItem::new(
                "view-grid",
                &t("menu-view-grid"),
                Some(Message::View(ViewMessage::ToggleGrid)),
            )
            .with_shortcut("shortcut-grid"), // Changed to key
            MenuItem::new(
                "view-guides",
                &t("menu-view-guides"),
                Some(Message::View(ViewMessage::ToggleGuides)),
            )
            .with_shortcut("shortcut-guides"), // Changed to key
            MenuItem::separator(),
            MenuItem::new("view-fullscreen", &t("menu-view-fullscreen"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-fullscreen"), // Changed to key
        ];

        MenuCategory::new(MenuCategoryId::View, &t("menu-view"), items)
    }

    /// Create Window menu (窗口)
    pub fn create_window_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("window-color-picker", &t("menu-window-color-picker"), Some(Message::ShowColorPicker)),
            MenuItem::new("window-color-palette", &t("menu-window-color-palette"), Some(Message::ShowColorPalette)),
            // Preferences was moved to Edit menu in docs, but can be here too if desired.
            // For now, following the doc and removing from here. If it should be here, add it back.
            // MenuItem::new("window-preferences", &t("menu-window-preferences"), Some(Message::ShowPreferences)),
        ];

        MenuCategory::new(MenuCategoryId::Window, &t("menu-window"), items)
    }

    /// Create Help menu (帮助)
    pub fn create_help_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("help-about", &t("menu-help-about"), Some(Message::ShowAbout))
                .with_shortcut("shortcut-about"), // Changed to key
            MenuItem::new("help-docs", &t("menu-help-help"), Some(Message::NewDocument)) // Placeholder
                .with_shortcut("shortcut-help-docs"), // Changed to key
        ];

        MenuCategory::new(MenuCategoryId::Help, &t("menu-help"), items)
    }
}
