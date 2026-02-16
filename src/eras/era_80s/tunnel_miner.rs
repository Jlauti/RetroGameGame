use bevy::prelude::*;

use crate::core::progression::PlayerProgress;
use crate::core::states::{Era, GameState, MiniGameId, PlayingState};
use crate::shared::components::{GridPosition, Health, Lives, Player, Score};
use crate::ui::colors;
use crate::ui::results::GameResults;

/// Tunnel Miner — inspired by Digger (1983).
/// Grid-based digging, emerald collection, enemy avoidance.
pub struct TunnelMinerPlugin;

impl Plugin for TunnelMinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            setup_tunnel_miner.run_if(in_state(PlayingState::TunnelMiner)),
        )
        .add_systems(
            Update,
            (
                player_movement,
                animate_sprites,
                dig_system,
                collect_emeralds,
                enemy_ai,
                enemy_spawner,
                gold_bag_physics,
                weapon_system,
                fireball_collision,
                cherry_system,
                collect_cherry,
                level_progression,
                check_death,
                update_hud,
                handle_pause,
            )
                .run_if(in_state(PlayingState::TunnelMiner)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_tunnel_miner);
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const GRID_WIDTH: i32 = 15;
const GRID_HEIGHT: i32 = 10;
const TILE_SIZE: f32 = 48.0;
const MOVE_SPEED: f32 = 5.0; // tiles per second

// Scoring
const EMERALD_POINTS: u64 = 25;
const EMERALD_STREAK_BONUS: u64 = 250; // 8 in a row
const GOLD_PILE_POINTS: u64 = 500;
const ENEMY_KILL_POINTS: u64 = 250;
const ENEMY_CRUSH_POINTS: u64 = 250;
const CHERRY_POINTS: u64 = 1000;
const COMPLETION_THRESHOLD: u64 = 1000;

const MAP_80S_GEMS: [[u8; 15]; 10] = [
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
    [1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

// ─── Components ────────────────────────────────────────────────────

#[derive(Component)]
struct TunnelMinerEntity; // Marker for cleanup

#[derive(Component)]
struct TunnelMinerPlayer {
    facing: Direction,
    move_timer: Timer,
    emerald_streak: u32,
    weapon_cooldown: Timer,
}

#[derive(Component)]
struct TunnelMinerHud;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct EarthTile;

#[derive(Component)]
struct Emerald;

#[derive(Component)]
struct GoldBag {
    falling: bool,
    fall_timer: Timer,
    fall_distance: i32,
    is_pile: bool,
}

#[derive(Component)]
struct Fireball {
    direction: Direction,
    timer: Timer,
}

#[derive(Component)]
struct Nobbin {
    hobbin: bool, // transformed into a Hobbin?
    move_timer: Timer,
    time_alive: f32,
}

#[derive(Resource)]
struct EnemySpawner {
    timer: Timer,
    total_to_spawn: u32,
    spawned_count: u32,
    active_count: u32,
}

#[derive(Component)]
struct Weapon {
    active: bool,
    recharge_timer: Timer,
    position: Vec2,
    direction: Direction,
}

#[derive(Component)]
struct Cherry;

#[derive(Resource)]
struct TunnelMinerState {
    level: u32,
    emeralds_remaining: u32,
    // Bonus Mode
    bonus_mode_active: bool,
    bonus_mode_timer: Timer,
    bonus_eat_score: u64, // 200, 400, 800...
    cherry_spawned: bool,
}

// ─── Setup ─────────────────────────────────────────────────────────

// ─── Animation Components ──────────────────────────────────────────

#[derive(Component)]
struct AnimationState {
    timer: Timer,
    frame_index: usize,
    current_animation: AnimationFn,
}

#[derive(Component)]
struct AnimationFrames {
    // We map a "state" (like Idle, Walk) to a list of image handles
    animations: std::collections::HashMap<AnimationFn, Vec<Handle<Image>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AnimationFn {
    Idle,
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_tunnel_miner(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up Tunnel Miner level...");

    // Initialize game state
    commands.insert_resource(TunnelMinerState {
        level: 1,
        emeralds_remaining: 0,
        bonus_mode_active: false,
        bonus_mode_timer: Timer::from_seconds(10.0, TimerMode::Once),
        bonus_eat_score: 200,
        cherry_spawned: false,
    });

    commands.insert_resource(EnemySpawner {
        timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        total_to_spawn: 5 + (2), // Base + Level adjustment (simplified)
        spawned_count: 0,
        active_count: 0,
    });

    commands.spawn((
        AudioPlayer::new(asset_server.load("music/Pixel Popcorn Rush.mp3")),
        PlaybackSettings::LOOP,
        TunnelMinerEntity,
    ));

    commands.insert_resource(Score::default());
    commands.insert_resource(Lives::new(3));

    // Grid origin (center the grid on screen)
    let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
    let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;

    // Load environment and item textures
    let earth_handle: Handle<Image> =
        asset_server.load("sprites/80s/tunnel_miner_environment/tunnel_miner_environment_001.png");

    // Load Gem Textures
    let emerald_handle: Handle<Image> =
        asset_server.load("sprites/80s/tunnel_miner_gems/tunnel_miner_gems_000.png");
    let gold_handle: Handle<Image> =
        asset_server.load("sprites/80s/tunnel_miner_gems/tunnel_miner_gems_025.png");

    // Load Player Animations
    // Explicitly listing frames to avoid generated text artifacts (e.g., "Move", "Left")
    // Clean frames identified by file size (> 10KB)

    let mut player_anims = std::collections::HashMap::new();

    let load_specific_frames = |indices: &[usize]| -> Vec<Handle<Image>> {
        let mut frames = Vec::new();
        for &i in indices {
            let path = format!(
                "sprites/80s/tunnel_miner_player/tunnel_miner_player_{:03}.png",
                i
            );
            frames.push(asset_server.load(path));
        }
        frames
    };

    // Idle: 001, 002, 003 (000 is "Idle" text)
    player_anims.insert(AnimationFn::Idle, load_specific_frames(&[1, 2, 3]));

    // Right: 004, 006, 007 (005 is "Move" text)
    player_anims.insert(AnimationFn::MoveRight, load_specific_frames(&[4, 6, 7]));

    // Left: 008, 011 (009 is "Move", 010 is "Left")
    player_anims.insert(AnimationFn::MoveLeft, load_specific_frames(&[8, 11]));

    // Up: 012, 013, 014 (015 is "Up" text)
    player_anims.insert(AnimationFn::MoveUp, load_specific_frames(&[12, 13, 14]));

    // Down: 016, 017, 018, 019 (020 is "Move", 021 is "Down")
    player_anims.insert(
        AnimationFn::MoveDown,
        load_specific_frames(&[16, 17, 18, 19]),
    );

    // Load Enemy Animations (Nobbins)
    // Assuming Row 1 is idle/move (Index 0-3)
    let mut enemy_anims = std::collections::HashMap::new();
    let load_enemy_frames = |start: usize, count: usize| -> Vec<Handle<Image>> {
        let mut frames = Vec::new();
        for i in 0..count {
            let path = format!(
                "sprites/80s/tunnel_miner_enemy_nobbins/tunnel_miner_enemy_nobbins_{:03}.png",
                start + i
            );
            frames.push(asset_server.load(path));
        }
        frames
    };
    enemy_anims.insert(AnimationFn::MoveRight, load_enemy_frames(0, 4)); // Reusing for all moves for now
    enemy_anims.insert(AnimationFn::MoveLeft, load_enemy_frames(0, 4));
    enemy_anims.insert(AnimationFn::MoveUp, load_enemy_frames(0, 4));
    enemy_anims.insert(AnimationFn::MoveDown, load_enemy_frames(0, 4));
    enemy_anims.insert(AnimationFn::Idle, load_enemy_frames(0, 4));

    // Background
    commands.spawn((
        TunnelMinerEntity,
        Sprite {
            color: colors::EGA_BLACK, // Use solid black for clean retro look
            custom_size: Some(Vec2::new(
                GRID_WIDTH as f32 * TILE_SIZE + 40.0,
                GRID_HEIGHT as f32 * TILE_SIZE + 40.0,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Spawn earth and emeralds
    let mut emerald_count = 0u32;
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let world_x = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            // Skip player starting position for earth
            if x != GRID_WIDTH / 2 || y != GRID_HEIGHT - 1 {
                commands.spawn((
                    TunnelMinerEntity,
                    EarthTile,
                    GridPosition::new(x, y),
                    Sprite {
                        image: earth_handle.clone(),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 0.0),
                ));
            }

            // Spawn emerald if defined in map
            if MAP_80S_GEMS[y as usize][x as usize] == 1 {
                emerald_count += 1;
                commands.spawn((
                    TunnelMinerEntity,
                    Emerald,
                    GridPosition::new(x, y),
                    Sprite {
                        image: emerald_handle.clone(),
                        custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 2.0),
                ));
            }
        }
    }

    commands.insert_resource(TunnelMinerState {
        level: 1,
        emeralds_remaining: emerald_count,
        bonus_mode_active: false,
        bonus_mode_timer: Timer::from_seconds(10.0, TimerMode::Once),
        bonus_eat_score: 200,
        cherry_spawned: false,
    });

    // Place gold bags
    let gold_positions = [(3, 2), (11, 4), (7, 6)];
    for (gx, gy) in gold_positions {
        let world_x = origin_x + gx as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        let world_y = origin_y + gy as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        commands.spawn((
            TunnelMinerEntity,
            GoldBag {
                falling: false,
                fall_timer: Timer::from_seconds(0.5, TimerMode::Once),
                fall_distance: 0,
                is_pile: false,
            },
            GridPosition::new(gx, gy),
            Sprite {
                image: gold_handle.clone(),
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 2.0),
        ));
    }

    // Spawn player
    let player_x = origin_x + (GRID_WIDTH / 2) as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let player_y = origin_y + (GRID_HEIGHT - 1) as f32 * TILE_SIZE + TILE_SIZE / 2.0;

    let start_anim = AnimationFn::Idle;
    let initial_sprite = player_anims.get(&start_anim).unwrap()[0].clone();

    commands.spawn((
        TunnelMinerEntity,
        Player,
        TunnelMinerPlayer {
            facing: Direction::Right,
            move_timer: Timer::from_seconds(1.0 / MOVE_SPEED, TimerMode::Repeating),
            emerald_streak: 0,
            weapon_cooldown: Timer::from_seconds(5.0, TimerMode::Once),
        },
        AnimationState {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
            current_animation: start_anim,
        },
        AnimationFrames {
            animations: player_anims,
        },
        // Start weapon ready
        Weapon {
            active: true,
            recharge_timer: Timer::from_seconds(0.0, TimerMode::Once),
            position: Vec2::ZERO,
            direction: Direction::Right,
        },
        GridPosition::new(GRID_WIDTH / 2, GRID_HEIGHT - 1),
        Health::new(1),
        Sprite {
            image: initial_sprite,
            custom_size: Some(Vec2::new(TILE_SIZE * 0.9, TILE_SIZE * 0.9)),
            ..default()
        },
        Transform::from_xyz(player_x, player_y, 3.0),
    ));

    // Initial enemies (just one to start, spawner handles the rest)
    // Actually, let Spawner handle all of them?
    // Let's spawn 1 immediately so player isn't lonely
    /*
    let enemy_positions = [(13, 9)]; // Top right corner
    let enemy_start_anim = AnimationFn::Idle;
    let enemy_initial_sprite = enemy_anims.get(&enemy_start_anim).unwrap()[0].clone();

    for (ex, ey) in enemy_positions {
       // ... (Moved to spawner logic)
    }
    */

    commands.insert_resource(TunnelMinerState {
        emeralds_remaining: emerald_count as u32,
        level: 1,
        bonus_mode_active: false,
        bonus_mode_timer: Timer::from_seconds(10.0, TimerMode::Once),
        bonus_eat_score: 200,
        cherry_spawned: false,
    });

    // Spawn HUD
    spawn_hud(&mut commands);

    info!("Tunnel Miner: {} emeralds placed", emerald_count);
}

fn should_place_emerald(x: i32, y: i32) -> bool {
    // Deterministic pattern for emerald placement
    let hash = ((x * 31 + y * 17 + 7) % 5) as u32;
    hash == 0 || hash == 3
}

fn spawn_hud(commands: &mut Commands) {
    commands
        .spawn((
            TunnelMinerEntity,
            TunnelMinerHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|hud| {
            // Score display
            hud.spawn((
                HudScore,
                Text::new("Score: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_YELLOW),
            ));

            // Level display
            hud.spawn((
                Text::new("Level 1 — TUNNEL MINER"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_GREEN),
            ));

            // Lives display
            hud.spawn((
                HudLives,
                Text::new("Lives: ❤❤❤"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::EGA_RED),
            ));
        });
}

#[derive(Component)]
struct HudScore;

#[derive(Component)]
struct HudLives;

// ─── Systems ───────────────────────────────────────────────────────

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &AnimationFrames, &mut Sprite)>,
) {
    for (mut state, frames, mut sprite) in &mut query {
        state.timer.tick(time.delta());
        if state.timer.just_finished() {
            if let Some(anim_frames) = frames.animations.get(&state.current_animation) {
                if !anim_frames.is_empty() {
                    state.frame_index = (state.frame_index + 1) % anim_frames.len();
                    sprite.image = anim_frames[state.frame_index].clone();
                }
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (
            &mut TunnelMinerPlayer,
            &mut GridPosition,
            &mut Transform,
            &mut AnimationState,
        ),
        With<Player>,
    >,
    mut other_queries: ParamSet<(
        Query<(&mut GridPosition, &mut Transform, &GoldBag), (With<GoldBag>, Without<Player>)>,
        Query<&GridPosition, (With<EarthTile>, Without<Player>, Without<GoldBag>)>,
    )>,
) {
    // Index obstacles for push validation
    let earth_positions: std::collections::HashSet<(i32, i32)> =
        other_queries.p1().iter().map(|p| (p.x, p.y)).collect();
    let mut bag_positions: std::collections::HashSet<(i32, i32)> = other_queries
        .p0()
        .iter()
        .filter(|(_, _, b)| !b.is_pile)
        .map(|(p, _, _)| (p.x, p.y))
        .collect();

    for (mut player, mut grid_pos, mut transform, mut anim_state) in &mut player_query {
        player.move_timer.tick(time.delta());

        let mut moving = false;
        let mut target_anim = anim_state.current_animation;
        let mut target_facing = player.facing;
        let mut target_pos = *grid_pos;

        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            target_pos.x -= 1;
            target_facing = Direction::Left;
            target_anim = AnimationFn::MoveLeft;
            moving = true;
        } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            target_pos.x += 1;
            target_facing = Direction::Right;
            target_anim = AnimationFn::MoveRight;
            moving = true;
        } else if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            target_pos.y += 1;
            target_facing = Direction::Up;
            target_anim = AnimationFn::MoveUp;
            moving = true;
        } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            target_pos.y -= 1;
            target_facing = Direction::Down;
            target_anim = AnimationFn::MoveDown;
            moving = true;
        } else {
            target_anim = AnimationFn::Idle;
        }

        // Update animation state
        if anim_state.current_animation != target_anim {
            anim_state.current_animation = target_anim;
            anim_state.frame_index = 0; // Reset frame when changing animation
        }

        if !player.move_timer.just_finished() || !moving {
            return;
        }

        // Check bounds
        if target_pos.x < 0
            || target_pos.x >= GRID_WIDTH
            || target_pos.y < 0
            || target_pos.y >= GRID_HEIGHT
        {
            return; // Cannot move out of bounds
        }

        player.facing = target_facing;

        let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
        let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;

        if target_pos != *grid_pos {
            // Check if target is a bag to push
            if bag_positions.contains(&(target_pos.x, target_pos.y))
                && (target_facing == Direction::Left || target_facing == Direction::Right)
            {
                let mut push_success = false;
                let push_target = if target_facing == Direction::Left {
                    (target_pos.x - 1, target_pos.y)
                } else {
                    (target_pos.x + 1, target_pos.y)
                };

                // Check if push target is within bounds and not blocked by earth or another bag
                if push_target.0 >= 0
                    && push_target.0 < GRID_WIDTH
                    && push_target.1 >= 0
                    && push_target.1 < GRID_HEIGHT
                    && !earth_positions.contains(&push_target)
                    && !bag_positions.contains(&push_target)
                {
                    // Success! Update bag pos
                    for (mut b_grid_pos, mut b_transform, bag) in other_queries.p0().iter_mut() {
                        if b_grid_pos.x == target_pos.x
                            && b_grid_pos.y == target_pos.y
                            && !bag.is_pile
                            && !bag.falling
                        {
                            b_grid_pos.x = push_target.0;
                            b_grid_pos.y = push_target.1;
                            b_transform.translation.x =
                                origin_x + b_grid_pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                            b_transform.translation.y =
                                origin_y + b_grid_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                            push_success = true;
                            break;
                        }
                    }
                }

                if push_success {
                    *grid_pos = target_pos;
                    transform.translation.x =
                        origin_x + grid_pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                    transform.translation.y =
                        origin_y + grid_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                    player.move_timer.reset();
                }
            } else if !bag_positions.contains(&(target_pos.x, target_pos.y)) {
                // Simple move (earth digging handled separately or implicitly)
                *grid_pos = target_pos;
                transform.translation.x =
                    origin_x + grid_pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                transform.translation.y =
                    origin_y + grid_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                player.move_timer.reset();
            }
        }
    }
}

fn dig_system(
    mut commands: Commands,
    player_query: Query<&GridPosition, With<Player>>,
    earth_query: Query<(Entity, &GridPosition), With<EarthTile>>,
) {
    for player_pos in &player_query {
        for (entity, earth_pos) in &earth_query {
            if player_pos == earth_pos {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn collect_emeralds(
    mut commands: Commands,
    mut tunnel_state: ResMut<TunnelMinerState>,
    mut score: ResMut<Score>,
    mut player_query: Query<(&GridPosition, &mut TunnelMinerPlayer), With<Player>>,
    emerald_query: Query<(Entity, &GridPosition), With<Emerald>>,
) {
    if let Some((player_pos, mut player)) = player_query.iter_mut().next() {
        for (entity, emerald_pos) in &emerald_query {
            if player_pos == emerald_pos {
                commands.entity(entity).despawn();
                score.add(EMERALD_POINTS);
                tunnel_state.emeralds_remaining = tunnel_state.emeralds_remaining.saturating_sub(1);

                player.emerald_streak += 1;
                if player.emerald_streak >= 8 {
                    score.add(EMERALD_STREAK_BONUS);
                    player.emerald_streak = 0;
                    info!("8 emerald streak bonus! +{}", EMERALD_STREAK_BONUS);
                }
            }
        }
    }
}

fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawner>,
    asset_server: Res<AssetServer>,
) {
    spawner.timer.tick(time.delta());

    if spawner.timer.just_finished() && spawner.spawned_count < spawner.total_to_spawn {
        // Spawn Enemy at Top Right
        let ex = GRID_WIDTH - 2; // Slightly offset from very corner
        let ey = GRID_HEIGHT - 1;

        let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
        let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
        let world_x = origin_x + ex as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        let world_y = origin_y + ey as f32 * TILE_SIZE + TILE_SIZE / 2.0;

        // Load anims again? Optimization: Store anims in a Resource or shared logic
        // For now, re-load (cached by asset server)
        let mut enemy_anims = std::collections::HashMap::new();
        let load_enemy_frames = |start: usize, count: usize| -> Vec<Handle<Image>> {
            let mut frames = Vec::new();
            for i in 0..count {
                let path = format!(
                    "sprites/80s/tunnel_miner_enemy_nobbins/tunnel_miner_enemy_nobbins_{:03}.png",
                    start + i
                );
                frames.push(asset_server.load(path));
            }
            frames
        };
        enemy_anims.insert(AnimationFn::MoveRight, load_enemy_frames(0, 4));
        enemy_anims.insert(AnimationFn::MoveLeft, load_enemy_frames(0, 4));
        enemy_anims.insert(AnimationFn::MoveUp, load_enemy_frames(0, 4));
        enemy_anims.insert(AnimationFn::MoveDown, load_enemy_frames(0, 4));
        enemy_anims.insert(AnimationFn::Idle, load_enemy_frames(0, 4));

        commands.spawn((
            TunnelMinerEntity,
            Nobbin {
                hobbin: false,
                move_timer: Timer::from_seconds(0.4, TimerMode::Repeating), // A bit faster than player default?
                time_alive: 0.0,
            },
            AnimationState {
                timer: Timer::from_seconds(0.15, TimerMode::Repeating),
                frame_index: 0,
                current_animation: AnimationFn::Idle,
            },
            AnimationFrames {
                animations: enemy_anims,
            },
            GridPosition::new(ex, ey),
            Health::new(1),
            Sprite {
                image: asset_server.load(
                    "sprites/80s/tunnel_miner_enemy_nobbins/tunnel_miner_enemy_nobbins_000.png",
                ),
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 3.0),
        ));

        spawner.spawned_count += 1;
        spawner.active_count += 1;
    }
}

fn enemy_ai(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_params: ParamSet<(
        Query<
            (
                &mut Nobbin,
                &mut GridPosition,
                &mut Transform,
                &mut AnimationState,
                &mut Sprite,
            ),
            With<Nobbin>,
        >, // p0
        Query<&GridPosition, With<Player>>,              // p1
        Query<(Entity, &GridPosition), With<EarthTile>>, // p2
        Query<(Entity, &GridPosition), With<Emerald>>,   // p3
        Query<(Entity, &GridPosition), With<GoldBag>>,   // p4
    )>,
    tunnel_miner_state: Res<TunnelMinerState>,
) {
    let bonus_active = tunnel_miner_state.bonus_mode_active;

    // 1. Get player position
    let player_pos = match enemy_params.p1().iter().next() {
        Some(p) => *p,
        None => return, // No player, no AI
    };

    // 2. Pre-collect entities for Hobbin digging to avoid borrow conflicts
    let earth_tiles: Vec<(Entity, GridPosition)> =
        enemy_params.p2().iter().map(|(e, p)| (e, *p)).collect();
    let emeralds: Vec<(Entity, GridPosition)> =
        enemy_params.p3().iter().map(|(e, p)| (e, *p)).collect();
    let bags: Vec<(Entity, GridPosition)> =
        enemy_params.p4().iter().map(|(e, p)| (e, *p)).collect();

    // 3. Create spatial maps for lookups
    let earth_positions: std::collections::HashSet<(i32, i32)> =
        earth_tiles.iter().map(|(_, p)| (p.x, p.y)).collect();
    let emerald_positions: std::collections::HashSet<(i32, i32)> =
        emeralds.iter().map(|(_, p)| (p.x, p.y)).collect();
    let bag_positions: std::collections::HashSet<(i32, i32)> =
        bags.iter().map(|(_, p)| (p.x, p.y)).collect();

    // 4. Process each enemy
    let delta = time.delta();
    let delta_secs = time.delta_secs();
    let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
    let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;

    for (mut nobbin, mut grid_pos, mut transform, mut anim_state, mut sprite) in
        enemy_params.p0().iter_mut()
    {
        nobbin.move_timer.tick(delta);
        nobbin.time_alive += delta_secs;

        // Visual feedback
        if bonus_active {
            sprite.color = Color::srgba(0.5, 0.5, 1.0, 1.0); // Flee mode
        } else if nobbin.hobbin {
            sprite.color = Color::srgba(0.2, 1.0, 0.2, 1.0); // Hobbin
        } else {
            sprite.color = Color::WHITE;
        }

        // Transformation
        if !bonus_active && !nobbin.hobbin && nobbin.time_alive > 10.0 {
            nobbin.hobbin = true;
            info!("Nobbin transformed into Hobbin!");
        }

        if !nobbin.move_timer.just_finished() {
            continue;
        }

        // Simple movement AI
        let dx = player_pos.x - grid_pos.x;
        let dy = player_pos.y - grid_pos.y;

        let (target_dx, target_dy) = if bonus_active {
            (-dx.signum(), -dy.signum()) // Flee
        } else {
            (dx.signum(), dy.signum()) // Chase
        };

        let mut candidates = Vec::new();
        if target_dx != 0 {
            candidates.push((target_dx, 0));
        }
        if target_dy != 0 {
            candidates.push((0, target_dy));
        }

        for (odx, ody) in candidates {
            let next_x = grid_pos.x + odx;
            let next_y = grid_pos.y + ody;

            if next_x < 0 || next_x >= GRID_WIDTH || next_y < 0 || next_y >= GRID_HEIGHT {
                continue;
            }

            let is_earth = earth_positions.contains(&(next_x, next_y));
            let is_bag = bag_positions.contains(&(next_x, next_y));
            let is_emerald = emerald_positions.contains(&(next_x, next_y));

            let blocked = is_earth || is_bag || is_emerald;
            let can_dig = nobbin.hobbin && !bonus_active;

            if !blocked || can_dig {
                // Perform Digging
                if blocked && can_dig {
                    if is_earth {
                        if let Some((ent, _)) = earth_tiles
                            .iter()
                            .find(|(_, p)| p.x == next_x && p.y == next_y)
                        {
                            commands.entity(*ent).despawn();
                        }
                    }
                    if is_emerald {
                        if let Some((ent, _)) = emeralds
                            .iter()
                            .find(|(_, p)| p.x == next_x && p.y == next_y)
                        {
                            commands.entity(*ent).despawn();
                        }
                    }
                    if is_bag {
                        if let Some((ent, _)) =
                            bags.iter().find(|(_, p)| p.x == next_x && p.y == next_y)
                        {
                            commands.entity(*ent).despawn();
                        }
                    }
                }

                // Actually move
                grid_pos.x = next_x;
                grid_pos.y = next_y;
                transform.translation.x =
                    origin_x + grid_pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                transform.translation.y =
                    origin_y + grid_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

                // Set animation
                anim_state.current_animation = match (odx, ody) {
                    (1, _) => AnimationFn::MoveRight,
                    (-1, _) => AnimationFn::MoveLeft,
                    (_, 1) => AnimationFn::MoveUp,
                    (_, -1) => AnimationFn::MoveDown,
                    _ => anim_state.current_animation,
                };

                break;
            }
        }
        nobbin.move_timer.reset();
    }
}

fn gold_bag_physics(
    mut commands: Commands,
    time: Res<Time>,
    mut bag_query: Query<(Entity, &mut GoldBag, &mut GridPosition, &mut Transform)>,
    earth_query: Query<&GridPosition, (With<EarthTile>, Without<GoldBag>)>,
    player_query: Query<(Entity, &GridPosition), (With<Player>, Without<GoldBag>)>,
    enemy_query: Query<(Entity, &GridPosition), (With<Nobbin>, Without<GoldBag>)>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    // Collect earth positions for collision check
    let earth_positions: std::collections::HashSet<(i32, i32)> =
        earth_query.iter().map(|p| (p.x, p.y)).collect();

    // Map bag positions to check for stacking
    let bag_positions: std::collections::HashSet<(i32, i32)> =
        bag_query.iter().map(|(_, _, p, _)| (p.x, p.y)).collect();

    for (entity, mut bag, mut grid_pos, mut transform) in &mut bag_query {
        // If it's already a pile, it doesn't fall or crush, it just waits to be collected
        if bag.is_pile {
            // Check collision with player for collection
            if let Some((_, player_pos)) = player_query.iter().next() {
                if *player_pos == *grid_pos {
                    score.add(GOLD_PILE_POINTS);
                    commands.entity(entity).despawn();
                    info!("Gold pile collected! +{}", GOLD_PILE_POINTS);
                }
            }
            continue;
        }

        // Logic for falling
        let below_pos = (grid_pos.x, grid_pos.y - 1);

        // Check what is below
        let is_earth_below = earth_positions.contains(&below_pos);
        let is_bag_below = bag_positions.contains(&below_pos);
        let is_bottom = below_pos.1 < 0;

        if !is_earth_below && !is_bag_below && !is_bottom {
            // Empty space below, start/continue falling
            if !bag.falling {
                // Just started falling logic (wobble time?)
                // For now, immediate fall after short delay
                bag.fall_timer.tick(time.delta());
                if bag.fall_timer.just_finished() {
                    bag.falling = true;
                    bag.fall_timer = Timer::from_seconds(0.2, TimerMode::Repeating);
                    // faster fall
                }
            } else {
                bag.fall_timer.tick(time.delta());
                if bag.fall_timer.just_finished() {
                    // Move down
                    grid_pos.y -= 1;
                    bag.fall_distance += 1;

                    // Update transform
                    let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
                    let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
                    transform.translation.y =
                        origin_y + grid_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

                    // Check crushing
                    // Player
                    if let Some((_, player_pos)) = player_query.iter().next() {
                        if *player_pos == *grid_pos {
                            // Kill player
                        }
                    }

                    // Enemy
                    for (enemy_entity, enemy_pos) in &enemy_query {
                        if *enemy_pos == *grid_pos {
                            commands.entity(enemy_entity).despawn();
                            score.add(ENEMY_CRUSH_POINTS);
                            info!("Enemy crushed! +{}", ENEMY_CRUSH_POINTS);
                        }
                    }
                }
            }
        } else {
            // Landed
            if bag.falling {
                bag.falling = false;
                bag.fall_timer = Timer::from_seconds(0.5, TimerMode::Once); // Reset for next fall

                if bag.fall_distance > 1 {
                    // Break open
                    bag.is_pile = true;
                    // Change sprite to gold pile
                    commands.entity(entity).insert(Sprite {
                        image: asset_server
                            .load("sprites/80s/tunnel_miner_gems/tunnel_miner_gems_025.png"),
                        color: Color::srgba(1.0, 0.8, 0.0, 1.0), // Gold tint
                        custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                        ..default()
                    });
                    info!("Gold bag broke open!");
                }
                bag.fall_distance = 0;
            }
        }
    }
}

fn weapon_system(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut TunnelMinerPlayer, &GridPosition, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut player, grid_pos, transform)) = player_query.single_mut() {
        player.weapon_cooldown.tick(time.delta());

        if keyboard.just_pressed(KeyCode::F1) && player.weapon_cooldown.remaining_secs() <= 0.0 {
            // Fire!
            player.weapon_cooldown.reset();

            let spawn_pos = transform.translation;
            let direction = player.facing;

            // Spawn Fireball
            commands.spawn((
                TunnelMinerEntity,
                Fireball {
                    direction,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                },
                GridPosition::new(grid_pos.x, grid_pos.y), // Start at player pos
                Sprite {
                    image: asset_server
                        .load("sprites/80s/tunnel_miner_gems/tunnel_miner_gems_005.png"),
                    color: Color::srgba(1.0, 0.5, 0.0, 1.0), // Orange glow
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.4, TILE_SIZE * 0.4)),
                    ..default()
                },
                Transform::from_xyz(spawn_pos.x, spawn_pos.y, 4.0),
            ));
            info!("Fired weapon!");
        }
    }
}

fn fireball_collision(
    mut commands: Commands,
    time: Res<Time>,
    mut fireball_query: Query<
        (Entity, &mut Fireball, &mut GridPosition, &mut Transform),
        Without<Nobbin>,
    >,
    enemy_query: Query<(Entity, &GridPosition), (With<Nobbin>, Without<Fireball>)>,
    mut score: ResMut<Score>,
) {
    for (fb_entity, mut fireball, mut fb_pos, mut fb_transform) in &mut fireball_query {
        fireball.timer.tick(time.delta());

        if fireball.timer.just_finished() {
            // Move fireball
            let mut new_pos = *fb_pos;
            match fireball.direction {
                Direction::Up => new_pos.y += 1,
                Direction::Down => new_pos.y -= 1,
                Direction::Left => new_pos.x -= 1,
                Direction::Right => new_pos.x += 1,
            }

            // Check bounds/walls (simple check)
            if new_pos.x < 0 || new_pos.x >= GRID_WIDTH || new_pos.y < 0 || new_pos.y >= GRID_HEIGHT
            {
                commands.entity(fb_entity).despawn();
                continue;
            }

            *fb_pos = new_pos;
            let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
            let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
            fb_transform.translation.x = origin_x + new_pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            fb_transform.translation.y = origin_y + new_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        }

        // Check enemy collision
        for (enemy_entity, enemy_pos) in &enemy_query {
            if *fb_pos == *enemy_pos {
                commands.entity(enemy_entity).despawn();
                commands.entity(fb_entity).despawn();
                score.add(ENEMY_KILL_POINTS);
                info!("Enemy shot! +{}", ENEMY_KILL_POINTS);
                break; // One kill per fireball usually
            }
        }
    }
}

fn cherry_system(
    mut commands: Commands,
    mut tunnel_state: ResMut<TunnelMinerState>,
    spawner: Res<EnemySpawner>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // Spawn Cherry logic: When timer finishes OR when all enemies spawned (let's say all spawned for now)
    if !tunnel_state.cherry_spawned && spawner.spawned_count >= spawner.total_to_spawn {
        // Determine spawn pos (Top Right usually)
        let cx = GRID_WIDTH - 1;
        let cy = GRID_HEIGHT - 1;
        let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
        let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
        let world_x = origin_x + cx as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        let world_y = origin_y + cy as f32 * TILE_SIZE + TILE_SIZE / 2.0;

        commands.spawn((
            TunnelMinerEntity,
            Cherry,
            GridPosition::new(cx, cy),
            Sprite {
                image: asset_server.load("sprites/80s/tunnel_miner_gems/tunnel_miner_gems_025.png"),
                color: Color::srgba(1.0, 0.0, 0.3, 1.0), // Cherry Red
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 2.0),
        ));
        tunnel_state.cherry_spawned = true;
        info!("Cherry Spawned!");
    }

    // Bonus Mode Logic
    if tunnel_state.bonus_mode_active {
        tunnel_state.bonus_mode_timer.tick(time.delta());
        if tunnel_state.bonus_mode_timer.remaining_secs() <= 0.0 {
            tunnel_state.bonus_mode_active = false;
            info!("Bonus Mode Ended.");
        }
    }
}

fn collect_cherry(
    mut commands: Commands,
    mut tunnel_state: ResMut<TunnelMinerState>,
    mut score: ResMut<Score>,
    player_query: Query<&GridPosition, With<Player>>,
    cherry_query: Query<(Entity, &GridPosition), With<Cherry>>,
) {
    if let Ok(player_pos) = player_query.single() {
        for (entity, cherry_pos) in &cherry_query {
            if *player_pos == *cherry_pos {
                commands.entity(entity).despawn();
                score.add(CHERRY_POINTS);

                // Activate Bonus Mode
                tunnel_state.bonus_mode_active = true;
                tunnel_state.bonus_mode_timer.reset(); // Reset to 10s
                tunnel_state.bonus_eat_score = 200;

                info!("Collected Cherry! Bonus Mode Active! +{}", CHERRY_POINTS);
            }
        }
    }
}

fn level_progression(
    mut tunnel_state: ResMut<TunnelMinerState>,
    spawner: Res<EnemySpawner>,
    enemy_query: Query<&Nobbin>,
    mut next_state: ResMut<NextState<GameState>>,
    mut results: ResMut<GameResults>,
    score: Res<Score>,
    progress: Res<PlayerProgress>,
) {
    if (tunnel_state.emeralds_remaining == 0 && tunnel_state.emeralds_remaining != 999)
        || (spawner.spawned_count >= spawner.total_to_spawn && enemy_query.iter().count() == 0)
    {
        // Level Complete!
        let game_id = MiniGameId {
            era: Era::The80s,
            index: 0,
        };

        results.game_name = "Tunnel Miner".to_string();
        results.score = score.value;
        results.high_score = progress.high_score(game_id);
        results.is_new_high = score.value > results.high_score;

        if tunnel_state.emeralds_remaining == 0 {
            info!(
                "All emeralds collected! Level {} complete.",
                tunnel_state.level
            );
        } else {
            info!(
                "All enemies defeated! Level {} complete.",
                tunnel_state.level
            );
        }

        tunnel_state.emeralds_remaining = 999; // Prevent multiple triggers
        next_state.set(GameState::Results);
    }
}

fn check_death(
    mut commands: Commands,
    player_query: Query<&GridPosition, With<Player>>,
    enemy_query: Query<(Entity, &GridPosition), With<Nobbin>>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut results: ResMut<crate::ui::results::GameResults>,
    progress: Res<PlayerProgress>,
    mut tunnel_state: ResMut<TunnelMinerState>,
) {
    let player_pos = match player_query.iter().next() {
        Some(p) => p,
        None => return,
    };

    for (enemy_entity, enemy_pos) in &enemy_query {
        if player_pos == enemy_pos {
            if tunnel_state.bonus_mode_active {
                // Eat Enemy!
                commands.entity(enemy_entity).despawn();
                score.add(tunnel_state.bonus_eat_score);
                info!("Ate Enemy! +{}", tunnel_state.bonus_eat_score);
                tunnel_state.bonus_eat_score *= 2; // Double points for next eat
            } else {
                // Die!
                lives.lose_one();
                info!("Player hit! Lives remaining: {}", lives.count);

                if lives.is_game_over() {
                    info!("Game Over! Final score: {}", score.value);

                    // Populate results
                    let game_id = crate::core::states::MiniGameId {
                        era: crate::core::states::Era::The80s,
                        index: 0,
                    };
                    results.game_name = "Tunnel Miner".to_string();
                    results.score = score.value;
                    results.high_score = progress.high_score(game_id);
                    results.is_new_high = score.value > results.high_score;

                    next_state.set(GameState::Results);
                }
            }
            return;
        }
    }
}

fn update_hud(
    score: Res<Score>,
    lives: Res<Lives>,
    mut score_text: Query<&mut Text, (With<HudScore>, Without<HudLives>)>,
    mut lives_text: Query<&mut Text, (With<HudLives>, Without<HudScore>)>,
) {
    for mut text in &mut score_text {
        **text = format!("Score: {}", score.value);
    }

    for mut text in &mut lives_text {
        let hearts: String = (0..lives.count).map(|_| '❤').collect();
        **text = format!("Lives: {}", hearts);
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    score: Res<Score>,
    mut results: ResMut<GameResults>,
    progress: Res<PlayerProgress>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let game_id = MiniGameId {
            era: Era::The80s,
            index: 0,
        };
        results.game_name = "Tunnel Miner".to_string();
        results.score = score.value;
        results.high_score = progress.high_score(game_id);

        next_state.set(GameState::Results);
    }
}

// ─── Cleanup ───────────────────────────────────────────────────────

fn cleanup_tunnel_miner(mut commands: Commands, query: Query<Entity, With<TunnelMinerEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<TunnelMinerState>();
}
