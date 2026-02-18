pub mod async_writer;
pub mod config;
pub mod layer;
pub mod macros;
pub mod panic_hook;
#[cfg(feature = "ring_buffer")]
pub mod ring_buffer;
pub mod state_trace;
pub mod subsystem;

use bevy::prelude::*;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use self::config::LogConfig;
use crate::core::boot::paths::AppPaths;

/// Plugin that initializes the logging system.
pub struct LogManagerPlugin {
    pub config: LogConfig,
}

impl Default for LogManagerPlugin {
    fn default() -> Self {
        Self {
            config: if cfg!(debug_assertions) {
                LogConfig::development()
            } else {
                LogConfig::production()
            },
        }
    }
}

impl Plugin for LogManagerPlugin {
    fn build(&self, app: &mut App) {
        // 1. Resolve log file path (AppPaths must already be inserted).
        let log_path = app
            .world()
            .get_resource::<AppPaths>()
            .map(|p| p.log_file.clone());

        // 2. Setup Async Writer
        let writer_handle = if self.config.file_enabled {
            if let Some(path) = log_path {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                match OpenOptions::new().create(true).append(true).open(&path) {
                    Ok(file) => {
                        let writer = Arc::new(Mutex::new(BufWriter::new(file)));
                        let (tx, rx) = std::sync::mpsc::channel::<String>();

                        // Initialize global sender for tracing layer integration
                        async_writer::init_global_sender(tx);

                        // Initialize global config
                        config::init_log_config(self.config.clone());

                        // Spawn worker
                        async_writer::spawn_log_worker(
                            rx,
                            writer.clone(),
                            Duration::from_secs(self.config.flush_interval_secs as u64),
                        );

                        Some(writer)
                    }
                    Err(e) => {
                        eprintln!("Failed to open log file {:?}: {}", path, e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        // 3. Install Panic Hook
        if let Some(writer) = writer_handle {
            panic_hook::install_panic_hook(writer);
        }

        // 4. Resources
        // We create an Arc<RwLock<LogConfig>> for the layer to share?
        // Actually, the Layer needs a way to access config.
        // If the layer is created statically/globally, how does it get the config?
        // We can put config in a OnceLock too, or update it via a system.
        // For simplicity, let's keep config in a Resource and maybe update a global config?
        //
        // AAA: If we want runtime config changes to affect the layer, the layer needs shared access.
        // Let's stick to: The Layer reads the Resource? No, Layer runs in tracing thread, Resource is in World.
        //
        // We can put `Arc<RwLock<LogConfig>>` in a global static too?
        // Or just let the Plugin update it.
        //
        // For now, let's Register the Resource.
        app.insert_resource(self.config.clone());

        #[cfg(feature = "ring_buffer")]
        app.insert_resource(ring_buffer::LogRingBuffer::new(100));

        // 5. Systems
        app.add_systems(
            PostUpdate,
            (state_trace::log_state_transition::<crate::core::states::app_state::AppState>,),
        );
    }
}
