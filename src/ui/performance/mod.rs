//! Performance optimization system for PSOC UI
//! Provides rendering optimization, memory management, and performance monitoring

pub mod rendering;
pub mod memory;
pub mod caching;

// Re-export main components
pub use rendering::{RenderingOptimizer, RenderingConfig, create_rendering_optimizer};
pub use memory::{MemoryManager, MemoryConfig, create_memory_manager};
pub use caching::{CacheManager, CacheConfig, create_cache_manager};

use std::time::{Duration, Instant};

/// Performance optimization system
#[derive(Debug, Clone)]
pub struct PerformanceSystem {
    /// Rendering optimizer
    pub rendering: RenderingOptimizer,
    /// Memory manager
    pub memory: MemoryManager,

    /// Cache manager
    pub cache: CacheManager,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
}

/// Performance metrics tracking
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Frame times (in milliseconds)
    pub frame_times: Vec<f32>,
    /// Memory usage (in bytes)
    pub memory_usage: u64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f32,
    /// Number of draw calls
    pub draw_calls: u32,
    /// GPU memory usage (in bytes)
    pub gpu_memory: u64,
    /// Last update time
    pub last_update: Instant,
}

/// Performance optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    /// Maximum quality, minimum optimization
    Quality,
    /// Balanced quality and performance
    Balanced,
    /// Maximum performance, minimum quality
    Performance,
    /// Custom optimization settings
    Custom,
}

/// Performance warning types
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceWarning {
    /// High frame time detected
    HighFrameTime(f32),
    /// High memory usage detected
    HighMemoryUsage(u64),
    /// Low cache hit rate detected
    LowCacheHitRate(f32),
    /// Too many draw calls
    TooManyDrawCalls(u32),
    /// GPU memory pressure
    GpuMemoryPressure(u64),
}

impl Default for PerformanceSystem {
    fn default() -> Self {
        Self {
            rendering: RenderingOptimizer::default(),
            memory: MemoryManager::default(),
            cache: CacheManager::default(),
            metrics: PerformanceMetrics::default(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_times: Vec::with_capacity(60), // Store last 60 frames
            memory_usage: 0,
            cache_hit_rate: 0.0,
            draw_calls: 0,
            gpu_memory: 0,
            last_update: Instant::now(),
        }
    }
}

impl PerformanceSystem {
    /// Create a new performance system
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a performance system optimized for quality
    pub fn quality_optimized() -> Self {
        let mut system = Self::default();
        system.set_optimization_level(OptimizationLevel::Quality);
        system
    }

    /// Create a performance system optimized for performance
    pub fn performance_optimized() -> Self {
        let mut system = Self::default();
        system.set_optimization_level(OptimizationLevel::Performance);
        system
    }

    /// Set optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        match level {
            OptimizationLevel::Quality => {
                self.rendering.set_quality_mode();
                self.memory.set_conservative_mode();
                self.cache.set_large_cache();
            },
            OptimizationLevel::Balanced => {
                self.rendering.set_balanced_mode();
                self.memory.set_balanced_mode();
                self.cache.set_medium_cache();
            },
            OptimizationLevel::Performance => {
                self.rendering.set_performance_mode();
                self.memory.set_aggressive_mode();
                self.cache.set_small_cache();
            },
            OptimizationLevel::Custom => {
                // Keep current settings
            },
        }
    }

    /// Update performance metrics
    pub fn update_metrics(&mut self, frame_time: f32) {
        // Update frame times
        self.metrics.frame_times.push(frame_time);
        if self.metrics.frame_times.len() > 60 {
            self.metrics.frame_times.remove(0);
        }

        // Update other metrics
        self.metrics.memory_usage = self.memory.get_memory_usage();
        self.metrics.cache_hit_rate = self.cache.get_hit_rate();
        self.metrics.draw_calls = self.rendering.get_draw_call_count();
        self.metrics.gpu_memory = self.rendering.get_gpu_memory_usage();
        self.metrics.last_update = Instant::now();

        // Check for performance warnings
        self.check_performance_warnings();
    }

    /// Check for performance warnings
    fn check_performance_warnings(&self) -> Vec<PerformanceWarning> {
        let mut warnings = Vec::new();

        // Check frame time
        if let Some(&latest_frame_time) = self.metrics.frame_times.last() {
            if latest_frame_time > 16.67 { // 60 FPS threshold
                warnings.push(PerformanceWarning::HighFrameTime(latest_frame_time));
            }
        }

        // Check memory usage (100MB threshold)
        if self.metrics.memory_usage > 100 * 1024 * 1024 {
            warnings.push(PerformanceWarning::HighMemoryUsage(self.metrics.memory_usage));
        }

        // Check cache hit rate
        if self.metrics.cache_hit_rate < 0.8 {
            warnings.push(PerformanceWarning::LowCacheHitRate(self.metrics.cache_hit_rate));
        }

        // Check draw calls
        if self.metrics.draw_calls > 1000 {
            warnings.push(PerformanceWarning::TooManyDrawCalls(self.metrics.draw_calls));
        }

        // Check GPU memory (500MB threshold)
        if self.metrics.gpu_memory > 500 * 1024 * 1024 {
            warnings.push(PerformanceWarning::GpuMemoryPressure(self.metrics.gpu_memory));
        }

        warnings
    }

    /// Get average frame time
    pub fn get_average_frame_time(&self) -> f32 {
        if self.metrics.frame_times.is_empty() {
            0.0
        } else {
            self.metrics.frame_times.iter().sum::<f32>() / self.metrics.frame_times.len() as f32
        }
    }

    /// Get current FPS
    pub fn get_fps(&self) -> f32 {
        let avg_frame_time = self.get_average_frame_time();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get performance score (0.0 to 1.0)
    pub fn get_performance_score(&self) -> f32 {
        let fps_score = (self.get_fps() / 60.0).min(1.0);
        let memory_score = 1.0 - (self.metrics.memory_usage as f32 / (100.0 * 1024.0 * 1024.0)).min(1.0);
        let cache_score = self.metrics.cache_hit_rate;
        let draw_call_score = 1.0 - (self.metrics.draw_calls as f32 / 1000.0).min(1.0);

        (fps_score + memory_score + cache_score + draw_call_score) / 4.0
    }

    /// Optimize for current conditions
    pub fn auto_optimize(&mut self) {
        let score = self.get_performance_score();
        
        if score < 0.5 {
            // Poor performance - switch to performance mode
            self.set_optimization_level(OptimizationLevel::Performance);
        } else if score > 0.8 {
            // Good performance - can afford quality mode
            self.set_optimization_level(OptimizationLevel::Quality);
        } else {
            // Moderate performance - use balanced mode
            self.set_optimization_level(OptimizationLevel::Balanced);
        }
    }

    /// Clear performance data
    pub fn clear_metrics(&mut self) {
        self.metrics.frame_times.clear();
        self.metrics.memory_usage = 0;
        self.metrics.cache_hit_rate = 0.0;
        self.metrics.draw_calls = 0;
        self.metrics.gpu_memory = 0;
        self.metrics.last_update = Instant::now();
    }

    /// Get performance report
    pub fn get_performance_report(&self) -> PerformanceReport {
        PerformanceReport {
            average_fps: self.get_fps(),
            average_frame_time: self.get_average_frame_time(),
            memory_usage_mb: self.metrics.memory_usage as f32 / (1024.0 * 1024.0),
            cache_hit_rate: self.metrics.cache_hit_rate,
            draw_calls: self.metrics.draw_calls,
            gpu_memory_mb: self.metrics.gpu_memory as f32 / (1024.0 * 1024.0),
            performance_score: self.get_performance_score(),
            warnings: self.check_performance_warnings(),
        }
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Average FPS
    pub average_fps: f32,
    /// Average frame time in milliseconds
    pub average_frame_time: f32,
    /// Memory usage in MB
    pub memory_usage_mb: f32,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f32,
    /// Number of draw calls
    pub draw_calls: u32,
    /// GPU memory usage in MB
    pub gpu_memory_mb: f32,
    /// Overall performance score (0.0 to 1.0)
    pub performance_score: f32,
    /// Performance warnings
    pub warnings: Vec<PerformanceWarning>,
}

/// Global performance system instance
pub static mut PERFORMANCE_SYSTEM: Option<PerformanceSystem> = None;

/// Get global performance system
pub fn get_performance_system() -> &'static mut PerformanceSystem {
    unsafe {
        PERFORMANCE_SYSTEM.get_or_insert_with(PerformanceSystem::default)
    }
}

/// Performance utilities
pub struct PerformanceUtils;

impl PerformanceUtils {
    /// Measure execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Check if device is low-end
    pub fn is_low_end_device() -> bool {
        // Simple heuristic - can be improved with actual device detection
        std::env::var("PSOC_LOW_END_DEVICE").is_ok()
    }

    /// Get recommended optimization level for device
    pub fn get_recommended_optimization() -> OptimizationLevel {
        if Self::is_low_end_device() {
            OptimizationLevel::Performance
        } else {
            OptimizationLevel::Balanced
        }
    }

    /// Format memory size for display
    pub fn format_memory_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_system_creation() {
        let system = PerformanceSystem::new();
        
        // Should have default configuration
        assert_eq!(system.metrics.frame_times.len(), 0);
        assert_eq!(system.metrics.memory_usage, 0);
        assert_eq!(system.metrics.cache_hit_rate, 0.0);
    }

    #[test]
    fn test_optimization_levels() {
        let mut system = PerformanceSystem::new();
        
        // Test different optimization levels
        system.set_optimization_level(OptimizationLevel::Quality);
        system.set_optimization_level(OptimizationLevel::Balanced);
        system.set_optimization_level(OptimizationLevel::Performance);
        system.set_optimization_level(OptimizationLevel::Custom);
        
        // Should not panic
        assert!(true);
    }

    #[test]
    fn test_metrics_update() {
        let mut system = PerformanceSystem::new();
        
        // Update metrics
        system.update_metrics(16.0);
        system.update_metrics(17.0);
        system.update_metrics(15.0);
        
        // Should have frame times
        assert_eq!(system.metrics.frame_times.len(), 3);
        assert_eq!(system.get_average_frame_time(), 16.0);
    }

    #[test]
    fn test_fps_calculation() {
        let mut system = PerformanceSystem::new();
        
        // Add frame times for 60 FPS
        system.update_metrics(16.67);
        
        let fps = system.get_fps();
        assert!((fps - 60.0).abs() < 1.0);
    }

    #[test]
    fn test_performance_score() {
        let system = PerformanceSystem::new();
        
        let score = system.get_performance_score();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_performance_warnings() {
        let mut system = PerformanceSystem::new();
        
        // Add high frame time
        system.update_metrics(50.0); // Very slow frame
        
        let warnings = system.check_performance_warnings();
        assert!(warnings.iter().any(|w| matches!(w, PerformanceWarning::HighFrameTime(_))));
    }

    #[test]
    fn test_performance_report() {
        let mut system = PerformanceSystem::new();
        system.update_metrics(16.0);
        
        let report = system.get_performance_report();
        assert!(report.average_fps > 0.0);
        assert!(report.performance_score >= 0.0);
    }

    #[test]
    fn test_performance_utils() {
        // Test time measurement
        let (result, duration) = PerformanceUtils::measure_time(|| {
            std::thread::sleep(Duration::from_millis(1));
            42
        });
        
        assert_eq!(result, 42);
        assert!(duration >= Duration::from_millis(1));
        
        // Test memory formatting
        assert_eq!(PerformanceUtils::format_memory_size(1024), "1.0 KB");
        assert_eq!(PerformanceUtils::format_memory_size(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn test_auto_optimization() {
        let mut system = PerformanceSystem::new();
        
        // Should not panic
        system.auto_optimize();
        assert!(true);
    }

    #[test]
    fn test_metrics_clearing() {
        let mut system = PerformanceSystem::new();
        system.update_metrics(16.0);
        
        assert!(!system.metrics.frame_times.is_empty());
        
        system.clear_metrics();
        assert!(system.metrics.frame_times.is_empty());
    }
}
