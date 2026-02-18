use super::language::Language;
use bevy::prelude::*;

/// Fired after the language has been successfully changed and the new bundles are loaded.
#[derive(Event, Debug, Clone)]
pub struct LanguageChanged {
    pub old: Language,
    pub new: Language,
}
