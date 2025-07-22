//! Adaptive toolbar system for responsive layouts
//! Provides intelligent toolbar layout based on screen size and available space

use std::collections::HashMap;

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Size, Alignment,
};
use serde::{Deserialize, Serialize};

use crate::tools::ToolType;
use crate::ui::theme::PsocTheme;
use crate::ui::animations::{TransitionManager, AnimationDirection};
use super::responsive_layout::{ScreenSize, ToolbarLayout};

/// Toolbar adaptation strategy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ToolbarStrategy {
    /// Always use full horizontal toolbar
    AlwaysHorizontal,
    /// Always use vertical toolbar
    AlwaysVertical,
    /// Adapt based on screen size
    Responsive,
    /// Compact mode with grouped tools
    Compact,
    /// Floating toolbar that can be moved
    Floating,
}

/// Tool group for compact layouts
#[derive(Debug, Clone, PartialEq)]
pub struct ToolGroup {
    /// Group name
    pub name: String,
    /// Tools in this group
    pub tools: Vec<ToolType>,
    /// Group icon
    pub icon: String,
    /// Whether group is expanded
    pub expanded: bool,
    /// Group priority (higher = more important)
    pub priority: u8,
}

/// Adaptive toolbar configuration
#[derive(Debug, Clone)]
pub struct AdaptiveToolbarConfig {
    /// Current strategy
    pub strategy: ToolbarStrategy,
    /// Maximum tools per row in compact mode
    pub max_tools_per_row: usize,
    /// Whether to show tool labels
    pub show_labels: bool,
    /// Tool icon size
    pub icon_size: f32,
    /// Toolbar padding
    pub padding: f32,
    /// Whether animations are enabled
    pub animations_enabled: bool,
    /// Tool groups for compact mode
    pub tool_groups: Vec<ToolGroup>,
}

impl Default for AdaptiveToolbarConfig {
    fn default() -> Self {
        Self {
            strategy: ToolbarStrategy::Responsive,
            max_tools_per_row: 6,
            show_labels: false,
            icon_size: 24.0,
            padding: 8.0,
            animations_enabled: true,
            tool_groups: Self::default_tool_groups(),
        }
    }
}

impl AdaptiveToolbarConfig {
    /// Create default tool groups
    fn default_tool_groups() -> Vec<ToolGroup> {
        vec![
            ToolGroup {
                name: "Selection".to_string(),
                tools: vec![ToolType::Select, ToolType::Move],
                icon: "🔲".to_string(),
                expanded: false,
                priority: 10,
            },
            ToolGroup {
                name: "Drawing".to_string(),
                tools: vec![ToolType::Brush, ToolType::Eraser],
                icon: "🖌️".to_string(),
                expanded: false,
                priority: 9,
            },
            ToolGroup {
                name: "Shapes".to_string(),
                tools: vec![ToolType::Rectangle, ToolType::Ellipse, ToolType::Line],
                icon: "⬜".to_string(),
                expanded: false,
                priority: 7,
            },
            ToolGroup {
                name: "Text".to_string(),
                tools: vec![ToolType::Text],
                icon: "📝".to_string(),
                expanded: false,
                priority: 6,
            },
            ToolGroup {
                name: "Utilities".to_string(),
                tools: vec![ToolType::Eyedropper, ToolType::Crop, ToolType::Transform],
                icon: "🔧".to_string(),
                expanded: false,
                priority: 5,
            },
        ]
    }
}

/// Adaptive toolbar manager
#[derive(Debug)]
pub struct AdaptiveToolbar {
    /// Configuration
    config: AdaptiveToolbarConfig,
    /// Current screen size
    screen_size: ScreenSize,
    /// Available space
    available_space: Size,
    /// Currently active tool
    active_tool: Option<ToolType>,
    /// Transition manager for animations
    transition_manager: TransitionManager,
    /// Tool visibility states
    tool_visibility: HashMap<ToolType, bool>,
}

impl Default for AdaptiveToolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveToolbar {
    /// Create a new adaptive toolbar
    pub fn new() -> Self {
        let mut tool_visibility = HashMap::new();
        
        // All tools visible by default
        for tool_type in [
            ToolType::Select, ToolType::EllipseSelect, ToolType::LassoSelect, ToolType::MagicWand,
            ToolType::Move, ToolType::Brush, ToolType::Eraser, ToolType::Rectangle,
            ToolType::Ellipse, ToolType::Line, ToolType::Polygon, ToolType::Text,
            ToolType::Gradient, ToolType::Eyedropper, ToolType::Crop, ToolType::Transform,
        ] {
            tool_visibility.insert(tool_type, true);
        }

        Self {
            config: AdaptiveToolbarConfig::default(),
            screen_size: ScreenSize::Large,
            available_space: Size::new(1200.0, 800.0),
            active_tool: None,
            transition_manager: TransitionManager::new(),
            tool_visibility,
        }
    }

    /// Update screen size and available space
    pub fn update_layout(&mut self, screen_size: ScreenSize, available_space: Size) {
        self.screen_size = screen_size;
        self.available_space = available_space;
        
        // Adapt strategy based on screen size if using responsive mode
        if self.config.strategy == ToolbarStrategy::Responsive {
            match screen_size {
                ScreenSize::Small => {
                    self.adapt_for_small_screen();
                },
                ScreenSize::Medium => {
                    self.adapt_for_medium_screen();
                },
                ScreenSize::Large | ScreenSize::ExtraLarge => {
                    self.adapt_for_large_screen();
                },
            }
        }
    }

    /// Adapt toolbar for small screens
    fn adapt_for_small_screen(&mut self) {
        // Use compact mode with tool groups
        self.config.show_labels = false;
        self.config.icon_size = 20.0;
        self.config.max_tools_per_row = 4;
        
        // Hide less important tools
        self.hide_tools_by_priority(6); // Hide tools with priority <= 6
    }

    /// Adapt toolbar for medium screens
    fn adapt_for_medium_screen(&mut self) {
        // Use horizontal layout with some tools hidden
        self.config.show_labels = false;
        self.config.icon_size = 22.0;
        self.config.max_tools_per_row = 6;
        
        // Hide only least important tools
        self.hide_tools_by_priority(4);
    }

    /// Adapt toolbar for large screens
    fn adapt_for_large_screen(&mut self) {
        // Use full horizontal layout
        self.config.show_labels = true;
        self.config.icon_size = 24.0;
        self.config.max_tools_per_row = 10;
        
        // Show all tools
        for visible in self.tool_visibility.values_mut() {
            *visible = true;
        }
    }

    /// Hide tools based on priority threshold
    fn hide_tools_by_priority(&mut self, min_priority: u8) {
        for group in &self.config.tool_groups {
            if group.priority <= min_priority {
                for tool in &group.tools {
                    self.tool_visibility.insert(*tool, false);
                }
            }
        }
    }

    /// Set active tool
    pub fn set_active_tool(&mut self, tool: Option<ToolType>) {
        if self.active_tool != tool {
            // Animate tool switch if animations are enabled
            if self.config.animations_enabled {
                if let Some(old_tool) = self.active_tool {
                    self.transition_manager.start_tool_switch(
                        format!("tool_{:?}", old_tool),
                        false, // deactivating
                    );
                }
                
                if let Some(new_tool) = tool {
                    self.transition_manager.start_tool_switch(
                        format!("tool_{:?}", new_tool),
                        true, // activating
                    );
                }
            }
            
            self.active_tool = tool;
        }
    }

    /// Get current toolbar layout
    pub fn get_current_layout(&self) -> ToolbarLayout {
        match self.config.strategy {
            ToolbarStrategy::AlwaysHorizontal => ToolbarLayout::Full,
            ToolbarStrategy::AlwaysVertical => ToolbarLayout::Vertical,
            ToolbarStrategy::Compact => ToolbarLayout::Compact,
            ToolbarStrategy::Floating => ToolbarLayout::Full, // TODO: Add floating layout
            ToolbarStrategy::Responsive => {
                match self.screen_size {
                    ScreenSize::Small => ToolbarLayout::Vertical,
                    ScreenSize::Medium => ToolbarLayout::Compact,
                    ScreenSize::Large | ScreenSize::ExtraLarge => ToolbarLayout::Full,
                }
            },
        }
    }

    /// Create toolbar element based on current configuration
    pub fn create_toolbar<Message: Clone + 'static>(
        &self,
        theme: PsocTheme,
        on_tool_select: impl Fn(ToolType) -> Message + 'static + Copy,
    ) -> Element<'static, Message> {
        match self.get_current_layout() {
            ToolbarLayout::Full => self.create_horizontal_toolbar(theme, on_tool_select),
            ToolbarLayout::Compact => self.create_compact_toolbar(theme, on_tool_select),
            ToolbarLayout::Vertical => self.create_vertical_toolbar(theme, on_tool_select),
        }
    }

    /// Create horizontal toolbar
    fn create_horizontal_toolbar<Message: Clone + 'static>(
        &self,
        theme: PsocTheme,
        on_tool_select: impl Fn(ToolType) -> Message + 'static + Copy,
    ) -> Element<'static, Message> {
        let palette = theme.palette();
        
        let tools = self.get_visible_tools();
        let tool_buttons: Vec<Element<Message>> = tools
            .into_iter()
            .map(|tool| {
                let is_active = self.active_tool == Some(tool);
                self.create_tool_button(tool, is_active, theme, on_tool_select)
            })
            .collect();

        container(
            row(tool_buttons)
                .spacing(4.0)
                .align_y(Alignment::Center)
        )
        .padding(self.config.padding)
        .style(move |_theme| {
            use iced::{Background, Border, Color, Shadow, Vector};
            
            iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    palette.glass_bg_medium.r,
                    palette.glass_bg_medium.g,
                    palette.glass_bg_medium.b,
                    0.9
                ))),
                border: Border {
                    color: Color::from_rgba(
                        palette.border.r,
                        palette.border.g,
                        palette.border.b,
                        0.3
                    ),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: palette.shadow_color(0.1),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                text_color: Some(palette.text),
            }
        })
        .into()
    }

    /// Create compact toolbar with tool groups
    fn create_compact_toolbar<Message: Clone + 'static>(
        &self,
        theme: PsocTheme,
        on_tool_select: impl Fn(ToolType) -> Message + 'static + Copy,
    ) -> Element<'static, Message> {
        let palette = theme.palette();
        
        // Create tool group buttons
        let group_buttons: Vec<Element<Message>> = self.config.tool_groups
            .iter()
            .filter(|group| group.tools.iter().any(|tool| *self.tool_visibility.get(tool).unwrap_or(&false)))
            .map(|group| {
                // For now, just show the first tool in each group
                if let Some(first_tool) = group.tools.first() {
                    let is_active = self.active_tool == Some(*first_tool);
                    self.create_tool_button(*first_tool, is_active, theme, on_tool_select)
                } else {
                    Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into()
                }
            })
            .collect();

        container(
            row(group_buttons)
                .spacing(2.0)
                .align_y(Alignment::Center)
        )
        .padding(self.config.padding / 2.0)
        .style(move |_theme| {
            use iced::{Background, Border, Color, Shadow, Vector};
            
            iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    palette.glass_bg_light.r,
                    palette.glass_bg_light.g,
                    palette.glass_bg_light.b,
                    0.8
                ))),
                border: Border {
                    color: Color::from_rgba(
                        palette.border.r,
                        palette.border.g,
                        palette.border.b,
                        0.2
                    ),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow {
                    color: palette.shadow_color(0.05),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 4.0,
                },
                text_color: Some(palette.text),
            }
        })
        .into()
    }

    /// Create vertical toolbar
    fn create_vertical_toolbar<Message: Clone + 'static>(
        &self,
        theme: PsocTheme,
        on_tool_select: impl Fn(ToolType) -> Message + 'static + Copy,
    ) -> Element<'static, Message> {
        let palette = theme.palette();
        
        let tools = self.get_visible_tools();
        let tool_buttons: Vec<Element<Message>> = tools
            .into_iter()
            .map(|tool| {
                let is_active = self.active_tool == Some(tool);
                self.create_tool_button(tool, is_active, theme, on_tool_select)
            })
            .collect();

        container(
            column(tool_buttons)
                .spacing(2.0)
                .align_x(Alignment::Center)
        )
        .padding(self.config.padding / 2.0)
        .style(move |_theme| {
            use iced::{Background, Border, Color, Shadow, Vector};
            
            iced::widget::container::Style {
                background: Some(Background::Color(Color::from_rgba(
                    palette.glass_bg_medium.r,
                    palette.glass_bg_medium.g,
                    palette.glass_bg_medium.b,
                    0.9
                ))),
                border: Border {
                    color: Color::from_rgba(
                        palette.border.r,
                        palette.border.g,
                        palette.border.b,
                        0.3
                    ),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: palette.shadow_color(0.1),
                    offset: Vector::new(2.0, 0.0),
                    blur_radius: 8.0,
                },
                text_color: Some(palette.text),
            }
        })
        .into()
    }

    /// Create a tool button
    fn create_tool_button<Message: Clone + 'static>(
        &self,
        tool: ToolType,
        is_active: bool,
        theme: PsocTheme,
        on_tool_select: impl Fn(ToolType) -> Message + 'static + Copy,
    ) -> Element<'static, Message> {
        let palette = theme.palette();
        let icon = self.get_tool_icon(tool);
        
        let button_content: Element<Message> = if self.config.show_labels {
            column![
                text(icon).size(self.config.icon_size),
                text(format!("{:?}", tool)).size(10.0)
            ]
            .align_x(Alignment::Center)
            .spacing(2.0)
            .into()
        } else {
            text(icon).size(self.config.icon_size).into()
        };

        button(button_content)
            .padding(6.0)
            .style(move |_theme, status| {
                use iced::{Background, Border, Color, Shadow, Vector};
                
                let (background, border_color, shadow) = if is_active {
                    (
                        Some(Background::Color(Color::from_rgba(
                            palette.tech_blue.r,
                            palette.tech_blue.g,
                            palette.tech_blue.b,
                            0.3
                        ))),
                        Color::from_rgba(
                            palette.tech_blue.r,
                            palette.tech_blue.g,
                            palette.tech_blue.b,
                            0.8
                        ),
                        Shadow {
                            color: Color::from_rgba(
                                palette.tech_blue.r,
                                palette.tech_blue.g,
                                palette.tech_blue.b,
                                0.2
                            ),
                            offset: Vector::new(0.0, 2.0),
                            blur_radius: 6.0,
                        },
                    )
                } else {
                    match status {
                        iced::widget::button::Status::Hovered => (
                            Some(Background::Color(Color::from_rgba(
                                palette.glass_bg_light.r,
                                palette.glass_bg_light.g,
                                palette.glass_bg_light.b,
                                0.6
                            ))),
                            Color::from_rgba(
                                palette.border.r,
                                palette.border.g,
                                palette.border.b,
                                0.5
                            ),
                            Shadow {
                                color: palette.shadow_color(0.1),
                                offset: Vector::new(0.0, 1.0),
                                blur_radius: 4.0,
                            },
                        ),
                        _ => (
                            Some(Background::Color(Color::TRANSPARENT)),
                            Color::TRANSPARENT,
                            Shadow {
                                color: Color::TRANSPARENT,
                                offset: Vector::new(0.0, 0.0),
                                blur_radius: 0.0,
                            },
                        ),
                    }
                };

                iced::widget::button::Style {
                    background,
                    text_color: if is_active { Color::WHITE } else { palette.text },
                    border: Border {
                        color: border_color,
                        width: if is_active { 2.0 } else { 1.0 },
                        radius: 4.0.into(),
                    },
                    shadow,
                }
            })
            .on_press(on_tool_select(tool))
            .into()
    }

    /// Get visible tools based on current configuration
    fn get_visible_tools(&self) -> Vec<ToolType> {
        self.tool_visibility
            .iter()
            .filter(|(_, &visible)| visible)
            .map(|(&tool, _)| tool)
            .collect()
    }

    /// Get icon for a tool
    fn get_tool_icon(&self, tool: ToolType) -> &'static str {
        match tool {
            ToolType::Select => "🔲",
            ToolType::EllipseSelect => "⭕",
            ToolType::LassoSelect => "🪢",
            ToolType::MagicWand => "🪄",
            ToolType::Move => "✋",
            ToolType::Brush => "🖌️",
            ToolType::Eraser => "🧽",
            ToolType::Rectangle => "⬜",
            ToolType::Ellipse => "⭕",
            ToolType::Line => "📏",
            ToolType::Polygon => "🔷",
            ToolType::Text => "📝",
            ToolType::Gradient => "🌈",
            ToolType::Eyedropper => "💧",
            ToolType::Crop => "✂️",
            ToolType::Transform => "🔄",
        }
    }

    /// Update animations and return whether any are active
    pub fn update_animations(&mut self) -> bool {
        self.transition_manager.update()
    }

    /// Set toolbar strategy
    pub fn set_strategy(&mut self, strategy: ToolbarStrategy) {
        self.config.strategy = strategy;
    }

    /// Get current strategy
    pub fn get_strategy(&self) -> ToolbarStrategy {
        self.config.strategy
    }

    /// Enable or disable animations
    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.config.animations_enabled = enabled;
        if !enabled {
            self.transition_manager.stop_all_animations();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_toolbar_creation() {
        let toolbar = AdaptiveToolbar::new();

        // Should have default configuration
        assert_eq!(toolbar.config.strategy, ToolbarStrategy::Responsive);
        assert_eq!(toolbar.config.max_tools_per_row, 6);
        assert!(toolbar.config.animations_enabled);

        // Should have tool groups
        assert!(toolbar.config.tool_groups.len() > 0);

        // All tools should be visible by default
        assert!(toolbar.tool_visibility.values().all(|&visible| visible));
    }

    #[test]
    fn test_screen_size_adaptation() {
        let mut toolbar = AdaptiveToolbar::new();

        // Test small screen adaptation
        toolbar.update_layout(ScreenSize::Small, Size::new(600.0, 400.0));
        assert_eq!(toolbar.screen_size, ScreenSize::Small);
        assert_eq!(toolbar.config.max_tools_per_row, 4);
        assert!(!toolbar.config.show_labels);

        // Test large screen adaptation
        toolbar.update_layout(ScreenSize::Large, Size::new(1200.0, 800.0));
        assert_eq!(toolbar.screen_size, ScreenSize::Large);
        assert!(toolbar.config.show_labels);
        assert_eq!(toolbar.config.max_tools_per_row, 10);
    }

    #[test]
    fn test_toolbar_strategies() {
        let mut toolbar = AdaptiveToolbar::new();

        // Test different strategies
        toolbar.set_strategy(ToolbarStrategy::AlwaysHorizontal);
        assert_eq!(toolbar.get_strategy(), ToolbarStrategy::AlwaysHorizontal);
        assert_eq!(toolbar.get_current_layout(), ToolbarLayout::Full);

        toolbar.set_strategy(ToolbarStrategy::AlwaysVertical);
        assert_eq!(toolbar.get_strategy(), ToolbarStrategy::AlwaysVertical);
        assert_eq!(toolbar.get_current_layout(), ToolbarLayout::Vertical);

        toolbar.set_strategy(ToolbarStrategy::Compact);
        assert_eq!(toolbar.get_strategy(), ToolbarStrategy::Compact);
        assert_eq!(toolbar.get_current_layout(), ToolbarLayout::Compact);
    }

    #[test]
    fn test_responsive_layout() {
        let mut toolbar = AdaptiveToolbar::new();
        toolbar.set_strategy(ToolbarStrategy::Responsive);

        // Small screen should use vertical layout
        toolbar.update_layout(ScreenSize::Small, Size::new(600.0, 400.0));
        assert_eq!(toolbar.get_current_layout(), ToolbarLayout::Vertical);

        // Medium screen should use compact layout
        toolbar.update_layout(ScreenSize::Medium, Size::new(900.0, 600.0));
        assert_eq!(toolbar.get_current_layout(), ToolbarLayout::Compact);

        // Large screen should use full layout
        toolbar.update_layout(ScreenSize::Large, Size::new(1200.0, 800.0));
        assert_eq!(toolbar.get_current_layout(), ToolbarLayout::Full);
    }

    #[test]
    fn test_tool_activation() {
        let mut toolbar = AdaptiveToolbar::new();

        // No tool should be active initially
        assert_eq!(toolbar.active_tool, None);

        // Activate a tool
        toolbar.set_active_tool(Some(ToolType::Brush));
        assert_eq!(toolbar.active_tool, Some(ToolType::Brush));

        // Switch to another tool
        toolbar.set_active_tool(Some(ToolType::Select));
        assert_eq!(toolbar.active_tool, Some(ToolType::Select));

        // Deactivate tool
        toolbar.set_active_tool(None);
        assert_eq!(toolbar.active_tool, None);
    }

    #[test]
    fn test_tool_visibility() {
        let toolbar = AdaptiveToolbar::new();

        // Get visible tools
        let visible_tools = toolbar.get_visible_tools();

        // Should have all tools visible by default
        assert!(visible_tools.contains(&ToolType::Select));
        assert!(visible_tools.contains(&ToolType::Brush));
        assert!(visible_tools.contains(&ToolType::Eraser));
        assert!(visible_tools.len() > 10); // Should have many tools
    }

    #[test]
    fn test_tool_icons() {
        let toolbar = AdaptiveToolbar::new();

        // Test tool icons
        assert_eq!(toolbar.get_tool_icon(ToolType::Select), "🔲");
        assert_eq!(toolbar.get_tool_icon(ToolType::Brush), "🖌️");
        assert_eq!(toolbar.get_tool_icon(ToolType::Eraser), "🧽");
        assert_eq!(toolbar.get_tool_icon(ToolType::Text), "📝");
        assert_eq!(toolbar.get_tool_icon(ToolType::Eyedropper), "💧");
    }

    #[test]
    fn test_tool_groups() {
        let config = AdaptiveToolbarConfig::default();

        // Should have default tool groups
        assert!(config.tool_groups.len() > 0);

        // Find selection group
        let selection_group = config.tool_groups.iter()
            .find(|g| g.name == "Selection")
            .unwrap();

        assert!(selection_group.tools.contains(&ToolType::Select));
        assert!(selection_group.tools.contains(&ToolType::Move));
        assert_eq!(selection_group.priority, 10);

        // Find drawing group
        let drawing_group = config.tool_groups.iter()
            .find(|g| g.name == "Drawing")
            .unwrap();

        assert!(drawing_group.tools.contains(&ToolType::Brush));
        assert!(drawing_group.tools.contains(&ToolType::Eraser));
        assert_eq!(drawing_group.priority, 9);
    }

    #[test]
    fn test_animations_control() {
        let mut toolbar = AdaptiveToolbar::new();

        // Should start with animations enabled
        assert!(toolbar.config.animations_enabled);

        // Disable animations
        toolbar.set_animations_enabled(false);
        assert!(!toolbar.config.animations_enabled);

        // Re-enable animations
        toolbar.set_animations_enabled(true);
        assert!(toolbar.config.animations_enabled);
    }

    #[test]
    fn test_toolbar_config() {
        let config = AdaptiveToolbarConfig::default();

        // Test default values
        assert_eq!(config.strategy, ToolbarStrategy::Responsive);
        assert_eq!(config.max_tools_per_row, 6);
        assert!(!config.show_labels);
        assert_eq!(config.icon_size, 24.0);
        assert_eq!(config.padding, 8.0);
        assert!(config.animations_enabled);
    }

    #[test]
    fn test_priority_based_hiding() {
        let mut toolbar = AdaptiveToolbar::new();

        // Hide tools by priority
        toolbar.hide_tools_by_priority(6);

        // Check that low priority tools are hidden
        let visible_tools = toolbar.get_visible_tools();

        // High priority tools should still be visible
        assert!(visible_tools.contains(&ToolType::Select));
        assert!(visible_tools.contains(&ToolType::Brush));

        // Some tools should be hidden based on group priorities
        let total_tools = toolbar.tool_visibility.len();
        let visible_count = visible_tools.len();
        assert!(visible_count < total_tools);
    }

    #[test]
    fn test_toolbar_layout_adaptation() {
        let mut toolbar = AdaptiveToolbar::new();

        // Test adaptation for different screen sizes
        toolbar.adapt_for_small_screen();
        assert!(!toolbar.config.show_labels);
        assert_eq!(toolbar.config.icon_size, 20.0);
        assert_eq!(toolbar.config.max_tools_per_row, 4);

        toolbar.adapt_for_medium_screen();
        assert!(!toolbar.config.show_labels);
        assert_eq!(toolbar.config.icon_size, 22.0);
        assert_eq!(toolbar.config.max_tools_per_row, 6);

        toolbar.adapt_for_large_screen();
        assert!(toolbar.config.show_labels);
        assert_eq!(toolbar.config.icon_size, 24.0);
        assert_eq!(toolbar.config.max_tools_per_row, 10);
    }

    #[test]
    fn test_toolbar_strategy_enum() {
        // Test all strategy variants
        let strategies = [
            ToolbarStrategy::AlwaysHorizontal,
            ToolbarStrategy::AlwaysVertical,
            ToolbarStrategy::Responsive,
            ToolbarStrategy::Compact,
            ToolbarStrategy::Floating,
        ];

        for strategy in strategies {
            let mut toolbar = AdaptiveToolbar::new();
            toolbar.set_strategy(strategy);
            assert_eq!(toolbar.get_strategy(), strategy);
        }
    }
}
