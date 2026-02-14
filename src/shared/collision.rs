use bevy::prelude::*;

use super::components::BoxCollider;

/// Simple AABB collision detection plugin.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, _app: &mut App) {
        // Collision checks will be run by the individual mini-games
        // that need them, using the `check_aabb_overlap` function.
    }
}

/// Check if two AABB colliders overlap.
pub fn check_aabb_overlap(
    pos_a: Vec3,
    col_a: &BoxCollider,
    pos_b: Vec3,
    col_b: &BoxCollider,
) -> bool {
    let dx = (pos_a.x - pos_b.x).abs();
    let dy = (pos_a.y - pos_b.y).abs();
    let overlap_x = col_a.half_width + col_b.half_width;
    let overlap_y = col_a.half_height + col_b.half_height;
    dx < overlap_x && dy < overlap_y
}
