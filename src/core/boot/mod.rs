//! Boot sequence module.

pub mod cli;
pub mod config;
pub mod metadata;
pub mod paths;
pub mod sequence;

pub use cli::CliArgs;
pub use config::BootConfig;
pub use metadata::AppMetadata;
pub use paths::AppPaths;
pub use sequence::{BootSequence, update_boot_progress};
