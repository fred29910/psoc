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
            MenuItem::new("new", &t("menu-file-new"), Message::NewDocument)
                .with_icon(Icon::New)
                .with_shortcut("Ctrl+N"),
            MenuItem::new("open", &t("menu-file-open"), Message::OpenDocument)
                .with_icon(Icon::Open)
                .with_shortcut("Ctrl+O"),
            MenuItem::separator(),
            MenuItem::new("save", &t("menu-file-save"), Message::SaveDocument)
                .with_icon(Icon::Save)
                .with_shortcut("Ctrl+S"),
            MenuItem::new("save_as", &t("menu-file-save-as"), Message::SaveAsDocument)
                .with_icon(Icon::SaveAs)
                .with_shortcut("Ctrl+Shift+S"),
            MenuItem::new("export", &t("menu-file-export"), Message::SaveAsDocument)
                .with_icon(Icon::Export)
                .with_shortcut("Ctrl+E"),
            MenuItem::new("import", &t("menu-file-import"), Message::OpenDocument)
                .with_icon(Icon::Import)
                .with_shortcut("Ctrl+I"),
            MenuItem::separator(),
            MenuItem::new("recent", &t("menu-file-recent"), Message::NewDocument)
                .with_icon(Icon::Open),
            MenuItem::separator(),
            MenuItem::new("exit", &t("menu-file-exit"), Message::Exit)
                .with_shortcut("Alt+F4"),
        ];

        MenuCategory::new(MenuCategoryId::File, items)
    }

    /// Create Edit menu (编辑)
    pub fn create_edit_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("undo", &t("menu-edit-undo"), Message::Undo)
                .with_icon(Icon::Undo)
                .with_shortcut("Ctrl+Z"),
            MenuItem::new("redo", &t("menu-edit-redo"), Message::Redo)
                .with_icon(Icon::Redo)
                .with_shortcut("Ctrl+Y"),
            MenuItem::separator(),
            MenuItem::new("cut", &t("menu-edit-cut"), Message::NewDocument)
                .with_icon(Icon::Cut)
                .with_shortcut("Ctrl+X"),
            MenuItem::new("copy", &t("menu-edit-copy"), Message::NewDocument)
                .with_icon(Icon::Copy)
                .with_shortcut("Ctrl+C"),
            MenuItem::new("paste", &t("menu-edit-paste"), Message::NewDocument)
                .with_icon(Icon::Paste)
                .with_shortcut("Ctrl+V"),
            MenuItem::new("delete", &t("menu-edit-delete"), Message::NewDocument)
                .with_icon(Icon::Delete)
                .with_shortcut("Delete"),
            MenuItem::separator(),
            MenuItem::new("select_all", &t("menu-edit-select-all"), Message::NewDocument)
                .with_shortcut("Ctrl+A"),
            MenuItem::new("deselect", &t("menu-edit-deselect"), Message::NewDocument)
                .with_shortcut("Ctrl+D"),
            MenuItem::separator(),
            MenuItem::new("preferences", &t("menu-edit-preferences"), Message::ShowPreferences)
                .with_icon(Icon::Settings)
                .with_shortcut("Ctrl+,"),
        ];

        MenuCategory::new(MenuCategoryId::Edit, items)
    }

    /// Create Image menu (图像)
    pub fn create_image_menu() -> MenuCategory<Message> {
        use crate::ui::application::AdjustmentMessage;

        let items = vec![
            MenuItem::new(
                "brightness_contrast",
                &t("menu-image-brightness-contrast"),
                Message::Adjustment(AdjustmentMessage::ShowBrightnessContrast),
            ),
            MenuItem::new(
                "hsl",
                &t("menu-image-hsl"),
                Message::Adjustment(AdjustmentMessage::ShowHsl),
            ),
            MenuItem::new(
                "color_balance",
                &t("menu-image-color-balance"),
                Message::Adjustment(AdjustmentMessage::ShowColorBalance),
            ),
            MenuItem::new(
                "curves",
                &t("menu-image-curves"),
                Message::Adjustment(AdjustmentMessage::ShowCurves),
            ),
            MenuItem::new(
                "levels",
                &t("menu-image-levels"),
                Message::Adjustment(AdjustmentMessage::ShowLevels),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "grayscale",
                &t("menu-image-grayscale"),
                Message::Adjustment(AdjustmentMessage::ShowGrayscale),
            ),
        ];

        MenuCategory::new(MenuCategoryId::Image, items)
    }

    /// Create Layer menu (图层)
    pub fn create_layer_menu() -> MenuCategory<Message> {
        use crate::ui::application::LayerMessage;

        let items = vec![
            MenuItem::new(
                "add_layer",
                &t("menu-layer-add-empty"),
                Message::Layer(LayerMessage::AddEmptyLayer),
            )
            .with_shortcut("Ctrl+Shift+N"),
            MenuItem::new(
                "add_layer_from_file",
                &t("menu-layer-add-from-file"),
                Message::Layer(LayerMessage::AddLayerFromFile),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "duplicate_layer",
                &t("menu-layer-duplicate"),
                Message::Layer(LayerMessage::DuplicateLayer(0)), // Will be updated with actual index
            )
            .with_shortcut("Ctrl+J"),
            MenuItem::new(
                "delete_layer",
                &t("menu-layer-delete"),
                Message::Layer(LayerMessage::DeleteLayer(0)),
            )
            .with_shortcut("Delete"),
        ];

        MenuCategory::new(MenuCategoryId::Layer, items)
    }

    /// Create Text menu (文字)
    pub fn create_text_menu() -> MenuCategory<Message> {
        use crate::tools::ToolType;

        let items = vec![
            MenuItem::new(
                "text_tool",
                &t("menu-text-tool"),
                Message::ToolChanged(ToolType::Text),
            )
            .with_icon(Icon::Text)
            .with_shortcut("T"),
        ];

        MenuCategory::new(MenuCategoryId::Text, items)
    }

    /// Create Select menu (选择)
    pub fn create_select_menu() -> MenuCategory<Message> {
        use crate::tools::ToolType;

        let items = vec![
            MenuItem::new(
                "select_tool",
                &t("menu-select-rectangle"),
                Message::ToolChanged(ToolType::Select),
            )
            .with_icon(Icon::Select)
            .with_shortcut("M"),
            MenuItem::new(
                "ellipse_select_tool",
                &t("menu-select-ellipse"),
                Message::ToolChanged(ToolType::EllipseSelect),
            )
            .with_shortcut("E"),
            MenuItem::new(
                "lasso_tool",
                &t("menu-select-lasso"),
                Message::ToolChanged(ToolType::LassoSelect),
            )
            .with_shortcut("L"),
            MenuItem::new(
                "magic_wand_tool",
                &t("menu-select-magic-wand"),
                Message::ToolChanged(ToolType::MagicWand),
            )
            .with_shortcut("W"),
            MenuItem::separator(),
            MenuItem::new("select_all", &t("menu-select-all"), Message::NewDocument)
                .with_shortcut("Ctrl+A"),
            MenuItem::new("deselect", &t("menu-select-deselect"), Message::NewDocument)
                .with_shortcut("Ctrl+D"),
            MenuItem::new("invert_selection", &t("menu-select-invert"), Message::NewDocument)
                .with_shortcut("Ctrl+Shift+I"),
        ];

        MenuCategory::new(MenuCategoryId::Select, items)
    }

    /// Create Filter menu (滤镜)
    pub fn create_filter_menu() -> MenuCategory<Message> {
        use crate::ui::application::AdjustmentMessage;

        let items = vec![
            MenuItem::new(
                "gaussian_blur",
                &t("menu-filter-gaussian-blur"),
                Message::Adjustment(AdjustmentMessage::ShowGaussianBlur),
            ),
            MenuItem::new(
                "motion_blur",
                &t("menu-filter-motion-blur"),
                Message::Adjustment(AdjustmentMessage::ShowMotionBlur),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "unsharp_mask",
                &t("menu-filter-unsharp-mask"),
                Message::Adjustment(AdjustmentMessage::ShowUnsharpMask),
            ),
            MenuItem::new(
                "sharpen",
                &t("menu-filter-sharpen"),
                Message::Adjustment(AdjustmentMessage::ShowSharpen),
            ),
            MenuItem::separator(),
            MenuItem::new(
                "add_noise",
                &t("menu-filter-add-noise"),
                Message::Adjustment(AdjustmentMessage::ShowAddNoise),
            ),
            MenuItem::new(
                "reduce_noise",
                &t("menu-filter-reduce-noise"),
                Message::Adjustment(AdjustmentMessage::ShowReduceNoise),
            ),
        ];

        MenuCategory::new(MenuCategoryId::Filter, items)
    }

    /// Create View menu (视图)
    pub fn create_view_menu() -> MenuCategory<Message> {
        use crate::ui::application::ViewMessage;

        let items = vec![
            MenuItem::new("zoom_in", &t("menu-view-zoom-in"), Message::ZoomIn)
                .with_shortcut("Ctrl++"),
            MenuItem::new("zoom_out", &t("menu-view-zoom-out"), Message::ZoomOut)
                .with_shortcut("Ctrl+-"),
            MenuItem::new("zoom_reset", &t("menu-view-zoom-reset"), Message::ZoomReset)
                .with_shortcut("Ctrl+0"),
            MenuItem::new("zoom_fit", &t("menu-view-zoom-fit"), Message::ZoomReset)
                .with_shortcut("Ctrl+Shift+0"),
            MenuItem::separator(),
            MenuItem::new(
                "toggle_rulers",
                &t("menu-view-rulers"),
                Message::View(ViewMessage::ToggleRulers),
            )
            .with_shortcut("Ctrl+R"),
            MenuItem::new(
                "toggle_grid",
                &t("menu-view-grid"),
                Message::View(ViewMessage::ToggleGrid),
            )
            .with_shortcut("Ctrl+G"),
            MenuItem::new(
                "toggle_guides",
                &t("menu-view-guides"),
                Message::View(ViewMessage::ToggleGuides),
            )
            .with_shortcut("Ctrl+;"),
            MenuItem::separator(),
            MenuItem::new("fullscreen", &t("menu-view-fullscreen"), Message::NewDocument)
                .with_shortcut("F11"),
        ];

        MenuCategory::new(MenuCategoryId::View, items)
    }

    /// Create Window menu (窗口)
    pub fn create_window_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("show_color_picker", &t("menu-window-color-picker"), Message::ShowColorPicker),
            MenuItem::new("show_color_palette", &t("menu-window-color-palette"), Message::ShowColorPalette),
            MenuItem::new("show_preferences", &t("menu-window-preferences"), Message::ShowPreferences),
        ];

        MenuCategory::new(MenuCategoryId::Window, items)
    }

    /// Create Help menu (帮助)
    pub fn create_help_menu() -> MenuCategory<Message> {
        let items = vec![
            MenuItem::new("about", &t("menu-help-about"), Message::ShowAbout)
                .with_shortcut("F1"),
            MenuItem::new("help", &t("menu-help-help"), Message::NewDocument)
                .with_shortcut("Ctrl+F1"),
        ];

        MenuCategory::new(MenuCategoryId::Help, items)
    }
}
