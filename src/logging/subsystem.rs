use bevy::prelude::*;

/// Identifies the originating framework subsystem for a log message.
///
/// Displayed as `[BOOT]`, `[ASSETS]`, etc. in log output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Subsystem {
    // ── Framework lifecycle ────────────────────────────────
    Boot,
    Config,
    Paths,
    Assets,
    SingleInstance,
    // ── UI layer ───────────────────────────────────────────
    Theme,
    Splash,
    Loading,
    Menu,
    Settings,
    Pause,
    Modal,
    Transitions,
    Diagnostics,
    // ── Localization ───────────────────────────────────────
    Locale,
    // ── Game (user-defined) ────────────────────────────────
    Game,
    // ── State machine ──────────────────────────────────────
    State,
    // ── Catch-all ──────────────────────────────────────────
    /// Used when a log originates outside a known subsystem.
    Unknown,
}

impl Subsystem {
    /// Short uppercase tag used in log output, e.g. `[BOOT]`.
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Boot => "[BOOT]",
            Self::Config => "[CFG]",
            Self::Paths => "[PATHS]",
            Self::Assets => "[ASSETS]",
            Self::SingleInstance => "[LOCK]",
            Self::Theme => "[THEME]",
            Self::Splash => "[SPLASH]",
            Self::Loading => "[LOAD]",
            Self::Menu => "[MENU]",
            Self::Settings => "[SETTINGS]",
            Self::Pause => "[PAUSE]",
            Self::Modal => "[MODAL]",
            Self::Transitions => "[FADE]",
            Self::Diagnostics => "[DIAG]",
            Self::Locale => "[LOCALE]",
            Self::Game => "[GAME]",
            Self::State => "[STATE]",
            Self::Unknown => "[?]",
        }
    }
}
