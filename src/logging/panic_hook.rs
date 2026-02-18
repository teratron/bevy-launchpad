use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

/// Install a panic hook that flushes the file sink before propagating the panic.
pub fn install_panic_hook(writer: Arc<Mutex<BufWriter<File>>>) {
    let prev_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        // 1. Format the panic message.
        let location = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()));
        let payload = info
            .payload()
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.as_str()))
            .unwrap_or("<non-string panic>");

        let msg = format!(
            "\n[CRASH] PANIC occurred at {}: {}\n",
            location.as_deref().unwrap_or("unknown"),
            payload
        );

        // 2. Write and flush to file.
        // We attempt to lock. If the worker thread holds it, we might block.
        // Since we are crashing, blocking is acceptable.
        // However, if the worker thread is the one that panicked, the lock might be poisoned.
        match writer.lock() {
            Ok(mut w) => {
                let _ = w.write_all(msg.as_bytes());
                let _ = w.flush();
            }
            Err(poisoned) => {
                // Lock is poisoned. Try to recover the guard to write the final message.
                // This is a "best effort" attempt.
                let mut w = poisoned.into_inner();
                let _ = w.write_all(msg.as_bytes());
                let _ = w.write_all(b"\n(Lock was poisoned)");
                let _ = w.flush();
            }
        }

        // 3. Propagate to the previous hook (likely Bevy's or default)
        prev_hook(info);
    }));
}
