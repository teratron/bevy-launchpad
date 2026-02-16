//! Utility functions and helpers.

pub mod persistence;
pub mod platform;
pub mod single_instance;
pub mod singleton;
pub mod validation;

pub use persistence::Persistence;
pub use single_instance::{SingleInstanceError, SingleInstanceLock, acquire_single_instance_lock};
pub use singleton::SingleInstance;
