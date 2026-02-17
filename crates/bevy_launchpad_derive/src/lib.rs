//! Procedural macros for bevy_launchpad.
//!
//! Do not use this crate directly — import via `bevy_launchpad::prelude::*`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Fields,
    Result,
};

// ── Naming convention ─────────────────────────────────────────────────────────

/// Maps conventional variant names to LaunchpadStates method names.
/// Key = variant name (exact, case-sensitive).
/// Value = method name.
const CONVENTION: &[(&str, &str)] = &[
    ("Booting", "booting"),
    ("Loading", "loading"),
    ("Splash",  "splash"),
    ("Menu",    "menu"),
    ("Playing", "playing"),
    ("Paused",  "paused"),
];

/// All methods that MUST be mapped for a valid impl.
const REQUIRED_METHODS: &[&str] =
    &["booting", "loading", "splash", "menu", "playing", "paused"];

// ── Entry point ───────────────────────────────────────────────────────────────

/// Derive macro that generates `impl LaunchpadStates for YourEnum`.
///
/// # Variant name convention
///
/// If your variant names match the convention exactly, no attributes are needed:
///
/// ```rust
/// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
/// #[derive(LaunchpadStates)]
/// enum GameState {
///     #[default]
///     Booting, Loading, Splash, Menu, Playing, Paused,
///     Credits,      // custom — ignored by this macro
///     LevelSelect,  // custom — ignored by this macro
/// }
/// ```
///
/// # Explicit attribute mapping
///
/// When your variant names differ from the convention, annotate them:
///
/// ```rust
/// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
/// #[derive(LaunchpadStates)]
/// enum MyState {
///     #[default]
///     #[launchpad(booting)] Init,
///     #[launchpad(loading)] Load,
///     #[launchpad(splash)]  Intro,
///     #[launchpad(menu)]    Home,
///     #[launchpad(playing)] InGame,
///     #[launchpad(paused)]  Pause,
///     Cutscene,   // custom
/// }
/// ```
///
/// Convention-based and attribute-based mappings can be mixed freely.
#[proc_macro_derive(LaunchpadStates, attributes(launchpad))]
pub fn derive_launchpad_states(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match impl_launchpad_states(&input) {
        Ok(ts)  => ts.into(),
        Err(e)  => e.to_compile_error().into(),
    }
}

// ── Core logic ────────────────────────────────────────────────────────────────

fn impl_launchpad_states(input: &DeriveInput) -> Result<TokenStream2> {
    let enum_name = &input.ident;

    // Only enums are supported
    let data_enum = match &input.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(
            enum_name,
            "#[derive(LaunchpadStates)] can only be applied to enums",
        )),
    };

    // Collect mapping: method_name → variant TokenStream
    // Use IndexMap-style ordered Vec so error messages are deterministic.
    let mut resolved: Vec<(&str, TokenStream2)> = Vec::new();

    // Track which methods have been explicitly attributed (for duplicate detection)
    let mut attributed: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for variant in &data_enum.variants {
        // Only unit variants are valid (no tuple/struct variants)
        if !matches!(&variant.fields, Fields::Unit) {
            return Err(Error::new_spanned(
                &variant.ident,
                "#[derive(LaunchpadStates)] only supports unit enum variants",
            ));
        }

        let variant_ident = &variant.ident;
        let variant_name  = variant_ident.to_string();

        // --- Pass 1: explicit #[launchpad(<method>)] attribute ---
        for attr in &variant.attrs {
            if !attr.path().is_ident("launchpad") {
                continue;
            }
            let method_ident: syn::Ident = attr.parse_args().map_err(|_| {
                Error::new_spanned(
                    attr,
                    "expected #[launchpad(<method_name>)], e.g. #[launchpad(booting)]",
                )
            })?;
            let method_name = method_ident.to_string();

            // Validate the method name is one of the required set
            if !REQUIRED_METHODS.contains(&method_name.as_str()) {
                return Err(Error::new_spanned(
                    &method_ident,
                    format!(
                        "`{}` is not a valid LaunchpadStates method. \
                         Valid values: {}",
                        method_name,
                        REQUIRED_METHODS.join(", ")
                    ),
                ));
            }

            // Detect duplicate attribute mapping
            if attributed.contains(&method_name) {
                return Err(Error::new_spanned(
                    variant_ident,
                    format!(
                        "duplicate #[launchpad({})] — another variant already \
                         maps to this method",
                        method_name
                    ),
                ));
            }

            attributed.insert(method_name.clone());

            // Remove any convention-based entry for this method (attr wins)
            resolved.retain(|(m, _)| *m != method_name.as_str());

            // Find the static str key
            let method_key = REQUIRED_METHODS
                .iter()
                .find(|&&m| m == method_name.as_str())
                .copied()
                .unwrap();

            resolved.push((method_key, quote! { #enum_name::#variant_ident }));
        }

        // --- Pass 2: naming convention (only if not already attributed) ---
        for &(conv_name, method_name) in CONVENTION {
            if variant_name == conv_name
                && !attributed.contains(method_name)
                && !resolved.iter().any(|(m, _)| *m == method_name)
            {
                resolved.push((method_name, quote! { #enum_name::#variant_ident }));
            }
        }
    }

    // --- Verify all required methods are present ---
    let mut missing: Vec<String> = Vec::new();
    for &method in REQUIRED_METHODS {
        if !resolved.iter().any(|(m, _)| *m == method) {
            let conventional_name = convention_variant_name(method);
            missing.push(format!(
                "  • `{}` — add a variant named `{}` or annotate one with #[launchpad({})]",
                method, conventional_name, method
            ));
        }
    }

    if !missing.is_empty() {
        return Err(Error::new_spanned(
            enum_name,
            format!(
                "#[derive(LaunchpadStates)] is missing mappings for:\n{}\n\n\
                 See bevy_launchpad documentation for examples.",
                missing.join("\n")
            ),
        ));
    }

    // --- Generate impl ---
    let get = |method: &str| -> TokenStream2 {
        resolved.iter()
            .find(|(m, _)| *m == method)
            .map(|(_, ts)| ts.clone())
            .unwrap()
    };

    let m_booting = get("booting");
    let m_loading = get("loading");
    let m_splash  = get("splash");
    let m_menu    = get("menu");
    let m_playing = get("playing");
    let m_paused  = get("paused");

    Ok(quote! {
        impl ::bevy_launchpad::core::states::mapping::LaunchpadStates
            for #enum_name
        {
            #[inline(always)]
            fn booting()  -> Self { #m_booting }
            #[inline(always)]
            fn loading()  -> Self { #m_loading }
            #[inline(always)]
            fn splash()   -> Self { #m_splash  }
            #[inline(always)]
            fn menu()     -> Self { #m_menu    }
            #[inline(always)]
            fn playing()  -> Self { #m_playing }
            #[inline(always)]
            fn paused()   -> Self { #m_paused  }
        }
    })
}

/// Returns the conventional variant name for a given method name.
/// Used in error messages to tell the developer exactly what to name their variant.
/// Returns the conventional variant name for a given method name.
/// Used in error messages to tell the developer exactly what to name their variant.
fn convention_variant_name(method: &str) -> String {
    match method {
        "booting" => "Booting".to_string(),
        "loading" => "Loading".to_string(),
        "splash"  => "Splash".to_string(),
        "menu"    => "Menu".to_string(),
        "playing" => "Playing".to_string(),
        "paused"  => "Paused".to_string(),
        _         => method.to_string(),
    }
}
