use tracing::{Event, Subscriber};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

use super::async_writer;
use super::config::{LOG_CONFIG, LogConfig};

/// A tracing Layer that formats events and sends them to the async worker.
#[derive(Default)]
pub struct AsyncFileLayer;

impl<S: Subscriber> Layer<S> for AsyncFileLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let Some(cfg_lock) = LOG_CONFIG.get() else {
            return;
        };
        let Ok(cfg) = cfg_lock.read() else { return };

        if !cfg.file_enabled {
            return;
        }
        let line = format_event(event, &cfg);
        async_writer::send_log(line);
    }
}

// ... formatting logic same as before ...

fn format_event(event: &Event<'_>, config: &LogConfig) -> String {
    let mut message = String::to_string(&String::new());

    let mut visitor = LogMessageVisitor {
        message: String::new(),
    };
    event.record(&mut visitor);

    if config.show_timestamp {
        message.push_str(&chrono_free_timestamp());
        message.push(' ');
    }

    let level_str = match *event.metadata().level() {
        tracing::Level::TRACE => "TRACE",
        tracing::Level::DEBUG => "DEBUG",
        tracing::Level::INFO => "INFO ",
        tracing::Level::WARN => "WARN ",
        tracing::Level::ERROR => "ERROR",
    };
    message.push_str(level_str);
    message.push(' ');

    // Scan for Subsystem tag in the message?
    // ltag! puts it at start.
    // If not present, we could check event fields?
    // For now, just append message.
    message.push_str(&visitor.message);

    message
}

struct LogMessageVisitor {
    message: String,
}

impl tracing::field::Visit for LogMessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            use std::fmt::Write;
            let _ = write!(self.message, "{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message.push_str(value);
        }
    }
}

fn chrono_free_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    format_unix_timestamp(secs, millis)
}

fn format_unix_timestamp(secs: u64, millis: u32) -> String {
    let days = secs / 86400;
    let rem_secs = secs % 86400;
    let hours = rem_secs / 3600;
    let rem_secs_2 = rem_secs % 3600;
    let minutes = rem_secs_2 / 60;
    let seconds = rem_secs_2 % 60;

    let z = days + 719468;
    let era = z / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = doe - doe / 1460 + doe / 36524 - doe / 146096;
    let y = (yoe as i32) + era as i32 * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = (mp as i32) + if mp < 10 { 3 } else { -9 };
    let y = y + if m <= 2 { 1 } else { 0 };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        y, m, d, hours, minutes, seconds, millis
    )
}
