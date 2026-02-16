//! Boot sequence module.

pub mod config;
pub mod paths;
pub mod sequence;

pub use config::BootConfig;
pub use paths::BootPaths;
pub use sequence::{BootSequence, update_boot_progress};
