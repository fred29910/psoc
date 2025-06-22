//! Responsive layout system for PSOC Image Editor
//! Provides adaptive UI components that respond to screen size and user preferences

use iced::{
    widget::{container, row, column, button, text, Space},
    Element, Length, Size, Point,
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::ui::theme::{PsocTheme, spacing};

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

/// Responsive layout manager
#[derive(Debug, Clone)]
pub struct ResponsiveLayoutManager {
    /// Current screen size
    pub screen_size: ScreenSize,
    /// Panel states
    pub panel_states: HashMap<PanelId, PanelState>,
    /// Window size
    pub window_size: Size,
    /// Whether layout is in compact mode
    pub compact_mode: bool,
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

    /// Enter compact mode (hide/minimize panels)
    pub fn enter_compact_mode(&mut self) {
        self.compact_mode = true;
        
        // Hide secondary panels on small screens
        if matches!(self.screen_size, ScreenSize::Small) {
            self.panel_states.get_mut(&PanelId::LayersPanel)
                .map(|state| state.visible = false);
            self.panel_states.get_mut(&PanelId::HistoryPanel)
                .map(|state| state.visible = false);
        }
        
        // Minimize remaining panels
        for state in self.panel_states.values_mut() {
            if state.visible {
                state.minimized = true;
            }
        }
    }

    /// Exit compact mode (restore panels)
    pub fn exit_compact_mode(&mut self) {
        self.compact_mode = false;
        
        // Restore panel visibility
        for state in self.panel_states.values_mut() {
            state.visible = true;
            state.minimized = false;
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

    /// Toggle panel visibility
    pub fn toggle_panel(&mut self, panel_id: PanelId) {
        if let Some(state) = self.panel_states.get_mut(&panel_id) {
            state.visible = !state.visible;
        }
    }

    /// Toggle panel minimized state
    pub fn toggle_panel_minimized(&mut self, panel_id: PanelId) {
        if let Some(state) = self.panel_states.get_mut(&panel_id) {
            if state.visible {
                state.minimized = !state.minimized;
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
        
        (self.window_size.width - left_panel_width - right_panel_width - spacing::MD * 3.0).max(300.0)
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
