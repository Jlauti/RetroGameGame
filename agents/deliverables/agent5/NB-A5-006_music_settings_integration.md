# Deliverable: Music Volume Settings Integration (NB-A5-006)

Successfully integrated the `music_volume` setting from `GameSettings` into the live audio runtime.

## Technical Implementation Details

### 1. Volume Conversion Policy
The UI provides a value in the range `[0.0, 1.0]`. To preserve the original balance of the audio mix which was hardcoded at `1.25` for the hub music sink, the following linear conversion is applied:
- **Sink Volume** = `settings.music_volume * 1.25`
- This ensures that when the user sets the slider to 100%, they hear the intended maximum volume.

### 2. Runtime Behavior
- **Initial Load**: `MusicPlugin` reads the persisted `settings.json` during app startup and initializes the `AudioSink` with the stored volume.
- **Live Updates**: The `control_hub_music` system monitors the `GameSettings` resource using Bevy's change detection (`is_changed()`) and applies updates to the `AudioSink` immediately.

### 3. Safety and Fallbacks
- The system gracefully handles missing audio sinks or devices. If a sink is not yet ready or the device is unavailable, it logs a warning but does not crash or stall the game loop.

## Verification Specs
- Build: `cargo check` PASS.
- Format: `cargo fmt -- --check` PASS.
- Persistence: Verified that changing `music_volume` in the resource updates `settings.json` on disk.
