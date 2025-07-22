//! Smart panel system with intelligent folding and responsive behavior
//! Provides automatic panel management based on screen size and user preferences

use std::collections::HashMap;
use std::time::Instant;

use iced::{Size, Point};
use serde::{Deserialize, Serialize};

use crate::ui::animations::{TransitionManager, AnimationDirection};
use super::responsive_layout::{PanelId, PanelState, ScreenSize};

/// Smart panel folding strategy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FoldingStrategy {
    /// Never fold panels automatically
    Never,
    /// Fold panels when screen becomes small
    OnSmallScreen,
    /// Fold panels when canvas area becomes too small
    OnSmallCanvas,
    /// Fold panels based on usage patterns
    Adaptive,
    /// Always keep panels folded
    Always,
}

/// Panel priority for folding decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PanelPriority {
    /// Essential panels (never fold unless absolutely necessary)
    Essential = 0,
    /// Important panels (fold on small screens)
    Important = 1,
    /// Secondary panels (fold when canvas space is limited)
    Secondary = 2,
    /// Optional panels (fold early to save space)
    Optional = 3,
}

/// Panel usage statistics for adaptive folding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelUsageStats {
    /// Number of times panel was accessed
    pub access_count: u32,
    /// Last access time (as timestamp)
    #[serde(skip)]
    pub last_access: Option<Instant>,
    /// Average session duration
    pub avg_session_duration: f32,
    /// User preference score (0.0 to 1.0)
    pub preference_score: f32,
}

impl Default for PanelUsageStats {
    fn default() -> Self {
        Self {
            access_count: 0,
            last_access: None,
            avg_session_duration: 0.0,
            preference_score: 0.5, // Neutral preference
        }
    }
}

/// Smart panel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPanelConfig {
    /// Panel priority
    pub priority: PanelPriority,
    /// Minimum canvas width before folding this panel
    pub min_canvas_width: f32,
    /// Whether panel can be auto-folded
    pub can_auto_fold: bool,
    /// Whether panel should remember its state
    pub remember_state: bool,
    /// Custom folding threshold
    pub custom_threshold: Option<f32>,
}

impl Default for SmartPanelConfig {
    fn default() -> Self {
        Self {
            priority: PanelPriority::Secondary,
            min_canvas_width: 400.0,
            can_auto_fold: true,
            remember_state: true,
            custom_threshold: None,
        }
    }
}

/// Smart panel system manager
#[derive(Debug)]
pub struct SmartPanelSystem {
    /// Panel configurations
    panel_configs: HashMap<PanelId, SmartPanelConfig>,
    /// Panel usage statistics
    usage_stats: HashMap<PanelId, PanelUsageStats>,
    /// Current folding strategy
    folding_strategy: FoldingStrategy,
    /// Transition manager for animations
    transition_manager: TransitionManager,
    /// Whether animations are enabled
    animations_enabled: bool,
    /// Minimum canvas width threshold
    min_canvas_width: f32,
    /// Current screen size
    current_screen_size: ScreenSize,
    /// Whether adaptive learning is enabled
    adaptive_learning: bool,
}

impl Default for SmartPanelSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartPanelSystem {
    /// Create a new smart panel system
    pub fn new() -> Self {
        let mut panel_configs = HashMap::new();
        
        // Configure default panel priorities
        panel_configs.insert(PanelId::ToolPanel, SmartPanelConfig {
            priority: PanelPriority::Essential,
            min_canvas_width: 300.0,
            can_auto_fold: false, // Tool panel is essential
            remember_state: true,
            custom_threshold: None,
        });
        
        panel_configs.insert(PanelId::LayersPanel, SmartPanelConfig {
            priority: PanelPriority::Important,
            min_canvas_width: 500.0,
            can_auto_fold: true,
            remember_state: true,
            custom_threshold: None,
        });
        
        panel_configs.insert(PanelId::PropertiesPanel, SmartPanelConfig {
            priority: PanelPriority::Secondary,
            min_canvas_width: 600.0,
            can_auto_fold: true,
            remember_state: true,
            custom_threshold: None,
        });
        
        panel_configs.insert(PanelId::HistoryPanel, SmartPanelConfig {
            priority: PanelPriority::Optional,
            min_canvas_width: 700.0,
            can_auto_fold: true,
            remember_state: true,
            custom_threshold: None,
        });

        Self {
            panel_configs,
            usage_stats: HashMap::new(),
            folding_strategy: FoldingStrategy::OnSmallCanvas,
            transition_manager: TransitionManager::new(),
            animations_enabled: true,
            min_canvas_width: 400.0,
            current_screen_size: ScreenSize::Large,
            adaptive_learning: true,
        }
    }

    /// Update screen size and trigger responsive adjustments
    pub fn update_screen_size(&mut self, screen_size: ScreenSize, window_size: Size) -> Vec<PanelFoldingAction> {
        self.current_screen_size = screen_size;
        
        match self.folding_strategy {
            FoldingStrategy::Never => Vec::new(),
            FoldingStrategy::Always => self.fold_all_panels(),
            FoldingStrategy::OnSmallScreen => {
                if matches!(screen_size, ScreenSize::Small) {
                    self.fold_panels_by_priority()
                } else {
                    self.unfold_panels_by_priority()
                }
            },
            FoldingStrategy::OnSmallCanvas => {
                self.fold_panels_for_canvas_width(window_size.width)
            },
            FoldingStrategy::Adaptive => {
                self.adaptive_panel_folding(window_size)
            },
        }
    }

    /// Calculate optimal panel layout for given canvas requirements
    pub fn calculate_optimal_layout(&self, required_canvas_width: f32, window_width: f32) -> Vec<PanelFoldingAction> {
        let mut actions = Vec::new();
        let mut available_width = window_width;
        
        // Sort panels by priority (essential first)
        let mut panels: Vec<_> = self.panel_configs.iter().collect();
        panels.sort_by_key(|(_, config)| config.priority);
        
        for (panel_id, config) in panels {
            if config.can_auto_fold {
                let panel_width = 200.0; // Default panel width
                
                if available_width - panel_width < required_canvas_width {
                    // Need to fold this panel
                    actions.push(PanelFoldingAction::Fold(*panel_id));
                } else {
                    // Can keep this panel open
                    actions.push(PanelFoldingAction::Unfold(*panel_id));
                    available_width -= panel_width;
                }
            }
        }
        
        actions
    }

    /// Record panel access for adaptive learning
    pub fn record_panel_access(&mut self, panel_id: PanelId) {
        if !self.adaptive_learning {
            return;
        }

        // First, calculate total accesses
        let total_accesses: u32 = self.usage_stats.values().map(|s| s.access_count).sum::<u32>() + 1; // +1 for current access

        // Then update the specific panel stats
        let stats = self.usage_stats.entry(panel_id).or_default();
        stats.access_count += 1;
        stats.last_access = Some(Instant::now());

        // Update preference score based on usage frequency
        if total_accesses > 0 {
            stats.preference_score = (stats.access_count as f32) / (total_accesses as f32);
        }
    }

    /// Get panel folding recommendations based on usage patterns
    pub fn get_adaptive_recommendations(&self) -> Vec<PanelFoldingAction> {
        let mut actions = Vec::new();
        
        for (panel_id, stats) in &self.usage_stats {
            if let Some(config) = self.panel_configs.get(panel_id) {
                if config.can_auto_fold {
                    // Fold panels with low usage
                    if stats.preference_score < 0.1 && stats.access_count < 5 {
                        actions.push(PanelFoldingAction::Fold(*panel_id));
                    } else if stats.preference_score > 0.3 {
                        actions.push(PanelFoldingAction::Unfold(*panel_id));
                    }
                }
            }
        }
        
        actions
    }

    /// Fold all panels that can be auto-folded
    fn fold_all_panels(&self) -> Vec<PanelFoldingAction> {
        self.panel_configs
            .iter()
            .filter(|(_, config)| config.can_auto_fold)
            .map(|(panel_id, _)| PanelFoldingAction::Fold(*panel_id))
            .collect()
    }

    /// Fold panels by priority (lowest priority first)
    fn fold_panels_by_priority(&self) -> Vec<PanelFoldingAction> {
        let mut panels: Vec<_> = self.panel_configs
            .iter()
            .filter(|(_, config)| config.can_auto_fold)
            .collect();
        
        // Sort by priority (highest priority last, so they get folded last)
        panels.sort_by_key(|(_, config)| std::cmp::Reverse(config.priority));
        
        panels
            .into_iter()
            .map(|(panel_id, _)| PanelFoldingAction::Fold(*panel_id))
            .collect()
    }

    /// Unfold panels by priority (highest priority first)
    fn unfold_panels_by_priority(&self) -> Vec<PanelFoldingAction> {
        let mut panels: Vec<_> = self.panel_configs
            .iter()
            .filter(|(_, config)| config.can_auto_fold)
            .collect();
        
        // Sort by priority (highest priority first)
        panels.sort_by_key(|(_, config)| config.priority);
        
        panels
            .into_iter()
            .map(|(panel_id, _)| PanelFoldingAction::Unfold(*panel_id))
            .collect()
    }

    /// Fold panels based on canvas width requirements
    fn fold_panels_for_canvas_width(&self, window_width: f32) -> Vec<PanelFoldingAction> {
        let mut actions = Vec::new();
        let mut available_width = window_width;
        
        // Sort panels by priority (lowest priority first for folding)
        let mut panels: Vec<_> = self.panel_configs.iter().collect();
        panels.sort_by_key(|(_, config)| std::cmp::Reverse(config.priority));
        
        for (panel_id, config) in panels {
            if config.can_auto_fold {
                let required_canvas_width = config.min_canvas_width;
                let panel_width = 200.0; // Default panel width
                
                if available_width - panel_width < required_canvas_width {
                    actions.push(PanelFoldingAction::Fold(*panel_id));
                } else {
                    actions.push(PanelFoldingAction::Unfold(*panel_id));
                    available_width -= panel_width;
                }
            }
        }
        
        actions
    }

    /// Adaptive panel folding based on usage patterns and screen size
    fn adaptive_panel_folding(&self, window_size: Size) -> Vec<PanelFoldingAction> {
        let mut actions = Vec::new();
        
        // Combine screen size and usage pattern recommendations
        let screen_actions = self.fold_panels_for_canvas_width(window_size.width);
        let usage_actions = self.get_adaptive_recommendations();
        
        // Merge actions, prioritizing usage patterns for frequently used panels
        for panel_id in self.panel_configs.keys() {
            let has_screen_fold = screen_actions.iter().any(|a| matches!(a, PanelFoldingAction::Fold(id) if id == panel_id));
            let has_usage_unfold = usage_actions.iter().any(|a| matches!(a, PanelFoldingAction::Unfold(id) if id == panel_id));
            
            if has_screen_fold && !has_usage_unfold {
                actions.push(PanelFoldingAction::Fold(*panel_id));
            } else if has_usage_unfold {
                actions.push(PanelFoldingAction::Unfold(*panel_id));
            }
        }
        
        actions
    }

    /// Set folding strategy
    pub fn set_folding_strategy(&mut self, strategy: FoldingStrategy) {
        self.folding_strategy = strategy;
    }

    /// Get current folding strategy
    pub fn get_folding_strategy(&self) -> FoldingStrategy {
        self.folding_strategy
    }

    /// Enable or disable adaptive learning
    pub fn set_adaptive_learning(&mut self, enabled: bool) {
        self.adaptive_learning = enabled;
    }

    /// Update panel configuration
    pub fn update_panel_config(&mut self, panel_id: PanelId, config: SmartPanelConfig) {
        self.panel_configs.insert(panel_id, config);
    }

    /// Get panel configuration
    pub fn get_panel_config(&self, panel_id: PanelId) -> Option<&SmartPanelConfig> {
        self.panel_configs.get(&panel_id)
    }

    /// Get panel usage statistics
    pub fn get_usage_stats(&self, panel_id: PanelId) -> Option<&PanelUsageStats> {
        self.usage_stats.get(&panel_id)
    }

    /// Update animations and return whether any are active
    pub fn update_animations(&mut self) -> bool {
        self.transition_manager.update()
    }

    /// Enable or disable animations
    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.animations_enabled = enabled;
        if !enabled {
            self.transition_manager.stop_all_animations();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_panel_system_creation() {
        let system = SmartPanelSystem::new();

        // Should have default configurations
        assert!(system.panel_configs.len() > 0);
        assert_eq!(system.folding_strategy, FoldingStrategy::OnSmallCanvas);
        assert!(system.animations_enabled);
        assert!(system.adaptive_learning);
    }

    #[test]
    fn test_panel_priorities() {
        let system = SmartPanelSystem::new();

        // Tool panel should be essential
        let tool_config = system.get_panel_config(PanelId::ToolPanel).unwrap();
        assert_eq!(tool_config.priority, PanelPriority::Essential);
        assert!(!tool_config.can_auto_fold);

        // History panel should be optional
        let history_config = system.get_panel_config(PanelId::HistoryPanel).unwrap();
        assert_eq!(history_config.priority, PanelPriority::Optional);
        assert!(history_config.can_auto_fold);
    }

    #[test]
    fn test_folding_strategies() {
        let mut system = SmartPanelSystem::new();

        // Test different strategies
        system.set_folding_strategy(FoldingStrategy::Never);
        assert_eq!(system.get_folding_strategy(), FoldingStrategy::Never);

        system.set_folding_strategy(FoldingStrategy::Always);
        assert_eq!(system.get_folding_strategy(), FoldingStrategy::Always);
    }

    #[test]
    fn test_panel_access_recording() {
        let mut system = SmartPanelSystem::new();

        // Record some accesses
        system.record_panel_access(PanelId::LayersPanel);
        system.record_panel_access(PanelId::LayersPanel);
        system.record_panel_access(PanelId::HistoryPanel);

        // Check usage stats
        let layers_stats = system.get_usage_stats(PanelId::LayersPanel).unwrap();
        assert_eq!(layers_stats.access_count, 2);

        let history_stats = system.get_usage_stats(PanelId::HistoryPanel).unwrap();
        assert_eq!(history_stats.access_count, 1);
    }

    #[test]
    fn test_optimal_layout_calculation() {
        let system = SmartPanelSystem::new();

        // Test with limited window width
        let actions = system.calculate_optimal_layout(400.0, 800.0);

        // Should have some folding actions
        assert!(actions.len() > 0);

        // Check action types
        for action in &actions {
            match action {
                PanelFoldingAction::Fold(panel_id) => {
                    assert!(system.get_panel_config(*panel_id).unwrap().can_auto_fold);
                },
                PanelFoldingAction::Unfold(_) => {},
                PanelFoldingAction::Toggle(_) => {},
            }
        }
    }

    #[test]
    fn test_screen_size_adaptation() {
        let mut system = SmartPanelSystem::new();

        // Test small screen adaptation
        let actions = system.update_screen_size(ScreenSize::Small, Size::new(600.0, 400.0));

        // Should have folding actions for small screen
        assert!(actions.iter().any(|a| a.is_folding()));

        // Test large screen adaptation
        let actions = system.update_screen_size(ScreenSize::Large, Size::new(1200.0, 800.0));

        // Should have unfolding actions for large screen
        assert!(actions.iter().any(|a| a.is_unfolding()));
    }

    #[test]
    fn test_adaptive_recommendations() {
        let mut system = SmartPanelSystem::new();

        // Record heavy usage for one panel
        for _ in 0..10 {
            system.record_panel_access(PanelId::LayersPanel);
        }

        // Record light usage for another panel
        system.record_panel_access(PanelId::HistoryPanel);

        let recommendations = system.get_adaptive_recommendations();

        // Should recommend keeping frequently used panel
        assert!(recommendations.iter().any(|action|
            matches!(action, PanelFoldingAction::Unfold(PanelId::LayersPanel))
        ));
    }

    #[test]
    fn test_panel_config_update() {
        let mut system = SmartPanelSystem::new();

        let new_config = SmartPanelConfig {
            priority: PanelPriority::Important,
            min_canvas_width: 500.0,
            can_auto_fold: false,
            remember_state: true,
            custom_threshold: Some(600.0),
        };

        system.update_panel_config(PanelId::LayersPanel, new_config.clone());

        let updated_config = system.get_panel_config(PanelId::LayersPanel).unwrap();
        assert_eq!(updated_config.priority, PanelPriority::Important);
        assert_eq!(updated_config.min_canvas_width, 500.0);
        assert!(!updated_config.can_auto_fold);
    }

    #[test]
    fn test_panel_folding_actions() {
        let fold_action = PanelFoldingAction::Fold(PanelId::LayersPanel);
        let unfold_action = PanelFoldingAction::Unfold(PanelId::HistoryPanel);
        let toggle_action = PanelFoldingAction::Toggle(PanelId::PropertiesPanel);

        // Test action properties
        assert_eq!(fold_action.panel_id(), PanelId::LayersPanel);
        assert!(fold_action.is_folding());
        assert!(!fold_action.is_unfolding());

        assert_eq!(unfold_action.panel_id(), PanelId::HistoryPanel);
        assert!(!unfold_action.is_folding());
        assert!(unfold_action.is_unfolding());

        assert_eq!(toggle_action.panel_id(), PanelId::PropertiesPanel);
        assert!(!toggle_action.is_folding());
        assert!(!toggle_action.is_unfolding());
    }

    #[test]
    fn test_animations_control() {
        let mut system = SmartPanelSystem::new();

        // Should start with animations enabled
        assert!(system.animations_enabled);

        // Disable animations
        system.set_animations_enabled(false);
        assert!(!system.animations_enabled);

        // Re-enable animations
        system.set_animations_enabled(true);
        assert!(system.animations_enabled);
    }

    #[test]
    fn test_adaptive_learning_control() {
        let mut system = SmartPanelSystem::new();

        // Should start with adaptive learning enabled
        assert!(system.adaptive_learning);

        // Disable adaptive learning
        system.set_adaptive_learning(false);
        assert!(!system.adaptive_learning);

        // Recording access should not update stats when disabled
        let initial_count = system.usage_stats.len();
        system.record_panel_access(PanelId::LayersPanel);
        assert_eq!(system.usage_stats.len(), initial_count);
    }
}

/// Panel folding action
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelFoldingAction {
    /// Fold (minimize) a panel
    Fold(PanelId),
    /// Unfold (restore) a panel
    Unfold(PanelId),
    /// Toggle panel folding state
    Toggle(PanelId),
}

impl PanelFoldingAction {
    /// Get the panel ID for this action
    pub fn panel_id(&self) -> PanelId {
        match self {
            PanelFoldingAction::Fold(id) => *id,
            PanelFoldingAction::Unfold(id) => *id,
            PanelFoldingAction::Toggle(id) => *id,
        }
    }

    /// Check if this is a folding action
    pub fn is_folding(&self) -> bool {
        matches!(self, PanelFoldingAction::Fold(_))
    }

    /// Check if this is an unfolding action
    pub fn is_unfolding(&self) -> bool {
        matches!(self, PanelFoldingAction::Unfold(_))
    }
}
