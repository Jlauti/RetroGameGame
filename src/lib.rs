pub mod core;
pub mod effects;
pub mod eras;
pub mod shared;
pub mod ui;

use bevy::prelude::*;

/// Root plugin that composes all sub-plugins together.
pub struct RetroGameGamePlugin;

impl Plugin for RetroGameGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            core::CorePlugin,
            ui::UiPlugin,
            effects::EffectsPlugin,
            shared::SharedPlugin,
            eras::ErasPlugin,
        ));
    }
}
