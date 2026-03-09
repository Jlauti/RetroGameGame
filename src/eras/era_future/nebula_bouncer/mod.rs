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
use crate::shared::components::Health;

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
            .register_type::<TerrainMotif>()
            .register_type::<PlacementZone>()
            .register_type::<DensityCadence>()
            .register_type::<SurfaceDurability>()
            .register_type::<BreakableHazardFamily>()
            .register_type::<BreakableRewardRole>()
            .register_type::<SurfaceRole>()
            .register_type::<SurfaceNormal>()
            .register_type::<TerrainContourSample>()
            .register_type::<BreakableHazard>()
            .register_type::<HealthDrop>()
            .register_type::<GroundSeam>()
            .register_type::<CrashVectorShard>()
            .register_type::<TopographyHex>()
            .register_type::<PendingCrashResult>()
            .register_type::<ProjectileEventKind>()
            .register_type::<NebulaRuntimeTelemetry>()
            .register_type::<ChunkRuntimeSnapshot>()
            .register_type::<NebulaProcgenValidationState>()
            .register_type::<NebulaValidationCommand>()
            .register_type::<CombatTokenPool>()
            .register_type::<HostileFireConfig>();
        app.register_type::<Health>();

        // Initialize resources
        app.insert_resource(KineticOrbPool::new(KineticOrbPool::DEFAULT_CAPACITY))
            .insert_resource(Gravity(Vec2::ZERO))
            .insert_resource(PendingCrashResult::default())
            .insert_resource(ProcgenValidatorTelemetry::default())
            .insert_resource(NebulaRuntimeTelemetry::default())
            .insert_resource(NebulaProcgenValidationState::default())
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
            attach_screen_shake_to_cameras.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_active_loadout_hotkeys.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            toggle_camera_shake.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            debug_telemetry_hotkey.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            toggle_debug_asset_overlay.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_debug_asset_overlay_text.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            handle_player_extrusion_collisions.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            advance_player_crash_sequence.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            handle_orb_collisions.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_enemy_status_effects.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            enemy_ai_system.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            combat_token_system.run_if(in_state(PlayingState::NebulaBouncer)),
        );

        app.add_systems(
            Update,
            cycle_feedback_profile.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            feedback_telemetry_hotkey.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            systems::update_level_scrolling.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            apply_validation_commands.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            player_movement.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_health_drops
                .before(handle_orb_collisions)
                .run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_player_health_hud.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            apply_visual_terrain_follow.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            orient_player_to_cursor.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            player_shoot.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            orient_orbs_to_velocity.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_trails.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_transient_vfx.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            apply_shake.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_hit_stop.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            enemy_movement_system.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            orient_enemies_for_attack.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            cull_behind_player_enemies.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            enemy_fire_system.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            handle_hostile_projectile_collisions.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            check_player_hostile_death.run_if(in_state(PlayingState::NebulaBouncer)),
        );
        app.add_systems(
            Update,
            update_horizon_backdrop.run_if(in_state(PlayingState::NebulaBouncer)),
        );

        app.add_systems(Update, finalize_nebula_despawn);
        app.add_systems(
            OnExit(PlayingState::NebulaBouncer),
            (cleanup_orb_pool, cleanup_camera_shake),
        );
    }
}
