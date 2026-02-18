/// Log a message with a typed subsystem tag.
///
/// # Examples
/// ```
/// ltag!(info, Boot, "Initializing version {}", env!("CARGO_PKG_VERSION"));
/// ltag!(warn, Assets, "Manifest not found at {:?}, using defaults", path);
/// ltag!(error, SingleInstance, "Lock acquisition failed: {}", e);
/// ```
#[macro_export]
macro_rules! ltag {
    ($level:ident, $sub:expr, $fmt:literal $(, $arg:expr)*) => {
        $crate::paste::paste! {
            bevy::log::[< $level >]!(
                concat!("{} ", $fmt),
                $crate::logging::subsystem::Subsystem::tag(&$sub)
                $(, $arg)*
            )
        }
    };
}
