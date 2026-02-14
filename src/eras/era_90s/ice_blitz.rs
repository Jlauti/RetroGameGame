use bevy::prelude::*;
use crate::core::states::{GameState, PlayingState};
use crate::shared::components::{Player, Score, Velocity};
use crate::ui::colors;
use crate::ui::results::GameResults;

/// Ice Blitz — inspired by NHL 98 (1997).
/// Fast-paced top-down arcade ice hockey.
pub struct IceBlitzPlugin;

impl Plugin for IceBlitzPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Scores>()
            .add_systems(
                OnEnter(GameState::Playing),
            setup_hockey.run_if(in_state(PlayingState::IceBlitz)),
        )
        .add_systems(
            Update,
            (
                player_skating,
                ai_behavior,
                puck_physics,
                stick_handling,
                goal_system,
                update_hud,
                handle_pause,
            )
                .run_if(in_state(PlayingState::IceBlitz)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_hockey);
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const RINK_WIDTH: f32 = 800.0;
const RINK_HEIGHT: f32 = 500.0;
const PUCK_SPEED_MAX: f32 = 600.0;
const SKATING_ACCEL: f32 = 400.0;
const SKATING_FRICTION: f32 = 0.98;
const GOAL_SIZE: f32 = 120.0;

// ─── Components ───────────────────────────────────────────────────

#[derive(Component)]
struct IceBlitzEntity;

#[derive(Component)]
struct HockeyPlayer {
    team_id: u8,
    has_puck: bool,
}

#[derive(Component)]
struct Puck;

#[derive(Component)]
struct Goal {
    team_id: u8,
}

#[derive(Component)]
struct HockeyHud;

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_hockey(mut commands: Commands) {
    info!("Setting up Ice Blitz...");

    // Rink Background
    commands.spawn((
        IceBlitzEntity,
        Sprite {
            color: Color::srgb(0.9, 0.95, 1.0), // Ice white
            custom_size: Some(Vec2::new(RINK_WIDTH, RINK_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Rink Border (Simple visual)
    commands.spawn((
        IceBlitzEntity,
        Sprite {
            color: Color::srgb(0.7, 0.1, 0.1), // Red line
            custom_size: Some(Vec2::new(RINK_WIDTH + 10.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, RINK_HEIGHT/2.0, 1.0),
    ));
    commands.spawn((
        IceBlitzEntity,
        Sprite {
            color: Color::srgb(0.7, 0.1, 0.1), // Red line
            custom_size: Some(Vec2::new(RINK_WIDTH + 10.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -RINK_HEIGHT/2.0, 1.0),
    ));

    // Goals
    commands.spawn((
        IceBlitzEntity,
        Goal { team_id: 1 }, // Left goal (scores for team 1)
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(20.0, GOAL_SIZE)),
            ..default()
        },
        Transform::from_xyz(-RINK_WIDTH/2.0 + 10.0, 0.0, 1.0),
    ));

    commands.spawn((
        IceBlitzEntity,
        Goal { team_id: 0 }, // Right goal (scores for team 0)
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(20.0, GOAL_SIZE)),
            ..default()
        },
        Transform::from_xyz(RINK_WIDTH/2.0 - 10.0, 0.0, 1.0),
    ));

    // Players
    // Player Team (0)
    commands.spawn((
        IceBlitzEntity,
        Player,
        HockeyPlayer { team_id: 0, has_puck: false },
        Velocity::default(),
        Sprite {
            color: colors::EGA_BLUE,
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(-150.0, 0.0, 2.0),
    ));

    // AI Team (1)
    commands.spawn((
        IceBlitzEntity,
        HockeyPlayer { team_id: 1, has_puck: false },
        Velocity::default(),
        Sprite {
            color: colors::EGA_RED,
            custom_size: Some(Vec2::new(24.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(150.0, 0.0, 2.0),
    ));

    // Puck
    commands.spawn((
        IceBlitzEntity,
        Puck,
        Velocity::default(),
        Sprite {
            color: Color::srgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));

    spawn_hockey_hud(&mut commands);
}

fn spawn_hockey_hud(commands: &mut Commands) {
    commands
        .spawn((
            IceBlitzEntity,
            HockeyHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(15.0)),
                column_gap: Val::Px(40.0),
                ..default()
            },
        ))
        .with_children(|hud| {
            hud.spawn((
                TeamScoreText { team_id: 0 },
                Text::new("BLUE: 0"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(colors::EGA_BRIGHT_CYAN),
            ));
            hud.spawn((
                Text::new("ICE BLITZ"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(colors::TEXT_SECONDARY),
            ));
            hud.spawn((
                TeamScoreText { team_id: 1 },
                Text::new("RED: 0"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(colors::EGA_RED),
            ));
        });
}

#[derive(Component)]
struct TeamScoreText { team_id: u8 }

// ─── Systems ───────────────────────────────────────────────────────

fn player_skating(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut vel, mut transform) in &mut query {
        let mut input = Vec2::ZERO;
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) { input.y += 1.0; }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) { input.y -= 1.0; }
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) { input.x -= 1.0; }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) { input.x += 1.0; }

        if input.length_squared() > 0.0 {
            let accel = input.normalize() * SKATING_ACCEL * time.delta_secs();
            vel.x += accel.x;
            vel.y += accel.y;
        }

        // Apply friction
        vel.x *= SKATING_FRICTION;
        vel.y *= SKATING_FRICTION;

        transform.translation.x += vel.x * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();

        // Rink boundaries
        transform.translation.x = transform.translation.x.clamp(-RINK_WIDTH/2.0 + 15.0, RINK_WIDTH/2.0 - 15.0);
        transform.translation.y = transform.translation.y.clamp(-RINK_HEIGHT/2.0 + 15.0, RINK_HEIGHT/2.0 - 15.0);
    }
}

fn ai_behavior(
    time: Res<Time>,
    puck_query: Query<&Transform, (With<Puck>, Without<HockeyPlayer>)>,
    mut ai_query: Query<(&mut Velocity, &mut Transform), (With<HockeyPlayer>, Without<Player>)>,
) {
    let puck_pos = match puck_query.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (mut vel, mut transform) in &mut ai_query {
        let to_puck = (puck_pos - transform.translation).truncate();
        if to_puck.length() > 20.0 {
            let accel = to_puck.normalize() * (SKATING_ACCEL * 0.8) * time.delta_secs();
            vel.x += accel.x;
            vel.y += accel.y;
        }

        vel.x *= SKATING_FRICTION;
        vel.y *= SKATING_FRICTION;

        transform.translation.x += vel.x * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();
    }
}

fn puck_physics(
    time: Res<Time>,
    mut puck_query: Query<(&mut Velocity, &mut Transform), With<Puck>>,
) {
    for (mut vel, mut transform) in &mut puck_query {
        transform.translation.x += vel.x * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();

        // Bounce off walls
        if transform.translation.x.abs() > RINK_WIDTH/2.0 - 5.0 {
            vel.x *= -0.8;
            transform.translation.x = (RINK_WIDTH/2.0 - 5.1) * transform.translation.x.signum();
        }
        if transform.translation.y.abs() > RINK_HEIGHT/2.0 - 5.0 {
            vel.y *= -0.8;
            transform.translation.y = (RINK_HEIGHT/2.0 - 5.1) * transform.translation.y.signum();
        }

        vel.x *= 0.99; // Less friction for puck
        vel.y *= 0.99;
    }
}

fn stick_handling(
    mut puck_query: Query<(&mut Velocity, &mut Transform), (With<Puck>, Without<HockeyPlayer>)>,
    mut players_query: Query<(&Transform, &mut HockeyPlayer, &Velocity), Without<Puck>>,
) {
    let (mut puck_vel, mut puck_transform) = match puck_query.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    for (player_transform, mut player, player_vel) in &mut players_query {
        let dist = (puck_transform.translation - player_transform.translation).length();
        if dist < 30.0 {
            // Stick interaction: snap puck to player or push it
            puck_vel.x = player_vel.x * 1.2;
            puck_vel.y = player_vel.y * 1.2;
        }
    }
}

fn goal_system(
    mut puck_query: Query<(&mut Velocity, &mut Transform), (With<Puck>, Without<Goal>)>,
    goal_query: Query<(&Transform, &Goal), Without<Puck>>,
    mut scores: ResMut<Scores>,
) {
    let (mut puck_vel, mut puck_transform) = match puck_query.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    for (goal_transform, goal) in &goal_query {
        let dist_x = (puck_transform.translation.x - goal_transform.translation.x).abs();
        let dist_y = (puck_transform.translation.y - goal_transform.translation.y).abs();
        
        if dist_x < 15.0 && dist_y < GOAL_SIZE/2.0 {
            info!("GOAL for Team {}!", 1 - goal.team_id);
            if goal.team_id == 0 { scores.team1 += 1; } else { scores.team0 += 1; }
            
            // Reset puck
            puck_transform.translation = Vec3::new(0.0, 0.0, 2.0);
            puck_vel.x = 0.0;
            puck_vel.y = 0.0;
        }
    }
}

#[derive(Resource, Default)]
struct Scores {
    team0: u32,
    team1: u32,
}

fn update_hud(
    scores: Res<Scores>,
    mut query: Query<(&mut Text, &TeamScoreText)>,
) {
    for (mut text, team_score) in &mut query {
        if team_score.team_id == 0 {
            **text = format!("BLUE: {}", scores.team0);
        } else {
            **text = format!("RED: {}", scores.team1);
        }
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut results: ResMut<GameResults>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        results.game_name = "Ice Blitz".to_string();
        next_state.set(GameState::Results);
    }
}

fn cleanup_hockey(mut commands: Commands, query: Query<Entity, With<IceBlitzEntity>>) {
    for entity in &query { commands.entity(entity).despawn(); }
}
