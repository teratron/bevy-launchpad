use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct SplashConfig {
    pub screens: Vec<SplashScreenConfig>,
    /// Show embedded branding when `screens` is empty. Default: true.
    pub show_default_branding: bool,
    /// Skip the entire splash sequence. Default: false.
    pub skip_all: bool,
}

impl Default for SplashConfig {
    fn default() -> Self {
        Self {
            screens: vec![],
            show_default_branding: true,
            skip_all: false,
        }
    }
}

impl SplashConfig {
    /// Returns the screens that will actually be displayed, applying all rules.
    pub fn effective_screens(&self) -> Vec<SplashScreenConfig> {
        if self.skip_all || std::env::var("BEVY_LAUNCHPAD_SKIP_SPLASH").is_ok() {
            return vec![];
        }
        if !self.screens.is_empty() {
            return self.screens.clone();
        }
        if self.show_default_branding {
            vec![SplashScreenConfig::default_branding()]
        } else {
            vec![]
        }
    }
}

// ── Single splash screen ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SplashScreenConfig {
    pub source: SplashSource,
    pub min_duration: f32,         // seconds — cannot be skipped before this
    pub max_duration: Option<f32>, // seconds — auto-advance; None = wait for input
    pub skip: SkipTrigger,
    pub fade_in: f32,
    pub fade_out: f32,
    pub background: Color,
}

#[derive(Debug, Clone)]
pub enum SplashSource {
    /// Path relative to developer's assets/ folder.
    File(String),
    /// Library's embedded "Powered by Bevy Launchpad" image.
    Embedded,
    /// No image — just a solid background color.
    ColorOnly,
}

#[derive(Debug, Clone, Default)]
pub enum SkipTrigger {
    #[default]
    AnyInput, // any keyboard key or mouse button
    EscapeOnly, // only the Escape key
    None,       // cannot be skipped (use for legal screens)
}

// ── Factory constructors ─────────────────────────────────────────────────────

impl SplashScreenConfig {
    /// Library's default branding screen.
    pub fn default_branding() -> Self {
        Self {
            source: SplashSource::Embedded,
            min_duration: 1.5,
            max_duration: Some(2.5),
            skip: SkipTrigger::AnyInput,
            fade_in: 0.3,
            fade_out: 0.3,
            background: Color::BLACK,
        }
    }

    /// Legal / age-rating screen — cannot be skipped.
    pub fn legal(path: &str) -> Self {
        Self {
            source: SplashSource::File(path.into()),
            min_duration: 5.0,
            max_duration: Some(5.0),
            skip: SkipTrigger::None,
            fade_in: 0.5,
            fade_out: 0.5,
            background: Color::WHITE,
        }
    }

    /// Publisher / studio logo screen.
    pub fn studio(path: &str) -> Self {
        Self {
            source: SplashSource::File(path.into()),
            min_duration: 2.0,
            max_duration: Some(3.0),
            skip: SkipTrigger::AnyInput,
            fade_in: 0.4,
            fade_out: 0.4,
            background: Color::BLACK,
        }
    }

    /// Engine / "powered by" logo screen.
    pub fn engine(path: &str) -> Self {
        Self {
            source: SplashSource::File(path.into()),
            min_duration: 1.5,
            max_duration: Some(2.5),
            skip: SkipTrigger::AnyInput,
            fade_in: 0.3,
            fade_out: 0.3,
            background: Color::BLACK,
        }
    }

    // ── Builder methods ───────────────────────────────────────────────────────

    pub fn with_duration(mut self, min: f32, max: f32) -> Self {
        self.min_duration = min;
        self.max_duration = Some(max);
        self
    }

    pub fn with_fade(mut self, fade_in: f32, fade_out: f32) -> Self {
        self.fade_in = fade_in;
        self.fade_out = fade_out;
        self
    }

    pub fn skippable(mut self, trigger: SkipTrigger) -> Self {
        self.skip = trigger;
        self
    }

    pub fn on_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
}
