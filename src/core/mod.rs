//! Core framework logic without UI dependencies.

pub mod boot;
pub mod loading;
pub mod plugin;
pub mod splash;
pub mod states;

pub use plugin::LaunchpadCorePlugin;
