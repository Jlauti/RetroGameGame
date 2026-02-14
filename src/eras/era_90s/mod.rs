pub mod depths_of_doom;
pub mod ice_blitz;
pub mod worm_wars;

use bevy::prelude::*;

/// Plugin for all 1990s era mini-games.
pub struct Era90sPlugin;

impl Plugin for Era90sPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            worm_wars::WormWarsPlugin,
            ice_blitz::IceBlitzPlugin,
            depths_of_doom::DepthsOfDoomPlugin,
        ));
    }
}
