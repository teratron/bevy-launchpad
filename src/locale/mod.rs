//! Internal localization module.

pub mod bundle;
pub mod cache;
pub mod component;
pub mod event;
pub mod language;
pub mod plugin;
pub mod resource;
pub mod utils;

pub use component::LocalizedText;
pub use event::LanguageChanged;
pub use language::Language;
pub use plugin::LocalizationPlugin;
pub use resource::Localization;
