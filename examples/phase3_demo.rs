//! Phase 3 Interactive Experience Optimization Demo
//! Demonstrates keyboard navigation, responsive layout, and enhanced user interaction features

use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Length, Size, Task, Theme,
};
use psoc::ui::components::{
    KeyboardNavigationManager, ResponsiveLayoutManager, FocusTarget, NavigationAction,
    PanelId, ScreenSize, KbNavMessage, ResponsiveLayoutMessage,
};

#[derive(Debug, Clone)]
pub enum DemoMessage {
    /// Keyboard navigation messages
    KeyboardNavigation(KbNavMessage),
    /// Responsive layout messages
    ResponsiveLayout(ResponsiveLayoutMessage),
    /// Simulate window resize
    SimulateResize(Size),
    /// Toggle feature demonstrations
    ToggleKeyboardNav,
    ToggleFocusIndicators,
    ToggleCompactMode,
    /// Focus specific target
    FocusTarget(FocusTarget),
    /// Toggle panel
    TogglePanel(PanelId),
}

pub struct Phase3Demo {
    keyboard_nav: KeyboardNavigationManager,
    layout_manager: ResponsiveLayoutManager,
    demo_window_sizes: Vec<(String, Size)>,
    current_size_index: usize,
}

impl Default for Phase3Demo {
    fn default() -> Self {
        Self {
            keyboard_nav: KeyboardNavigationManager::new(),
            layout_manager: ResponsiveLayoutManager::new(),
            demo_window_sizes: vec![
                ("Mobile (600x400)".to_string(), Size::new(600.0, 400.0)),
                ("Tablet (900x600)".to_string(), Size::new(900.0, 600.0)),
                ("Desktop (1200x800)".to_string(), Size::new(1200.0, 800.0)),
                ("Large (1600x1000)".to_string(), Size::new(1600.0, 1000.0)),
            ],
            current_size_index: 2, // Start with desktop size
        }
    }
}

impl Phase3Demo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: DemoMessage) -> Task<DemoMessage> {
        match message {
            DemoMessage::KeyboardNavigation(kb_msg) => {
                match kb_msg {
                    KbNavMessage::KeyPressed(key, modifiers) => {
                        if let Some(action) = self.keyboard_nav.handle_key_press(key, modifiers) {
                            if let Some(_target) = self.keyboard_nav.execute_action(action) {
                                println!("Keyboard navigation: {:?}", action);
                            }
                        }
                    }
                    KbNavMessage::KeyReleased(key, modifiers) => {
                        self.keyboard_nav.handle_key_release(key, modifiers);
                    }
                    KbNavMessage::Focus(target) => {
                        self.keyboard_nav.tab_order.focus(target);
                        println!("Focused: {:?}", target);
                    }
                    KbNavMessage::ClearFocus => {
                        self.keyboard_nav.tab_order.clear();
                        println!("Focus cleared");
                    }
                    KbNavMessage::ToggleEnabled => {
                        self.keyboard_nav.set_enabled(!self.keyboard_nav.enabled);
                        println!("Keyboard navigation enabled: {}", self.keyboard_nav.enabled);
                    }
                    KbNavMessage::ToggleFocusIndicators => {
                        self.keyboard_nav.set_show_focus_indicators(!self.keyboard_nav.show_focus_indicators);
                        println!("Focus indicators: {}", self.keyboard_nav.show_focus_indicators);
                    }
                }
            }
            DemoMessage::ResponsiveLayout(layout_msg) => {
                match layout_msg {
                    ResponsiveLayoutMessage::WindowResized(size) => {
                        self.layout_manager.update_window_size(size);
                        println!("Window resized to: {:?}, Screen size: {:?}, Compact: {}", 
                                size, self.layout_manager.screen_size, self.layout_manager.compact_mode);
                    }
                    ResponsiveLayoutMessage::TogglePanel(panel_id) => {
                        self.layout_manager.toggle_panel(panel_id);
                        println!("Toggled panel: {:?}", panel_id);
                    }
                    ResponsiveLayoutMessage::TogglePanelMinimized(panel_id) => {
                        self.layout_manager.toggle_panel_minimized(panel_id);
                        println!("Toggled panel minimized: {:?}", panel_id);
                    }
                    ResponsiveLayoutMessage::ResizePanel(panel_id, width) => {
                        self.layout_manager.resize_panel(panel_id, width);
                        println!("Resized panel {:?} to width: {:.1}", panel_id, width);
                    }
                    ResponsiveLayoutMessage::ToggleCompactMode => {
                        if self.layout_manager.compact_mode {
                            self.layout_manager.exit_compact_mode();
                        } else {
                            self.layout_manager.enter_compact_mode();
                        }
                        println!("Compact mode: {}", self.layout_manager.compact_mode);
                    }
                }
            }
            DemoMessage::SimulateResize(size) => {
                return Task::done(DemoMessage::ResponsiveLayout(ResponsiveLayoutMessage::WindowResized(size)));
            }
            DemoMessage::ToggleKeyboardNav => {
                return Task::done(DemoMessage::KeyboardNavigation(KbNavMessage::ToggleEnabled));
            }
            DemoMessage::ToggleFocusIndicators => {
                return Task::done(DemoMessage::KeyboardNavigation(KbNavMessage::ToggleFocusIndicators));
            }
            DemoMessage::ToggleCompactMode => {
                return Task::done(DemoMessage::ResponsiveLayout(ResponsiveLayoutMessage::ToggleCompactMode));
            }
            DemoMessage::FocusTarget(target) => {
                return Task::done(DemoMessage::KeyboardNavigation(KbNavMessage::Focus(target)));
            }
            DemoMessage::TogglePanel(panel_id) => {
                return Task::done(DemoMessage::ResponsiveLayout(ResponsiveLayoutMessage::TogglePanel(panel_id)));
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<DemoMessage> {
        let title = text("Phase 3: Interactive Experience Optimization Demo")
            .size(24)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.0, 0.75, 1.0)),
            });

        let keyboard_nav_section = self.keyboard_navigation_section();
        let responsive_layout_section = self.responsive_layout_section();
        let demo_controls = self.demo_controls();

        container(
            column![
                title,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                keyboard_nav_section,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                responsive_layout_section,
                Space::new(Length::Fill, Length::Fixed(20.0)),
                demo_controls,
            ]
            .spacing(10)
            .padding(20)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn keyboard_navigation_section(&self) -> Element<DemoMessage> {
        let section_title = text("🎹 Keyboard Navigation")
            .size(18)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.0, 0.75, 1.0)),
            });

        let status_text = format!(
            "Enabled: {} | Focus Indicators: {} | Current Focus: {:?}",
            self.keyboard_nav.enabled,
            self.keyboard_nav.show_focus_indicators,
            self.keyboard_nav.get_focused_target()
        );

        let status = text(status_text).size(12);

        let controls = row![
            button("Toggle Navigation").on_press(DemoMessage::ToggleKeyboardNav),
            button("Toggle Focus Indicators").on_press(DemoMessage::ToggleFocusIndicators),
            button("Focus Menu").on_press(DemoMessage::FocusTarget(FocusTarget::MenuBar)),
            button("Focus Canvas").on_press(DemoMessage::FocusTarget(FocusTarget::Canvas)),
        ]
        .spacing(10);

        let instructions = text("Instructions: Use Tab/Shift+Tab to navigate, Enter to activate, Esc to cancel")
            .size(11)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
            });

        column![section_title, status, controls, instructions]
            .spacing(8)
            .into()
    }

    fn responsive_layout_section(&self) -> Element<DemoMessage> {
        let section_title = text("📱 Responsive Layout")
            .size(18)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.0, 0.75, 1.0)),
            });

        let layout_info = format!(
            "Screen: {:?} | Window: {:.0}x{:.0} | Compact: {} | Canvas Width: {:.0}",
            self.layout_manager.screen_size,
            self.layout_manager.window_size.width,
            self.layout_manager.window_size.height,
            self.layout_manager.compact_mode,
            self.layout_manager.get_canvas_width()
        );

        let status = text(layout_info).size(12);

        let size_buttons: Vec<Element<DemoMessage>> = self.demo_window_sizes
            .iter()
            .enumerate()
            .map(|(index, (name, size))| {
                let is_current = index == self.current_size_index;
                button(text(name).size(11))
                    .on_press(DemoMessage::SimulateResize(*size))
                    .style(if is_current {
                        button::primary
                    } else {
                        button::secondary
                    })
                    .into()
            })
            .collect();

        let size_controls = row(size_buttons).spacing(8);

        let panel_controls = row![
            button("Toggle Tool Panel").on_press(DemoMessage::TogglePanel(PanelId::ToolPanel)),
            button("Toggle Properties").on_press(DemoMessage::TogglePanel(PanelId::PropertiesPanel)),
            button("Toggle Layers").on_press(DemoMessage::TogglePanel(PanelId::LayersPanel)),
            button("Toggle Compact").on_press(DemoMessage::ToggleCompactMode),
        ]
        .spacing(10);

        let panel_status = format!(
            "Panels - Tool: {} | Properties: {} | Layers: {} | History: {}",
            if self.layout_manager.is_panel_expanded(PanelId::ToolPanel) { "Expanded" } else { "Hidden" },
            if self.layout_manager.is_panel_expanded(PanelId::PropertiesPanel) { "Expanded" } else { "Hidden" },
            if self.layout_manager.is_panel_expanded(PanelId::LayersPanel) { "Expanded" } else { "Hidden" },
            if self.layout_manager.is_panel_expanded(PanelId::HistoryPanel) { "Expanded" } else { "Hidden" },
        );

        let panel_info = text(panel_status).size(11);

        column![section_title, status, size_controls, panel_controls, panel_info]
            .spacing(8)
            .into()
    }

    fn demo_controls(&self) -> Element<DemoMessage> {
        let section_title = text("🎮 Demo Controls")
            .size(18)
            .style(|_theme| iced::widget::text::Style {
                color: Some(iced::Color::from_rgb(0.0, 0.75, 1.0)),
            });

        let instructions = column![
            text("Keyboard Navigation Features:").size(14),
            text("• Tab/Shift+Tab: Navigate between UI elements").size(11),
            text("• Enter: Activate focused element").size(11),
            text("• Escape: Cancel/close current operation").size(11),
            text("• Alt+M: Toggle menu bar activation").size(11),
            text("").size(8),
            text("Responsive Layout Features:").size(14),
            text("• Automatic panel hiding on small screens").size(11),
            text("• Panel resizing and minimize/maximize").size(11),
            text("• Adaptive canvas sizing").size(11),
            text("• Compact mode for mobile devices").size(11),
        ]
        .spacing(4);

        column![section_title, instructions]
            .spacing(8)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    iced::application("Phase 3 Demo - PSOC Interactive Experience", Phase3Demo::update, Phase3Demo::view)
        .theme(Phase3Demo::theme)
        .run()
}
