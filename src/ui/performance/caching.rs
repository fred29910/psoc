//! Caching system for performance optimization
//! Provides intelligent caching of UI elements, textures, and computed data

use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

/// Cache manager for performance optimization
#[derive(Debug, Clone)]
pub struct CacheManager {
    /// Configuration
    pub config: CacheConfig,
    /// Cache statistics
    pub stats: CacheStats,
    /// Cache entries
    pub entries: HashMap<u64, CacheEntry>,
    /// Next entry ID
    pub next_id: u64,
    /// Last cleanup time
    pub last_cleanup: Instant,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum cache size (bytes)
    pub max_size: u64,
    /// Maximum number of entries
    pub max_entries: usize,
    /// Entry time-to-live
    pub ttl: Duration,
    /// Enable LRU eviction
    pub enable_lru: bool,
    /// Cleanup interval
    pub cleanup_interval: Duration,
    /// Cache hit threshold for keeping entries
    pub hit_threshold: u32,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Current cache size (bytes)
    pub current_size: u64,
    /// Number of entries
    pub entry_count: usize,
    /// Number of evictions
    pub evictions: u64,
    /// Last update time
    pub last_update: Instant,
}

/// Cache entry information
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Entry ID
    pub id: u64,
    /// Data size (bytes)
    pub size: u64,
    /// Creation time
    pub created_at: Instant,
    /// Last access time
    pub last_accessed: Instant,
    /// Access count
    pub access_count: u32,
    /// Entry type
    pub entry_type: CacheEntryType,
    /// Priority for eviction
    pub priority: CachePriority,
}

/// Types of cache entries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheEntryType {
    /// Rendered UI elements
    UIElement,
    /// Text rendering cache
    Text,
    /// Image/texture cache
    Image,
    /// Computed layouts
    Layout,
    /// Shader programs
    Shader,
    /// Font data
    Font,
}

/// Cache priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CachePriority {
    /// Low priority - evict first
    Low = 0,
    /// Normal priority
    Normal = 1,
    /// High priority - keep longer
    High = 2,
    /// Critical priority - never evict
    Critical = 3,
}

impl Default for CacheManager {
    fn default() -> Self {
        Self {
            config: CacheConfig::default(),
            stats: CacheStats::default(),
            entries: HashMap::new(),
            next_id: 1,
            last_cleanup: Instant::now(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 128 * 1024 * 1024, // 128MB
            max_entries: 10000,
            ttl: Duration::from_secs(300), // 5 minutes
            enable_lru: true,
            cleanup_interval: Duration::from_secs(60),
            hit_threshold: 5,
        }
    }
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            current_size: 0,
            entry_count: 0,
            evictions: 0,
            last_update: Instant::now(),
        }
    }
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Set small cache configuration
    pub fn set_small_cache(&mut self) {
        self.config.max_size = 64 * 1024 * 1024; // 64MB
        self.config.max_entries = 5000;
        self.config.ttl = Duration::from_secs(180); // 3 minutes
    }

    /// Set medium cache configuration
    pub fn set_medium_cache(&mut self) {
        self.config.max_size = 128 * 1024 * 1024; // 128MB
        self.config.max_entries = 10000;
        self.config.ttl = Duration::from_secs(300); // 5 minutes
    }

    /// Set large cache configuration
    pub fn set_large_cache(&mut self) {
        self.config.max_size = 256 * 1024 * 1024; // 256MB
        self.config.max_entries = 20000;
        self.config.ttl = Duration::from_secs(600); // 10 minutes
    }

    /// Add entry to cache
    pub fn insert(&mut self, size: u64, entry_type: CacheEntryType, priority: CachePriority) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let entry = CacheEntry {
            id,
            size,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            access_count: 1,
            entry_type,
            priority,
        };

        // Check if we need to make space
        self.ensure_space(size);

        self.entries.insert(id, entry);
        self.stats.current_size += size;
        self.stats.entry_count += 1;

        id
    }

    /// Get entry from cache
    pub fn get(&mut self, id: u64) -> Option<&CacheEntry> {
        if let Some(entry) = self.entries.get_mut(&id) {
            entry.last_accessed = Instant::now();
            entry.access_count += 1;
            self.stats.hits += 1;
            Some(entry)
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// Remove entry from cache
    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(entry) = self.entries.remove(&id) {
            self.stats.current_size = self.stats.current_size.saturating_sub(entry.size);
            self.stats.entry_count = self.stats.entry_count.saturating_sub(1);
            true
        } else {
            false
        }
    }

    /// Check if entry exists in cache
    pub fn contains(&self, id: u64) -> bool {
        self.entries.contains_key(&id)
    }

    /// Ensure there's enough space for new entry
    fn ensure_space(&mut self, required_size: u64) {
        // Check size limit
        while self.stats.current_size + required_size > self.config.max_size {
            if !self.evict_one() {
                break; // No more entries to evict
            }
        }

        // Check entry count limit
        while self.stats.entry_count >= self.config.max_entries {
            if !self.evict_one() {
                break; // No more entries to evict
            }
        }
    }

    /// Evict one cache entry
    fn evict_one(&mut self) -> bool {
        let candidate = if self.config.enable_lru {
            self.find_lru_candidate()
        } else {
            self.find_oldest_candidate()
        };

        if let Some(id) = candidate {
            self.remove(id);
            self.stats.evictions += 1;
            true
        } else {
            false
        }
    }

    /// Find LRU candidate for eviction
    fn find_lru_candidate(&self) -> Option<u64> {
        self.entries
            .iter()
            .filter(|(_, entry)| entry.priority != CachePriority::Critical)
            .min_by_key(|(_, entry)| (entry.priority, entry.last_accessed))
            .map(|(id, _)| *id)
    }

    /// Find oldest candidate for eviction
    fn find_oldest_candidate(&self) -> Option<u64> {
        self.entries
            .iter()
            .filter(|(_, entry)| entry.priority != CachePriority::Critical)
            .min_by_key(|(_, entry)| (entry.priority, entry.created_at))
            .map(|(id, _)| *id)
    }

    /// Perform cache cleanup
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        let mut to_remove = Vec::new();

        // Find expired entries
        for (id, entry) in &self.entries {
            if entry.priority != CachePriority::Critical {
                let age = now.duration_since(entry.created_at);
                let idle_time = now.duration_since(entry.last_accessed);

                // Remove if TTL exceeded or low usage
                if age > self.config.ttl || 
                   (entry.access_count < self.config.hit_threshold && idle_time > self.config.ttl / 2) {
                    to_remove.push(*id);
                }
            }
        }

        // Remove identified entries
        for id in to_remove {
            self.remove(id);
            self.stats.evictions += 1;
        }

        self.last_cleanup = now;
    }

    /// Get cache hit rate
    pub fn get_hit_rate(&self) -> f32 {
        let total = self.stats.hits + self.stats.misses;
        if total == 0 {
            0.0
        } else {
            self.stats.hits as f32 / total as f32
        }
    }

    /// Get cache usage percentage
    pub fn get_usage_percentage(&self) -> f32 {
        if self.config.max_size == 0 {
            0.0
        } else {
            (self.stats.current_size as f32 / self.config.max_size as f32) * 100.0
        }
    }

    /// Get entries by type
    pub fn get_entries_by_type(&self, entry_type: CacheEntryType) -> Vec<u64> {
        self.entries
            .iter()
            .filter(|(_, entry)| entry.entry_type == entry_type)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Clear cache of specific type
    pub fn clear_type(&mut self, entry_type: CacheEntryType) {
        let to_remove: Vec<u64> = self.get_entries_by_type(entry_type);
        for id in to_remove {
            self.remove(id);
        }
    }

    /// Clear entire cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.stats.current_size = 0;
        self.stats.entry_count = 0;
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Check if cleanup is needed
    pub fn should_cleanup(&self) -> bool {
        self.last_cleanup.elapsed() > self.config.cleanup_interval
    }

    /// Get cache efficiency score (0.0 to 1.0)
    pub fn get_efficiency(&self) -> f32 {
        let hit_rate = self.get_hit_rate();
        let usage_efficiency = 1.0 - (self.get_usage_percentage() / 100.0).min(1.0);
        let eviction_efficiency = if self.stats.hits + self.stats.misses > 0 {
            1.0 - (self.stats.evictions as f32 / (self.stats.hits + self.stats.misses) as f32).min(1.0)
        } else {
            1.0
        };

        (hit_rate + usage_efficiency + eviction_efficiency) / 3.0
    }

    /// Get cache report
    pub fn get_cache_report(&self) -> CacheReport {
        CacheReport {
            hit_rate: self.get_hit_rate(),
            usage_percentage: self.get_usage_percentage(),
            current_size_mb: self.stats.current_size as f32 / (1024.0 * 1024.0),
            entry_count: self.stats.entry_count,
            total_hits: self.stats.hits,
            total_misses: self.stats.misses,
            total_evictions: self.stats.evictions,
            efficiency: self.get_efficiency(),
        }
    }
}

/// Cache performance report
#[derive(Debug, Clone)]
pub struct CacheReport {
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f32,
    /// Cache usage percentage
    pub usage_percentage: f32,
    /// Current cache size in MB
    pub current_size_mb: f32,
    /// Number of cache entries
    pub entry_count: usize,
    /// Total cache hits
    pub total_hits: u64,
    /// Total cache misses
    pub total_misses: u64,
    /// Total evictions
    pub total_evictions: u64,
    /// Cache efficiency score
    pub efficiency: f32,
}

/// Create default cache manager instance
pub fn create_cache_manager() -> CacheManager {
    CacheManager::default()
}

/// Global cache manager instance (use create_cache_manager() instead)
pub static CACHE_MANAGER_INIT: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manager_creation() {
        let manager = CacheManager::new();
        
        assert_eq!(manager.stats.current_size, 0);
        assert_eq!(manager.entries.len(), 0);
    }

    #[test]
    fn test_cache_insertion() {
        let mut manager = CacheManager::new();
        
        let id = manager.insert(1024, CacheEntryType::UIElement, CachePriority::Normal);
        assert_eq!(manager.stats.current_size, 1024);
        assert_eq!(manager.entries.len(), 1);
        assert!(manager.contains(id));
    }

    #[test]
    fn test_cache_retrieval() {
        let mut manager = CacheManager::new();
        
        let id = manager.insert(1024, CacheEntryType::UIElement, CachePriority::Normal);
        
        // First access should be a hit
        assert!(manager.get(id).is_some());
        assert_eq!(manager.stats.hits, 1);
        
        // Non-existent entry should be a miss
        assert!(manager.get(999).is_none());
        assert_eq!(manager.stats.misses, 1);
    }

    #[test]
    fn test_cache_eviction() {
        let mut manager = CacheManager::new();
        manager.config.max_size = 2000; // Small cache
        
        let id1 = manager.insert(1000, CacheEntryType::UIElement, CachePriority::Normal);
        let id2 = manager.insert(1000, CacheEntryType::Text, CachePriority::Normal);
        
        // Should fit both
        assert!(manager.contains(id1));
        assert!(manager.contains(id2));
        
        // This should trigger eviction
        let _id3 = manager.insert(1500, CacheEntryType::Image, CachePriority::Normal);
        
        // At least one should be evicted
        assert!(manager.stats.evictions > 0);
    }

    #[test]
    fn test_cache_priorities() {
        let mut manager = CacheManager::new();
        manager.config.max_size = 2000;
        
        let critical_id = manager.insert(1000, CacheEntryType::UIElement, CachePriority::Critical);
        let normal_id = manager.insert(1000, CacheEntryType::Text, CachePriority::Normal);
        
        // Force eviction
        let _new_id = manager.insert(1500, CacheEntryType::Image, CachePriority::Normal);
        
        // Critical entry should still exist
        assert!(manager.contains(critical_id));
    }

    #[test]
    fn test_hit_rate_calculation() {
        let mut manager = CacheManager::new();
        
        let id = manager.insert(1024, CacheEntryType::UIElement, CachePriority::Normal);
        
        // 2 hits, 1 miss
        manager.get(id);
        manager.get(id);
        manager.get(999);
        
        let hit_rate = manager.get_hit_rate();
        assert!((hit_rate - 0.6667).abs() < 0.01); // 2/3 ≈ 0.6667
    }

    #[test]
    fn test_cache_cleanup() {
        let mut manager = CacheManager::new();
        manager.config.ttl = Duration::from_millis(1);
        
        let id = manager.insert(1024, CacheEntryType::UIElement, CachePriority::Normal);
        
        // Wait for TTL to expire
        std::thread::sleep(Duration::from_millis(2));
        
        manager.cleanup();
        assert!(!manager.contains(id));
    }

    #[test]
    fn test_cache_types() {
        let mut manager = CacheManager::new();
        
        let ui_id = manager.insert(1024, CacheEntryType::UIElement, CachePriority::Normal);
        let text_id = manager.insert(512, CacheEntryType::Text, CachePriority::Normal);
        
        let ui_entries = manager.get_entries_by_type(CacheEntryType::UIElement);
        assert_eq!(ui_entries.len(), 1);
        assert!(ui_entries.contains(&ui_id));
        
        manager.clear_type(CacheEntryType::Text);
        assert!(!manager.contains(text_id));
        assert!(manager.contains(ui_id));
    }

    #[test]
    fn test_cache_report() {
        let mut manager = CacheManager::new();
        manager.insert(1024 * 1024, CacheEntryType::UIElement, CachePriority::Normal); // 1MB
        
        let report = manager.get_cache_report();
        assert_eq!(report.current_size_mb, 1.0);
        assert_eq!(report.entry_count, 1);
    }
}
