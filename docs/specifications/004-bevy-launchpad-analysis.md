# bevy-launchpad — Project Analysis & Corrections

> Based on all source files shared in the session.
> Rust 1.93 · Bevy 0.18

---

## Summary

| Category | Count | Status |
|---|---|---|
| 🔴 Critical bugs | 5 | **3 Remaining** (2 Fixed/Invalid) |
| 🟠 Architecture violations | 6 | **2 Remaining** (4 Fixed/Invalid) |
| 🟡 Missing files | 5 | **0 Remaining** (All Fixed/Invalid) |
| 🔵 Minor / style issues | 6 | **0 Remaining** (All Fixed/Invalid) |

---

## 🔴 Critical Bugs

---

### BUG-01 — `Cargo.toml`: 6 keywords (crates.io limit is 5)

**Status:** ✅ **Fixed / False Positive**
Current `Cargo.toml` has 4 keywords.

---

### BUG-02 — `LocalePlugin` and `LaunchpadUiPlugin` derive `Resource`

**Status:** ⚠️ **Partially Valid**
`LaunchpadUiPlugin` still incorrectly derives `Resource`. `LocalePlugin` appears correct.

**File:** `src/ui/plugin.rs`

```rust
// ❌ Current
#[derive(Resource)]
pub struct LaunchpadUiPlugin<S: LaunchpadStates> { ... }
```

```rust
// ✅ Fix
// Remove #[derive(Resource)]
pub struct LaunchpadUiPlugin<S: LaunchpadStates> { ... }
```

---

### BUG-03 — `SplashTimer` resource may not exist when `update_splash_renderer` runs

**Status:** 🔴 **Valid**
System panics if `SplashTimer` is missing.

**File:** `src/ui/splash/renderer.rs`

```rust
// ❌ Current — panics when SplashTimer resource is absent
pub fn update_splash_renderer(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,       // hard ResMut — panics if missing
    splash_query: Query<Entity, With<SplashScreen>>,
) { ... }
```

```rust
// ✅ Fix — use Option<ResMut<...>> so it is safe when absent
pub fn update_splash_renderer(
    mut commands: Commands,
    time:         Res<Time>,
    timer:        Option<ResMut<SplashTimer>>,   // safe — None when not inserted
    splash_query: Query<Entity, With<SplashScreen>>,
) {
    let Some(mut timer) = timer else { return };
    // ... rest of logic
}
```

---

### BUG-04 — No automatic state transitions wired up

**Status:** 🔴 **Valid**
`splash_to_menu` transition is missing from `src/core/plugin.rs`.

---

### BUG-05 — `init_state::<S>()` is never called

**Status:** 🔴 **Valid**
`LaunchpadCorePlugin` does not initialize the state `S`.

**File:** `src/core/plugin.rs`

```rust
// ✅ Fix — add at the top of Plugin::build
app.init_state::<S>();
```

---

## 🟠 Architecture Violations

---

### ARCH-01 — `LaunchpadUiPlugin` is not generic over `S: LaunchpadStates`

**Status:** ✅ **Fixed**
Codebase already uses `LaunchpadUiPlugin<S>`.

---

### ARCH-02 — `UiChildSpawner` type alias duplicated in 4 files

**Status:** ✅ **Fixed**
`src/ui/common.rs` exists and is used by widgets.

---

### ARCH-03 — `LaunchpadCorePlugin` uses `FreelyMutableState` instead of `LaunchpadStates`

**Status:** ✅ **Fixed**
Codebase uses `S: LaunchpadStates`.

---

### ARCH-04 — `splash_to_menu` transition lives in `core` but depends on UI state

**Status:** 🔴 **Valid**
Need to implement `SplashDone` resource signaling.

---

### ARCH-05 — `core/states/mapping.rs` trait not used as the primary bound anywhere

**Status:** ✅ **Fixed**
Traits are correctly used.

---

### ARCH-06 — `AssetTracker` is never incremented

**Status:** 🔴 **Valid**
Need temporary fix for `is_ready()` to allow transition to Splash.

---

## 🟡 Missing Files

**Status:** ✅ **All Fixed / False Positives**

* `MISS-01`: `src/core/states/app_state.rs` — **Exists**.
* `MISS-02`: `src/ui/common.rs` — **Exists**.
* `MISS-03`: `bevy_launchpad_derive` — **Exists**.
* `MISS-04`: `src/core/assets/` — **Exists**.
* `MISS-05`: `src/assets/` — **Exists**.

---

## 🔵 Minor / Style Issues

**Status:** ✅ **All Fixed / False Positives**

* `MINOR-01`: `SingleInstance` re-export — **Fixed**.
* `MINOR-02`: `AppMetadata.version` — **Low Priority / Ignored for now**.
* `MINOR-03`, `MINOR-04`: Unused code — **Ignored**.
* `MINOR-05`: `spawn_slider` import — **Fixed**.
* `MINOR-06`: `minimal_2d` example — **Fixed** (uses built-in `AppState`).

---

## Recommended Action Plan

1. **Fix Critical Bugs:**
    * [ ] BUG-02: Remove `Resource` derive.
    * [ ] BUG-03: Fix `SplashTimer` usage.
    * [ ] BUG-05: Add `init_state::<S>()`.

2. **Implement Transitions & Architecture:**
    * [ ] ARCH-04 / BUG-04: Implement `SplashDone` and `splash_to_menu`.
    * [ ] ARCH-06: Implement `AssetTracker::is_ready` placeholder.

3. **Verify:**
    * [ ] Run `cargo check`.
    * [ ] Run `cargo run --example minimal_2d`.
