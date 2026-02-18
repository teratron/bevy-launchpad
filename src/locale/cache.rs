use super::resource::Localization;
use bevy::prelude::*;
use std::collections::HashMap;

/// Per-locale string cache. Eliminates repeated Fluent lookups during the same frame.
/// Automatically invalidated when the language changes.
#[derive(Default)]
pub struct LocalizedStrings {
    cache: HashMap<String, String>,
}

impl LocalizedStrings {
    /// Fetch a localized string, using the cache when possible.
    pub fn get(&mut self, key: &str, loc: &Localization) -> String {
        // Fast path: check cache
        if let Some(val) = self.cache.get(key) {
            return val.clone();
        }

        // Slow path: resolve and cache
        let val = loc.t(key);
        self.cache.insert(key.to_string(), val.clone());
        val
    }

    /// Invalidate the entire cache (called automatically on language change).
    pub fn invalidate(&mut self) {
        self.cache.clear();
    }
}
