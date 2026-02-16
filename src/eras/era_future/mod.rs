pub mod nebula_bouncer;

use bevy::prelude::*;

/// Plugin for the Future era mini-games.
pub struct EraFuturePlugin;

impl Plugin for EraFuturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((nebula_bouncer::NebulaBouncerPlugin,));
    }
}
