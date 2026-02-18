# Logging System Specification

**Module:** `src/logging` (internal module)  
**Version:** 0.2.0  
**Dependencies:** Bevy only — zero external logging crates. Standard library for I/O and threading.  
**Status:** Core Subsystem

---

## 1. Goals

| Goal | Description |
|------|-------------|
| **Zero extra deps** | Built entirely on top of `bevy::log` (which wraps `tracing`) and `std`. No external crates like `log4rs` or `crossbeam`. |
| **Async I/O (AAA)** | **Non-blocking logging**. The main thread pushes formatted strings to a channel; a background worker thread handles disk I/O. Prevents frame stutter. |
| **File sink** | Write the session log to `AppPaths::log_file` with automatic rotation on startup. |
| **Structured tags** | Every log line emitted by the framework carries a typed subsystem tag, formatted consistently as `[TAG]`. |
| **Per-subsystem filtering** | Runtime-configurable filter per subsystem without restarting the process. |
| **Ring buffer** | Optional in-memory circular buffer for the diagnostics overlay (F1 screen). |
| **State change tracing** | Automatic one-line log whenever `AppState` transitions. |
| **Dev / Prod defaults** | Debug mode enables verbose output; production mode suppresses noise. |
| **Panic capture** | The panic hook claims the file lock and writes a final entry before unwinding. |
| **No self-reference** | Log messages are always plain English literals. They never go through the localization system. |

---

## 2. Module Layout

The logging system is a core internal module, not a separate crate. This simplifies dependency management (access to `AppPaths`, `AppMetadata`) and compilation.

```
src/logging/
├── mod.rs            ← public API, LogManagerPlugin
├── config.rs         ← LogConfig resource (filter, level, settings)
├── async_writer.rs   ← internal worker thread & channel logic
├── layer.rs          ← generic tracing Layer implementation
├── subsystem.rs      ← Subsystem enum, tag formatting
├── state_trace.rs    ← automatic AppState transition logging
├── panic_hook.rs     ← panic handler
├── macros.rs         ← ltag! convenience macro
└── ring_buffer.rs    ← LogRingBuffer (feature = "ring_buffer")
```

---

## 3. Core Types

### 3.1 Subsystem Enum

Identifies the originating framework subsystem for a log message.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subsystem {
    Boot,
    Config,
    Paths,
    Assets,
    SingleInstance,
    Theme,
    Splash,
    Loading,
    Menu,
    Settings,
    Pause,
    Modal,
    Transitions,
    Diagnostics,
    Locale,
    Game,
    State,
    Unknown,
}

impl Subsystem {
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Boot          => "[BOOT]",
            Self::Config        => "[CFG]",
            // ... (mappings for all variants)
            Self::Unknown       => "[?]",
        }
    }
}
```

### 3.2 LogConfig Resource

Runtime-configurable settings.

```rust
#[derive(Resource, Debug, Clone)]
pub struct LogConfig {
    pub level: LogLevel,
    pub subsystem_levels: HashMap<Subsystem, LogLevel>,
    pub show_timestamp: bool,
    pub show_state: bool,
    pub file_enabled: bool,
    pub rotate_keep: usize,
    /// Flush interval is handled by the async worker.
    pub flush_interval_secs: u32, 
}
```

### 3.3 LogLevel

Mirrors `bevy::log::Level` but is serializable/reflectable.

---

## 4. The `ltag!` Macro

The primary interface for logging.

```rust
#[macro_export]
macro_rules! ltag {
    ($level:ident, $sub:expr, $fmt:literal $(, $arg:expr)*) => {
        $crate::paste::paste! {
            bevy::log::[< $level >]!(
                concat!("{} ", $fmt),
                $crate::logging::Subsystem::tag(&$sub)
                $(, $arg)*
            )
        }
    };
}
```

---

## 5. Async File Architecture (AAA Pattern)

To guarantee that logging never causes frame skips (stuttering), all disk I/O is offloaded to a dedicated thread.

### 5.1 Architecture

```
[Main Thread / Tracing]
   │
   ▼
ltag!(...) ──► Tracing Subscriber ──► AsyncFileLayer
                                            │
                                            ▼
                                     mpsc::Sender<String>
                                            │
                                     (Non-blocking Send)
                                            │
                                            ▼
[Worker Thread] ◄──────────────────── mpsc::Receiver<String>
   │
   ▼
Buffer (User Space) ──► Mutex<BufWriter<File>> ──► Disk
```

### 5.2 AsyncFileLayer

A custom `tracing_subscriber::Layer` that formats the log message into a `String` and sends it to the worker channel.

```rust
pub struct AsyncFileLayer {
    sender: std::sync::mpsc::Sender<String>,
    config: Arc<RwLock<LogConfig>>,
}

impl<S: Subscriber> Layer<S> for AsyncFileLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        if !self.config.file_enabled { return; }
        
        // Format message to string (CPU only, fast)
        let line = format_event(event, &self.config);
        
        // Send to worker (Memory op, near instant)
        let _ = self.sender.send(line);
    }
}
```

### 5.3 Log Worker Thread

Spawned once during plugin initialization.

```rust
pub fn spawn_log_worker(
    receiver: std::sync::mpsc::Receiver<String>,
    writer: Arc<Mutex<BufWriter<File>>>,
    flush_interval: Duration,
) {
    std::thread::Builder::new()
        .name("log-worker".into())
        .spawn(move || {
            let mut last_flush = Instant::now();
            
            // Loop forever until channel disconnects (app exit)
            loop {
                // block until message available or timeout (for flush)
                match receiver.recv_timeout(flush_interval) {
                    Ok(line) => {
                        // Lock only to write
                        if let Ok(mut w) = writer.lock() {
                            let _ = w.write_all(line.as_bytes());
                            let _ = w.write_all(b"\n");
                        }
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        // Flush periodically
                        if let Ok(mut w) = writer.lock() {
                            let _ = w.flush();
                        }
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        })
        .expect("Failed to spawn log worker");
}
```

**Key Benefit:** The main thread never waits for disk spin-up or OS buffers.

---

## 6. Panic Handling

When a panic occurs, the main thread (or the crashing thread) is unwinding. The background worker might still be processing. We need to maintain the log file integrity.

**Strategy:** The `Arc<Mutex<BufWriter<File>>>` is shared.

1. **Worker:** Locks it briefly for each batch.
2. **Panic Hook:** Attempts to lock it.
    * If successful: Writes the panic message and flushes synchronously.
    * If contended: The worker is likely writing. The hook waits (spins/blocks) until the lock is available.

```rust
pub fn install_panic_hook(writer: Arc<Mutex<BufWriter<File>>>) {
    let prev_hook = std::panic::take_hook();
    
    std::panic::set_hook(Box::new(move |info| {
        let panic_msg = format_panic(info);
        
        // CRITICAL: We are crashing. We must write this.
        // We accept blocking here because the app is dying anyway.
        if let Ok(mut w) = writer.lock() {
            let _ = w.write_all(panic_msg.as_bytes());
            let _ = w.flush();
        } else {
            // If lock is poisoned (worker thread crashed?), we can't write safely.
            eprintln!("Log writer lock poisoned during panic!");
        }
        
        prev_hook(info);
    }));
}
```

---

## 7. Configuration & State Tracing

(Same as previous logic, adapted for new module structure)

---

## 8. Plugin Implementation

```rust
pub struct LogManagerPlugin {
    pub config: LogConfig,
}

impl Plugin for LogManagerPlugin {
    fn build(&self, app: &mut App) {
        // 1. Setup paths/config
        // 2. Open log file
        let file = OpenOptions::new().create(true).append(true).open(&path).unwrap();
        let writer = Arc::new(Mutex::new(BufWriter::new(file)));
        
        // 3. Create Channel
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        
        // 4. Spawn Worker
        spawn_log_worker(rx, writer.clone(), Duration::from_secs(5));
        
        // 5. Build Layer
        let layer = AsyncFileLayer::new(tx, self.config.clone());
        
        // 6. Install Layer (workaround for Bevy < 0.14 dynamic layering)
        // ... implementation details ...
        
        // 7. Install Panic Hook
        install_panic_hook(writer);
    }
}
```

---

## 9. Integration

Add `LogManagerPlugin` early in the app build chain.

```rust
// main.rs
app.add_plugins(bevy_launchpad::logging::LogManagerPlugin::default());
```
