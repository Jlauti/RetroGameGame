use crate::core::states::{GameState, PlayingState};
use crate::shared::components::{Health, Player, Velocity};
use crate::ui::colors;
use crate::ui::results::GameResults;
use bevy::prelude::*;

/// Star Goose — inspired by Star Goose (1988).
/// Vertical-scrolling shooter with resource management.
pub struct StarGoosePlugin;

impl Plugin for StarGoosePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarGooseResources>()
            .init_resource::<TunnelState>()
            .add_systems(
                OnEnter(GameState::Playing),
                setup_star_goose.run_if(in_state(PlayingState::StarGoose)),
            )
            .add_systems(
                Update,
                (
                    ship_movement,
                    ship_shoot,
                    missile_movement,
                    scroll_tunnel,
                    wall_collision,
                    combat_collisions,
                    resource_drain,
                    collection_system,
                    enemy_system,
                    update_hud,
                    handle_pause,
                )
                    .run_if(in_state(PlayingState::StarGoose)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_star_goose);
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const SCROLL_SPEED: f32 = 120.0;
const SHIP_SPEED: f32 = 250.0;
const MISSILE_SPEED: f32 = 500.0;
const BLOCK_SIZE: f32 = 64.0; // Larger blocks for the "chunky" look
const TUNNEL_WIDTH_BLOCKS: i32 = 8; // Inner width in blocks

// ─── Components ───────────────────────────────────────────────────

#[derive(Component)]
struct StarGooseEntity;

#[derive(Component)]
struct Crystal;

#[derive(Component)]
struct FuelPod;

#[derive(Component)]
struct Missile;

#[derive(Component)]
struct WallBlock;

#[derive(Component)]
struct TunnelSegment;

#[derive(Component)]
struct StarGooseHud;

#[derive(Component)]
struct HudText {
    field: HudField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HudField {
    Fuel,
    Ammo,
    Shield,
    Score,
}

#[derive(Component)]
struct Enemy {
    enemy_type: EnemyType,
    health: i32,
}

#[derive(Clone, Copy, PartialEq)]
enum EnemyType {
    Mine,
    Chaser,
}

// ─── Resources ─────────────────────────────────────────────────────

#[derive(Resource)]
struct StarGooseResources {
    fuel: f32,
    ammo: u32,
    shield: u32,
    score: u32,
}

impl Default for StarGooseResources {
    fn default() -> Self {
        Self {
            fuel: 100.0,
            ammo: 50,
            shield: 100,
            score: 0,
        }
    }
}

#[derive(Resource)]
struct TunnelState {
    next_y: f32,
    center_x_index: i32, // In block units, 0 is center
    steps_until_turn: i32,
    current_direction: i32, // -1, 0, 1
}

impl Default for TunnelState {
    fn default() -> Self {
        Self {
            next_y: -400.0,
            center_x_index: 0,
            steps_until_turn: 5,
            current_direction: 0,
        }
    }
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_star_goose(mut commands: Commands) {
    info!("Setting up Star Goose...");

    commands.insert_resource(StarGooseResources::default());

    let mut tunnel_state = TunnelState::default();
    tunnel_state.next_y = -400.0;

    // Pre-spawn tunnel
    while tunnel_state.next_y < 600.0 {
        spawn_tunnel_row(&mut commands, &mut tunnel_state);
    }

    commands.insert_resource(tunnel_state);

    // Background
    commands.spawn((
        StarGooseEntity,
        Sprite {
            color: Color::srgb(0.05, 0.05, 0.1), // Very dark blue/black
            custom_size: Some(Vec2::new(1000.0, 2000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    // Player Ship
    commands.spawn((
        StarGooseEntity,
        Player,
        Velocity::default(),
        Health::new(100),
        Sprite {
            color: colors::EGA_BRIGHT_CYAN,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 1.0),
    ));

    spawn_goose_hud(&mut commands);
}

fn spawn_tunnel_row(commands: &mut Commands, state: &mut TunnelState) {
    let y = state.next_y;
    let cx = state.center_x_index as f32 * BLOCK_SIZE;

    let half_width = (TUNNEL_WIDTH_BLOCKS as f32 * BLOCK_SIZE) / 2.0;
    let left_wall_x = cx - half_width - (BLOCK_SIZE / 2.0);
    let right_wall_x = cx + half_width + (BLOCK_SIZE / 2.0);

    // Left Wall Block
    commands.spawn((
        StarGooseEntity,
        WallBlock,
        Sprite {
            color: colors::EGA_BRIGHT_BLUE,
            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            ..default()
        },
        Transform::from_xyz(left_wall_x, y, 0.0),
    ));
    commands.spawn((
        StarGooseEntity,
        WallBlock,
        Sprite {
            color: colors::EGA_BLUE,
            custom_size: Some(Vec2::new(BLOCK_SIZE * 10.0, BLOCK_SIZE)),
            ..default()
        },
        Transform::from_xyz(left_wall_x - (BLOCK_SIZE * 5.5), y, -0.1),
    ));

    // Right Wall Block
    commands.spawn((
        StarGooseEntity,
        WallBlock,
        Sprite {
            color: colors::EGA_BRIGHT_BLUE,
            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            ..default()
        },
        Transform::from_xyz(right_wall_x, y, 0.0),
    ));
    commands.spawn((
        StarGooseEntity,
        WallBlock,
        Sprite {
            color: colors::EGA_BLUE,
            custom_size: Some(Vec2::new(BLOCK_SIZE * 10.0, BLOCK_SIZE)),
            ..default()
        },
        Transform::from_xyz(right_wall_x + (BLOCK_SIZE * 5.5), y, -0.1),
    ));

    // Floor pattern
    let is_even = (state.next_y / BLOCK_SIZE) as i32 % 2 == 0;
    let floor_color = if is_even {
        Color::srgba(0.1, 0.1, 0.3, 0.5)
    } else {
        Color::srgba(0.15, 0.15, 0.35, 0.5)
    };

    commands.spawn((
        StarGooseEntity,
        TunnelSegment,
        Sprite {
            color: floor_color,
            custom_size: Some(Vec2::new(half_width * 2.0, BLOCK_SIZE)),
            ..default()
        },
        Transform::from_xyz(cx, y, -5.0),
    ));

    // Spawning Items/Enemies
    let rng = rand::random::<f32>();
    if rng < 0.05 {
        // Crystal
        let offset = (rand::random::<f32>() - 0.5) * (half_width * 1.5);
        commands.spawn((
            StarGooseEntity,
            Crystal,
            Sprite {
                color: colors::EGA_BRIGHT_GREEN,
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            Transform::from_xyz(cx + offset, y, 0.5),
        ));
    } else if rng < 0.08 {
        // Fuel
        let offset = (rand::random::<f32>() - 0.5) * (half_width * 1.5);
        commands.spawn((
            StarGooseEntity,
            FuelPod,
            Sprite {
                color: colors::EGA_BRIGHT_RED,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            Transform::from_xyz(cx + offset, y, 0.5),
        ));
    } else if rng < 0.12 {
        // Mine Enemy
        let offset = (rand::random::<f32>() - 0.5) * (half_width * 1.4);
        commands.spawn((
            StarGooseEntity,
            Enemy {
                enemy_type: EnemyType::Mine,
                health: 1,
            },
            Sprite {
                color: colors::EGA_BRIGHT_WHITE,
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..default()
            },
            Transform::from_xyz(cx + offset, y, 0.6),
        ));
    } else if rng < 0.14 {
        // Chaser Enemy
        let offset = (rand::random::<f32>() - 0.5) * (half_width * 1.0);
        commands.spawn((
            StarGooseEntity,
            Enemy {
                enemy_type: EnemyType::Chaser,
                health: 3,
            },
            Sprite {
                color: colors::EGA_BRIGHT_MAGENTA,
                custom_size: Some(Vec2::new(28.0, 28.0)),
                ..default()
            },
            Transform::from_xyz(cx + offset, y, 0.6),
        ));
    }

    // Advance state
    state.next_y += BLOCK_SIZE;
    state.steps_until_turn -= 1;

    if state.steps_until_turn <= 0 {
        let r = rand::random::<f32>();
        if r < 0.3 {
            state.current_direction = -1;
        } else if r < 0.6 {
            state.current_direction = 1;
        } else {
            state.current_direction = 0;
        }
        state.steps_until_turn = rand::random::<i32>() % 4 + 2;
    }

    state.center_x_index += state.current_direction;
    state.center_x_index = state.center_x_index.clamp(-5, 5);
}

fn spawn_goose_hud(commands: &mut Commands) {
    commands
        .spawn((
            StarGooseEntity,
            StarGooseHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|hud| {
            // FUEL
            hud.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("FUEL"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_RED),
                ));
                col.spawn((
                    HudText {
                        field: HudField::Fuel,
                    },
                    Text::new("0"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_WHITE),
                ));
            });

            // AMMO
            hud.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("AMMO"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_YELLOW),
                ));
                col.spawn((
                    HudText {
                        field: HudField::Ammo,
                    },
                    Text::new("0"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_WHITE),
                ));
            });

            // SHIELD
            hud.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("SHIELD"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_CYAN),
                ));
                col.spawn((
                    HudText {
                        field: HudField::Shield,
                    },
                    Text::new("0"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_WHITE),
                ));
            });

            // SCORE
            hud.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|col| {
                col.spawn((
                    Text::new("SCORE"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_WHITE),
                ));
                col.spawn((
                    HudText {
                        field: HudField::Score,
                    },
                    Text::new("0"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_WHITE),
                ));
            });
        });
}

// ─── Systems ───────────────────────────────────────────────────────

fn scroll_tunnel(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform),
        Or<(
            With<WallBlock>,
            With<TunnelSegment>,
            With<Crystal>,
            With<FuelPod>,
            With<Enemy>,
        )>,
    >,
    mut state: ResMut<TunnelState>,
) {
    let dt = time.delta_secs();
    let move_dist = SCROLL_SPEED * dt;

    for (entity, mut transform) in &mut query {
        transform.translation.y -= move_dist;
        if transform.translation.y < -600.0 {
            commands.entity(entity).despawn();
        }
    }

    state.next_y -= move_dist;

    while state.next_y < 600.0 {
        spawn_tunnel_row(&mut commands, &mut state);
    }
}

fn wall_collision(
    mut player_query: Query<&mut Transform, With<Player>>,
    wall_query: Query<&Transform, (With<WallBlock>, Without<Player>)>,
) {
    let mut player_transform = match player_query.iter_mut().next() {
        Some(p) => p,
        None => return,
    };

    let player_size = 24.0;
    let block_size = BLOCK_SIZE;

    for wall_transform in &wall_query {
        let diff = player_transform.translation - wall_transform.translation;
        let dist = diff.abs();

        if dist.x < (block_size / 2.0 + player_size / 2.0)
            && dist.y < (block_size / 2.0 + player_size / 2.0)
        {
            let overlap = Vec2::new(
                (block_size / 2.0 + player_size / 2.0) - dist.x,
                (block_size / 2.0 + player_size / 2.0) - dist.y,
            );

            if overlap.x < overlap.y {
                let dir = if diff.x > 0.0 { 1.0 } else { -1.0 };
                player_transform.translation.x += overlap.x * dir;
            } else {
                let dir = if diff.y > 0.0 { 1.0 } else { -1.0 };
                player_transform.translation.y += overlap.y * dir;
            }
        }
    }
}

fn combat_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    missile_query: Query<(Entity, &Transform), With<Missile>>,
    mut resources: ResMut<StarGooseResources>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut p_health, p_transform) = match player_query.iter_mut().next() {
        Some(p) => p,
        None => return,
    };

    // Player vs Enemy
    for (e_entity, e_transform, _) in &enemy_query {
        if (p_transform.translation - e_transform.translation).length() < 30.0 {
            commands.entity(e_entity).despawn();
            resources.shield = resources.shield.saturating_sub(20);

            if resources.shield == 0 {
                p_health.damage(10);
            }
            if p_health.current <= 0 {
                next_state.set(GameState::Results);
            }
        }
    }

    // Missile vs Enemy
    for (m_entity, m_transform) in &missile_query {
        for (e_entity, e_transform, mut enemy) in &mut enemy_query {
            if (m_transform.translation - e_transform.translation).length() < 30.0 {
                commands.entity(m_entity).despawn();
                enemy.health -= 1;
                if enemy.health <= 0 {
                    commands.entity(e_entity).despawn();
                    resources.score += 50;
                }
                break;
            }
        }
    }
}

fn ship_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut delta = Vec2::ZERO;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            delta.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            delta.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            delta.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            delta.y -= 1.0;
        }

        if delta != Vec2::ZERO {
            delta = delta.normalize();
        }

        transform.translation += delta.extend(0.0) * SHIP_SPEED * time.delta_secs();
        transform.translation.y = transform.translation.y.clamp(-350.0, 350.0);
    }
}

fn ship_shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut resources: ResMut<StarGooseResources>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if resources.ammo > 0 {
            if let Some(player_transform) = player_query.iter().next() {
                commands.spawn((
                    StarGooseEntity,
                    Missile,
                    Sprite {
                        color: colors::EGA_BRIGHT_YELLOW,
                        custom_size: Some(Vec2::new(8.0, 20.0)),
                        ..default()
                    },
                    Transform::from_xyz(
                        player_transform.translation.x,
                        player_transform.translation.y + 20.0,
                        0.0,
                    ),
                ));
                resources.ammo -= 1;
            }
        }
    }
}

fn missile_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Missile>>,
) {
    for (entity, mut transform) in &mut query {
        transform.translation.y += MISSILE_SPEED * time.delta_secs();
        if transform.translation.y > 450.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn resource_drain(
    time: Res<Time>,
    mut resources: ResMut<StarGooseResources>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    resources.fuel -= 2.0 * time.delta_secs();
    if resources.fuel <= 0.0 {
        info!("Out of fuel!");
        next_state.set(GameState::Results);
    }
}

fn collection_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(Entity, &Transform, Option<&Crystal>, Option<&FuelPod>)>,
    mut resources: ResMut<StarGooseResources>,
) {
    let p_pos = match player_query.iter().next() {
        Some(t) => t.translation,
        None => return,
    };

    for (entity, transform, crystal, fuel) in &item_query {
        if (p_pos - transform.translation).length() < 30.0 {
            commands.entity(entity).despawn();
            if crystal.is_some() {
                resources.score += 100;
            }
            if fuel.is_some() {
                resources.fuel = (resources.fuel + 15.0).min(100.0);
            }
        }
    }
}

fn enemy_system(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
) {
    let dt = time.delta_secs();

    for (mut transform, enemy) in &mut enemy_query {
        match enemy.enemy_type {
            EnemyType::Mine => {
                // Rotate mine
                transform.rotation *= Quat::from_rotation_z(1.0 * dt);
            }
            EnemyType::Chaser => {
                // Move towards player
                if let Some(player_transform) = player_query.iter().next() {
                    let dir =
                        (player_transform.translation - transform.translation).normalize_or_zero();
                    // Move
                    transform.translation += dir * 80.0 * dt;
                }
            }
        }
    }
}

fn update_hud(resources: Res<StarGooseResources>, mut texts: Query<(&mut Text, &HudText)>) {
    for (mut text, hud_field) in &mut texts {
        match hud_field.field {
            HudField::Fuel => **text = format!("{:.0}%", resources.fuel),
            HudField::Ammo => **text = format!("{}", resources.ammo),
            HudField::Shield => **text = format!("{}%", resources.shield),
            HudField::Score => **text = format!("{}", resources.score),
        }
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut results: ResMut<GameResults>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        results.game_name = "Star Goose".to_string();
        next_state.set(GameState::Results);
    }
}

fn cleanup_star_goose(mut commands: Commands, query: Query<Entity, With<StarGooseEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
