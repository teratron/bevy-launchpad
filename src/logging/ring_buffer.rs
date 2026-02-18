use bevy::prelude::*;
use std::collections::VecDeque;

use super::config::LogLevel;
use super::subsystem::Subsystem;

/// A single captured log record stored in the ring buffer.
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub timestamp: f64, // elapsed seconds since app start
    pub level: LogLevel,
    pub subsystem: Option<Subsystem>,
    pub message: String,
    pub state_name: Option<String>, // if LogConfig::show_state is true
}

/// In-memory circular buffer of recent log records.
///
/// Available only with feature `ring_buffer`. Consumed by the F1 diagnostics overlay.
#[derive(Resource)]
pub struct LogRingBuffer {
    records: VecDeque<LogRecord>,
    capacity: usize,
}

impl LogRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            records: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Push a new record, dropping the oldest if at capacity.
    pub fn push(&mut self, record: LogRecord) {
        if self.records.len() >= self.capacity {
            self.records.pop_front();
        }
        self.records.push_back(record);
    }

    /// Iterate most-recent-first.
    pub fn iter_recent(&self) -> impl Iterator<Item = &LogRecord> {
        self.records.iter().rev()
    }

    /// Returns all records at or above `min_level`.
    pub fn filter_level(&self, min_level: LogLevel) -> Vec<&LogRecord> {
        self.records
            .iter()
            .filter(|r| r.level >= min_level)
            .collect()
    }

    /// Clears all records.
    pub fn clear(&mut self) {
        self.records.clear();
    }
}
