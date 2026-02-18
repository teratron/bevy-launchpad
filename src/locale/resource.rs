use super::{bundle::FluentBundleType, language::Language};
use bevy::prelude::*;
use fluent_bundle::FluentArgs;

/// Resource that owns the active and fallback Fluent bundles.
/// registered as `NonSend` because `FluentBundle` (with default memoizer) is `!Sync`.
pub struct Localization {
    /// Currently active language.
    pub language: Language,
    /// Bundle for the active language.
    main_bundle: FluentBundleType,
    /// Fallback bundle (always en-US). Used when a key is missing in `main_bundle`.
    fallback_bundle: FluentBundleType,
}

impl Localization {
    /// Create a new Localization from pre-built bundles.
    pub fn new(language: Language, main: FluentBundleType, fallback: FluentBundleType) -> Self {
        Self {
            language,
            main_bundle: main,
            fallback_bundle: fallback,
        }
    }

    /// Translate a key. Returns the key string itself if not found in any bundle.
    pub fn t(&self, key: &str) -> String {
        let args = FluentArgs::new();
        self.t_args(key, &args)
    }

    /// Translate a key with Fluent arguments.
    pub fn t_args(&self, key: &str, args: &FluentArgs) -> String {
        // 1. Try main bundle
        if let Some(msg) = self.main_bundle.get_message(key)
            && let Some(pattern) = msg.value()
        {
            let mut errors = vec![];
            let value = self
                .main_bundle
                .format_pattern(pattern, Some(args), &mut errors);
            if errors.is_empty() {
                return value.into_owned();
            } else {
                warn!("Fluent errors for key '{}' (main): {:?}", key, errors);
            }
        }

        // 2. Try fallback bundle
        if let Some(msg) = self.fallback_bundle.get_message(key)
            && let Some(pattern) = msg.value()
        {
            let mut errors = vec![];
            let value = self
                .fallback_bundle
                .format_pattern(pattern, Some(args), &mut errors);
            if errors.is_empty() {
                return value.into_owned();
            } else {
                warn!("Fluent errors for key '{}' (fallback): {:?}", key, errors);
            }
        }

        // 3. Fallback to key
        key.to_string()
    }

    /// Check whether a key exists in any bundle.
    pub fn has_key(&self, key: &str) -> bool {
        self.main_bundle.has_message(key) || self.fallback_bundle.has_message(key)
    }

    /// Update the current language and main bundle.
    pub fn update_language(&mut self, language: Language, main_bundle: FluentBundleType) {
        self.language = language;
        self.main_bundle = main_bundle;
    }
}
