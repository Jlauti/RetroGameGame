use crate::eras::era_future::nebula_bouncer::components::*;
use crate::eras::era_future::nebula_bouncer::procgen::*;
use crate::eras::era_future::nebula_bouncer::resources::{
    ActiveLoadout, HitStop, KineticOrbPool, OrbSpawnStats, OrbSynergyMatrix,
    resolve_orb_spawn_stats,
};
use crate::shared::components::Health;
use avian2d::prelude::*;
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
// use rand::prelude::*; // Use explicit random calls

const SHIP_FORWARD_OFFSET_RADIANS: f32 = -std::f32::consts::FRAC_PI_2;
const ORB_FORWARD_OFFSET_RADIANS: f32 = -std::f32::consts::FRAC_PI_2;
const TELEMETRY_COOLDOWN_SECS: f32 = 0.25;
const BASE_ORB_DAMAGE: f32 = 10.0;
const BASE_ORB_SPEED: f32 = 500.0;
const BASE_ORB_BOUNCES: u32 = 3;
const BASE_ORB_RADIUS: f32 = 5.0;
const BASE_ORB_TRAIL_WIDTH: f32 = 4.0;
const VOID_DOT_TICK_INTERVAL_SECS: f32 = 0.5;

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

pub fn setup_nebula_bouncer(
    mut commands: Commands,
    mut library: ResMut<ChunkLibrary>,
    mut procgen_state: ResMut<ProcGenState>,
) {
    info!("Nebula Bouncer scaffold loaded (Avian 2D integrated).");
    // Ensure gravity is zero for top-down physics
    commands.insert_resource(Gravity(Vec2::ZERO));

    // Spawn Player
    commands.spawn((
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
        Sprite {
            color: Color::srgb(0.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
    ));

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

    // Validate static chunks for softlocks
    for chunk in &library.chunks {
        if let ValidationResult::Fail(msg) = validate_softlock_constraints(&chunk.walls) {
            warn!("Chunk '{}' failed softlock validation: {}", chunk.name, msg);
        }
    }

    procgen_state.next_spawn_y = 0.0;
    procgen_state.last_chunk_bottom_profile = [false; PROFILE_RESOLUTION];
    procgen_state.current_pacing = ChunkPacing::Open;
    procgen_state.previous_pacing = ChunkPacing::Open;
    procgen_state.chunks_in_current_pacing = 0;

    // Spawn first chunk
    spawn_next_chunk(&mut commands, &mut procgen_state, &*library);
}

pub fn spawn_orb_pool(mut commands: Commands, mut orb_pool: ResMut<KineticOrbPool>) {
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
) {
    // Optional: Despawn all orbs on exit to clean up memory
    for entity in q_orbs.iter() {
        commands.entity(entity).despawn();
    }
    for entity in q_player.iter() {
        commands.entity(entity).despawn();
    }
    orb_pool.inactive.clear();
    orb_pool.active_count = 0;
    info!("Cleaned up Nebula Bouncer entities");
}

pub fn cleanup_camera_shake(
    mut commands: Commands,
    mut q_cameras: Query<(Entity, &mut Transform, &ScreenShake), With<Camera>>,
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
    q_cameras: Query<Entity, (With<Camera>, Without<ScreenShake>)>,
) {
    for entity in &q_cameras {
        commands.entity(entity).insert(ScreenShake::default());
    }
}

pub fn handle_orb_collisions(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    mut orbs: Query<(Entity, &mut KineticOrb)>,
    mut orb_pool: ResMut<KineticOrbPool>,
    mut shake: Query<&mut ScreenShake>,
    mut hit_stop: ResMut<HitStop>,
    mut enemies: Query<(Entity, &mut Health, &mut EnemyStatusEffects), With<Enemy>>,
) {
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
            if let Ok((e, mut orb)) = orbs.get_mut(entity) {
                // Determine what we hit
                let other = if entity == e1 { e2 } else { e1 };

                // Check if hit enemy
                if let Ok((enemy_entity, mut hp, mut status_effects)) = enemies.get_mut(other) {
                    hp.damage(orb.damage as i32);
                    apply_enemy_status_effects(&mut status_effects, &orb);

                    // Trigger Hit Feedback
                    hit_stop.timer =
                        (orb.damage * 0.005 + orb.void_dot_duration_secs * 0.03).clamp(0.05, 0.3);
                    if let Some(mut s) = shake.iter_mut().next() {
                        s.intensity += orb.damage * 3.0;
                        s.decay = 8.0;
                    }

                    if hp.is_dead() {
                        commands.entity(enemy_entity).despawn();
                        // Optional: Spawn explosion or particles here
                        // commands.spawn(ExplosionBundle::new(transform...));
                    }
                } else {
                    // Hit something else (Wall?)
                    // Minor shake for wall hits
                    if let Some(mut s) = shake.iter_mut().next() {
                        s.intensity += 2.0;
                        s.decay = 10.0;
                    }
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
    mut procgen_state: ResMut<ProcGenState>,
    library: Res<ChunkLibrary>,
    mut q_chunks: Query<(Entity, &mut Transform), With<ChunkMember>>,
) {
    const SCROLL_SPEED: f32 = 150.0;
    let dt = time.delta_secs();
    let delta_y = SCROLL_SPEED * dt;

    // Movement
    for (entity, mut transform) in &mut q_chunks {
        transform.translation.y -= delta_y;
        if transform.translation.y < -1200.0 {
            commands.entity(entity).despawn();
        }
    }

    procgen_state.next_spawn_y -= delta_y;

    // Spawn when needed
    if procgen_state.next_spawn_y < 1200.0 {
        spawn_next_chunk(&mut commands, &mut procgen_state, &*library);
    }
}

pub fn spawn_next_chunk(commands: &mut Commands, state: &mut ProcGenState, library: &ChunkLibrary) {
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

    let Some(selected) = select_chunk(library, &state.last_chunk_bottom_profile, target_pacing)
    else {
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

    // Spawn walls
    for wall in &selected.walls {
        commands.spawn((
            Wall,
            ChunkMember,
            Sprite {
                color: Color::srgb(0.3, 0.3, 0.9),
                custom_size: Some(wall.size),
                ..default()
            },
            Transform::from_xyz(wall.position.x, chunk_y + wall.position.y, 0.0)
                .with_rotation(Quat::from_rotation_z(wall.rotation)),
            RigidBody::Static,
            Collider::rectangle(wall.size.x, wall.size.y),
            Friction::new(0.0),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            CollisionLayers::new(GameLayer::Wall, [GameLayer::Projectile, GameLayer::Player]),
        ));
    }

    // Spawn ORE
    for spawn in &selected.spawns {
        match spawn.spawn_type {
            SpawnType::Enemy => {
                commands.spawn((
                    Enemy,
                    ChunkMember,
                    Sprite {
                        color: Color::srgb(0.9, 0.1, 0.1),
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    Transform::from_xyz(spawn.position.x, chunk_y + spawn.position.y, 1.0),
                    RigidBody::Dynamic,
                    Collider::circle(15.0),
                    CollisionLayers::new(
                        GameLayer::Enemy,
                        [GameLayer::Projectile, GameLayer::Player],
                    ),
                    Health::new(50), // Enemy HP
                    EnemyStatusEffects::default(),
                ));
            }
            _ => {
                // TODO: Resources/Hazards
            }
        }
    }

    // Update state
    state.next_spawn_y += selected.height;
    state.last_chunk_bottom_profile = selected.bottom_profile;

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
        "Nebula telemetry | active_orbs={} inactive_pool={} enemies={} chunk_members={} loadout={}+{}",
        orb_pool.active_count,
        orb_pool.inactive.len(),
        q_enemies.iter().count(),
        q_chunk_members.iter().count(),
        loadout.element.as_str(),
        loadout.modifier.as_str()
    );
    *last_log_time = now;
}

pub fn orient_player_to_cursor(
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
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
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    else {
        return;
    };

    for mut player_transform in &mut q_player {
        let aim_dir = cursor_pos - player_transform.translation.truncate();
        if let Some(angle) = facing_angle(aim_dir, SHIP_FORWARD_OFFSET_RADIANS) {
            player_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn player_shoot(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_player: Query<&Transform, With<PlayerShip>>,
    mut orb_pool: ResMut<KineticOrbPool>,
    loadout: Res<ActiveLoadout>,
    synergy_matrix: Res<OrbSynergyMatrix>,
    q_enemies: Query<&Transform, With<Enemy>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Some(player_transform) = q_player.iter().next() else {
            return;
        };
        let Some(orb_entity) = orb_pool.pop() else {
            return;
        };

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
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            let mut direction =
                (cursor_pos - player_transform.translation.truncate()).normalize_or_zero();

            // AIM ASSIST
            let assist_cone = 30.0_f32.to_radians();
            let mut best_target_dir = None;
            let mut best_dist_sq = 600.0 * 600.0;

            for enemy_trans in &q_enemies {
                let to_enemy =
                    enemy_trans.translation.truncate() - player_transform.translation.truncate();
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

            let profile = synergy_matrix.get(loadout.element, loadout.modifier);
            let base_stats = OrbSpawnStats::new(
                BASE_ORB_DAMAGE,
                BASE_ORB_SPEED,
                BASE_ORB_BOUNCES,
                BASE_ORB_RADIUS,
            );
            let resolved_stats = resolve_orb_spawn_stats(base_stats, profile);

            // Spawn/Activate Orb
            let orb_rotation = facing_angle(direction, ORB_FORWARD_OFFSET_RADIANS)
                .map(Quat::from_rotation_z)
                .unwrap_or_default();
            commands.entity(orb_entity).insert((
                Collider::circle(resolved_stats.radius),
                Transform::from_translation(player_transform.translation)
                    .with_rotation(orb_rotation),
                LinearVelocity(direction * resolved_stats.speed),
                Visibility::Visible,
                RigidBody::Dynamic,
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

pub fn orient_orbs_to_velocity(mut query: Query<(&LinearVelocity, &mut Transform, &KineticOrb)>) {
    for (velocity, mut transform, orb) in &mut query {
        if !orb.active {
            continue;
        }

        if let Some(angle) = facing_angle(velocity.0, ORB_FORWARD_OFFSET_RADIANS) {
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
    mut query: Query<(&mut Transform, &mut ScreenShake), With<Camera>>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut shake) in &mut query {
        // Remove last frame's offset so shake never accumulates as drift.
        transform.translation.x -= shake.last_offset.x;
        transform.translation.y -= shake.last_offset.y;
        shake.last_offset = Vec2::ZERO;

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
