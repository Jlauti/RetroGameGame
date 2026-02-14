use bevy::prelude::*;

use super::components::Velocity;

/// Simple 2D physics plugin — applies velocity to transform.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_velocity);
    }
}

/// Moves entities based on their velocity.
fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in &mut query {
        transform.translation.x += vel.x * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();
    }
}
