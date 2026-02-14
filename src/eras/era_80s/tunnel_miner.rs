use bevy::prelude::*;

use crate::core::progression::PlayerProgress;
use crate::core::states::{GameState, PlayingState, Era, MiniGameId};
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
                dig_system,
                collect_emeralds,
                enemy_ai,
                gold_bag_physics,
                weapon_system,
                check_death,
                update_hud,
                handle_pause,
            )
                .run_if(in_state(PlayingState::TunnelMiner)),
        )
        .add_systems(
            OnExit(GameState::Playing),
            cleanup_tunnel_miner,
        );
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
const ENEMY_CRUSH_POINTS: u64 = 100;
const COMPLETION_THRESHOLD: u64 = 1000;

// ─── Components ────────────────────────────────────────────────────

#[derive(Component)]
struct TunnelMinerEntity; // Marker for cleanup

#[derive(Component)]
struct TunnelMinerPlayer {
    facing: Direction,
    move_timer: Timer,
    emerald_streak: u32,
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
}

#[derive(Component)]
struct Nobbin {
    hobbin: bool, // transformed into a Hobbin?
    move_timer: Timer,
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
    cherry_mode: bool,
    cherry_timer: Timer,
    emeralds_remaining: u32,
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_tunnel_miner(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up Tunnel Miner level...");

    // Initialize game state
    commands.insert_resource(TunnelMinerState {
        level: 1,
        cherry_mode: false,
        cherry_timer: Timer::from_seconds(10.0, TimerMode::Once),
        emeralds_remaining: 0,
    });

    commands.insert_resource(Score::default());
    commands.insert_resource(Lives::new(3));

    // Grid origin (center the grid on screen)
    let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
    let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;

    // Load textures
    let background_handle: Handle<Image> = asset_server.load("sprites/era_80s/tunnel_miner/background.png");
    let earth_handle: Handle<Image> = asset_server.load("sprites/era_80s/tunnel_miner/earth.png");
    let emerald_handle: Handle<Image> = asset_server.load("sprites/era_80s/tunnel_miner/emerald.png");
    let gold_handle: Handle<Image> = asset_server.load("sprites/era_80s/tunnel_miner/gold_bag.png");
    let player_handle: Handle<Image> = asset_server.load("sprites/era_80s/tunnel_miner/player.png");
    let nobbin_handle: Handle<Image> = asset_server.load("sprites/era_80s/tunnel_miner/nobbin.png");

    // Background
    commands.spawn((
        TunnelMinerEntity,
        Sprite {
            color: colors::CGA_BLACK,
            custom_size: Some(Vec2::new(
                GRID_WIDTH as f32 * TILE_SIZE + 40.0,
                GRID_HEIGHT as f32 * TILE_SIZE + 40.0,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Spawn earth tiles (the grid)
    let mut emerald_count = 0u32;
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let world_x = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            // Skip player starting position
            if x == GRID_WIDTH / 2 && y == GRID_HEIGHT - 1 {
                continue;
            }

            // Determine tile type
            let is_emerald = should_place_emerald(x, y);

            // Earth tile
            commands.spawn((
                TunnelMinerEntity,
                EarthTile,
                GridPosition::new(x, y),
                Sprite {
                    image: earth_handle.clone(),
                    color: Color::srgb(0.55, 0.35, 0.15),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
            ));

            // Emerald on top of earth
            if is_emerald {
                emerald_count += 1;
                commands.spawn((
                    TunnelMinerEntity,
                    Emerald,
                    GridPosition::new(x, y),
                    Sprite {
                        image: emerald_handle.clone(),
                        color: colors::CGA_BRIGHT_GREEN,
                        custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                        ..default()
                    },
                    Transform::from_xyz(world_x, world_y, 1.0),
                ));
            }
        }
    }

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
            },
            GridPosition::new(gx, gy),
            Sprite {
                image: gold_handle.clone(),
                color: colors::CGA_YELLOW,
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 2.0),
        ));
    }

    // Spawn player
    let player_x = origin_x + (GRID_WIDTH / 2) as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    let player_y = origin_y + (GRID_HEIGHT - 1) as f32 * TILE_SIZE + TILE_SIZE / 2.0;
    commands.spawn((
        TunnelMinerEntity,
        Player,
        TunnelMinerPlayer {
            facing: Direction::Right,
            move_timer: Timer::from_seconds(1.0 / MOVE_SPEED, TimerMode::Repeating),
            emerald_streak: 0,
        },
        GridPosition::new(GRID_WIDTH / 2, GRID_HEIGHT - 1),
        Health::new(1),
        Sprite {
            image: player_handle.clone(),
            custom_size: Some(Vec2::new(TILE_SIZE * 0.9, TILE_SIZE * 0.9)),
            ..default()
        },
        Transform::from_xyz(player_x, player_y, 3.0),
    ));

    // Spawn enemies (Nobbins)
    let enemy_positions = [(2, 1), (12, 1)];
    for (ex, ey) in enemy_positions {
        let world_x = origin_x + ex as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        let world_y = origin_y + ey as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        commands.spawn((
            TunnelMinerEntity,
            Nobbin {
                hobbin: false,
                move_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            },
            GridPosition::new(ex, ey),
            Health::new(1),
            Sprite {
                image: nobbin_handle.clone(),
                color: colors::CGA_RED,
                custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(world_x, world_y, 3.0),
        ));
    }

    commands.insert_resource(TunnelMinerState {
        emeralds_remaining: emerald_count as u32,
        level: 1,
        cherry_mode: false,
        cherry_timer: Timer::from_seconds(10.0, TimerMode::Once),
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
                TextColor(colors::CGA_BRIGHT_YELLOW),
            ));

            // Level display
            hud.spawn((
                Text::new("Level 1 — TUNNEL MINER"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::CGA_BRIGHT_GREEN),
            ));

            // Lives display
            hud.spawn((
                HudLives,
                Text::new("Lives: ❤❤❤"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::CGA_RED),
            ));
        });
}

#[derive(Component)]
struct HudScore;

#[derive(Component)]
struct HudLives;

// ─── Systems ───────────────────────────────────────────────────────

fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<
        (&mut TunnelMinerPlayer, &mut GridPosition, &mut Transform),
        With<Player>,
    >,
) {
    for (mut player, mut grid_pos, mut transform) in &mut player_query {
        player.move_timer.tick(time.delta());

        if !player.move_timer.just_finished() {
            return;
        }

        let mut moved = false;
        let mut new_pos = *grid_pos;

        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            new_pos.y += 1;
            player.facing = Direction::Up;
            moved = true;
        } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            new_pos.y -= 1;
            player.facing = Direction::Down;
            moved = true;
        } else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            new_pos.x -= 1;
            player.facing = Direction::Left;
            moved = true;
        } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            new_pos.x += 1;
            player.facing = Direction::Right;
            moved = true;
        }

        if moved
            && new_pos.x >= 0
            && new_pos.x < GRID_WIDTH
            && new_pos.y >= 0
            && new_pos.y < GRID_HEIGHT
        {
            *grid_pos = new_pos;
            let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
            let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
            transform.translation.x = origin_x + new_pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            transform.translation.y = origin_y + new_pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            player.move_timer.reset();
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
                // The user's edit was syntactically incorrect and introduced an undefined variable.
                // Assuming the intent was to add a condition related to a timer or event,
                // but without further context, the original logic is preserved to maintain
                // a syntactically correct and functional file.
                // The instruction "Fix TunnelMinerState init and update Timer/EventReader usage in transitions.rs."
                // does not directly apply to this specific code block in a way that makes the provided
                // snippet valid here.
                commands.entity(entity).despawn();
            }
        }
    }
}

fn collect_emeralds(
    mut commands: Commands,
    player_query: Query<&GridPosition, With<Player>>,
    emerald_query: Query<(Entity, &GridPosition), With<Emerald>>,
    mut score: ResMut<Score>,
    mut player_state: Query<&mut TunnelMinerPlayer>,
) {
    for player_pos in &player_query {
        for (entity, emerald_pos) in &emerald_query {
            if player_pos == emerald_pos {
                commands.entity(entity).despawn();
                score.add(EMERALD_POINTS);

                if let Ok(mut player) = player_state.single_mut() {
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
}

fn enemy_ai(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Nobbin, &mut GridPosition, &mut Transform), (With<Nobbin>, Without<Player>, Without<EarthTile>)>,
    player_query: Query<&GridPosition, (With<Player>, Without<Nobbin>)>,
    earth_query: Query<&GridPosition, (With<EarthTile>, Without<Nobbin>)>,
) {
    let player_pos = match player_query.iter().next() {
        Some(p) => *p,
        None => return,
    };

    // Collect earth positions for pathfinding
    let earth_positions: std::collections::HashSet<(i32, i32)> = earth_query
        .iter()
        .map(|p| (p.x, p.y))
        .collect();

    for (mut nobbin, mut grid_pos, mut transform) in &mut enemy_query {
        nobbin.move_timer.tick(time.delta());
        if !nobbin.move_timer.just_finished() {
            continue;
        }
        nobbin.move_timer.reset();

        // Simple chase AI: move towards player
        let dx = player_pos.x - grid_pos.x;
        let dy = player_pos.y - grid_pos.y;

        let mut candidates = Vec::new();

        // Prefer the axis with larger distance
        if dx.abs() >= dy.abs() {
            if dx != 0 {
                candidates.push(GridPosition::new(
                    grid_pos.x + dx.signum(),
                    grid_pos.y,
                ));
            }
            if dy != 0 {
                candidates.push(GridPosition::new(
                    grid_pos.x,
                    grid_pos.y + dy.signum(),
                ));
            }
        } else {
            if dy != 0 {
                candidates.push(GridPosition::new(
                    grid_pos.x,
                    grid_pos.y + dy.signum(),
                ));
            }
            if dx != 0 {
                candidates.push(GridPosition::new(
                    grid_pos.x + dx.signum(),
                    grid_pos.y,
                ));
            }
        }

        // Nobbins only move through tunnels (no earth)
        // Hobbins can dig through earth
        for candidate in candidates {
            if candidate.x >= 0
                && candidate.x < GRID_WIDTH
                && candidate.y >= 0
                && candidate.y < GRID_HEIGHT
            {
                let blocked = earth_positions.contains(&(candidate.x, candidate.y));
                if !blocked || nobbin.hobbin {
                    *grid_pos = candidate;
                    let origin_x = -(GRID_WIDTH as f32 * TILE_SIZE) / 2.0;
                    let origin_y = -(GRID_HEIGHT as f32 * TILE_SIZE) / 2.0;
                    transform.translation.x =
                        origin_x + candidate.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                    transform.translation.y =
                        origin_y + candidate.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                    break;
                }
            }
        }
    }
}

fn gold_bag_physics(
    // TODO: Re-implement gold bag falling/crushing logic.
    // Stubbed out to resolve Bevy query conflict panic at startup.
) {
}

fn weapon_system(
    // Placeholder for weapon firing (single-shot with recharge)
    // Will be implemented with sprite-based projectile
) {
}

fn check_death(
    player_query: Query<&GridPosition, With<Player>>,
    enemy_query: Query<&GridPosition, With<Nobbin>>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GameState>>,
    score: Res<Score>,
    mut results: ResMut<crate::ui::results::GameResults>,
    progress: Res<PlayerProgress>,
) {
    let player_pos = match player_query.iter().next() {
        Some(p) => p,
        None => return,
    };

    for enemy_pos in &enemy_query {
        if player_pos == enemy_pos {
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

fn cleanup_tunnel_miner(
    mut commands: Commands,
    query: Query<Entity, With<TunnelMinerEntity>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<TunnelMinerState>();
}
