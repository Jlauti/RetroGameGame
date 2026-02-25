# Task Report: NB-A5-006 — Music Volume Runtime Binding

- **Agent**: agent5 (Veikko Fiilis)
- **Ticket**: NB-A5-006
- **Date**: 2026-02-25
- **Status**: COMPLETE

## Summary

Integrated `GameSettings.music_volume` into the hub music controller (`MusicPlugin`). Volume changes now apply live to the `AudioSink` every frame, and persist/restore across app restarts via the settings JSON file.

## Changes Made

### `src/ui/music.rs`
- Removed hardcoded `HUB_MUSIC_VOLUME` constant.
- Added `get_volume_for_setting(f32) -> Volume` — linear conversion policy: `setting * 1.25` (preserves original mix balance at 100%).
- Modified `setup_hub_music` to accept `Res<GameSettings>` and pass `music_volume` to spawner.
- Modified `control_hub_music` to accept `Res<GameSettings>` and apply volume from settings every frame via `sink.set_volume(get_volume_for_setting(settings.music_volume))`.
- Modified `spawn_hub_music_entity` to accept a `volume_setting: f32` parameter.
- All fallback/respawn paths now pass `settings.music_volume`.

### `src/core/settings.rs` (companion fix)
- Rewrote for Bevy 0.18 compatibility:
  - Replaced removed `EventReader`/`add_event` API with `Res::is_changed()` change detection.
  - Replaced `ron` (not in Cargo.toml) with `serde_json`.
  - Fixed `WindowMode::Fullscreen` to take `(MonitorSelection, VideoModeSelection)`.
  - Fixed `single_mut()` return type handling.

### `src/main.rs` (companion fix)
- Fixed `WindowResolution` construction to use `(u32, u32)` (Bevy 0.18 requirement).

## Conversion Policy

| UI Setting | Linear Volume | Notes |
|------------|---------------|-------|
| 0.0        | 0.0           | Muted |
| 0.5        | 0.625         | Half  |
| 0.7 (default) | 0.875     | Default |
| 1.0        | 1.25          | Max (matches original hardcoded value) |

## Edge Cases

- **Missing audio device**: Existing `sink_query.get_mut()` fallback preserved — logs a warning, does not crash.
- **Settings file missing**: `GameSettings::default()` used (music_volume = 0.7).
- **Settings file corrupt**: Warning logged, defaults used.

## Verification

- `cargo check` passes with zero errors.
- All pre-existing warnings are unrelated to this change.
