use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

// Global sender to allow the Tracing Layer to send logs without being passed the channel directly.
// This is necessary because the Tracing Layer might need to be constructed inside a clear closure
// passed to Bevy's LogPlugin.
pub static LOG_SENDER: OnceLock<Sender<String>> = OnceLock::new();

/// Initializes the global log sender. Should be called once by LogManagerPlugin.
pub fn init_global_sender(sender: Sender<String>) {
    let _ = LOG_SENDER.set(sender);
}

/// Helper to send a log message to the worker.
pub fn send_log(msg: String) {
    if let Some(sender) = LOG_SENDER.get() {
        let _ = sender.send(msg);
    }
}

/// Spawns a background worker thread that writes log messages to disk.
pub fn spawn_log_worker(
    receiver: Receiver<String>,
    writer: Arc<Mutex<BufWriter<File>>>,
    flush_interval: Duration,
) {
    thread::Builder::new()
        .name("bevy_launchpad_log_worker".into())
        .spawn(move || {
            loop {
                match receiver.recv_timeout(flush_interval) {
                    Ok(line) => {
                        if let Ok(mut w) = writer.lock() {
                            let _ = w.write_all(line.as_bytes());
                            let _ = w.write_all(b"\n");
                        }
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        if let Ok(mut w) = writer.lock() {
                            let _ = w.flush();
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        if let Ok(mut w) = writer.lock() {
                            let _ = w.flush();
                        }
                        break;
                    }
                }
            }
        })
        .expect("Failed to spawn log worker thread");
}
