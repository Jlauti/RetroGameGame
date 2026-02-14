use bevy::prelude::*;
use crate::core::states::{GameState, PlayingState};
use crate::shared::components::{GridPosition, Health, Player, Score};
use crate::ui::colors;
use crate::ui::results::GameResults;
use std::collections::HashMap;

/// Depths of Doom — inspired by ADOM (1994).
/// Turn-based roguelike RPG with procedural dungeons.
pub struct DepthsOfDoomPlugin;

impl Plugin for DepthsOfDoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DungeonMap>()
            .add_systems(
                OnEnter(GameState::Playing),
                setup_dungeon.run_if(in_state(PlayingState::DepthsOfDoom)),
            )
            .add_systems(
                Update,
                (
                    handle_input,
                    monster_ai,
                    check_death,
                    update_hud,
                    handle_pause,
                )
                    .run_if(in_state(PlayingState::DepthsOfDoom)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_dungeon);
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const MAP_WIDTH: i32 = 40;
const MAP_HEIGHT: i32 = 30;
const TILE_SIZE: f32 = 24.0;

// ─── Components ───────────────────────────────────────────────────

#[derive(Component)]
struct DoomEntity;

#[derive(Component)]
struct Monster;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct DungeonHud;

// ─── Resources ─────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct DungeonMap {
    tiles: HashMap<(i32, i32), TileType>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileType {
    Floor,
    Wall,
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_dungeon(mut commands: Commands, mut map: ResMut<DungeonMap>) {
    info!("Generating Depths of Doom...");

    map.tiles.clear();

    // Simple procedural dungeon: random walls
    for x in -20..20 {
        for y in -15..15 {
            let is_wall = (x == -20 || x == 19 || y == -15 || y == 14) || (rand::random::<f32>() < 0.15);
            let tile = if is_wall { TileType::Wall } else { TileType::Floor };
            map.tiles.insert((x, y), tile);

            let world_pos = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);

            if is_wall {
                commands.spawn((
                    DoomEntity,
                    Wall,
                    GridPosition::new(x, y),
                    Sprite {
                        color: Color::srgb(0.3, 0.2, 0.2),
                        custom_size: Some(Vec2::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                        ..default()
                    },
                    Transform::from_translation(world_pos),
                ));
            } else {
                 commands.spawn((
                    DoomEntity,
                    Sprite {
                        color: Color::srgb(0.1, 0.1, 0.1),
                        custom_size: Some(Vec2::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                        ..default()
                    },
                    Transform::from_translation(world_pos.with_z(-0.1)),
                ));
            }
        }
    }

    // Spawn Player
    commands.spawn((
        DoomEntity,
        Player,
        GridPosition::new(0, 0),
        Health::new(100),
        Sprite {
            color: colors::CGA_BRIGHT_CYAN,
            custom_size: Some(Vec2::new(TILE_SIZE * 0.8, TILE_SIZE * 0.8)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    // Spawn Monsters
    for i in 0..5 {
        let mx = (rand::random::<f32>() * 30.0 - 15.0) as i32;
        let my = (rand::random::<f32>() * 20.0 - 10.0) as i32;

        if map.tiles.get(&(mx, my)) == Some(&TileType::Floor) {
            commands.spawn((
                DoomEntity,
                Monster,
                GridPosition::new(mx, my),
                Health::new(20),
                Sprite {
                    color: colors::CGA_RED,
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.7, TILE_SIZE * 0.7)),
                    ..default()
                },
                Transform::from_xyz(mx as f32 * TILE_SIZE, my as f32 * TILE_SIZE, 1.0),
            ));
        }
    }

    spawn_doom_hud(&mut commands);
}

fn spawn_doom_hud(commands: &mut Commands) {
    commands
        .spawn((
            DoomEntity,
            DungeonHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|hud| {
            hud.spawn((
                DoomHpText,
                Text::new("HP: 100/100"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(colors::CGA_BRIGHT_GREEN),
            ));
            hud.spawn((
                Text::new("DEPTHS OF DOOM — B1"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(colors::TEXT_SECONDARY),
            ));
        });
}

#[derive(Component)]
struct DoomHpText;

// ─── Systems ───────────────────────────────────────────────────────

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    map: Res<DungeonMap>,
    mut player_query: Query<(&mut GridPosition, &mut Transform, &Health), (With<Player>, Without<Monster>)>,
    mut monster_query: Query<(Entity, &GridPosition, &mut Health), (With<Monster>, Without<Player>)>,
    mut commands: Commands,
) {
    let mut move_delta = (0, 0);

    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) { move_delta.1 += 1; }
    else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) { move_delta.1 -= 1; }
    else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) { move_delta.0 -= 1; }
    else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) { move_delta.0 += 1; }

    if move_delta == (0, 0) { return; }

    let (mut pos, mut transform, _hp) = match player_query.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let target_pos = (pos.x + move_delta.0, pos.y + move_delta.1);

    // Check for combat
    for (m_entity, m_pos, mut m_hp) in &mut monster_query {
        if (m_pos.x, m_pos.y) == target_pos {
            info!("Bump! Damage dealt.");
            m_hp.damage(10);
            if m_hp.is_dead() {
                commands.entity(m_entity).despawn();
            }
            return; // Attack ends the turn move
        }
    }

    // Check for walls
    if map.tiles.get(&target_pos) == Some(&TileType::Floor) {
        pos.x = target_pos.0;
        pos.y = target_pos.1;
        transform.translation.x = pos.x as f32 * TILE_SIZE;
        transform.translation.y = pos.y as f32 * TILE_SIZE;
    }
}

fn monster_ai(
    player_query: Query<&GridPosition, (With<Player>, Without<Monster>)>,
    mut monster_query: Query<(&mut GridPosition, &mut Transform), (With<Monster>, Without<Player>)>,
    map: Res<DungeonMap>,
) {
    // Basic chase AI if turn-based was implemented with a turn resource.
    // For now, moves randomly on update (simplified).
}

fn check_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(hp) = player_query.single() {
        if hp.is_dead() {
            next_state.set(GameState::Results);
        }
    }
}

fn update_hud(
    player_query: Query<&Health, With<Player>>,
    mut hp_text: Query<&mut Text, With<DoomHpText>>,
) {
    if let Ok(hp) = player_query.single() {
        for mut text in &mut hp_text {
            **text = format!("HP: {}/{}", hp.current, hp.max);
        }
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut results: ResMut<GameResults>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        results.game_name = "Depths of Doom".to_string();
        next_state.set(GameState::Results);
    }
}

fn cleanup_dungeon(mut commands: Commands, query: Query<Entity, With<DoomEntity>>) {
    for entity in &query { commands.entity(entity).despawn(); }
}
