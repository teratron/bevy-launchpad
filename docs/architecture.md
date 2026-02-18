# Architecture

## Dependency graph

```plaintext
utils  ◄──────────────────────────────┐
  ▲                                   │
core  (boot, states, loading, splash) │
  ▲                                   │
ui    (theme, widgets, menu, modal)   │
  ▲                                   │
locale                                │
  ▲                                   │
lib.rs  (LaunchpadPlugin) ────────────┘
```

## Automatic transition flow

```plaintext
Booting ──(boot done)──► Loading ──(assets ready)──► Splash ──(screens done)──► Menu
                                                          │
                                        (skip_all=true or no screens)
                                                          ▼
                                                        Menu
```

## File structure

```plaintext

```
