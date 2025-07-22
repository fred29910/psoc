//! Memory management for optimal performance
//! Provides memory tracking, garbage collection, and optimization

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory manager for performance optimization
#[derive(Debug, Clone)]
pub struct MemoryManager {
    /// Configuration
    pub config: MemoryConfig,
    /// Memory statistics
    pub stats: MemoryStats,
    /// Allocated objects tracking
    pub allocations: HashMap<u64, AllocationInfo>,
    /// Next allocation ID
    pub next_id: u64,
    /// Last cleanup time
    pub last_cleanup: Instant,
}

/// Memory management configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Maximum memory usage (bytes)
    pub max_memory: u64,
    /// Memory warning threshold (0.0 to 1.0)
    pub warning_threshold: f32,
    /// Cleanup interval
    pub cleanup_interval: Duration,
    /// Enable automatic cleanup
    pub auto_cleanup: bool,
    /// Aggressive cleanup mode
    pub aggressive_cleanup: bool,
    /// Memory pool sizes
    pub pool_sizes: Vec<usize>,
    /// Enable memory pooling
    pub enable_pooling: bool,
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total allocated memory (bytes)
    pub total_allocated: u64,
    /// Peak memory usage (bytes)
    pub peak_usage: u64,
    /// Number of allocations
    pub allocation_count: u64,
    /// Number of deallocations
    pub deallocation_count: u64,
    /// Memory fragmentation ratio (0.0 to 1.0)
    pub fragmentation: f32,
    /// Last update time
    pub last_update: Instant,
}

/// Information about a memory allocation
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    /// Allocation ID
    pub id: u64,
    /// Size in bytes
    pub size: u64,
    /// Allocation time
    pub allocated_at: Instant,
    /// Last access time
    pub last_accessed: Instant,
    /// Allocation type
    pub allocation_type: AllocationType,
    /// Reference count
    pub ref_count: u32,
}

/// Types of memory allocations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationType {
    /// Image data
    Image,
    /// Text rendering cache
    Text,
    /// UI element cache
    UI,
    /// Canvas data
    Canvas,
    /// Temporary buffers
    Temporary,
    /// System allocations
    System,
}

/// Memory management mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryMode {
    /// Conservative memory usage
    Conservative,
    /// Balanced memory usage
    Balanced,
    /// Aggressive memory optimization
    Aggressive,
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self {
            config: MemoryConfig::default(),
            stats: MemoryStats::default(),
            allocations: HashMap::new(),
            next_id: 1,
            last_cleanup: Instant::now(),
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory: 512 * 1024 * 1024, // 512MB default
            warning_threshold: 0.8,
            cleanup_interval: Duration::from_secs(30),
            auto_cleanup: true,
            aggressive_cleanup: false,
            pool_sizes: vec![64, 256, 1024, 4096, 16384, 65536],
            enable_pooling: true,
        }
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            total_allocated: 0,
            peak_usage: 0,
            allocation_count: 0,
            deallocation_count: 0,
            fragmentation: 0.0,
            last_update: Instant::now(),
        }
    }
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Set conservative mode
    pub fn set_conservative_mode(&mut self) {
        self.config.max_memory = 256 * 1024 * 1024; // 256MB
        self.config.warning_threshold = 0.7;
        self.config.aggressive_cleanup = false;
        self.config.cleanup_interval = Duration::from_secs(60);
    }

    /// Set balanced mode
    pub fn set_balanced_mode(&mut self) {
        self.config.max_memory = 512 * 1024 * 1024; // 512MB
        self.config.warning_threshold = 0.8;
        self.config.aggressive_cleanup = false;
        self.config.cleanup_interval = Duration::from_secs(30);
    }

    /// Set aggressive mode
    pub fn set_aggressive_mode(&mut self) {
        self.config.max_memory = 1024 * 1024 * 1024; // 1GB
        self.config.warning_threshold = 0.9;
        self.config.aggressive_cleanup = true;
        self.config.cleanup_interval = Duration::from_secs(10);
    }

    /// Allocate memory
    pub fn allocate(&mut self, size: u64, allocation_type: AllocationType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let info = AllocationInfo {
            id,
            size,
            allocated_at: Instant::now(),
            last_accessed: Instant::now(),
            allocation_type,
            ref_count: 1,
        };

        self.allocations.insert(id, info);
        self.stats.total_allocated += size;
        self.stats.allocation_count += 1;

        // Update peak usage
        if self.stats.total_allocated > self.stats.peak_usage {
            self.stats.peak_usage = self.stats.total_allocated;
        }

        // Check if cleanup is needed
        if self.should_cleanup() {
            self.cleanup();
        }

        id
    }

    /// Deallocate memory
    pub fn deallocate(&mut self, id: u64) -> bool {
        if let Some(info) = self.allocations.remove(&id) {
            self.stats.total_allocated = self.stats.total_allocated.saturating_sub(info.size);
            self.stats.deallocation_count += 1;
            true
        } else {
            false
        }
    }

    /// Access memory allocation (update last accessed time)
    pub fn access(&mut self, id: u64) -> bool {
        if let Some(info) = self.allocations.get_mut(&id) {
            info.last_accessed = Instant::now();
            true
        } else {
            false
        }
    }

    /// Increment reference count
    pub fn add_ref(&mut self, id: u64) -> bool {
        if let Some(info) = self.allocations.get_mut(&id) {
            info.ref_count += 1;
            true
        } else {
            false
        }
    }

    /// Decrement reference count
    pub fn release_ref(&mut self, id: u64) -> bool {
        if let Some(info) = self.allocations.get_mut(&id) {
            info.ref_count = info.ref_count.saturating_sub(1);
            if info.ref_count == 0 {
                self.deallocate(id);
            }
            true
        } else {
            false
        }
    }

    /// Check if cleanup should be performed
    fn should_cleanup(&self) -> bool {
        if !self.config.auto_cleanup {
            return false;
        }

        // Check memory threshold
        let usage_ratio = self.stats.total_allocated as f32 / self.config.max_memory as f32;
        if usage_ratio > self.config.warning_threshold {
            return true;
        }

        // Check time interval
        self.last_cleanup.elapsed() > self.config.cleanup_interval
    }

    /// Perform memory cleanup
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        let mut to_remove = Vec::new();

        // Find allocations to clean up
        for (id, info) in &self.allocations {
            let should_remove = if self.config.aggressive_cleanup {
                // Aggressive: remove unused allocations older than 30 seconds
                info.ref_count == 0 && now.duration_since(info.last_accessed) > Duration::from_secs(30)
            } else {
                // Conservative: remove unused allocations older than 5 minutes
                info.ref_count == 0 && now.duration_since(info.last_accessed) > Duration::from_secs(300)
            };

            if should_remove {
                to_remove.push(*id);
            }
        }

        // Remove identified allocations
        for id in to_remove {
            self.deallocate(id);
        }

        self.last_cleanup = now;
        self.update_fragmentation();
    }

    /// Force cleanup of specific allocation type
    pub fn cleanup_type(&mut self, allocation_type: AllocationType) {
        let to_remove: Vec<u64> = self
            .allocations
            .iter()
            .filter(|(_, info)| info.allocation_type == allocation_type && info.ref_count == 0)
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove {
            self.deallocate(id);
        }
    }

    /// Update memory fragmentation calculation
    fn update_fragmentation(&mut self) {
        // Simplified fragmentation calculation
        // In a real implementation, this would analyze actual memory layout
        let allocation_count = self.allocations.len() as f32;
        if allocation_count > 0.0 {
            self.stats.fragmentation = (allocation_count / 1000.0).min(1.0);
        } else {
            self.stats.fragmentation = 0.0;
        }
    }

    /// Get memory usage by type
    pub fn get_usage_by_type(&self) -> HashMap<AllocationType, u64> {
        let mut usage = HashMap::new();

        for info in self.allocations.values() {
            *usage.entry(info.allocation_type).or_insert(0) += info.size;
        }

        usage
    }

    /// Get memory usage percentage
    pub fn get_usage_percentage(&self) -> f32 {
        if self.config.max_memory == 0 {
            0.0
        } else {
            (self.stats.total_allocated as f32 / self.config.max_memory as f32) * 100.0
        }
    }

    /// Get current memory usage
    pub fn get_memory_usage(&self) -> u64 {
        self.stats.total_allocated
    }

    /// Check if memory is under pressure
    pub fn is_under_pressure(&self) -> bool {
        self.get_usage_percentage() > (self.config.warning_threshold * 100.0)
    }

    /// Get oldest allocations
    pub fn get_oldest_allocations(&self, count: usize) -> Vec<u64> {
        let mut allocations: Vec<_> = self.allocations.iter().collect();
        allocations.sort_by_key(|(_, info)| info.allocated_at);
        allocations.into_iter().take(count).map(|(id, _)| *id).collect()
    }

    /// Get least recently used allocations
    pub fn get_lru_allocations(&self, count: usize) -> Vec<u64> {
        let mut allocations: Vec<_> = self.allocations.iter().collect();
        allocations.sort_by_key(|(_, info)| info.last_accessed);
        allocations.into_iter().take(count).map(|(id, _)| *id).collect()
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = MemoryStats::default();
    }

    /// Get memory report
    pub fn get_memory_report(&self) -> MemoryReport {
        MemoryReport {
            total_allocated_mb: self.stats.total_allocated as f32 / (1024.0 * 1024.0),
            peak_usage_mb: self.stats.peak_usage as f32 / (1024.0 * 1024.0),
            usage_percentage: self.get_usage_percentage(),
            allocation_count: self.stats.allocation_count,
            deallocation_count: self.stats.deallocation_count,
            fragmentation: self.stats.fragmentation,
            usage_by_type: self.get_usage_by_type(),
            under_pressure: self.is_under_pressure(),
        }
    }
}

/// Memory usage report
#[derive(Debug, Clone)]
pub struct MemoryReport {
    /// Total allocated memory in MB
    pub total_allocated_mb: f32,
    /// Peak memory usage in MB
    pub peak_usage_mb: f32,
    /// Memory usage percentage
    pub usage_percentage: f32,
    /// Number of allocations
    pub allocation_count: u64,
    /// Number of deallocations
    pub deallocation_count: u64,
    /// Memory fragmentation ratio
    pub fragmentation: f32,
    /// Memory usage by type
    pub usage_by_type: HashMap<AllocationType, u64>,
    /// Whether memory is under pressure
    pub under_pressure: bool,
}

/// Create default memory manager instance
pub fn create_memory_manager() -> MemoryManager {
    MemoryManager::default()
}

/// Global memory manager instance (use create_memory_manager() instead)
pub static MEMORY_MANAGER_INIT: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager_creation() {
        let manager = MemoryManager::new();
        
        assert_eq!(manager.stats.total_allocated, 0);
        assert_eq!(manager.allocations.len(), 0);
    }

    #[test]
    fn test_memory_allocation() {
        let mut manager = MemoryManager::new();
        
        let id = manager.allocate(1024, AllocationType::Image);
        assert_eq!(manager.stats.total_allocated, 1024);
        assert_eq!(manager.allocations.len(), 1);
        assert!(manager.allocations.contains_key(&id));
    }

    #[test]
    fn test_memory_deallocation() {
        let mut manager = MemoryManager::new();
        
        let id = manager.allocate(1024, AllocationType::Image);
        assert!(manager.deallocate(id));
        assert_eq!(manager.stats.total_allocated, 0);
        assert_eq!(manager.allocations.len(), 0);
    }

    #[test]
    fn test_reference_counting() {
        let mut manager = MemoryManager::new();
        
        let id = manager.allocate(1024, AllocationType::Image);
        manager.add_ref(id);
        
        let info = manager.allocations.get(&id).unwrap();
        assert_eq!(info.ref_count, 2);
        
        manager.release_ref(id);
        assert!(manager.allocations.contains_key(&id)); // Should still exist
        
        manager.release_ref(id);
        assert!(!manager.allocations.contains_key(&id)); // Should be deallocated
    }

    #[test]
    fn test_memory_modes() {
        let mut manager = MemoryManager::new();
        
        manager.set_conservative_mode();
        assert_eq!(manager.config.max_memory, 256 * 1024 * 1024);
        
        manager.set_balanced_mode();
        assert_eq!(manager.config.max_memory, 512 * 1024 * 1024);
        
        manager.set_aggressive_mode();
        assert_eq!(manager.config.max_memory, 1024 * 1024 * 1024);
    }

    #[test]
    fn test_usage_by_type() {
        let mut manager = MemoryManager::new();
        
        manager.allocate(1024, AllocationType::Image);
        manager.allocate(512, AllocationType::Text);
        manager.allocate(256, AllocationType::Image);
        
        let usage = manager.get_usage_by_type();
        assert_eq!(usage.get(&AllocationType::Image), Some(&1280));
        assert_eq!(usage.get(&AllocationType::Text), Some(&512));
    }

    #[test]
    fn test_memory_pressure() {
        let mut manager = MemoryManager::new();
        manager.config.max_memory = 1000;
        manager.config.warning_threshold = 0.8;
        
        // Allocate below threshold
        manager.allocate(700, AllocationType::Image);
        assert!(!manager.is_under_pressure());
        
        // Allocate above threshold
        manager.allocate(200, AllocationType::Text);
        assert!(manager.is_under_pressure());
    }

    #[test]
    fn test_cleanup() {
        let mut manager = MemoryManager::new();
        manager.config.aggressive_cleanup = true;
        
        let id = manager.allocate(1024, AllocationType::Temporary);
        manager.release_ref(id); // Set ref count to 0
        
        // Simulate time passage
        if let Some(info) = manager.allocations.get_mut(&id) {
            info.last_accessed = Instant::now() - Duration::from_secs(60);
        }
        
        manager.cleanup();
        assert!(!manager.allocations.contains_key(&id));
    }

    #[test]
    fn test_memory_report() {
        let mut manager = MemoryManager::new();
        manager.allocate(1024 * 1024, AllocationType::Image); // 1MB
        
        let report = manager.get_memory_report();
        assert_eq!(report.total_allocated_mb, 1.0);
        assert!(report.usage_by_type.contains_key(&AllocationType::Image));
    }
}
