//! Utility functions and helpers.

pub mod persistence;
pub mod platform;
pub mod single_instance;
pub mod singleton;
pub mod validation;

pub use persistence::{Persistable, load_ron, save_ron};
pub use single_instance::{SingleInstanceError, SingleInstanceLock, acquire_single_instance_lock};
// pub use singleton::SingleInstance; // Removed in dead code cleanup
