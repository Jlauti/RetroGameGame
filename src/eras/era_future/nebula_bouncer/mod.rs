use avian2d::prelude::*;
use bevy::prelude::*;

pub mod components;
pub mod procgen;
pub mod resources;
pub mod systems;

use components::*;
use procgen::*;
use resources::*;
use systems::*;

use crate::core::states::PlayingState;

/// Scaffold plugin for Nebula Bouncer in the Future era.
pub struct NebulaBouncerPlugin;

impl Plugin for NebulaBouncerPlugin {
    fn build(&self, app: &mut App) {
        // Add Avian Physics.
        // Note: This adds physics globally. If we need to isolate it, we might need
        // to pause physics when not in NebulaBouncer state.
        app.add_plugins(PhysicsPlugins::default());
        // Optional: Debug plugin for development
        #[cfg(debug_assertions)]
        app.add_plugins(PhysicsDebugPlugin::default());

        // Register components for reflection
        app.register_type::<KineticOrb>()
            .register_type::<PlayerShip>()
            .register_type::<Enemy>()
            .register_type::<Wall>();

        // Initialize resources
        app.insert_resource(KineticOrbPool::new(KineticOrbPool::DEFAULT_CAPACITY))
            .insert_resource(Gravity(Vec2::ZERO)) // ensure 2D gravity is zero
            .insert_resource(ProcgenValidatorTelemetry::default())
            .insert_resource(ChunkLibrary::default())
            .insert_resource(ProcGenState::default())
            .init_resource::<HitStop>();

        app.register_type::<ChunkLibrary>()
            .register_type::<ProcGenState>();

        // Add systems
        app.add_systems(
            OnEnter(PlayingState::NebulaBouncer),
            (setup_nebula_bouncer, spawn_orb_pool),
        );

        app.add_systems(
            Update,
            (
                attach_screen_shake_to_cameras,
                debug_telemetry_hotkey,
                handle_orb_collisions,
                systems::update_level_scrolling,
                player_movement,
                orient_player_to_cursor,
                player_shoot,
                orient_orbs_to_velocity,
                update_trails,
                apply_shake,
                update_hit_stop,
            )
                .run_if(in_state(PlayingState::NebulaBouncer)),
        );

        app.add_systems(
            OnExit(PlayingState::NebulaBouncer),
            (cleanup_orb_pool, cleanup_camera_shake),
        );

        // Add pre-solver hook if needed
        // app.add_systems(PostProcessCollisions, collision_hook_system.run_if(in_state(PlayingState::NebulaBouncer)));
    }
}
