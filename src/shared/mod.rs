pub mod collision;
pub mod components;
pub mod physics;

use bevy::prelude::*;

/// Shared systems used across multiple mini-games.
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((physics::PhysicsPlugin, collision::CollisionPlugin));
    }
}
