# Settings Contract (NB-A1-005)

This document defines the canonical UX and data contract for the global Settings system in **RetroGameGame**.

## UX Flow

The Settings system is accessible from two main contexts: the **Hub** and **In-Game**.

### 1. Hub Access
- **Entry**: Available from the main carousel/hub menu (e.g., `GameState::Menu`).
- **Interaction**: Navigation via `WASD`/`Arrows`, Selection via `Confirm` (Enter/Space), Back via `Cancel` (Esc).
- **Exit**: Returning to the hub menu.

### 2. In-Game Access
- **Entry**: Pressing `Esc` (input `pause`/`cancel`) during active gameplay.
- **State Change**: Game state transitions to `GameState::Settings` (or a sub-state that pauses game logic).
- **Exit**: 
    - **Resume**: Pressing `Esc` again or selecting "Back/Apply" returns to `GameState::Playing`.
    - **Quit**: Selecting "Quit" triggers a confirmation overlay before returning to the Hub/Title.

### 3. Apply vs. Cancel
- **Apply**: Selecting "Apply" or "Back" (if changes are auto-saved) persists settings to disk immediately.
- **Cancel**: (Optional/Era-dependent) Reverting unapplied changes. For simplicity in the initial implementation, settings are applied on change or on exiting the menu.

---

## Data Schema (Config Contract)

Settings are persisted in **RON** (Rusty Object Notation) format.

| Field | Type | Range / Options | Default | Description |
|-------|------|-----------------|---------|-------------|
| `resolution` | `(u32, u32)` | Standard 16:9/4:3 pairs | `(1280, 720)` | Window dimensions in pixels. |
| `display_mode` | `enum` | `Windowed`, `Borderless`, `Fullscreen` | `Windowed` | OS-level windowing mode. |
| `music_volume` | `f32` | `0.0` to `1.0` | `0.7` | Linear volume gain for music tracks. |
| `quit_behavior` | `enum` | `ToHub`, `ToDesktop` | `ToHub` | Default action for the "Quit" button in-game. |

### Concrete Schema Example (RON)
```ron
(
    resolution: (1280, 720),
    display_mode: Windowed,
    music_volume: 0.7,
    quit_behavior: ToHub,
)
```

---

## Persistence Policy

- **File Path**: `settings.ron` in the application data directory (e.g., `%APPDATA%/RetroGameGame/` on Windows).
- **Load Timing**: 
    - At application boot (`GameState::Boot`).
    - Fallback to constants if the file is missing or contains invalid RON.
- **Save Timing**: 
    - When the user exits the Settings menu via "Apply" or "Back".
    - On application shutdown.
- **Error Handling**: 
    - If Corrupt: Rename to `settings.ron.bak` and generate a new default file.
    - If Read-Only: Log error and maintain settings in memory for the session.

---

## Quit Confirmation UX

To prevent accidental data loss during a run:
- **Copy**: "Are you sure you want to quit? Unsaved progress in this run will be lost."
- **Options**: `[ QUIT ]` (Magenta highlight) / `[ CANCEL ]` (Standard neon highlight).
