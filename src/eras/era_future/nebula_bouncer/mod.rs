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
        app.add_plugins(PhysicsPlugins::default());

        #[cfg(debug_assertions)]
        if std::env::var_os("NB_PHYSICS_DEBUG").is_some() {
            app.add_plugins(PhysicsDebugPlugin::default());
        }

        // Register types for reflection
        app.register_type::<KineticOrb>()
            .register_type::<OrbElement>()
            .register_type::<OrbModifier>()
            .register_type::<EnemyStatusEffects>()
            .register_type::<PlayerShip>()
            .register_type::<Enemy>()
            .register_type::<EnemyRole>()
            .register_type::<EnemyState>()
            .register_type::<EnemyAI>()
            .register_type::<HostileProjectile>()
            .register_type::<HostileFireSource>()
            .register_type::<Wall>()
            .register_type::<HexExtrusion>()
            .register_type::<NebulaGameplayCamera>()
            .register_type::<PlayerVisualRoot>()
            .register_type::<PlayerSurfaceRole>()
            .register_type::<SurfaceArchetype>()
            .register_type::<SurfaceRole>()
            .register_type::<SurfaceNormal>()
            .register_type::<TerrainContourSample>()
            .register_type::<GroundSeam>()
            .register_type::<CrashVectorShard>()
            .register_type::<TopographyHex>()
            .register_type::<PendingCrashResult>()
            .register_type::<ProjectileEventKind>()
            .register_type::<NebulaRuntimeTelemetry>()
            .register_type::<NebulaValidationCommand>()
            .register_type::<CombatTokenPool>()
            .register_type::<HostileFireConfig>();

        // Initialize resources
        app.insert_resource(KineticOrbPool::new(KineticOrbPool::DEFAULT_CAPACITY))
            .insert_resource(Gravity(Vec2::ZERO))
            .insert_resource(PendingCrashResult::default())
            .insert_resource(ProcgenValidatorTelemetry::default())
            .insert_resource(NebulaRuntimeTelemetry::default())
            .insert_resource(NebulaValidationCommand::default())
            .insert_resource(ChunkLibrary::default())
            .insert_resource(ProcGenState::default())
            .insert_resource(load_asset_manifest())
            .insert_resource(load_chunk_assignment_profiles())
            .insert_resource(load_sprite_orientation_config())
            .init_resource::<ActiveLoadout>()
            .init_resource::<OrbSynergyMatrix>()
            .init_resource::<CameraFeedbackSettings>()
            .init_resource::<HitStop>()
            .init_resource::<NebulaRunStats>()
            .init_resource::<CombatTokenPool>()
            .init_resource::<HostileFireConfig>();

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
                debug_telemetry_hotkey,
                toggle_debug_asset_overlay,
                update_debug_asset_overlay_text,
                handle_player_extrusion_collisions,
                advance_player_crash_sequence,
                handle_orb_collisions,
                update_enemy_status_effects,
                enemy_ai_system,
                combat_token_system,
            )
                .run_if(in_state(PlayingState::NebulaBouncer)),
        );

        app.add_systems(
            Update,
            (
                cycle_feedback_profile,
                feedback_telemetry_hotkey,
                systems::update_level_scrolling,
                apply_validation_commands,
                player_movement,
                apply_visual_terrain_follow,
                orient_player_to_cursor,
                player_shoot,
                orient_orbs_to_velocity,
                update_trails,
                update_transient_vfx,
                apply_shake,
                update_hit_stop,
                enemy_movement_system,
                cull_behind_player_enemies,
                enemy_fire_system,
                handle_hostile_projectile_collisions,
                check_player_hostile_death,
                update_horizon_backdrop,
            )
                .run_if(in_state(PlayingState::NebulaBouncer)),
        );

        app.add_systems(Update, finalize_nebula_despawn);
        app.add_systems(
            OnExit(PlayingState::NebulaBouncer),
            (cleanup_orb_pool, cleanup_camera_shake),
        );
    }
}
