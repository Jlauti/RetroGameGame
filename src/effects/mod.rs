pub mod crt;
pub mod transitions;

use bevy::prelude::*;

/// Visual effects plugin: CRT shader, screen transitions, era-specific effects.
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            crt::CrtPlugin,
            transitions::TransitionsPlugin,
        ));
    }
}
