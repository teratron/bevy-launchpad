use bevy::prelude::*;

/// Attach to any `Text`-bearing entity to opt into automatic re-translation on language change.
#[derive(Component, Debug, Clone)]
pub struct LocalizedText {
    /// The Fluent message key.
    pub key: String,
}

impl LocalizedText {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}
