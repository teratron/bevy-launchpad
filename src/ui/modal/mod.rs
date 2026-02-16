//! Modal windows module.

pub mod alert;
pub mod confirm;
pub mod dialog;

pub use dialog::{ModalRoot, spawn_dialog};
