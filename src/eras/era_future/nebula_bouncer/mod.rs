use avian2d::prelude::*;
use bevy::prelude::*;

pub mod components;
pub mod procgen;
pub mod resources;
pub mod systems;
pub mod test_api;
pub mod topography;

use components::*;
use procgen::*;
use resources::*;
use systems::*;
use topography::*;

use crate::core::states::PlayingState;

/// Scaffold plugin for Nebula Bouncer in the Future era.
pub struct NebulaBouncerPlugin;

impl Plugin for NebulaBouncerPlugin {
    fn build(&self, app: &mut App) {
        // Add Avian Physics.
        // Note: This adds physics globally. If we need to isolate it, we might need
        // to pause physics when not in NebulaBouncer state.
        app.add_plugins(PhysicsPlugins::default());
        // Optional: Debug plugin for development (off by default to avoid gameplay HUD noise).
        // Set NB_PHYSICS_DEBUG=1 to enable collider rendering.
        #[cfg(debug_assertions)]
        if std::env::var_os("NB_PHYSICS_DEBUG").is_some() {
            app.add_plugins(PhysicsDebugPlugin::default());
        }

        // Register components for reflection
        app.register_type::<KineticOrb>()
            .register_type::<OrbElement>()
            .register_type::<OrbModifier>()
            .register_type::<EnemyStatusEffects>()
            .register_type::<PlayerShip>()
            .register_type::<Enemy>()
            .register_type::<Wall>()
            .register_type::<HexExtrusion>()
            .register_type::<NebulaGameplayCamera>()
            .register_type::<PlayerVisualRoot>()
            .register_type::<TopographyHex>();

        // Initialize resources
        app.insert_resource(KineticOrbPool::new(KineticOrbPool::DEFAULT_CAPACITY))
            .insert_resource(Gravity(Vec2::ZERO)) // ensure 2D gravity is zero
            .insert_resource(ProcgenValidatorTelemetry::default())
            .insert_resource(ChunkLibrary::default())
            .insert_resource(ProcGenState::default())
            .insert_resource(load_asset_manifest())
            .insert_resource(load_chunk_assignment_profiles())
            .insert_resource(load_sprite_orientation_config())
            .init_resource::<ActiveLoadout>()
            .init_resource::<OrbSynergyMatrix>()
            .init_resource::<CameraFeedbackSettings>()
            .init_resource::<HitStop>()
            .init_resource::<NebulaRunStats>();

        app.register_type::<ChunkLibrary>()
            .register_type::<ProcGenState>();

        // Add systems
        app.add_systems(OnEnter(PlayingState::NebulaBouncer), setup_nebula_bouncer);

        app.add_systems(
            Update,
            (
                attach_screen_shake_to_cameras,
                update_active_loadout_hotkeys,
                toggle_camera_shake,
                cycle_feedback_profile,
                feedback_telemetry_hotkey,
                debug_telemetry_hotkey,
                toggle_debug_asset_overlay,
                update_debug_asset_overlay_text,
                handle_player_extrusion_collisions,
                handle_orb_collisions,
                update_enemy_status_effects,
                systems::update_level_scrolling,
                player_movement,
                orient_player_to_cursor,
                player_shoot,
                orient_orbs_to_velocity,
                update_trails,
                update_transient_vfx,
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
