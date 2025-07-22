//! Responsive layout system for PSOC Image Editor
//! Provides adaptive UI components that respond to screen size and user preferences

use iced::{
    widget::{container, row, column, button, text, Space},
    Element, Length, Size, Point,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::ui::theme::PsocTheme;
use crate::ui::animations::{TransitionManager, AnimationDirection};

/// Screen size breakpoints for responsive design
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScreenSize {
    /// Small screens (< 768px width)
    Small,
    /// Medium screens (768px - 1024px width)
    Medium,
    /// Large screens (1024px - 1440px width)
    Large,
    /// Extra large screens (> 1440px width)
    ExtraLarge,
}

impl ScreenSize {
    /// Determine screen size from window dimensions
    pub fn from_window_size(size: Size) -> Self {
        match size.width {
            w if w < 768.0 => ScreenSize::Small,
            w if w < 1024.0 => ScreenSize::Medium,
            w if w < 1440.0 => ScreenSize::Large,
            _ => ScreenSize::ExtraLarge,
        }
    }

    /// Get recommended panel width for this screen size
    pub fn panel_width(&self) -> f32 {
        match self {
            ScreenSize::Small => 200.0,
            ScreenSize::Medium => 250.0,
            ScreenSize::Large => 280.0,
            ScreenSize::ExtraLarge => 320.0,
        }
    }

    /// Whether panels should be collapsible on this screen size
    pub fn should_collapse_panels(&self) -> bool {
        matches!(self, ScreenSize::Small | ScreenSize::Medium)
    }
}

/// Panel state for responsive behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    /// Whether the panel is visible
    pub visible: bool,
    /// Whether the panel is minimized
    pub minimized: bool,
    /// Current panel width
    pub width: f32,
    /// Current panel height
    pub height: f32,
    /// Minimum allowed width
    pub min_width: f32,
    /// Maximum allowed width
    pub max_width: f32,
    /// Whether the panel can be resized
    pub resizable: bool,
    /// Panel position (for floating panels) - using (f32, f32) instead of Point for serialization
    pub position: Option<(f32, f32)>,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            visible: true,
            minimized: false,
            width: 250.0,
            height: 400.0,
            min_width: 150.0,
            max_width: 400.0,
            resizable: true,
            position: None,
        }
    }
}

/// Panel identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelId {
    /// Left tool panel
    ToolPanel,
    /// Right properties panel
    PropertiesPanel,
    /// Layers panel
    LayersPanel,
    /// History panel
    HistoryPanel,
}

/// Toolbar layout modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarLayout {
    /// Full horizontal toolbar
    Full,
    /// Compact horizontal toolbar
    Compact,
    /// Vertical toolbar for small screens
    Vertical,
}

/// Enhanced responsive layout manager with animation support
#[derive(Debug)]
pub struct ResponsiveLayoutManager {
    /// Current screen size
    pub screen_size: ScreenSize,
    /// Panel states
    pub panel_states: HashMap<PanelId, PanelState>,
    /// Window size
    pub window_size: Size,
    /// Whether layout is in compact mode
    pub compact_mode: bool,
    /// Transition animation manager
    pub transition_manager: TransitionManager,
    /// Whether animations are enabled
    pub animations_enabled: bool,
}

impl Default for ResponsiveLayoutManager {
    fn default() -> Self {
        let mut panel_states = HashMap::new();

        // Initialize default panel states
        panel_states.insert(PanelId::ToolPanel, PanelState {
            width: 200.0,
            min_width: 150.0,
            max_width: 300.0,
            ..Default::default()
        });

        panel_states.insert(PanelId::PropertiesPanel, PanelState {
            width: 250.0,
            min_width: 200.0,
            max_width: 350.0,
            ..Default::default()
        });

        panel_states.insert(PanelId::LayersPanel, PanelState {
            width: 250.0,
            min_width: 200.0,
            max_width: 350.0,
            ..Default::default()
        });

        panel_states.insert(PanelId::HistoryPanel, PanelState {
            width: 200.0,
            min_width: 150.0,
            max_width: 300.0,
            ..Default::default()
        });

        Self {
            screen_size: ScreenSize::Large,
            panel_states,
            window_size: Size::new(1200.0, 800.0),
            compact_mode: false,
            transition_manager: TransitionManager::new(),
            animations_enabled: true,
        }
    }
}

impl ResponsiveLayoutManager {
    /// Create a new responsive layout manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Update window size and recalculate layout
    pub fn update_window_size(&mut self, size: Size) {
        self.window_size = size;
        self.screen_size = ScreenSize::from_window_size(size);
        
        // Auto-adjust panel visibility based on screen size
        if self.screen_size.should_collapse_panels() && !self.compact_mode {
            self.enter_compact_mode();
        } else if !self.screen_size.should_collapse_panels() && self.compact_mode {
            self.exit_compact_mode();
        }
        
        // Adjust panel widths for screen size
        self.adjust_panel_widths();
    }

    /// Enter compact mode (hide/minimize panels) with animation
    pub fn enter_compact_mode(&mut self) {
        self.compact_mode = true;

        // Hide secondary panels on small screens with slide animation
        if matches!(self.screen_size, ScreenSize::Small) {
            if let Some(state) = self.panel_states.get_mut(&PanelId::LayersPanel) {
                if state.visible && self.animations_enabled {
                    self.transition_manager.start_panel_slide(
                        "layers_panel".to_string(),
                        AnimationDirection::Right,
                        state.width,
                    );
                }
                state.visible = false;
            }

            if let Some(state) = self.panel_states.get_mut(&PanelId::HistoryPanel) {
                if state.visible && self.animations_enabled {
                    self.transition_manager.start_panel_slide(
                        "history_panel".to_string(),
                        AnimationDirection::Right,
                        state.width,
                    );
                }
                state.visible = false;
            }
        }

        // Minimize remaining panels with collapse animation
        for (panel_id, state) in &mut self.panel_states {
            if state.visible && !state.minimized {
                if self.animations_enabled {
                    let panel_name = format!("{:?}_panel", panel_id).to_lowercase();
                    self.transition_manager.start_panel_expand(
                        panel_name,
                        false, // collapsing
                        Size::new(state.width, state.height),
                    );
                }
                state.minimized = true;
            }
        }
    }

    /// Exit compact mode (restore panels) with animation
    pub fn exit_compact_mode(&mut self) {
        self.compact_mode = false;

        // Restore panel visibility with animations
        for (panel_id, state) in &mut self.panel_states {
            if !state.visible {
                if self.animations_enabled {
                    let panel_name = format!("{:?}_panel", panel_id).to_lowercase();
                    self.transition_manager.start_panel_slide(
                        panel_name,
                        AnimationDirection::Left,
                        state.width,
                    );
                }
                state.visible = true;
            }

            if state.minimized {
                if self.animations_enabled {
                    let panel_name = format!("{:?}_panel", panel_id).to_lowercase();
                    self.transition_manager.start_panel_expand(
                        panel_name,
                        true, // expanding
                        Size::new(state.width, state.height),
                    );
                }
                state.minimized = false;
            }
        }
    }

    /// Adjust panel widths based on screen size
    fn adjust_panel_widths(&mut self) {
        let recommended_width = self.screen_size.panel_width();
        
        for state in self.panel_states.values_mut() {
            if state.resizable {
                state.width = recommended_width.clamp(state.min_width, state.max_width);
            }
        }
    }

    /// Toggle panel visibility with animation
    pub fn toggle_panel(&mut self, panel_id: PanelId) {
        if let Some(state) = self.panel_states.get_mut(&panel_id) {
            let was_visible = state.visible;
            state.visible = !state.visible;

            if self.animations_enabled {
                let panel_name = format!("{:?}_panel", panel_id).to_lowercase();
                if state.visible {
                    // Panel becoming visible - slide in
                    self.transition_manager.start_panel_slide(
                        panel_name,
                        AnimationDirection::Left,
                        state.width,
                    );
                } else {
                    // Panel becoming hidden - slide out
                    self.transition_manager.start_panel_slide(
                        panel_name,
                        AnimationDirection::Right,
                        state.width,
                    );
                }
            }
        }
    }

    /// Toggle panel minimized state with animation
    pub fn toggle_panel_minimized(&mut self, panel_id: PanelId) {
        if let Some(state) = self.panel_states.get_mut(&panel_id) {
            if state.visible {
                state.minimized = !state.minimized;

                if self.animations_enabled {
                    let panel_name = format!("{:?}_panel", panel_id).to_lowercase();
                    self.transition_manager.start_panel_expand(
                        panel_name,
                        !state.minimized, // expanding if not minimized
                        Size::new(state.width, state.height),
                    );
                }
            }
        }
    }

    /// Resize panel
    pub fn resize_panel(&mut self, panel_id: PanelId, new_width: f32) {
        if let Some(state) = self.panel_states.get_mut(&panel_id) {
            if state.resizable {
                state.width = new_width.clamp(state.min_width, state.max_width);
            }
        }
    }

    /// Get panel state
    pub fn get_panel_state(&self, panel_id: PanelId) -> Option<&PanelState> {
        self.panel_states.get(&panel_id)
    }

    /// Check if panel is visible and not minimized
    pub fn is_panel_expanded(&self, panel_id: PanelId) -> bool {
        self.panel_states.get(&panel_id)
            .map(|state| state.visible && !state.minimized)
            .unwrap_or(false)
    }

    /// Get effective panel width (0 if hidden/minimized)
    pub fn get_effective_panel_width(&self, panel_id: PanelId) -> f32 {
        if self.is_panel_expanded(panel_id) {
            self.panel_states.get(&panel_id)
                .map(|state| state.width)
                .unwrap_or(0.0)
        } else {
            0.0
        }
    }

    /// Calculate available canvas width
    pub fn get_canvas_width(&self) -> f32 {
        let left_panel_width = self.get_effective_panel_width(PanelId::ToolPanel);
        let right_panel_width = self.get_effective_panel_width(PanelId::PropertiesPanel);

        (self.window_size.width - left_panel_width - right_panel_width - 48.0).max(300.0) // Using fixed spacing
    }

    /// Update animations and return whether any are active
    pub fn update_animations(&mut self) -> bool {
        self.transition_manager.update()
    }

    /// Get current animation state for a panel
    pub fn get_panel_animation_state(&self, panel_id: PanelId) -> Option<crate::ui::animations::TransitionState> {
        let panel_name = format!("{:?}_panel", panel_id).to_lowercase();
        self.transition_manager.get_current_state(&panel_name)
    }

    /// Enable or disable animations
    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.animations_enabled = enabled;
        if !enabled {
            self.transition_manager.stop_all_animations();
        }
    }

    /// Check if any panel animations are active
    pub fn has_active_animations(&self) -> bool {
        self.transition_manager.active_animation_count() > 0
    }

    /// Get recommended toolbar layout
    pub fn get_toolbar_layout(&self) -> ToolbarLayout {
        match self.screen_size {
            ScreenSize::Small => ToolbarLayout::Vertical,
            ScreenSize::Medium => ToolbarLayout::Compact,
            _ => ToolbarLayout::Full,
        }
    }

    /// Check if layout should use compact mode
    pub fn should_use_compact_mode(&self) -> bool {
        self.screen_size.should_collapse_panels()
    }
}

/// Messages for responsive layout
#[derive(Debug, Clone)]
pub enum ResponsiveLayoutMessage {
    /// Window was resized
    WindowResized(Size),
    /// Toggle panel visibility
    TogglePanel(PanelId),
    /// Toggle panel minimized state
    TogglePanelMinimized(PanelId),
    /// Resize panel
    ResizePanel(PanelId, f32),
    /// Enter/exit compact mode
    ToggleCompactMode,
}
