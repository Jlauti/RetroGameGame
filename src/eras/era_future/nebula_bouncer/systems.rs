use crate::eras::era_future::nebula_bouncer::components::*;
use crate::eras::era_future::nebula_bouncer::procgen::*;
use crate::eras::era_future::nebula_bouncer::resources::{
    ActiveLoadout, CameraFeedbackSettings, ChunkAssignmentProfiles, EnemyArchetype, HitStop,
    KineticOrbPool, NebulaAssetManifest, NebulaMaterials, OrbSpawnStats, OrbSynergyMatrix,
    ProcgenValidatorTelemetry, SpriteOrientationConfig, TerrainTheme, compute_hit_stop_duration,
    feedback_tuning, next_shake_intensity, resolve_orb_spawn_stats,
};
use crate::eras::era_future::nebula_bouncer::topography::spawn_chunk_topography;
use crate::shared::components::Health;
use avian2d::prelude::*;
use bevy::camera::ScalingMode;
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use std::path::PathBuf;
// use rand::prelude::*; // Use explicit random calls

const TELEMETRY_COOLDOWN_SECS: f32 = 0.25;
const BASE_ORB_DAMAGE: f32 = 10.0;
const BASE_ORB_SPEED: f32 = 500.0;
const BASE_ORB_BOUNCES: u32 = 3;
const BASE_ORB_RADIUS: f32 = 5.0;
const BASE_ORB_TRAIL_WIDTH: f32 = 4.0;
const GROUND_CHUNK_WIDTH: f32 = 960.0;
const VOID_DOT_TICK_INTERVAL_SECS: f32 = 0.5;
const PREFLIGHT_SUMMARY_REL_PATH: &str =
    "agents/deliverables/codex_worker2/NB-CX-006_preflight_summary.txt";
const FEEDBACK_TELEMETRY_COOLDOWN_SECS: f32 = 0.35;
const MODEL_UNIT_TO_WORLD: f32 = 120.0;
const PLAYER_MODEL_VISUAL_LIFT: f32 = 42.0;
const PLAYER_MODEL_FACING_FIX_RADIANS: f32 = 0.0;
const SCOUT_SPRITE_SIZE: Vec2 = Vec2::new(62.0, 62.0);
const HEAVY_SPRITE_SIZE: Vec2 = Vec2::new(78.0, 78.0);
const INTERCEPTOR_SPRITE_SIZE: Vec2 = Vec2::new(70.0, 70.0);
const BULWARK_SPRITE_SIZE: Vec2 = Vec2::new(86.0, 86.0);
const FLOOR_TILE_SIZE: Vec2 = Vec2::new(128.0, 128.0);
const WALL_VISUAL_THICKNESS: f32 = 36.0;
const WALL_SEGMENT_MAX_LENGTH: f32 = 96.0;
const BASE_ENEMY_COLLIDER_RADIUS: f32 = 15.0;
const ORB_VISUAL_SCALE: f32 = 5.0;
const PLAYER_MUZZLE_FORWARD_OFFSET: f32 = 22.0;
const TRANSIENT_VFX_BASE_LIFETIME_SECS: f32 = 0.24;
const TRANSIENT_VFX_BASE_SIZE: f32 = 70.0;

fn enemy_sprite_size(archetype: EnemyArchetype) -> Vec2 {
    match archetype {
        EnemyArchetype::Scout => SCOUT_SPRITE_SIZE,
        EnemyArchetype::Interceptor => INTERCEPTOR_SPRITE_SIZE,
        EnemyArchetype::Heavy => HEAVY_SPRITE_SIZE,
        EnemyArchetype::Bulwark => BULWARK_SPRITE_SIZE,
    }
}

fn enemy_base_hp(archetype: EnemyArchetype) -> i32 {
    match archetype {
        EnemyArchetype::Scout => 42,
        EnemyArchetype::Interceptor => 56,
        EnemyArchetype::Heavy => 72,
        EnemyArchetype::Bulwark => 95,
    }
}

fn enemy_tint(archetype: EnemyArchetype, terrain_theme: TerrainTheme) -> Color {
    match terrain_theme {
        TerrainTheme::Standard => match archetype {
            EnemyArchetype::Scout => Color::srgb(0.92, 0.98, 1.0),
            EnemyArchetype::Interceptor => Color::srgb(1.0, 0.95, 0.84),
            EnemyArchetype::Heavy => Color::srgb(1.0, 0.88, 0.82),
            EnemyArchetype::Bulwark => Color::srgb(0.94, 0.92, 1.0),
        },
        TerrainTheme::Cold => match archetype {
            EnemyArchetype::Scout => Color::srgb(0.82, 0.95, 1.0),
            EnemyArchetype::Interceptor => Color::srgb(0.90, 0.96, 1.0),
            EnemyArchetype::Heavy => Color::srgb(0.78, 0.91, 1.0),
            EnemyArchetype::Bulwark => Color::srgb(0.75, 0.86, 1.0),
        },
        TerrainTheme::Hazard => match archetype {
            EnemyArchetype::Scout => Color::srgb(1.0, 0.88, 0.82),
            EnemyArchetype::Interceptor => Color::srgb(1.0, 0.84, 0.76),
            EnemyArchetype::Heavy => Color::srgb(1.0, 0.76, 0.70),
            EnemyArchetype::Bulwark => Color::srgb(1.0, 0.74, 0.66),
        },
    }
}

fn fold_seed(seed: u64, value: u64) -> u64 {
    seed.wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_left(7) ^ value
}

fn assignment_seed(chunk: &ChunkSchema, spawn: &SpawnDef, chunk_y: f32, spawn_index: usize) -> u64 {
    let pacing_tag = match chunk.pacing {
        ChunkPacing::Open => 1u64,
        ChunkPacing::Transition => 2u64,
        ChunkPacing::Dense => 3u64,
    };
    let mut seed = fold_seed(0xCBF2_9CE4_8422_2325, spawn_index as u64);
    seed = fold_seed(seed, chunk.height.to_bits() as u64);
    seed = fold_seed(seed, chunk_y.to_bits() as u64);
    seed = fold_seed(seed, spawn.position.x.to_bits() as u64);
    seed = fold_seed(seed, spawn.position.y.to_bits() as u64);
    seed = fold_seed(seed, pacing_tag);
    for b in chunk.name.as_bytes() {
        seed = fold_seed(seed, *b as u64);
    }
    seed
}

#[derive(Component)]
pub struct NebulaDebugOverlayRoot;

#[derive(Component)]
pub struct NebulaDebugOverlayText;

#[derive(Component)]
pub struct GroundVisual;

#[derive(Component)]
pub struct WallVisual;

#[derive(Component)]
pub struct TransientVfx {
    ttl_secs: f32,
    shrink_per_sec: f32,
}

fn facing_angle(direction: Vec2, forward_offset: f32) -> Option<f32> {
    if direction.length_squared() <= f32::EPSILON {
        None
    } else {
        Some(direction.y.atan2(direction.x) + forward_offset)
    }
}

fn element_trail_color(element: OrbElement) -> Color {
    match element {
        OrbElement::Plasma => Color::srgb(1.0, 0.5, 0.0),
        OrbElement::Cryo => Color::srgb(0.45, 0.85, 1.0),
        OrbElement::Tesla => Color::srgb(0.95, 0.95, 0.35),
        OrbElement::Void => Color::srgb(0.62, 0.45, 1.0),
    }
}

fn apply_enemy_status_effects(status: &mut EnemyStatusEffects, orb: &KineticOrb) {
    if orb.cryo_duration_secs > 0.0 && orb.cryo_slow_factor < 1.0 {
        status.cryo_slow_timer = status.cryo_slow_timer.max(orb.cryo_duration_secs);
        status.cryo_slow_factor = status
            .cryo_slow_factor
            .min(orb.cryo_slow_factor.clamp(0.25, 1.0));
    }

    if orb.void_dot_duration_secs > 0.0 && orb.void_dot_dps > 0.0 {
        status.void_dot_timer = status.void_dot_timer.max(orb.void_dot_duration_secs);
        status.void_dot_dps = status.void_dot_dps.max(orb.void_dot_dps);
        if status.void_dot_tick_timer <= 0.0 {
            status.void_dot_tick_timer = VOID_DOT_TICK_INTERVAL_SECS;
        }
    }
}

fn advance_enemy_status_effects(status: &mut EnemyStatusEffects, health: &mut Health, dt: f32) {
    if dt <= 0.0 {
        return;
    }

    if status.cryo_slow_timer > 0.0 {
        status.cryo_slow_timer = (status.cryo_slow_timer - dt).max(0.0);
        if status.cryo_slow_timer <= 0.0 {
            status.cryo_slow_factor = 1.0;
        }
    }

    if status.void_dot_timer > 0.0 && status.void_dot_dps > 0.0 {
        status.void_dot_timer = (status.void_dot_timer - dt).max(0.0);
        status.void_dot_tick_timer -= dt;

        while status.void_dot_tick_timer <= 0.0 && status.void_dot_timer > 0.0 {
            let tick_damage = (status.void_dot_dps * VOID_DOT_TICK_INTERVAL_SECS)
                .round()
                .max(1.0) as i32;
            health.damage(tick_damage);
            status.void_dot_tick_timer += VOID_DOT_TICK_INTERVAL_SECS;
        }

        if status.void_dot_timer <= 0.0 {
            status.void_dot_dps = 0.0;
            status.void_dot_tick_timer = 0.0;
        }
    }
}

fn preflight_artifact_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(PREFLIGHT_SUMMARY_REL_PATH)
}

fn spawn_transient_vfx(
    commands: &mut Commands,
    asset_server: &AssetServer,
    nebula_mats: &NebulaMaterials,
    materials: &mut Assets<StandardMaterial>,
    texture_path: &str,
    at: Vec3,
    color: Color,
    size: f32,
    ttl_secs: f32,
) {
    let lifetime = ttl_secs.max(0.05);
    commands.spawn((
        Mesh3d(nebula_mats.quad_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            base_color_texture: Some(asset_server.load(texture_path.to_string())),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_xyz(at.x, at.y, depth::PARTICLES).with_scale(Vec3::splat(size.max(4.0))),
        TransientVfx {
            ttl_secs: lifetime,
            shrink_per_sec: (size.max(4.0) * 0.9) / lifetime,
        },
    ));
}

fn spawn_chunk_floor_tiles(
    commands: &mut Commands,
    asset_server: &AssetServer,
    assets: &NebulaAssetManifest,
    nebula_mats: &NebulaMaterials,
    materials: &mut Assets<StandardMaterial>,
    chunk_center_y: f32,
    chunk_height: f32,
    terrain_theme: TerrainTheme,
) {
    let tile_w = FLOOR_TILE_SIZE.x.max(8.0);
    let tile_h = FLOOR_TILE_SIZE.y.max(8.0);
    let cols = (GROUND_CHUNK_WIDTH / tile_w).ceil().max(1.0) as i32;
    let rows = (chunk_height / tile_h).ceil().max(1.0) as i32;
    let start_x = -GROUND_CHUNK_WIDTH * 0.5 + tile_w * 0.5;
    let start_y = chunk_center_y - chunk_height * 0.5 + tile_h * 0.5;

    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * tile_w;
            let y = start_y + row as f32 * tile_h;
            commands.spawn((
                ChunkMember,
                GroundVisual,
                Mesh3d(nebula_mats.quad_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: terrain_theme.floor_tint(),
                    base_color_texture: Some(asset_server.load(assets.ground_tile.clone())),
                    unlit: true,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                Transform::from_xyz(x, y, depth::BACKGROUND)
                    .with_scale(FLOOR_TILE_SIZE.extend(1.0)),
            ));
        }
    }
}

fn spawn_wall_visual_segments(
    commands: &mut Commands,
    asset_server: &AssetServer,
    assets: &NebulaAssetManifest,
    nebula_mats: &NebulaMaterials,
    materials: &mut Assets<StandardMaterial>,
    chunk_center_y: f32,
    wall: &WallDef,
    terrain_theme: TerrainTheme,
) {
    let major_is_x = wall.size.x >= wall.size.y;
    let major_len = if major_is_x { wall.size.x } else { wall.size.y }.max(1.0);
    let segment_target = WALL_SEGMENT_MAX_LENGTH.max(8.0);
    let segment_count = (major_len / segment_target).ceil().max(1.0) as usize;
    let segment_len = major_len / segment_count as f32;
    let wall_center = Vec2::new(wall.position.x, chunk_center_y + wall.position.y);
    let rotation = Mat2::from_angle(wall.rotation);
    let visual_thickness = WALL_VISUAL_THICKNESS.max(1.0);

    for idx in 0..segment_count {
        let offset = -major_len * 0.5 + segment_len * (idx as f32 + 0.5);
        let local_offset = if major_is_x {
            Vec2::new(offset, 0.0)
        } else {
            Vec2::new(0.0, offset)
        };
        let world_pos = wall_center + rotation * local_offset;
        let sprite_size = if major_is_x {
            Vec2::new(segment_len, visual_thickness)
        } else {
            Vec2::new(visual_thickness, segment_len)
        };

        commands.spawn((
            ChunkMember,
            WallVisual,
            Mesh3d(nebula_mats.quad_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: terrain_theme.wall_tint(),
                base_color_texture: Some(asset_server.load(assets.wall_tile.clone())),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(world_pos.x, world_pos.y, depth::WALL)
                .with_rotation(Quat::from_rotation_z(wall.rotation))
                .with_scale(sprite_size.extend(1.0)),
        ));
    }
}

pub fn setup_nebula_bouncer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_manifest: Res<NebulaAssetManifest>,
    chunk_assignment_profiles: Res<ChunkAssignmentProfiles>,
    mut library: ResMut<ChunkLibrary>,
    mut procgen_state: ResMut<ProcGenState>,
    mut validator_telemetry: ResMut<ProcgenValidatorTelemetry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut orb_pool: ResMut<KineticOrbPool>,
    q_stale_camera2d: Query<Entity, With<Camera2d>>,
) {
    info!("Nebula Bouncer scaffold loaded (Avian 2D integrated).");
    // Ensure gravity is zero for top-down physics
    commands.insert_resource(Gravity(Vec2::ZERO));

    // Despawn leftover Camera2d from previous screens (menu / era-select)
    // to prevent 2D sprite pass from conflicting with 3D isometric view.
    for entity in q_stale_camera2d.iter() {
        commands.entity(entity).despawn();
    }

    let quad_mesh = meshes.add(Rectangle::new(1.0, 1.0));

    // Build a true 3D hexagonal pillar mesh using Bevy's Extrusion primitive.
    // Radius=0.5 so scale=hex_size gives the correct footprint.
    // Extrude by a large depth (e.g. 50.0) so they extend deep into the floor.
    let hex_mesh = meshes.add(Extrusion::new(RegularPolygon::new(0.5, 6), 50.0));

    let hex_texture: Handle<Image> =
        asset_server.load(crate::eras::era_future::nebula_bouncer::topography::HEX_OUTLINE_TEXTURE);
    let nebula_mats = NebulaMaterials {
        quad_mesh,
        hex_mesh,
        wall_material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        // NB-A2-010 pass6: Solid-color materials for 3D hex prisms.
        // No texture — the Extrusion side faces show as lit terrain walls.
        hex_material_t0: materials.add(StandardMaterial {
            base_color: Color::srgba(0.02, 0.06, 0.18, 0.90),
            emissive: bevy::color::palettes::css::DARK_CYAN.into(),
            unlit: false,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        hex_material_t1: materials.add(StandardMaterial {
            base_color: Color::srgba(0.10, 0.02, 0.22, 0.90),
            emissive: bevy::color::palettes::css::DARK_VIOLET.into(),
            unlit: false,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        hex_material_t2: materials.add(StandardMaterial {
            base_color: Color::srgba(0.02, 0.18, 0.10, 0.90),
            emissive: bevy::color::palettes::css::DARK_GREEN.into(),
            unlit: false,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        hex_material_t3: materials.add(StandardMaterial {
            base_color: Color::srgba(0.20, 0.02, 0.16, 0.90),
            emissive: bevy::color::palettes::css::DEEP_PINK.into(),
            unlit: false,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        hex_texture,
    };
    commands.insert_resource(NebulaMaterials {
        quad_mesh: nebula_mats.quad_mesh.clone(),
        hex_mesh: nebula_mats.hex_mesh.clone(),
        wall_material: nebula_mats.wall_material.clone(),
        hex_material_t0: nebula_mats.hex_material_t0.clone(),
        hex_material_t1: nebula_mats.hex_material_t1.clone(),
        hex_material_t2: nebula_mats.hex_material_t2.clone(),
        hex_material_t3: nebula_mats.hex_material_t3.clone(),
        hex_texture: nebula_mats.hex_texture.clone(),
    });

    // Spawn Player
    commands
        .spawn((
            PlayerShip,
            RigidBody::Dynamic,
            Collider::circle(15.0),
            LinearVelocity::ZERO,
            AngularVelocity::ZERO,
            Transform::from_xyz(0.0, -200.0, depth::PLAYER),
            CollisionLayers::new(
                GameLayer::Player,
                [GameLayer::Enemy, GameLayer::Wall, GameLayer::Projectile],
            ),
            Restitution::new(0.5),
            Friction::new(0.1),
        ))
        .with_children(|parent| {
            // glTF assets are authored Y-up; rotate into the XY gameplay plane and scale to world units.
            parent.spawn((
                PlayerVisualRoot,
                SceneRoot(asset_server.load(asset_manifest.player_ship.clone())),
                Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_MODEL_VISUAL_LIFT))
                    .with_rotation(
                        // World-space Z(PI) spins facing 180° AFTER X(PI/2) tilts upright.
                        Quat::from_rotation_z(std::f32::consts::PI)
                            * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                    )
                    .with_scale(Vec3::splat(MODEL_UNIT_TO_WORLD)),
            ));
        });

    // Spawn 3D camera and lighting for the glTF models
    // NB-A2-010 pass4: Perspective projection for true depth foreshortening.
    // Camera sits behind and above the ship, looking forward.
    let cam_height = 280.0; // height above the Z=0 gameplay plane
    let cam_behind = -420.0; // behind the player on Y axis
    let look_ahead = 600.0; // how far ahead of player to aim camera

    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0f32.to_radians(),
            near: 1.0,
            far: 10000.0,
            ..default()
        }),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Transform::from_xyz(0.0, cam_behind, cam_height)
            .looking_at(Vec3::new(0.0, look_ahead, 0.0), Vec3::Z),
        NebulaBouncerContext,
        NebulaGameplayCamera,
    ));

    // Main directional light (sun-like) angled for dramatic contrast.
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            illuminance: 15000.0, // Reduced further for darker scene
            ..default()
        },
        Transform::from_xyz(500.0, -1000.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        NebulaBouncerContext,
    ));

    // Add a very subtle fill to keep models slightly visible in shadows.
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0, // Minimal fill for deep shadows
            color: Color::srgb(0.86, 0.90, 1.0),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(500.0, 350.0, 760.0).looking_at(Vec3::ZERO, Vec3::Z),
        NebulaBouncerContext,
    ));

    // Spawn hidden debug overlay (F12 toggles) to verify asset usage in HITL sessions.
    commands
        .spawn((
            NebulaDebugOverlayRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(640.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.08, 0.82)),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                NebulaDebugOverlayText,
                Text::new("Nebula Debug Overlay (F12)"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.95, 1.0)),
            ));
        });

    // -----------------------------------------------------------------------
    // CHUNK LIBRARY INITIALIZATION
    // -----------------------------------------------------------------------

    // A simple "Empty" chunk
    let empty_chunk = ChunkSchema {
        name: "Basic Open".to_string(),
        height: 800.0,
        weight: 1.0,
        pacing: ChunkPacing::Open,
        ..default()
    };

    // Corridor chunk (Transition)
    let corridor_chunk = ChunkSchema {
        name: "Narrow Corridor".to_string(),
        height: 600.0,
        weight: 2.0,
        pacing: ChunkPacing::Transition,
        walls: vec![
            WallDef {
                position: Vec2::new(-350.0, 0.0),
                size: Vec2::new(100.0, 600.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(350.0, 0.0),
                size: Vec2::new(100.0, 600.0),
                rotation: 0.0,
            },
        ],
        ..default()
    };

    // Dense obstacle chunk
    let dense_chunk = ChunkSchema {
        name: "Asteroid Dense".to_string(),
        height: 1000.0,
        weight: 1.5,
        pacing: ChunkPacing::Dense,
        walls: vec![
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(150.0, 150.0),
                rotation: 0.5,
            },
            WallDef {
                position: Vec2::new(-250.0, 300.0),
                size: Vec2::new(100.0, 100.0),
                rotation: -0.3,
            },
            WallDef {
                position: Vec2::new(250.0, -300.0),
                size: Vec2::new(120.0, 120.0),
                rotation: 0.8,
            },
        ],
        spawns: vec![
            SpawnDef {
                spawn_type: SpawnType::Enemy,
                position: Vec2::new(-100.0, 100.0),
            },
            SpawnDef {
                spawn_type: SpawnType::Enemy,
                position: Vec2::new(100.0, -100.0),
            },
        ],
        ..default()
    };

    // Transition chunk with side walls (flaring out)
    let mut flare_chunk = ChunkSchema {
        name: "Flare Transition".to_string(),
        height: 600.0,
        weight: 1.0,
        pacing: ChunkPacing::Transition,
        walls: vec![
            WallDef {
                position: Vec2::new(-380.0, 0.0),
                size: Vec2::new(40.0, 600.0),
                rotation: 0.2,
            },
            WallDef {
                position: Vec2::new(380.0, 0.0),
                size: Vec2::new(40.0, 600.0),
                rotation: -0.2,
            },
        ],
        ..default()
    };
    flare_chunk.top_profile = [false; PROFILE_RESOLUTION];
    flare_chunk.bottom_profile = [false; PROFILE_RESOLUTION];

    library.chunks.push(empty_chunk);
    library.chunks.push(corridor_chunk);
    library.chunks.push(dense_chunk);
    library.chunks.push(flare_chunk);

    // Zigzag Corridor (Transition)
    let zigzag_chunk = ChunkSchema {
        name: "Zigzag Corridor".to_string(),
        height: 800.0,
        weight: 1.0,
        pacing: ChunkPacing::Transition,
        walls: vec![
            WallDef {
                position: Vec2::new(-200.0, -200.0),
                size: Vec2::new(400.0, 50.0),
                rotation: 0.5,
            },
            WallDef {
                position: Vec2::new(200.0, 200.0),
                size: Vec2::new(400.0, 50.0),
                rotation: 0.5,
            },
        ],
        ..default()
    };
    library.chunks.push(zigzag_chunk);

    // Pillar Field (Dense)
    let pillar_chunk = ChunkSchema {
        name: "Pillar Field".to_string(),
        height: 800.0,
        weight: 1.0,
        pacing: ChunkPacing::Dense,
        walls: vec![
            WallDef {
                position: Vec2::new(-150.0, -150.0),
                size: Vec2::new(50.0, 50.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(150.0, -150.0),
                size: Vec2::new(50.0, 50.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(0.0, 0.0),
                size: Vec2::new(50.0, 50.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(-150.0, 150.0),
                size: Vec2::new(50.0, 50.0),
                rotation: 0.0,
            },
            WallDef {
                position: Vec2::new(150.0, 150.0),
                size: Vec2::new(50.0, 50.0),
                rotation: 0.0,
            },
        ],
        spawns: vec![SpawnDef {
            spawn_type: SpawnType::Enemy,
            position: Vec2::new(0.0, 200.0),
        }],
        ..default()
    };
    library.chunks.push(pillar_chunk);

    let policy = ProcgenValidationPolicy::default();
    let preflight = run_preflight_validation(&library, &policy);
    let preflight_artifact = preflight_artifact_path();
    match write_preflight_summary_artifact(&preflight_artifact, &preflight) {
        Ok(()) => {
            validator_telemetry.set_preflight(&preflight, preflight_artifact.display().to_string());
            info!("{}", format_preflight_summary(&preflight));
        }
        Err(err) => {
            warn!(
                "Failed to write procgen preflight summary artifact at {}: {}",
                preflight_artifact.display(),
                err
            );
        }
    }

    procgen_state.next_spawn_y = 0.0;
    procgen_state.last_chunk_bottom_profile = [false; PROFILE_RESOLUTION];
    procgen_state.current_pacing = ChunkPacing::Open;
    procgen_state.previous_pacing = ChunkPacing::Open;
    procgen_state.chunks_in_current_pacing = 0;
    procgen_state.chunks_spawned = 0;
    if procgen_state.global_seed == 0 {
        procgen_state.global_seed = 0x4E42_5F53_4545_445F;
    }

    // Spawn first chunk
    spawn_next_chunk(
        &mut commands,
        &asset_server,
        &asset_manifest,
        &chunk_assignment_profiles,
        &mut procgen_state,
        &*library,
        &mut validator_telemetry,
        &nebula_mats,
        &mut materials,
    );

    // Initialize the kinetic orb pool
    spawn_orb_pool(
        &mut commands,
        &asset_server,
        &asset_manifest,
        &mut orb_pool,
        &nebula_mats,
        &mut materials,
    );
}

pub fn spawn_orb_pool(
    commands: &mut Commands,
    asset_server: &AssetServer,
    asset_manifest: &NebulaAssetManifest,
    orb_pool: &mut KineticOrbPool,
    nebula_mats: &NebulaMaterials,
    materials: &mut Assets<StandardMaterial>,
) {
    let current_count = orb_pool.inactive.len() + orb_pool.active_count;
    let needed = KineticOrbPool::DEFAULT_CAPACITY.saturating_sub(current_count);

    if needed > 0 {
        for _ in 0..needed {
            let id = commands
                .spawn((
                    KineticOrb::default(),
                    RigidBody::Dynamic,
                    Collider::circle(BASE_ORB_RADIUS),
                    Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
                    Friction::new(0.0), // No friction in space
                    Mass(1.0),
                    Transform::from_xyz(9999.0, 9999.0, depth::PROJECTILE),
                    Visibility::Hidden,
                    CollisionLayers::new(
                        GameLayer::Projectile,
                        [GameLayer::Enemy, GameLayer::Wall],
                    ),
                    // RigidBody::Disabled,      // Start disabled
                    CollisionEventsEnabled, // Enable contact reporting
                    ProjectileTrail {
                        points: Vec::new(),
                        max_length: 20,
                        width: BASE_ORB_TRAIL_WIDTH,
                        color: element_trail_color(OrbElement::default()),
                    },
                    Mesh3d(nebula_mats.quad_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: element_trail_color(OrbElement::default()),
                        base_color_texture: Some(
                            asset_server.load(asset_manifest.kinetic_orb.clone()),
                        ),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    })),
                ))
                .id();
            orb_pool.inactive.push(id);
        }
        info!(
            "Spawned {} kinetic orbs into pool matching capacity {}",
            needed,
            KineticOrbPool::DEFAULT_CAPACITY
        );
    }
}

pub fn cleanup_orb_pool(
    mut commands: Commands,
    mut orb_pool: ResMut<KineticOrbPool>,
    q_orbs: Query<Entity, With<KineticOrb>>,
    q_player: Query<Entity, With<PlayerShip>>,
    q_overlay: Query<Entity, With<NebulaDebugOverlayRoot>>,
    q_vfx: Query<Entity, With<TransientVfx>>,
    q_context: Query<Entity, With<NebulaBouncerContext>>,
) {
    // Optional: Despawn all orbs on exit to clean up memory
    for entity in q_orbs.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_player.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_overlay.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
    for entity in q_vfx.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_context.iter() {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
    orb_pool.inactive.clear();
    orb_pool.active_count = 0;
    info!("Cleaned up Nebula Bouncer entities");
}

pub fn cleanup_camera_shake(
    mut commands: Commands,
    mut q_cameras: Query<
        (Entity, &mut Transform, &ScreenShake),
        (With<Camera>, With<NebulaGameplayCamera>),
    >,
) {
    for (entity, mut transform, shake) in &mut q_cameras {
        // Restore camera base position before removing shake.
        transform.translation.x -= shake.last_offset.x;
        transform.translation.y -= shake.last_offset.y;
        commands.entity(entity).remove::<ScreenShake>();
    }
}

pub fn attach_screen_shake_to_cameras(
    mut commands: Commands,
    q_cameras: Query<
        Entity,
        (
            With<Camera>,
            With<NebulaGameplayCamera>,
            Without<ScreenShake>,
        ),
    >,
) {
    for entity in &q_cameras {
        commands.entity(entity).insert(ScreenShake::default());
    }
}

pub fn handle_orb_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    asset_server: Res<AssetServer>,
    asset_manifest: Res<NebulaAssetManifest>,
    mut orbs: Query<(Entity, &mut KineticOrb, &Transform)>,
    mut orb_pool: ResMut<KineticOrbPool>,
    feedback_settings: Res<CameraFeedbackSettings>,
    mut shake: Query<&mut ScreenShake, With<NebulaGameplayCamera>>,
    mut hit_stop: ResMut<HitStop>,
    mut enemies: Query<(Entity, &mut Health, &mut EnemyStatusEffects), With<Enemy>>,
    nebula_mats: Res<NebulaMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let profile = feedback_settings.profile;
    let profile_tuning = feedback_tuning(profile);

    for event in collision_events.read() {
        let e1 = event.collider1;
        let e2 = event.collider2;
        let (orb_entity, _other_entity) = if orbs.contains(e1) {
            (Some(e1), Some(e2))
        } else if orbs.contains(e2) {
            (Some(e2), Some(e1))
        } else {
            (None, None)
        };

        if let Some(entity) = orb_entity {
            if let Ok((e, mut orb, orb_transform)) = orbs.get_mut(entity) {
                // Determine what we hit
                let other = if entity == e1 { e2 } else { e1 };

                // Check if hit enemy
                if let Ok((enemy_entity, mut hp, mut status_effects)) = enemies.get_mut(other) {
                    hp.damage(orb.damage as i32);
                    apply_enemy_status_effects(&mut status_effects, &orb);
                    spawn_transient_vfx(
                        &mut commands,
                        &asset_server,
                        &nebula_mats,
                        &mut materials,
                        &asset_manifest.vfx_impact_flash,
                        orb_transform.translation,
                        element_trail_color(orb.element),
                        TRANSIENT_VFX_BASE_SIZE,
                        TRANSIENT_VFX_BASE_LIFETIME_SECS,
                    );
                    spawn_transient_vfx(
                        &mut commands,
                        &asset_server,
                        &nebula_mats,
                        &mut materials,
                        &asset_manifest.vfx_hit_ring,
                        orb_transform.translation,
                        Color::srgba(1.0, 1.0, 1.0, 0.82),
                        TRANSIENT_VFX_BASE_SIZE * 1.2,
                        TRANSIENT_VFX_BASE_LIFETIME_SECS * 1.15,
                    );

                    // Trigger Hit Feedback
                    hit_stop.timer =
                        (orb.damage * 0.005 + orb.void_dot_duration_secs * 0.03).clamp(0.05, 0.3);
                    hit_stop.timer = hit_stop
                        .timer
                        .max(compute_hit_stop_duration(orb.damage, profile));
                    if let Some(mut s) = shake.iter_mut().next() {
                        s.intensity = next_shake_intensity(
                            s.intensity,
                            orb.damage,
                            false,
                            feedback_settings.shake_enabled,
                            profile,
                        );
                        s.decay = profile_tuning.shake_decay;
                    }

                    if hp.is_dead() {
                        // Defer despawn: remove physics components so the solver
                        // won't reference this entity, then hide it.
                        // The scrolling system will despawn it on the next frame.
                        commands
                            .entity(enemy_entity)
                            .remove::<RigidBody>()
                            .remove::<Collider>()
                            .insert(Visibility::Hidden);
                    }
                } else {
                    // Hit something else (Wall?)
                    // Minor shake for wall hits
                    if let Some(mut s) = shake.iter_mut().next() {
                        s.intensity = next_shake_intensity(
                            s.intensity,
                            orb.damage,
                            true,
                            feedback_settings.shake_enabled,
                            profile,
                        );
                        s.decay = profile_tuning.shake_decay;
                    }
                    spawn_transient_vfx(
                        &mut commands,
                        &asset_server,
                        &nebula_mats,
                        &mut materials,
                        &asset_manifest.vfx_hit_ring,
                        orb_transform.translation,
                        Color::srgba(0.94, 0.98, 1.0, 0.7),
                        TRANSIENT_VFX_BASE_SIZE * 0.9,
                        TRANSIENT_VFX_BASE_LIFETIME_SECS * 0.85,
                    );
                }

                if orb.bounces_remaining > 0 {
                    orb.bounces_remaining -= 1;
                    orb.damage *= 1.10 + ((orb.speed_scale - 1.0) * 0.2);
                } else {
                    // Deactivate
                    commands
                        .entity(e)
                        // .insert(RigidBody::Disabled)
                        .insert(Visibility::Hidden)
                        .remove::<LinearVelocity>();

                    orb.active = false;
                    orb_pool.push(e);
                }
            }
        }
    }
}

pub fn update_level_scrolling(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_manifest: Res<NebulaAssetManifest>,
    chunk_assignment_profiles: Res<ChunkAssignmentProfiles>,
    mut procgen_state: ResMut<ProcGenState>,
    library: Res<ChunkLibrary>,
    mut validator_telemetry: ResMut<ProcgenValidatorTelemetry>,
    nebula_mats: Res<NebulaMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_chunks: Query<(Entity, &mut Transform, Option<&RigidBody>), With<ChunkMember>>,
) {
    const SCROLL_SPEED: f32 = 150.0;
    const VISUAL_DESPAWN_Y: f32 = -1200.0;
    // Delay despawn for physics bodies so we don't remove colliders near active contacts.
    // This mitigates rare Avian solver panics from stale manifold handles during cleanup.
    const PHYSICS_DESPAWN_Y: f32 = -5000.0;
    let dt = time.delta_secs();
    let delta_y = SCROLL_SPEED * dt;

    // Movement
    for (entity, mut transform, rigid_body) in &mut q_chunks {
        transform.translation.y -= delta_y;
        let despawn_y = if rigid_body.is_some() {
            PHYSICS_DESPAWN_Y
        } else {
            VISUAL_DESPAWN_Y
        };
        if transform.translation.y < despawn_y {
            commands.entity(entity).despawn();
        }
    }

    procgen_state.next_spawn_y -= delta_y;

    // Spawn when needed
    if procgen_state.next_spawn_y < 1200.0 {
        spawn_next_chunk(
            &mut commands,
            &asset_server,
            &asset_manifest,
            &chunk_assignment_profiles,
            &mut procgen_state,
            &*library,
            &mut validator_telemetry,
            &nebula_mats,
            &mut materials,
        );
    }
}

pub fn spawn_next_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    asset_manifest: &NebulaAssetManifest,
    assignment_profiles: &ChunkAssignmentProfiles,
    state: &mut ProcGenState,
    library: &ChunkLibrary,
    telemetry: &mut ProcgenValidatorTelemetry,
    nebula_mats: &NebulaMaterials,
    materials: &mut Assets<StandardMaterial>,
) {
    // Determine next target pacing
    let target_pacing = match state.current_pacing {
        ChunkPacing::Open => {
            if state.chunks_in_current_pacing >= 2 {
                ChunkPacing::Transition
            } else {
                ChunkPacing::Open
            }
        }
        ChunkPacing::Transition => {
            if state.chunks_in_current_pacing >= 1 {
                if state.previous_pacing == ChunkPacing::Open {
                    ChunkPacing::Dense
                } else {
                    ChunkPacing::Open
                }
            } else {
                ChunkPacing::Transition
            }
        }
        ChunkPacing::Dense => {
            if state.chunks_in_current_pacing >= 1 {
                ChunkPacing::Transition
            } else {
                ChunkPacing::Dense
            }
        }
    };

    let policy = ProcgenValidationPolicy::default();
    let (selected, rejection_counters) = select_chunk_validated(
        library,
        &state.last_chunk_bottom_profile,
        target_pacing,
        &policy,
    );
    telemetry.record_runtime_rejections(&rejection_counters);

    let Some(selected) = selected else {
        warn!(
            "No candidates found for procgen chunk matching profile {:?} with pacing {:?}!",
            state.last_chunk_bottom_profile, target_pacing
        );
        return;
    };

    // Update pacing state
    if selected.pacing != state.current_pacing {
        state.previous_pacing = state.current_pacing;
        state.current_pacing = selected.pacing;
        state.chunks_in_current_pacing = 1;
    } else {
        state.chunks_in_current_pacing += 1;
    }

    // Spawn the chunk
    let chunk_y = state.next_spawn_y + selected.height / 2.0;
    let terrain_theme = assignment_profiles.terrain_theme_for(selected.pacing);
    let health_scale = assignment_profiles.enemy_health_scale_for(selected.pacing);

    // Spawn floor as modular tiles instead of one stretched sprite.
    spawn_chunk_floor_tiles(
        commands,
        asset_server,
        asset_manifest,
        nebula_mats,
        materials,
        chunk_y,
        selected.height,
        terrain_theme,
    );

    // Spawn deterministic topography from procgen data layer.
    let topography =
        generate_chunk_topography(selected.height, state.global_seed, state.chunks_spawned);
    spawn_chunk_topography(
        commands,
        asset_server,
        nebula_mats,
        chunk_y,
        selected.height,
        &topography,
    );

    // Spawn walls
    for wall in &selected.walls {
        commands.spawn((
            Wall,
            ChunkMember,
            Transform::from_xyz(wall.position.x, chunk_y + wall.position.y, depth::WALL)
                .with_rotation(Quat::from_rotation_z(wall.rotation)),
            RigidBody::Static,
            Collider::rectangle(wall.size.x, wall.size.y),
            Friction::new(0.0),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            CollisionLayers::new(GameLayer::Wall, [GameLayer::Projectile, GameLayer::Player]),
        ));

        spawn_wall_visual_segments(
            commands,
            asset_server,
            asset_manifest,
            nebula_mats,
            materials,
            chunk_y,
            wall,
            terrain_theme,
        );
    }

    // Spawn ORE
    for (spawn_index, spawn) in selected.spawns.iter().enumerate() {
        match spawn.spawn_type {
            SpawnType::Enemy => {
                let seed = assignment_seed(selected, spawn, chunk_y, spawn_index);
                let archetype = assignment_profiles.enemy_archetype_for(selected.pacing, seed);
                let enemy_size = enemy_sprite_size(archetype);
                let enemy_hp = ((enemy_base_hp(archetype) as f32) * health_scale)
                    .round()
                    .max(1.0) as i32;
                let scale_factor = enemy_size.x / 64.0;
                let model_scale = scale_factor * MODEL_UNIT_TO_WORLD;
                commands
                    .spawn((
                        Enemy,
                        ChunkMember,
                        Transform::from_xyz(
                            spawn.position.x,
                            chunk_y + spawn.position.y,
                            depth::ENEMY,
                        )
                        .with_scale(Vec3::ONE),
                        RigidBody::Dynamic,
                        Collider::circle(BASE_ENEMY_COLLIDER_RADIUS * scale_factor),
                        CollisionLayers::new(
                            GameLayer::Enemy,
                            [GameLayer::Projectile, GameLayer::Player],
                        ),
                        Health::new(enemy_hp),
                        EnemyStatusEffects::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SceneRoot(
                                asset_server.load(asset_manifest.enemy_model_default.clone()),
                            ),
                            Transform::from_rotation(
                                Quat::from_rotation_z(std::f32::consts::PI)
                                    * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                            )
                            .with_scale(Vec3::splat(model_scale)),
                        ));
                    });
            }
            _ => {
                // TODO: Resources/Hazards
            }
        }
    }

    // Update state
    state.next_spawn_y += selected.height;
    state.last_chunk_bottom_profile = selected.bottom_profile;
    state.chunks_spawned += 1;
    telemetry.record_selection();

    info!("Spawned chunk: {} at y={}", selected.name, chunk_y);
}

pub fn player_movement(
    _time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut Transform), With<PlayerShip>>,
) {
    for (mut velocity, mut _transform) in &mut query {
        let mut direction = Vec2::ZERO;
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        velocity.0 = direction * 300.0;
    }
}

pub fn update_active_loadout_hotkeys(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut loadout: ResMut<ActiveLoadout>,
) {
    let mut changed = false;
    if input.just_pressed(KeyCode::F6) {
        loadout.cycle_element();
        changed = true;
    }
    if input.just_pressed(KeyCode::F7) {
        loadout.cycle_modifier();
        changed = true;
    }

    if !changed {
        return;
    }

    let now = time.elapsed_secs();
    if now - loadout.last_telemetry_time < TELEMETRY_COOLDOWN_SECS {
        return;
    }

    info!(
        "Nebula loadout | element={} modifier={}",
        loadout.element.as_str(),
        loadout.modifier.as_str()
    );
    loadout.last_telemetry_time = now;
}

pub fn debug_telemetry_hotkey(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    orb_pool: Res<KineticOrbPool>,
    loadout: Res<ActiveLoadout>,
    validator_telemetry: Res<ProcgenValidatorTelemetry>,
    q_enemies: Query<(), With<Enemy>>,
    q_chunk_members: Query<(), With<ChunkMember>>,
    mut last_log_time: Local<f32>,
) {
    if !input.just_pressed(KeyCode::F8) {
        return;
    }

    let now = time.elapsed_secs();
    if now - *last_log_time < TELEMETRY_COOLDOWN_SECS {
        return;
    }

    info!(
        "Nebula telemetry | active_orbs={} inactive_pool={} enemies={} chunk_members={} loadout={}+{} selected_chunks={} reject_profile={} reject_concave={} reject_exit_angle={}",
        orb_pool.active_count,
        orb_pool.inactive.len(),
        q_enemies.iter().count(),
        q_chunk_members.iter().count(),
        loadout.element.as_str(),
        loadout.modifier.as_str(),
        validator_telemetry.selected_chunks,
        validator_telemetry.profile_mismatch_rejections,
        validator_telemetry.concave_trap_rejections,
        validator_telemetry.exit_angle_fail_rejections
    );
    *last_log_time = now;
}

pub fn toggle_camera_shake(
    input: Res<ButtonInput<KeyCode>>,
    mut feedback_settings: ResMut<CameraFeedbackSettings>,
) {
    if !input.just_pressed(KeyCode::F10) {
        return;
    }

    feedback_settings.shake_enabled = !feedback_settings.shake_enabled;
    info!(
        "Camera shake toggled: {}",
        if feedback_settings.shake_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
}

pub fn cycle_feedback_profile(
    input: Res<ButtonInput<KeyCode>>,
    mut feedback_settings: ResMut<CameraFeedbackSettings>,
) {
    if !input.just_pressed(KeyCode::F11) {
        return;
    }

    feedback_settings.profile = feedback_settings.profile.next();
    let tuning = feedback_tuning(feedback_settings.profile);
    info!(
        "Feedback profile: {} (threshold={:.1}, cap={:.1}, decay={:.1})",
        feedback_settings.profile.as_str(),
        tuning.shake_damage_threshold,
        tuning.shake_cap,
        tuning.shake_decay
    );
}

pub fn feedback_telemetry_hotkey(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    feedback_settings: Res<CameraFeedbackSettings>,
    hit_stop: Res<HitStop>,
    q_shake: Query<&ScreenShake, (With<Camera>, With<NebulaGameplayCamera>)>,
    mut last_log_time: Local<f32>,
) {
    if !input.just_pressed(KeyCode::F9) {
        return;
    }

    let now = time.elapsed_secs();
    if now - *last_log_time < FEEDBACK_TELEMETRY_COOLDOWN_SECS {
        return;
    }

    let (shake_intensity, shake_decay) = q_shake
        .iter()
        .next()
        .map(|s| (s.intensity, s.decay))
        .unwrap_or((0.0, 0.0));
    info!(
        "Feedback telemetry | profile={} shake_enabled={} shake_intensity={:.2} shake_decay={:.2} hit_stop={:.3}",
        feedback_settings.profile.as_str(),
        feedback_settings.shake_enabled,
        shake_intensity,
        shake_decay,
        hit_stop.timer.max(0.0)
    );
    *last_log_time = now;
}

pub fn toggle_debug_asset_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut q_overlay: Query<&mut Visibility, With<NebulaDebugOverlayRoot>>,
) {
    if !input.just_pressed(KeyCode::F12) {
        return;
    }

    if let Some(mut visibility) = q_overlay.iter_mut().next() {
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            _ => Visibility::Visible,
        };
    }
}

pub fn update_debug_asset_overlay_text(
    asset_manifest: Res<NebulaAssetManifest>,
    chunk_profiles: Res<ChunkAssignmentProfiles>,
    sprite_orientation: Res<SpriteOrientationConfig>,
    q_player: Query<&Transform, With<PlayerShip>>,
    q_enemy: Query<&Transform, With<Enemy>>,
    q_wall: Query<&Transform, With<WallVisual>>,
    q_ground: Query<&Transform, With<GroundVisual>>,
    q_orb: Query<&Transform, With<KineticOrb>>,
    mut q_text: Query<&mut Text, With<NebulaDebugOverlayText>>,
) {
    let Some(mut text) = q_text.iter_mut().next() else {
        return;
    };

    let player_size = q_player
        .iter()
        .next()
        .map(|t| format!("{:.0}x{:.0}", t.scale.x, t.scale.y))
        .unwrap_or_else(|| "n/a".to_string());
    let enemy_size = q_enemy
        .iter()
        .next()
        .map(|t| format!("{:.0}x{:.0}", t.scale.x, t.scale.y))
        .unwrap_or_else(|| "n/a".to_string());
    let wall_size = q_wall
        .iter()
        .next()
        .map(|t| format!("{:.0}x{:.0}", t.scale.x, t.scale.y))
        .unwrap_or_else(|| "n/a".to_string());
    let ground_size = q_ground
        .iter()
        .next()
        .map(|t| format!("{:.0}x{:.0}", t.scale.x, t.scale.y))
        .unwrap_or_else(|| "n/a".to_string());
    let orb_size = q_orb
        .iter()
        .next()
        .map(|t| format!("{:.0}x{:.0}", t.scale.x, t.scale.y))
        .unwrap_or_else(|| "n/a".to_string());

    **text = format!(
        "Nebula Asset Overlay (F12)\n\
player: {player_path} | size={player_size}\n\
enemy(s): scout={enemy_scout} interceptor={enemy_interceptor} heavy={enemy_heavy} bulwark={enemy_bulwark} | sample_size={enemy_size} | count={enemy_count}\n\
wall: {wall_path} | sample_size={wall_size} | count={wall_count}\n\
ground: {ground_path} | sample_size={ground_size} | count={ground_count}\n\
orb: {orb_path} | sample_size={orb_size} | count={orb_count}\n\
vfx: impact={vfx_impact} ring={vfx_ring}\n\
assignment(open/tr/dense): {open_weights} / {transition_weights} / {dense_weights}\n\
orientation_offsets_deg: player={player_deg:.1} orb={orb_deg:.1} enemy={enemy_deg:.1}",
        player_path = asset_manifest.player_ship.as_str(),
        enemy_scout = asset_manifest.enemy_scout.as_str(),
        enemy_interceptor = asset_manifest.enemy_interceptor.as_str(),
        enemy_heavy = asset_manifest.enemy_heavy.as_str(),
        enemy_bulwark = asset_manifest.enemy_bulwark.as_str(),
        wall_path = asset_manifest.wall_tile.as_str(),
        ground_path = asset_manifest.ground_tile.as_str(),
        orb_path = asset_manifest.kinetic_orb.as_str(),
        vfx_impact = asset_manifest.vfx_impact_flash.as_str(),
        vfx_ring = asset_manifest.vfx_hit_ring.as_str(),
        open_weights = format!(
            "s:{:.2} i:{:.2} h:{:.2} b:{:.2}",
            chunk_profiles.open.enemy_weights.scout,
            chunk_profiles.open.enemy_weights.interceptor,
            chunk_profiles.open.enemy_weights.heavy,
            chunk_profiles.open.enemy_weights.bulwark,
        ),
        transition_weights = format!(
            "s:{:.2} i:{:.2} h:{:.2} b:{:.2}",
            chunk_profiles.transition.enemy_weights.scout,
            chunk_profiles.transition.enemy_weights.interceptor,
            chunk_profiles.transition.enemy_weights.heavy,
            chunk_profiles.transition.enemy_weights.bulwark,
        ),
        dense_weights = format!(
            "s:{:.2} i:{:.2} h:{:.2} b:{:.2}",
            chunk_profiles.dense.enemy_weights.scout,
            chunk_profiles.dense.enemy_weights.interceptor,
            chunk_profiles.dense.enemy_weights.heavy,
            chunk_profiles.dense.enemy_weights.bulwark,
        ),
        enemy_count = q_enemy.iter().count(),
        wall_count = q_wall.iter().count(),
        ground_count = q_ground.iter().count(),
        orb_count = q_orb.iter().count(),
        player_deg = sprite_orientation.player_forward_offset_deg,
        orb_deg = sprite_orientation.orb_forward_offset_deg,
        enemy_deg = sprite_orientation.enemy_forward_offset_deg,
    );
}

pub fn orient_player_to_cursor(
    sprite_orientation: Res<SpriteOrientationConfig>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<NebulaGameplayCamera>>,
    mut q_player: Query<&mut Transform, With<PlayerShip>>,
) {
    let Some(window) = q_window.iter().next() else {
        return;
    };
    let Some((camera, camera_transform)) = q_camera.iter().next() else {
        return;
    };
    let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| {
            // Intersect with Z=0 plane (gameplay plane)
            // ray.origin + t * ray.direction = (x, y, 0)
            // origin.z + t * direction.z = 0  => t = -origin.z / direction.z
            if ray.direction.z.abs() < 1e-6 {
                None
            } else {
                let t = -ray.origin.z / ray.direction.z;
                if t < 0.0 {
                    return None;
                }
                Some(ray.origin.truncate() + ray.direction.truncate() * t)
            }
        })
    else {
        return;
    };

    for mut player_transform in &mut q_player {
        let aim_dir = cursor_pos - player_transform.translation.truncate();
        if let Some(angle) =
            facing_angle(aim_dir, sprite_orientation.player_forward_offset_radians())
        {
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn player_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_manifest: Res<NebulaAssetManifest>,
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<NebulaGameplayCamera>>,
    q_player: Query<(&Transform, &Children), With<PlayerShip>>,
    q_player_visual: Query<&GlobalTransform, With<PlayerVisualRoot>>,
    mut orb_pool: ResMut<KineticOrbPool>,
    loadout: Res<ActiveLoadout>,
    synergy_matrix: Res<OrbSynergyMatrix>,
    sprite_orientation: Res<SpriteOrientationConfig>,
    q_enemies: Query<&Transform, With<Enemy>>,
    nebula_mats: Res<NebulaMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Some((player_transform, player_children)) = q_player.iter().next() else {
            return;
        };
        let Some(orb_entity) = orb_pool.pop() else {
            return;
        };

        let player_origin = player_children
            .iter()
            .find_map(|child| {
                q_player_visual
                    .get(child)
                    .ok()
                    .map(|visual| visual.translation().truncate())
            })
            .unwrap_or(player_transform.translation.truncate());

        // Use iter().next() for safety and simplicity
        let Some(window) = q_window.iter().next() else {
            orb_pool.push(orb_entity);
            return;
        };
        let Some((camera, camera_transform)) = q_camera.iter().next() else {
            orb_pool.push(orb_entity);
            return;
        };

        if let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .and_then(|ray| {
                // Intersect with Z=0 plane (gameplay plane)
                if ray.direction.z.abs() < 1e-6 {
                    None
                } else {
                    let t = -ray.origin.z / ray.direction.z;
                    if t < 0.0 {
                        return None;
                    }
                    Some(ray.origin.truncate() + ray.direction.truncate() * t)
                }
            })
        {
            let mut direction = (cursor_pos - player_origin).normalize_or_zero();
            if direction.length_squared() <= f32::EPSILON {
                direction = player_transform
                    .rotation
                    .mul_vec3(Vec3::Y)
                    .truncate()
                    .normalize_or_zero();
            }
            if direction.length_squared() <= f32::EPSILON {
                orb_pool.push(orb_entity);
                return;
            }

            // AIM ASSIST
            let assist_cone = 30.0_f32.to_radians();
            let mut best_target_dir = None;
            let mut best_dist_sq = 600.0 * 600.0;

            for enemy_trans in &q_enemies {
                let to_enemy = enemy_trans.translation.truncate() - player_origin;
                let dist_sq = to_enemy.length_squared();

                if dist_sq < best_dist_sq {
                    let dir_to_enemy = to_enemy.normalize_or_zero();
                    let dot = direction.dot(dir_to_enemy);
                    if dot > assist_cone.cos() {
                        best_dist_sq = dist_sq;
                        best_target_dir = Some(dir_to_enemy);
                    }
                }
            }

            if let Some(target_dir) = best_target_dir {
                direction = target_dir;
            }
            let orb_spawn_origin =
                player_origin + direction * PLAYER_MUZZLE_FORWARD_OFFSET.max(BASE_ORB_RADIUS);

            let profile = synergy_matrix.get(loadout.element, loadout.modifier);
            let base_stats = OrbSpawnStats::new(
                BASE_ORB_DAMAGE,
                BASE_ORB_SPEED,
                BASE_ORB_BOUNCES,
                BASE_ORB_RADIUS,
            );
            let resolved_stats = resolve_orb_spawn_stats(base_stats, profile);

            // Spawn/Activate Orb
            let orb_rotation =
                facing_angle(direction, sprite_orientation.orb_forward_offset_radians())
                    .map(Quat::from_rotation_z)
                    .unwrap_or_default();
            commands.entity(orb_entity).insert((
                Collider::circle(resolved_stats.radius),
                Transform::from_xyz(orb_spawn_origin.x, orb_spawn_origin.y, depth::PROJECTILE)
                    .with_rotation(orb_rotation),
                LinearVelocity(direction * resolved_stats.speed),
                Visibility::Visible,
                RigidBody::Dynamic,
                Mesh3d(nebula_mats.quad_mesh.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: element_trail_color(loadout.element),
                    base_color_texture: Some(asset_server.load(asset_manifest.kinetic_orb.clone())),
                    unlit: true,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                })),
                KineticOrb {
                    active: true,
                    bounces_remaining: resolved_stats.bounces,
                    damage: resolved_stats.damage,
                    element: loadout.element,
                    modifier: loadout.modifier,
                    radius_scale: profile.radius_scale,
                    speed_scale: profile.speed_scale,
                    cryo_slow_factor: profile.cryo_slow_factor,
                    cryo_duration_secs: profile.cryo_duration_secs,
                    void_dot_dps: profile.void_dot_dps,
                    void_dot_duration_secs: profile.void_dot_duration_secs,
                    ..default()
                },
                ProjectileTrail {
                    points: Vec::new(),
                    max_length: 20,
                    width: BASE_ORB_TRAIL_WIDTH * profile.radius_scale.clamp(0.7, 1.4),
                    color: element_trail_color(loadout.element),
                },
            ));
        } else {
            orb_pool.push(orb_entity);
        }
    }
}

pub fn orient_orbs_to_velocity(
    sprite_orientation: Res<SpriteOrientationConfig>,
    mut query: Query<(&LinearVelocity, &mut Transform, &KineticOrb)>,
) {
    for (velocity, mut transform, orb) in &mut query {
        if !orb.active {
            continue;
        }

        if let Some(angle) =
            facing_angle(velocity.0, sprite_orientation.orb_forward_offset_radians())
        {
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn update_trails(
    mut gizmos: Gizmos,
    mut query: Query<(&Transform, &mut ProjectileTrail, &KineticOrb)>,
) {
    for (transform, mut trail, orb) in &mut query {
        if !orb.active {
            trail.points.clear();
            continue;
        }

        trail.points.push(transform.translation);
        if trail.points.len() > trail.max_length {
            trail.points.remove(0);
        }

        if trail.points.len() > 1 {
            for i in 0..trail.points.len() - 1 {
                gizmos.line(
                    trail.points[i],
                    trail.points[i + 1],
                    trail.color.with_alpha(i as f32 / trail.points.len() as f32),
                );
            }
        }
    }
}

pub fn update_transient_vfx(
    time: Res<Time>,
    mut commands: Commands,
    mut q_vfx: Query<(
        Entity,
        &mut TransientVfx,
        &mut Transform,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (entity, mut vfx, mut transform, material_handle) in &mut q_vfx {
        vfx.ttl_secs -= dt;
        let next_scale = (transform.scale.x - vfx.shrink_per_sec * dt).max(2.0);
        transform.scale = Vec3::splat(next_scale);

        let alpha = (vfx.ttl_secs / TRANSIENT_VFX_BASE_LIFETIME_SECS).clamp(0.0, 1.0);
        if let Some(mat) = materials.get_mut(material_handle) {
            mat.base_color.set_alpha(alpha);
        }

        if vfx.ttl_secs <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_enemy_status_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Health, &mut EnemyStatusEffects), With<Enemy>>,
) {
    let dt = time.delta_secs();
    for (entity, mut health, mut status_effects) in &mut enemies {
        advance_enemy_status_effects(&mut status_effects, &mut health, dt);
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn apply_shake(
    time: Res<Time<Real>>,
    feedback_settings: Res<CameraFeedbackSettings>,
    mut query: Query<
        (&mut Transform, &mut ScreenShake),
        (With<Camera>, With<NebulaGameplayCamera>),
    >,
) {
    let dt = time.delta_secs();
    let tuning = feedback_tuning(feedback_settings.profile);
    for (mut transform, mut shake) in &mut query {
        // Remove last frame's offset so shake never accumulates as drift.
        transform.translation.x -= shake.last_offset.x;
        transform.translation.y -= shake.last_offset.y;
        shake.last_offset = Vec2::ZERO;
        shake.decay = tuning.shake_decay;

        if !feedback_settings.shake_enabled {
            shake.intensity = 0.0;
            continue;
        }

        if shake.intensity > 0.0 {
            let offset = Vec2::new(
                (rand::random::<f32>() - 0.5) * shake.intensity,
                (rand::random::<f32>() - 0.5) * shake.intensity,
            );
            transform.translation.x += offset.x;
            transform.translation.y += offset.y;
            shake.last_offset = offset;
            shake.intensity = (shake.intensity - shake.decay * dt).max(0.0);
        }
    }
}

pub fn update_hit_stop(
    time: Res<Time<Real>>,
    mut hit_stop: ResMut<HitStop>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if hit_stop.timer > 0.0 {
        hit_stop.timer -= time.delta_secs();
        virtual_time.set_relative_speed(0.0);
    } else {
        virtual_time.set_relative_speed(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eras::era_future::nebula_bouncer::resources::{
        ActiveLoadout, BOUNCE_MAX, BOUNCE_MIN, DAMAGE_MAX, DAMAGE_MIN, RADIUS_MAX, RADIUS_MIN,
        SPEED_MAX, SPEED_MIN,
    };
    use std::collections::HashSet;

    #[test]
    fn synergy_matrix_covers_all_16_combinations() {
        let matrix = OrbSynergyMatrix::default();
        assert_eq!(matrix.iter().count(), OrbSynergyMatrix::ENTRY_COUNT);

        let mut seen = HashSet::new();
        for (element, modifier, _) in matrix.iter() {
            seen.insert((element.index(), modifier.index()));
        }
        assert_eq!(seen.len(), OrbSynergyMatrix::ENTRY_COUNT);
    }

    #[test]
    fn resolved_spawn_stats_respect_clamp_policy() {
        let base = OrbSpawnStats::new(10.0, 500.0, 3, 5.0);
        let max_profile =
            crate::eras::era_future::nebula_bouncer::resources::OrbSynergyProfile::new(
                99.0, 99.0, 99, 99.0, 1.0, 0.0, 0.0, 0.0,
            );
        let min_profile =
            crate::eras::era_future::nebula_bouncer::resources::OrbSynergyProfile::new(
                0.01, 0.01, -99, 0.01, 1.0, 0.0, 0.0, 0.0,
            );

        let max_stats = resolve_orb_spawn_stats(base, max_profile);
        assert_eq!(max_stats.damage, DAMAGE_MAX);
        assert_eq!(max_stats.speed, SPEED_MAX);
        assert_eq!(max_stats.bounces, BOUNCE_MAX as u32);
        assert_eq!(max_stats.radius, RADIUS_MAX);

        let min_stats = resolve_orb_spawn_stats(base, min_profile);
        assert_eq!(min_stats.damage, DAMAGE_MIN);
        assert_eq!(min_stats.speed, SPEED_MIN);
        assert_eq!(min_stats.bounces, BOUNCE_MIN as u32);
        assert_eq!(min_stats.radius, RADIUS_MIN);
    }

    #[test]
    fn loadout_cycles_follow_expected_order() {
        let mut loadout = ActiveLoadout::default();

        let mut seen_elements = Vec::with_capacity(OrbElement::ALL.len());
        for _ in 0..OrbElement::ALL.len() {
            seen_elements.push(loadout.element);
            loadout.cycle_element();
        }
        assert_eq!(seen_elements, OrbElement::ALL);
        assert_eq!(loadout.element, OrbElement::ALL[0]);

        let mut seen_modifiers = Vec::with_capacity(OrbModifier::ALL.len());
        for _ in 0..OrbModifier::ALL.len() {
            seen_modifiers.push(loadout.modifier);
            loadout.cycle_modifier();
        }
        assert_eq!(seen_modifiers, OrbModifier::ALL);
        assert_eq!(loadout.modifier, OrbModifier::ALL[0]);
    }

    #[test]
    fn cryo_and_void_status_timers_apply_and_expire() {
        let mut health = Health::new(50);
        let mut status = EnemyStatusEffects::default();
        let orb = KineticOrb {
            cryo_slow_factor: 0.6,
            cryo_duration_secs: 1.0,
            void_dot_dps: 4.0,
            void_dot_duration_secs: 1.6,
            ..default()
        };

        apply_enemy_status_effects(&mut status, &orb);
        assert_eq!(status.cryo_slow_factor, 0.6);
        assert_eq!(status.cryo_slow_timer, 1.0);
        assert_eq!(status.void_dot_dps, 4.0);
        assert_eq!(status.void_dot_timer, 1.6);
        assert_eq!(status.void_dot_tick_timer, VOID_DOT_TICK_INTERVAL_SECS);

        advance_enemy_status_effects(&mut status, &mut health, 0.25);
        assert_eq!(health.current, 50);

        advance_enemy_status_effects(&mut status, &mut health, 0.30);
        advance_enemy_status_effects(&mut status, &mut health, 0.40);
        advance_enemy_status_effects(&mut status, &mut health, 0.10);
        assert_eq!(health.current, 46);

        advance_enemy_status_effects(&mut status, &mut health, 2.0);
        assert_eq!(status.cryo_slow_timer, 0.0);
        assert_eq!(status.cryo_slow_factor, 1.0);
        assert_eq!(status.void_dot_timer, 0.0);
        assert_eq!(status.void_dot_dps, 0.0);
        assert_eq!(status.void_dot_tick_timer, 0.0);
    }
}
