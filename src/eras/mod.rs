pub mod era_80s;
pub mod era_90s;
pub mod shared;

use bevy::prelude::*;

/// Plugin that registers all era plugins.
pub struct ErasPlugin;

impl Plugin for ErasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            era_80s::Era80sPlugin,
            era_90s::Era90sPlugin,
        ));
    }
}
