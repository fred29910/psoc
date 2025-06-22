//! Migration utilities for transitioning from legacy to enhanced i18n system
//!
//! This module provides utilities to help migrate from the original i18n system
//! to the enhanced system with lazy loading and better performance.

use super::{LocalizationManager, Language as LegacyLanguage};
use super::enhanced::{EnhancedLocalizationManager, Language as EnhancedLanguage};
use tracing::{info, warn, error};

/// Migration helper to convert from legacy to enhanced system
pub struct I18nMigration;

impl I18nMigration {
    /// Convert legacy Language to enhanced Language
    pub fn convert_language(legacy: LegacyLanguage) -> EnhancedLanguage {
        match legacy {
            LegacyLanguage::English => EnhancedLanguage::English,
            LegacyLanguage::ChineseSimplified => EnhancedLanguage::ChineseSimplified,
        }
    }

    /// Convert enhanced Language to legacy Language
    pub fn convert_language_back(enhanced: EnhancedLanguage) -> Option<LegacyLanguage> {
        match enhanced {
            EnhancedLanguage::English => Some(LegacyLanguage::English),
            EnhancedLanguage::ChineseSimplified => Some(LegacyLanguage::ChineseSimplified),
            _ => None, // Enhanced system supports more languages
        }
    }

    /// Migrate settings from legacy to enhanced system
    pub fn migrate_language_settings(
        legacy_manager: &LocalizationManager,
        enhanced_manager: &mut EnhancedLocalizationManager,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let legacy_lang = legacy_manager.current_language();
        let enhanced_lang = Self::convert_language(legacy_lang);
        
        info!("Migrating language setting from {:?} to {:?}", legacy_lang, enhanced_lang);
        enhanced_manager.set_language(enhanced_lang)?;
        
        Ok(())
    }

    /// Validate that all translations from legacy system exist in enhanced system
    pub fn validate_translation_compatibility(
        legacy_manager: &LocalizationManager,
        enhanced_manager: &mut EnhancedLocalizationManager,
        test_keys: &[&str],
    ) -> ValidationReport {
        let mut report = ValidationReport::new();
        
        for &key in test_keys {
            let legacy_translation = legacy_manager.translate(key);
            let enhanced_translation = enhanced_manager.translate(key);
            
            if legacy_translation != enhanced_translation {
                if legacy_translation == key && enhanced_translation == key {
                    // Both systems return the key (translation not found)
                    report.missing_keys.push(key.to_string());
                } else if legacy_translation == key {
                    // Legacy returns key, enhanced has translation
                    report.improved_translations.push(key.to_string());
                } else if enhanced_translation == key {
                    // Legacy has translation, enhanced returns key
                    report.lost_translations.push(key.to_string());
                } else {
                    // Both have translations but they differ
                    report.different_translations.push(TranslationDifference {
                        key: key.to_string(),
                        legacy: legacy_translation,
                        enhanced: enhanced_translation,
                    });
                }
            } else {
                report.matching_translations += 1;
            }
        }
        
        report
    }

    /// Benchmark performance difference between systems
    pub fn benchmark_performance(
        legacy_manager: &LocalizationManager,
        enhanced_manager: &mut EnhancedLocalizationManager,
        test_keys: &[&str],
        iterations: usize,
    ) -> PerformanceBenchmark {
        use std::time::Instant;
        
        // Benchmark legacy system
        let start = Instant::now();
        for _ in 0..iterations {
            for &key in test_keys {
                let _ = legacy_manager.translate(key);
            }
        }
        let legacy_duration = start.elapsed();
        
        // Benchmark enhanced system (first run - includes lazy loading)
        let start = Instant::now();
        for _ in 0..iterations {
            for &key in test_keys {
                let _ = enhanced_manager.translate(key);
            }
        }
        let enhanced_duration_first = start.elapsed();
        
        // Benchmark enhanced system (second run - uses cache)
        let start = Instant::now();
        for _ in 0..iterations {
            for &key in test_keys {
                let _ = enhanced_manager.translate(key);
            }
        }
        let enhanced_duration_cached = start.elapsed();
        
        PerformanceBenchmark {
            legacy_duration,
            enhanced_duration_first,
            enhanced_duration_cached,
            iterations,
            keys_tested: test_keys.len(),
        }
    }
}

/// Validation report for translation compatibility
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub matching_translations: usize,
    pub missing_keys: Vec<String>,
    pub improved_translations: Vec<String>,
    pub lost_translations: Vec<String>,
    pub different_translations: Vec<TranslationDifference>,
}

impl ValidationReport {
    fn new() -> Self {
        Self {
            matching_translations: 0,
            missing_keys: Vec::new(),
            improved_translations: Vec::new(),
            lost_translations: Vec::new(),
            different_translations: Vec::new(),
        }
    }

    /// Check if migration is safe (no lost translations)
    pub fn is_migration_safe(&self) -> bool {
        self.lost_translations.is_empty() && self.different_translations.is_empty()
    }

    /// Get total number of issues
    pub fn total_issues(&self) -> usize {
        self.missing_keys.len() + 
        self.lost_translations.len() + 
        self.different_translations.len()
    }

    /// Print a summary of the validation
    pub fn print_summary(&self) {
        info!("Translation Validation Report:");
        info!("  Matching translations: {}", self.matching_translations);
        info!("  Missing keys: {}", self.missing_keys.len());
        info!("  Improved translations: {}", self.improved_translations.len());
        info!("  Lost translations: {}", self.lost_translations.len());
        info!("  Different translations: {}", self.different_translations.len());
        
        if !self.lost_translations.is_empty() {
            warn!("Lost translations: {:?}", self.lost_translations);
        }
        
        if !self.different_translations.is_empty() {
            warn!("Different translations found:");
            for diff in &self.different_translations {
                warn!("  {}: '{}' -> '{}'", diff.key, diff.legacy, diff.enhanced);
            }
        }
        
        if self.is_migration_safe() {
            info!("Migration is SAFE - no translations will be lost");
        } else {
            error!("Migration has ISSUES - review lost/different translations");
        }
    }
}

/// Translation difference between legacy and enhanced systems
#[derive(Debug, Clone)]
pub struct TranslationDifference {
    pub key: String,
    pub legacy: String,
    pub enhanced: String,
}

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct PerformanceBenchmark {
    pub legacy_duration: std::time::Duration,
    pub enhanced_duration_first: std::time::Duration,
    pub enhanced_duration_cached: std::time::Duration,
    pub iterations: usize,
    pub keys_tested: usize,
}

impl PerformanceBenchmark {
    /// Calculate performance improvement ratio
    pub fn cache_improvement_ratio(&self) -> f64 {
        self.enhanced_duration_first.as_nanos() as f64 / self.enhanced_duration_cached.as_nanos() as f64
    }

    /// Calculate performance vs legacy ratio
    pub fn vs_legacy_ratio(&self) -> f64 {
        self.legacy_duration.as_nanos() as f64 / self.enhanced_duration_cached.as_nanos() as f64
    }

    /// Print benchmark results
    pub fn print_results(&self) {
        info!("Performance Benchmark Results:");
        info!("  Iterations: {}, Keys tested: {}", self.iterations, self.keys_tested);
        info!("  Legacy system: {:?}", self.legacy_duration);
        info!("  Enhanced system (first run): {:?}", self.enhanced_duration_first);
        info!("  Enhanced system (cached): {:?}", self.enhanced_duration_cached);
        info!("  Cache improvement: {:.2}x faster", self.cache_improvement_ratio());
        info!("  vs Legacy: {:.2}x faster", self.vs_legacy_ratio());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_conversion() {
        assert_eq!(
            I18nMigration::convert_language(LegacyLanguage::English),
            EnhancedLanguage::English
        );
        assert_eq!(
            I18nMigration::convert_language(LegacyLanguage::ChineseSimplified),
            EnhancedLanguage::ChineseSimplified
        );
    }

    #[test]
    fn test_language_conversion_back() {
        assert_eq!(
            I18nMigration::convert_language_back(EnhancedLanguage::English),
            Some(LegacyLanguage::English)
        );
        assert_eq!(
            I18nMigration::convert_language_back(EnhancedLanguage::ChineseSimplified),
            Some(LegacyLanguage::ChineseSimplified)
        );
        assert_eq!(
            I18nMigration::convert_language_back(EnhancedLanguage::Japanese),
            None
        );
    }

    #[test]
    fn test_validation_report() {
        let report = ValidationReport::new();
        assert!(report.is_migration_safe());
        assert_eq!(report.total_issues(), 0);
    }
}
