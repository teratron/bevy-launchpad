use crate::utils::platform::get_data_dir;
use bevy::prelude::*;
use std::path::PathBuf;

/// Resource that stores resolved platform-specific paths.
#[derive(Resource, Debug, Clone)]
pub struct AppPaths {
    /// Directory for configuration and persistent data.
    pub data_dir: PathBuf,
    /// Directory for game assets.
    pub assets_dir: PathBuf,
    /// Full path to the settings file.
    pub settings_file: PathBuf,
    /// Path to the session log file.
    pub log_file: PathBuf,
    /// Path to the file used for single-instance locking.
    pub instance_lock_file: PathBuf,
}

impl AppPaths {
    /// Resolves paths based on the application name and environment.
    pub fn new(app_name: &str) -> Self {
        let data_dir = get_data_dir(app_name);

        // Asset directory resolution logic (similar to old project)
        let assets_dir = resolve_assets_dir();

        Self {
            settings_file: data_dir.join("settings.ron"),
            log_file: data_dir.join("session.log"),
            instance_lock_file: data_dir.join("instance.lock"),
            data_dir,
            assets_dir,
        }
    }

    /// Ensures that the data directory exists on disk.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        if !self.data_dir.exists() {
            std::fs::create_dir_all(&self.data_dir)?;
        }
        Ok(())
    }
}

fn resolve_assets_dir() -> PathBuf {
    let mut assets_dir = PathBuf::from("assets");

    if !assets_dir.exists()
        && let Some(exe_dir) = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
    {
        let local_assets = exe_dir.join("assets");
        if local_assets.exists() {
            return local_assets;
        }

        // Try parent of exe (e.g. from target/debug/ up to project root)
        if let Some(parent) = exe_dir.parent() {
            let parent_assets = parent.join("assets");
            if parent_assets.exists() {
                assets_dir = parent_assets;
            }
        }
    }

    if assets_dir.exists() {
        assets_dir.canonicalize().unwrap_or(assets_dir)
    } else {
        assets_dir
    }
}
