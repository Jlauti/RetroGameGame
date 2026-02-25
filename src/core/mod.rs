pub mod input;
pub mod progression;
pub mod settings;
pub mod states;

use bevy::prelude::*;

/// Core plugin: game states, progression/save system, input abstraction.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            states::StatesPlugin,
            progression::ProgressionPlugin,
            input::InputPlugin,
            settings::SettingsPlugin,
        ));
    }
}
