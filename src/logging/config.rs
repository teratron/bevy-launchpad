use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::subsystem::Subsystem;

use std::sync::{OnceLock, RwLock};

pub static LOG_CONFIG: OnceLock<RwLock<LogConfig>> = OnceLock::new();

pub fn init_log_config(config: LogConfig) {
    let _ = LOG_CONFIG.set(RwLock::new(config));
}

/// Mirrors `bevy::log::Level` but is `Copy`, serializable, and reflectable.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    /// Completely silence a subsystem.
    Off,
}

impl From<LogLevel> for bevy::log::Level {
    fn from(l: LogLevel) -> Self {
        match l {
            LogLevel::Trace => bevy::log::Level::TRACE,
            LogLevel::Debug => bevy::log::Level::DEBUG,
            LogLevel::Info => bevy::log::Level::INFO,
            LogLevel::Warn => bevy::log::Level::WARN,
            LogLevel::Error => bevy::log::Level::ERROR,
            LogLevel::Off => bevy::log::Level::ERROR, // closest approximation
        }
    }
}

/// Runtime-configurable logging settings.
///
/// Inserted as a Bevy resource during plugin setup.
/// Changes take effect on the next frame via `apply_log_config_changes`.
#[derive(Resource, Debug, Clone)]
pub struct LogConfig {
    /// Global minimum log level. Messages below this level are silently dropped.
    pub level: LogLevel,

    /// Per-subsystem overrides.
    /// If a subsystem is absent from this map, `level` applies.
    ///
    /// Example: disable `[LOAD]` spam during gameplay:
    /// ```
    /// config.subsystem_levels.insert(Subsystem::Loading, LogLevel::Warn);
    /// ```
    pub subsystem_levels: HashMap<Subsystem, LogLevel>,

    /// When `true`, each log line includes a wall-clock timestamp.
    pub show_timestamp: bool,

    /// When `true`, each log line includes the Bevy `AppState` name.
    pub show_state: bool,

    /// When `true`, write output to the file sink in addition to stdout.
    pub file_enabled: bool,

    /// Number of old session log files to retain alongside the current one.
    /// Files are named `session.log`, `session.1.log`, `session.2.log`, etc.
    /// Set to 0 to disable rotation (always overwrite `session.log`).
    pub rotate_keep: usize,

    /// Flush interval in seconds.
    pub flush_interval_secs: u32,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            subsystem_levels: HashMap::new(),
            show_timestamp: true,
            show_state: false,
            file_enabled: true,
            rotate_keep: 3,
            flush_interval_secs: 5,
        }
    }
}

impl LogConfig {
    /// Preset for release builds: Info level, no state annotations, file enabled.
    pub fn production() -> Self {
        Self::default()
    }

    /// Preset for debug builds: Debug level, timestamps + state, file enabled.
    pub fn development() -> Self {
        Self {
            level: LogLevel::Debug,
            show_state: true,
            flush_interval_secs: 1,
            ..Self::default()
        }
    }

    /// Returns the effective log level for the given subsystem.
    pub fn effective_level(&self, subsystem: Subsystem) -> LogLevel {
        self.subsystem_levels
            .get(&subsystem)
            .copied()
            .unwrap_or(self.level)
    }
}
