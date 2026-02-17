use crate::core::splash::sequence::SplashSource;
use bevy::prelude::*;
use std::path::PathBuf;

/// Resolution priority chain:
///
/// 1. Env var override   (BEVY_LAUNCHPAD_ASSETS_ROOT / per-asset overrides)
/// 2. Developer explicit path (set via builder)
/// 3. Bevy default: AssetServer resolves ./assets/ automatically
/// 4. Embedded library fallback (embedded_asset!)
/// 5. Panic with a human-readable message listing all tried paths
#[derive(Debug, Clone)]
pub enum AssetPath {
    /// Relative path for Bevy's AssetServer ("branding/logo.png")
    Project(String),
    /// Compiled into the binary ("embedded://bevy_launchpad/fonts/NotoSans-Regular.ttf")
    Embedded(&'static str),
    /// Absolute filesystem path (mods, editor, env-var override)
    Absolute(PathBuf),
}

impl AssetPath {
    pub fn as_load_path(&self) -> String {
        match self {
            Self::Project(p) => p.clone(),
            Self::Embedded(p) => p.to_string(),
            Self::Absolute(p) => p.to_string_lossy().into_owned(),
        }
    }
}

/// How to locate the developer's assets/ root.
#[derive(Resource, Debug, Clone)]
pub enum AssetsRootStrategy {
    /// Let Bevy handle it — works for both `cargo run` and installed binaries.
    BevyDefault,
    /// Developer-supplied explicit path (relative or absolute).
    Explicit(PathBuf),
    /// Ordered list — first existing directory wins.
    /// Useful for DLC packs, mod support, or split asset locations.
    SearchPaths(Vec<PathBuf>),
}

impl Default for AssetsRootStrategy {
    fn default() -> Self {
        if let Ok(p) = std::env::var("BEVY_LAUNCHPAD_ASSETS_ROOT") {
            return Self::Explicit(PathBuf::from(p));
        }
        Self::BevyDefault
    }
}

pub struct AssetResolver;

impl AssetResolver {
    /// Resolve a font path.
    pub fn font(user_path: Option<&str>, embedded_fallback: &'static str) -> AssetPath {
        if let Ok(p) = std::env::var("BEVY_LAUNCHPAD_FONT_OVERRIDE") {
            return AssetPath::Absolute(PathBuf::from(p));
        }
        match user_path {
            Some(p) => AssetPath::Project(p.into()),
            None => AssetPath::Embedded(embedded_fallback),
        }
    }

    /// Resolve a splash screen image. Returns None when skipping.
    pub fn splash_image(source: &SplashSource) -> Option<AssetPath> {
        if std::env::var("BEVY_LAUNCHPAD_SKIP_SPLASH").is_ok() {
            return None;
        }
        match source {
            SplashSource::File(p) => Some(AssetPath::Project(p.clone())),
            SplashSource::Embedded => Some(AssetPath::Embedded(
                "embedded://bevy_launchpad/branding/default_splash.png",
            )),
            SplashSource::ColorOnly => None,
        }
    }
}
