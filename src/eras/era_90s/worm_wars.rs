use crate::core::states::{GameState, PlayingState};
use crate::shared::components::{Health, Player, Score, Velocity};
use crate::ui::colors;
use crate::ui::results::GameResults;
use bevy::prelude::*;
use std::collections::VecDeque;

/// Worm Wars — inspired by Worms (1995).
/// Turn-based artillery strategy with gravity and projectiles.
pub struct WormWarsPlugin;

impl Plugin for WormWarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WormWarsState>()
            .add_systems(
                OnEnter(GameState::Playing),
                setup_worms.run_if(in_state(PlayingState::WormWars)),
            )
            .add_systems(
                Update,
                (
                    turn_logic,
                    worm_movement,
                    gravity_system,
                    projectile_system,
                    explosion_system,
                    update_hud,
                    handle_pause,
                )
                    .run_if(in_state(PlayingState::WormWars)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_worms);
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const WORM_SIZE: f32 = 16.0;
const GRAVITY: f32 = -500.0;
const WALK_SPEED: f32 = 100.0;
const JUMP_IMPULSE: f32 = 250.0;
const TURN_DURATION: f32 = 30.0;

// ─── Components ───────────────────────────────────────────────────

#[derive(Component)]
struct WormWarsEntity;

#[derive(Component)]
struct Worm {
    team_id: u8,
    is_active: bool,
}

#[derive(Component)]
struct Projectile {
    radius: f32,
    damage: f32,
}

#[derive(Component)]
struct Terrain;

#[derive(Component)]
struct WormHud;

// ─── Resources ─────────────────────────────────────────────────────

#[derive(Resource)]
struct WormWarsState {
    teams_count: u8,
    active_team: u8,
    turn_timer: Timer,
    turn_queue: VecDeque<Entity>,
    is_windy: bool,
    wind_force: f32,
}

impl Default for WormWarsState {
    fn default() -> Self {
        Self {
            teams_count: 2,
            active_team: 0,
            turn_timer: Timer::from_seconds(TURN_DURATION, TimerMode::Once),
            turn_queue: VecDeque::new(),
            is_windy: true,
            wind_force: 50.0,
        }
    }
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_worms(mut commands: Commands, mut state: ResMut<WormWarsState>) {
    info!("Setting up Worm Wars...");

    // Reset state
    state.active_team = 0;
    state.turn_timer.reset();
    state.turn_queue.clear();

    // Camera
    // (Already spawned in main menu boot, but individual games might want their own)

    // Terrain (Placeholder — a flat green floor for now)
    commands.spawn((
        WormWarsEntity,
        Terrain,
        Sprite {
            color: Color::srgb(0.1, 0.4, 0.1),
            custom_size: Some(Vec2::new(1000.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -250.0, 0.0),
    ));

    // Spawn Worms
    let worm_positions = [
        (Vec2::new(-200.0, 0.0), 0),
        (Vec2::new(-100.0, 0.0), 0),
        (Vec2::new(100.0, 0.0), 1),
        (Vec2::new(200.0, 0.0), 1),
    ];

    for (pos, team) in worm_positions {
        let worm = commands
            .spawn((
                WormWarsEntity,
                Worm {
                    team_id: team,
                    is_active: false,
                },
                Health::new(100),
                Velocity::default(),
                Sprite {
                    color: if team == 0 {
                        colors::EGA_BRIGHT_CYAN
                    } else {
                        colors::EGA_RED
                    },
                    custom_size: Some(Vec2::new(WORM_SIZE, WORM_SIZE * 1.5)),
                    ..default()
                },
                Transform::from_translation(pos.extend(1.0)),
            ))
            .id();

        state.turn_queue.push_back(worm);
    }

    // Activate first worm
    if let Some(first_worm) = state.turn_queue.front() {
        commands.entity(*first_worm).insert(ActiveWorm);
    }

    // Spawn HUD
    spawn_worms_hud(&mut commands);
}

#[derive(Component)]
struct ActiveWorm;

fn spawn_worms_hud(commands: &mut Commands) {
    commands
        .spawn((
            WormWarsEntity,
            WormHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|hud| {
            hud.spawn((
                Text::new("WORM WARS"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_GREEN),
            ));

            hud.spawn((
                TurnTimerText,
                Text::new("Time: 30"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_YELLOW),
            ));

            hud.spawn((
                WindText,
                Text::new("Wind: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::EGA_CYAN),
            ));
        });
}

#[derive(Component)]
struct TurnTimerText;

#[derive(Component)]
struct WindText;

// ─── Systems ───────────────────────────────────────────────────────

fn turn_logic(
    time: Res<Time>,
    mut state: ResMut<WormWarsState>,
    mut commands: Commands,
    active_worm_query: Query<Entity, With<ActiveWorm>>,
) {
    state.turn_timer.tick(time.delta());

    if state.turn_timer.just_finished() {
        // Switch turn
        if let Ok(old_active) = active_worm_query.single() {
            commands.entity(old_active).remove::<ActiveWorm>();
        }

        // Rotate queue
        if let Some(last) = state.turn_queue.pop_front() {
            state.turn_queue.push_back(last);
        }

        if let Some(next) = state.turn_queue.front() {
            commands.entity(*next).insert(ActiveWorm);
            state.turn_timer.reset();
            state.active_team = 1 - state.active_team; // Toggle for 2 teams
            state.wind_force = (rand::random::<f32>() - 0.5) * 100.0;
            info!("Turn switched to Team {}", state.active_team);
        }
    }
}

fn worm_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<ActiveWorm>>,
) {
    for (mut transform, mut vel) in &mut query {
        let mut move_dir = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            move_dir -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            move_dir += 1.0;
        }

        transform.translation.x += move_dir * WALK_SPEED * time.delta_secs();

        if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
            // Jump if near ground
            if transform.translation.y < -190.0 {
                vel.y = JUMP_IMPULSE;
            }
        }
    }
}

fn gravity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), Without<Terrain>>,
) {
    for (mut transform, mut vel) in &mut query {
        vel.y += GRAVITY * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();

        // Simple floor collision
        if transform.translation.y < -200.0 {
            transform.translation.y = -200.0;
            vel.y = 0.0;
        }
    }
}

fn projectile_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active_worm_query: Query<&Transform, (With<ActiveWorm>, Without<Projectile>)>,
    mut projectile_query: Query<
        (Entity, &mut Transform, &mut Velocity, &Projectile),
        Without<ActiveWorm>,
    >,
    time: Res<Time>,
    state: Res<WormWarsState>,
) {
    // Fire bazooka
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(worm_transform) = active_worm_query.single() {
            commands.spawn((
                WormWarsEntity,
                Projectile {
                    radius: 5.0,
                    damage: 50.0,
                },
                Velocity::new(300.0, 300.0), // Hardcoded arc for now
                Sprite {
                    color: colors::EGA_BRIGHT_WHITE,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                Transform::from_translation(worm_transform.translation + Vec3::new(0.0, 20.0, 0.0)),
            ));
        }
    }

    // Update projectiles
    for (entity, mut transform, mut vel, _proj) in &mut projectile_query {
        // Wind affect
        vel.x += state.wind_force * time.delta_secs();

        // Collision check
        if transform.translation.y < -200.0 {
            info!("Projectile hit ground!");
            commands.entity(entity).despawn();
            // TODO: Trigger explosion
        }
    }
}

fn explosion_system() {
    // Placeholder for destructible terrain updates
}

fn update_hud(
    state: Res<WormWarsState>,
    mut timer_query: Query<&mut Text, (With<TurnTimerText>, Without<WindText>)>,
    mut wind_query: Query<&mut Text, (With<WindText>, Without<TurnTimerText>)>,
) {
    for mut text in &mut timer_query {
        **text = format!("Time: {:.0}", state.turn_timer.remaining_secs());
    }
    for mut text in &mut wind_query {
        **text = format!("Wind: {:.0}", state.wind_force);
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut results: ResMut<GameResults>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        results.game_name = "Worm Wars".to_string();
        next_state.set(GameState::Results);
    }
}

fn cleanup_worms(mut commands: Commands, query: Query<Entity, With<WormWarsEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
