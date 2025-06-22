//! Keyboard navigation system for PSOC Image Editor
//! Provides comprehensive keyboard navigation and accessibility features

use iced::{keyboard, Element};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::shortcuts::{ShortcutKey, ShortcutModifiers, ShortcutAction};
use super::menu_system::MenuCategoryId;

/// Focus target for keyboard navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusTarget {
    /// Menu bar
    MenuBar,
    /// Specific menu category
    MenuCategory(MenuCategoryId),
    /// Tool panel
    ToolPanel,
    /// Canvas area
    Canvas,
    /// Properties panel
    PropertiesPanel,
    /// Layers panel
    LayersPanel,
    /// History panel
    HistoryPanel,
    /// Status bar
    StatusBar,
}

/// Tab order for focus navigation
#[derive(Debug, Clone)]
pub struct TabOrder {
    /// Ordered list of focusable targets
    pub targets: Vec<FocusTarget>,
    /// Current focus index
    pub current_index: Option<usize>,
}

impl Default for TabOrder {
    fn default() -> Self {
        Self {
            targets: vec![
                FocusTarget::MenuBar,
                FocusTarget::ToolPanel,
                FocusTarget::Canvas,
                FocusTarget::LayersPanel,
                FocusTarget::PropertiesPanel,
                FocusTarget::HistoryPanel,
                FocusTarget::StatusBar,
            ],
            current_index: None,
        }
    }
}

impl TabOrder {
    /// Move focus to next target
    pub fn next(&mut self) -> Option<FocusTarget> {
        if self.targets.is_empty() {
            return None;
        }

        let next_index = match self.current_index {
            Some(index) => (index + 1) % self.targets.len(),
            None => 0,
        };

        self.current_index = Some(next_index);
        Some(self.targets[next_index])
    }

    /// Move focus to previous target
    pub fn previous(&mut self) -> Option<FocusTarget> {
        if self.targets.is_empty() {
            return None;
        }

        let prev_index = match self.current_index {
            Some(index) => {
                if index == 0 {
                    self.targets.len() - 1
                } else {
                    index - 1
                }
            }
            None => self.targets.len() - 1,
        };

        self.current_index = Some(prev_index);
        Some(self.targets[prev_index])
    }

    /// Set focus to specific target
    pub fn focus(&mut self, target: FocusTarget) -> bool {
        if let Some(index) = self.targets.iter().position(|&t| t == target) {
            self.current_index = Some(index);
            true
        } else {
            false
        }
    }

    /// Get currently focused target
    pub fn current(&self) -> Option<FocusTarget> {
        self.current_index.map(|index| self.targets[index])
    }

    /// Clear focus
    pub fn clear(&mut self) {
        self.current_index = None;
    }
}

/// Keyboard navigation manager
#[derive(Debug, Clone)]
pub struct KeyboardNavigationManager {
    /// Tab order for focus navigation
    pub tab_order: TabOrder,
    /// Whether keyboard navigation is enabled
    pub enabled: bool,
    /// Whether to show focus indicators
    pub show_focus_indicators: bool,
    /// Custom key bindings for navigation
    pub key_bindings: HashMap<(ShortcutKey, ShortcutModifiers), NavigationAction>,
    /// Whether Alt key is currently pressed (for menu activation)
    pub alt_pressed: bool,
}

/// Navigation actions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationAction {
    /// Move focus to next element
    FocusNext,
    /// Move focus to previous element
    FocusPrevious,
    /// Activate current element
    Activate,
    /// Cancel/escape current operation
    Cancel,
    /// Focus specific target
    FocusTarget(FocusTarget),
    /// Toggle menu bar activation
    ToggleMenuBar,
}

impl Default for KeyboardNavigationManager {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        
        // Standard navigation keys
        key_bindings.insert(
            (ShortcutKey::Tab, ShortcutModifiers::new()),
            NavigationAction::FocusNext,
        );
        key_bindings.insert(
            (ShortcutKey::Tab, ShortcutModifiers::new().with_shift(true)),
            NavigationAction::FocusPrevious,
        );
        key_bindings.insert(
            (ShortcutKey::Enter, ShortcutModifiers::new()),
            NavigationAction::Activate,
        );
        key_bindings.insert(
            (ShortcutKey::Escape, ShortcutModifiers::new()),
            NavigationAction::Cancel,
        );
        
        // Alt key for menu activation
        key_bindings.insert(
            (ShortcutKey::Character('m'), ShortcutModifiers::new().with_alt(true)),
            NavigationAction::ToggleMenuBar,
        );

        Self {
            tab_order: TabOrder::default(),
            enabled: true,
            show_focus_indicators: true,
            key_bindings,
            alt_pressed: false,
        }
    }
}

impl KeyboardNavigationManager {
    /// Create a new keyboard navigation manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Handle keyboard input
    pub fn handle_key_press(&mut self, key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<NavigationAction> {
        if !self.enabled {
            return None;
        }

        // Convert iced types to our types
        let shortcut_key = match key {
            keyboard::Key::Character(c) => {
                c.chars().next().map(|ch| ShortcutKey::Character(ch.to_ascii_lowercase()))
            }
            keyboard::Key::Named(named) => match named {
                keyboard::key::Named::Tab => Some(ShortcutKey::Tab),
                keyboard::key::Named::Enter => Some(ShortcutKey::Enter),
                keyboard::key::Named::Escape => Some(ShortcutKey::Escape),
                keyboard::key::Named::ArrowUp => Some(ShortcutKey::ArrowUp),
                keyboard::key::Named::ArrowDown => Some(ShortcutKey::ArrowDown),
                keyboard::key::Named::ArrowLeft => Some(ShortcutKey::ArrowLeft),
                keyboard::key::Named::ArrowRight => Some(ShortcutKey::ArrowRight),
                _ => None,
            },
            _ => None,
        };

        let shortcut_modifiers = ShortcutModifiers {
            ctrl: modifiers.command(),
            shift: modifiers.shift(),
            alt: modifiers.alt(),
            meta: false, // Add the missing meta field
        };

        // Check for Alt key press for menu activation
        if shortcut_modifiers.alt && !self.alt_pressed {
            self.alt_pressed = true;
            return Some(NavigationAction::ToggleMenuBar);
        }

        if let Some(key) = shortcut_key {
            self.key_bindings.get(&(key, shortcut_modifiers)).copied()
        } else {
            None
        }
    }

    /// Handle key release
    pub fn handle_key_release(&mut self, key: keyboard::Key, _modifiers: keyboard::Modifiers) {
        // Reset Alt key state
        if matches!(key, keyboard::Key::Named(keyboard::key::Named::Alt)) {
            self.alt_pressed = false;
        }
    }

    /// Execute navigation action
    pub fn execute_action(&mut self, action: NavigationAction) -> Option<FocusTarget> {
        match action {
            NavigationAction::FocusNext => self.tab_order.next(),
            NavigationAction::FocusPrevious => self.tab_order.previous(),
            NavigationAction::FocusTarget(target) => {
                self.tab_order.focus(target);
                Some(target)
            }
            NavigationAction::Cancel => {
                self.tab_order.clear();
                None
            }
            NavigationAction::ToggleMenuBar => {
                if self.tab_order.current() == Some(FocusTarget::MenuBar) {
                    self.tab_order.clear();
                    None
                } else {
                    self.tab_order.focus(FocusTarget::MenuBar);
                    Some(FocusTarget::MenuBar)
                }
            }
            NavigationAction::Activate => {
                // Return current focus for activation
                self.tab_order.current()
            }
        }
    }

    /// Get currently focused target
    pub fn get_focused_target(&self) -> Option<FocusTarget> {
        self.tab_order.current()
    }

    /// Check if target is focused
    pub fn is_focused(&self, target: FocusTarget) -> bool {
        self.tab_order.current() == Some(target)
    }

    /// Enable/disable keyboard navigation
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.tab_order.clear();
        }
    }

    /// Enable/disable focus indicators
    pub fn set_show_focus_indicators(&mut self, show: bool) {
        self.show_focus_indicators = show;
    }

    /// Add custom key binding
    pub fn add_key_binding(&mut self, key: ShortcutKey, modifiers: ShortcutModifiers, action: NavigationAction) {
        self.key_bindings.insert((key, modifiers), action);
    }

    /// Remove key binding
    pub fn remove_key_binding(&mut self, key: ShortcutKey, modifiers: ShortcutModifiers) {
        self.key_bindings.remove(&(key, modifiers));
    }
}

/// Messages for keyboard navigation
#[derive(Debug, Clone)]
pub enum KeyboardNavigationMessage {
    /// Key was pressed
    KeyPressed(keyboard::Key, keyboard::Modifiers),
    /// Key was released
    KeyReleased(keyboard::Key, keyboard::Modifiers),
    /// Focus target
    Focus(FocusTarget),
    /// Clear focus
    ClearFocus,
    /// Toggle navigation enabled state
    ToggleEnabled,
    /// Toggle focus indicators
    ToggleFocusIndicators,
}
