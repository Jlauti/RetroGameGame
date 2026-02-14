pub mod cosmic_captain;
pub mod star_goose;
pub mod tunnel_miner;

use bevy::prelude::*;

/// Plugin for all 1980s era mini-games.
pub struct Era80sPlugin;

impl Plugin for Era80sPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            tunnel_miner::TunnelMinerPlugin,
            cosmic_captain::CosmicCaptainPlugin,
            star_goose::StarGoosePlugin,
        ));
    }
}
