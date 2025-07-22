//! Rendering optimization for improved performance
//! Provides draw call batching, culling, and rendering optimizations

use std::collections::HashMap;
use iced::{Rectangle, Size, Point};

/// Rendering optimizer configuration
#[derive(Debug, Clone)]
pub struct RenderingOptimizer {
    /// Configuration
    pub config: RenderingConfig,
    /// Draw call statistics
    pub stats: RenderingStats,
    /// Viewport for culling
    pub viewport: Rectangle,
    /// Dirty regions for partial updates
    pub dirty_regions: Vec<Rectangle>,
}

/// Rendering configuration
#[derive(Debug, Clone)]
pub struct RenderingConfig {
    /// Enable frustum culling
    pub enable_culling: bool,
    /// Enable draw call batching
    pub enable_batching: bool,
    /// Enable dirty region tracking
    pub enable_dirty_regions: bool,
    /// Maximum batch size
    pub max_batch_size: usize,
    /// Culling margin (pixels)
    pub culling_margin: f32,
    /// Enable level-of-detail
    pub enable_lod: bool,
    /// LOD distance thresholds
    pub lod_thresholds: Vec<f32>,
    /// Enable occlusion culling
    pub enable_occlusion_culling: bool,
    /// Render quality level
    pub quality_level: RenderQuality,
}

/// Rendering quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderQuality {
    /// Low quality for maximum performance
    Low,
    /// Medium quality for balanced performance
    Medium,
    /// High quality for best visual results
    High,
    /// Ultra quality for maximum visual fidelity
    Ultra,
}

/// Rendering statistics
#[derive(Debug, Clone)]
pub struct RenderingStats {
    /// Total draw calls
    pub draw_calls: u32,
    /// Batched draw calls
    pub batched_calls: u32,
    /// Culled objects
    pub culled_objects: u32,
    /// Rendered objects
    pub rendered_objects: u32,
    /// GPU memory usage (bytes)
    pub gpu_memory: u64,
    /// Texture memory usage (bytes)
    pub texture_memory: u64,
    /// Vertex buffer memory (bytes)
    pub vertex_memory: u64,
}

/// Drawable object for optimization
#[derive(Debug, Clone)]
pub struct DrawableObject {
    /// Object ID
    pub id: u32,
    /// Bounding rectangle
    pub bounds: Rectangle,
    /// Z-order for depth sorting
    pub z_order: i32,
    /// Object type for batching
    pub object_type: DrawableType,
    /// Visibility flag
    pub visible: bool,
    /// Level of detail
    pub lod_level: u32,
}

/// Types of drawable objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawableType {
    /// UI elements (buttons, panels, etc.)
    UI,
    /// Text rendering
    Text,
    /// Images and textures
    Image,
    /// Vector graphics
    Vector,
    /// Canvas content
    Canvas,
    /// Effects and overlays
    Effect,
}

/// Batch of drawable objects
#[derive(Debug, Clone)]
pub struct DrawBatch {
    /// Batch type
    pub batch_type: DrawableType,
    /// Objects in this batch
    pub objects: Vec<DrawableObject>,
    /// Combined bounding rectangle
    pub bounds: Rectangle,
}

impl Default for RenderingOptimizer {
    fn default() -> Self {
        Self {
            config: RenderingConfig::default(),
            stats: RenderingStats::default(),
            viewport: Rectangle::new(Point::ORIGIN, Size::new(1920.0, 1080.0)),
            dirty_regions: Vec::new(),
        }
    }
}

impl Default for RenderingConfig {
    fn default() -> Self {
        Self {
            enable_culling: true,
            enable_batching: true,
            enable_dirty_regions: true,
            max_batch_size: 100,
            culling_margin: 10.0,
            enable_lod: true,
            lod_thresholds: vec![100.0, 500.0, 1000.0],
            enable_occlusion_culling: false,
            quality_level: RenderQuality::Medium,
        }
    }
}

impl Default for RenderingStats {
    fn default() -> Self {
        Self {
            draw_calls: 0,
            batched_calls: 0,
            culled_objects: 0,
            rendered_objects: 0,
            gpu_memory: 0,
            texture_memory: 0,
            vertex_memory: 0,
        }
    }
}

impl RenderingOptimizer {
    /// Create a new rendering optimizer
    pub fn new() -> Self {
        Self::default()
    }

    /// Set quality mode
    pub fn set_quality_mode(&mut self) {
        self.config.quality_level = RenderQuality::High;
        self.config.enable_culling = true;
        self.config.enable_batching = true;
        self.config.enable_dirty_regions = true;
        self.config.enable_lod = true;
        self.config.max_batch_size = 50; // Smaller batches for quality
    }

    /// Set balanced mode
    pub fn set_balanced_mode(&mut self) {
        self.config.quality_level = RenderQuality::Medium;
        self.config.enable_culling = true;
        self.config.enable_batching = true;
        self.config.enable_dirty_regions = true;
        self.config.enable_lod = true;
        self.config.max_batch_size = 100;
    }

    /// Set performance mode
    pub fn set_performance_mode(&mut self) {
        self.config.quality_level = RenderQuality::Low;
        self.config.enable_culling = true;
        self.config.enable_batching = true;
        self.config.enable_dirty_regions = true;
        self.config.enable_lod = true;
        self.config.max_batch_size = 200; // Larger batches for performance
    }

    /// Update viewport for culling
    pub fn set_viewport(&mut self, viewport: Rectangle) {
        self.viewport = viewport;
    }

    /// Add dirty region
    pub fn add_dirty_region(&mut self, region: Rectangle) {
        if self.config.enable_dirty_regions {
            self.dirty_regions.push(region);
        }
    }

    /// Clear dirty regions
    pub fn clear_dirty_regions(&mut self) {
        self.dirty_regions.clear();
    }

    /// Perform frustum culling on objects
    pub fn cull_objects(&self, objects: &[DrawableObject]) -> Vec<DrawableObject> {
        if !self.config.enable_culling {
            return objects.to_vec();
        }

        let expanded_viewport = Rectangle::new(
            Point::new(
                self.viewport.x - self.config.culling_margin,
                self.viewport.y - self.config.culling_margin,
            ),
            Size::new(
                self.viewport.width + 2.0 * self.config.culling_margin,
                self.viewport.height + 2.0 * self.config.culling_margin,
            ),
        );

        objects
            .iter()
            .filter(|obj| {
                obj.visible && self.intersects_viewport(&obj.bounds, &expanded_viewport)
            })
            .cloned()
            .collect()
    }

    /// Check if object intersects viewport
    fn intersects_viewport(&self, bounds: &Rectangle, viewport: &Rectangle) -> bool {
        !(bounds.x + bounds.width < viewport.x
            || bounds.x > viewport.x + viewport.width
            || bounds.y + bounds.height < viewport.y
            || bounds.y > viewport.y + viewport.height)
    }

    /// Batch objects by type for efficient rendering
    pub fn batch_objects(&self, objects: &[DrawableObject]) -> Vec<DrawBatch> {
        if !self.config.enable_batching {
            return objects
                .iter()
                .map(|obj| DrawBatch {
                    batch_type: obj.object_type,
                    objects: vec![obj.clone()],
                    bounds: obj.bounds,
                })
                .collect();
        }

        let mut batches: HashMap<DrawableType, Vec<DrawableObject>> = HashMap::new();

        // Group objects by type
        for obj in objects {
            batches.entry(obj.object_type).or_default().push(obj.clone());
        }

        // Create batches with size limits
        let mut result = Vec::new();
        for (batch_type, mut objects) in batches {
            // Sort by z-order for proper rendering
            objects.sort_by_key(|obj| obj.z_order);

            // Split into chunks based on max batch size
            for chunk in objects.chunks(self.config.max_batch_size) {
                let bounds = self.calculate_combined_bounds(chunk);
                result.push(DrawBatch {
                    batch_type,
                    objects: chunk.to_vec(),
                    bounds,
                });
            }
        }

        result
    }

    /// Calculate combined bounding rectangle for objects
    fn calculate_combined_bounds(&self, objects: &[DrawableObject]) -> Rectangle {
        if objects.is_empty() {
            return Rectangle::new(Point::ORIGIN, Size::ZERO);
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for obj in objects {
            min_x = min_x.min(obj.bounds.x);
            min_y = min_y.min(obj.bounds.y);
            max_x = max_x.max(obj.bounds.x + obj.bounds.width);
            max_y = max_y.max(obj.bounds.y + obj.bounds.height);
        }

        Rectangle::new(
            Point::new(min_x, min_y),
            Size::new(max_x - min_x, max_y - min_y),
        )
    }

    /// Apply level-of-detail optimization
    pub fn apply_lod(&self, objects: &mut [DrawableObject], camera_distance: f32) {
        if !self.config.enable_lod {
            return;
        }

        for obj in objects {
            obj.lod_level = self.calculate_lod_level(camera_distance);
        }
    }

    /// Calculate LOD level based on distance
    fn calculate_lod_level(&self, distance: f32) -> u32 {
        for (level, &threshold) in self.config.lod_thresholds.iter().enumerate() {
            if distance < threshold {
                return level as u32;
            }
        }
        self.config.lod_thresholds.len() as u32
    }

    /// Update rendering statistics
    pub fn update_stats(&mut self, draw_calls: u32, rendered_objects: u32, culled_objects: u32) {
        self.stats.draw_calls = draw_calls;
        self.stats.rendered_objects = rendered_objects;
        self.stats.culled_objects = culled_objects;
        self.stats.batched_calls = draw_calls; // Simplified
    }

    /// Get draw call count
    pub fn get_draw_call_count(&self) -> u32 {
        self.stats.draw_calls
    }

    /// Get GPU memory usage
    pub fn get_gpu_memory_usage(&self) -> u64 {
        self.stats.gpu_memory
    }

    /// Get rendering efficiency (0.0 to 1.0)
    pub fn get_efficiency(&self) -> f32 {
        if self.stats.draw_calls == 0 {
            return 1.0;
        }

        let total_objects = self.stats.rendered_objects + self.stats.culled_objects;
        if total_objects == 0 {
            return 1.0;
        }

        // Culling efficiency: higher is better (more objects culled)
        let culling_efficiency = self.stats.culled_objects as f32 / total_objects as f32;

        // Batching efficiency: more objects per draw call is better, but cap at 1.0
        let batching_efficiency = if self.stats.draw_calls > 0 {
            (self.stats.rendered_objects as f32 / self.stats.draw_calls as f32 / 10.0).min(1.0)
        } else {
            1.0
        };

        ((culling_efficiency + batching_efficiency) / 2.0).min(1.0).max(0.0)
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = RenderingStats::default();
    }
}

/// Create default rendering optimizer instance
pub fn create_rendering_optimizer() -> RenderingOptimizer {
    RenderingOptimizer::default()
}

/// Global rendering optimizer instance (use create_rendering_optimizer() instead)
pub static RENDERING_OPTIMIZER_INIT: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_optimizer_creation() {
        let optimizer = RenderingOptimizer::new();
        
        assert!(optimizer.config.enable_culling);
        assert!(optimizer.config.enable_batching);
        assert_eq!(optimizer.stats.draw_calls, 0);
    }

    #[test]
    fn test_quality_modes() {
        let mut optimizer = RenderingOptimizer::new();
        
        optimizer.set_quality_mode();
        assert_eq!(optimizer.config.quality_level, RenderQuality::High);
        
        optimizer.set_balanced_mode();
        assert_eq!(optimizer.config.quality_level, RenderQuality::Medium);
        
        optimizer.set_performance_mode();
        assert_eq!(optimizer.config.quality_level, RenderQuality::Low);
    }

    #[test]
    fn test_viewport_culling() {
        let optimizer = RenderingOptimizer::new();
        
        let objects = vec![
            DrawableObject {
                id: 1,
                bounds: Rectangle::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0)),
                z_order: 0,
                object_type: DrawableType::UI,
                visible: true,
                lod_level: 0,
            },
            DrawableObject {
                id: 2,
                bounds: Rectangle::new(Point::new(2000.0, 2000.0), Size::new(100.0, 100.0)),
                z_order: 0,
                object_type: DrawableType::UI,
                visible: true,
                lod_level: 0,
            },
        ];
        
        let culled = optimizer.cull_objects(&objects);
        
        // First object should be visible, second should be culled
        assert_eq!(culled.len(), 1);
        assert_eq!(culled[0].id, 1);
    }

    #[test]
    fn test_object_batching() {
        let optimizer = RenderingOptimizer::new();
        
        let objects = vec![
            DrawableObject {
                id: 1,
                bounds: Rectangle::new(Point::ORIGIN, Size::new(100.0, 100.0)),
                z_order: 0,
                object_type: DrawableType::UI,
                visible: true,
                lod_level: 0,
            },
            DrawableObject {
                id: 2,
                bounds: Rectangle::new(Point::new(100.0, 0.0), Size::new(100.0, 100.0)),
                z_order: 1,
                object_type: DrawableType::UI,
                visible: true,
                lod_level: 0,
            },
            DrawableObject {
                id: 3,
                bounds: Rectangle::new(Point::new(200.0, 0.0), Size::new(100.0, 100.0)),
                z_order: 0,
                object_type: DrawableType::Text,
                visible: true,
                lod_level: 0,
            },
        ];
        
        let batches = optimizer.batch_objects(&objects);
        
        // Should have 2 batches (UI and Text)
        assert_eq!(batches.len(), 2);
    }

    #[test]
    fn test_dirty_regions() {
        let mut optimizer = RenderingOptimizer::new();
        
        let region = Rectangle::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));
        optimizer.add_dirty_region(region);
        
        assert_eq!(optimizer.dirty_regions.len(), 1);
        
        optimizer.clear_dirty_regions();
        assert_eq!(optimizer.dirty_regions.len(), 0);
    }

    #[test]
    fn test_efficiency_calculation() {
        let mut optimizer = RenderingOptimizer::new();
        
        optimizer.update_stats(10, 100, 50);
        
        let efficiency = optimizer.get_efficiency();
        assert!(efficiency >= 0.0 && efficiency <= 1.0);
    }
}
