use bevy::prelude::*;
use crate::core::states::{GameState, PlayingState};
use crate::shared::components::{Health, Player, Score, Velocity};
use crate::ui::colors;
use crate::ui::results::GameResults;

/// Cosmic Captain — inspired by Captain Comic (1988).
/// Side-scrolling platformer with gravity and collectible abilities.
pub struct CosmicCaptainPlugin;

impl Plugin for CosmicCaptainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            setup_captain.run_if(in_state(PlayingState::CosmicCaptain)),
        )
        .add_systems(
            Update,
            (
                captain_movement,
                platform_collision,
                collect_items,
                enemy_ai,
                update_hud,
                handle_pause,
            )
                .run_if(in_state(PlayingState::CosmicCaptain)),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_captain);
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const GRAVITY: f32 = -600.0;
const JUMP_FORCE: f32 = 300.0;
const MOVE_SPEED: f32 = 180.0;
const TILE_SIZE: f32 = 32.0;

// ─── Components ───────────────────────────────────────────────────

#[derive(Component)]
struct CaptainEntity;

#[derive(Component)]
struct Captain {
    can_shoot: bool,
    has_boots: bool,
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Item {
    kind: ItemKind,
}

enum ItemKind {
    BlastolaCola, // Shoot
    Boots,        // Higher jump
}

#[derive(Component)]
struct CaptainHud;

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_captain(mut commands: Commands) {
    info!("Setting up Cosmic Captain...");

    // Background (Dark blue space)
    commands.spawn((
        CaptainEntity,
        Sprite {
            color: Color::srgb(0.0, 0.0, 0.05),
            custom_size: Some(Vec2::new(2000.0, 1000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Ground & Platforms
    for i in -20..20 {
        commands.spawn((
            CaptainEntity,
            Platform,
            Sprite {
                color: Color::srgb(0.4, 0.3, 0.2), // Brown rock
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(i as f32 * TILE_SIZE, -150.0, 0.0),
        ));
    }

    // Floating platforms
    let plats = [(-100.0, -50.0), (0.0, 20.0), (100.0, -50.0)];
    for (px, py) in plats {
        commands.spawn((
            CaptainEntity,
            Platform,
            Sprite {
                color: Color::srgb(0.3, 0.4, 0.5), // Blueish tech
                custom_size: Some(Vec2::new(TILE_SIZE * 3.0, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(px, py, 0.0),
        ));
    }

    // Player
    commands.spawn((
        CaptainEntity,
        Player,
        Captain { can_shoot: false, has_boots: false },
        Health::new(12),
        Velocity::default(),
        Sprite {
            color: colors::CGA_BRIGHT_CYAN,
            custom_size: Some(Vec2::new(28.0, 36.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 1.0),
    ));

    // Items
    commands.spawn((
        CaptainEntity,
        Item { kind: ItemKind::BlastolaCola },
        Sprite {
            color: colors::CGA_YELLOW,
            custom_size: Some(Vec2::new(16.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 60.0, 0.5),
    ));

    spawn_captain_hud(&mut commands);
}

fn spawn_captain_hud(commands: &mut Commands) {
    commands
        .spawn((
            CaptainEntity,
            CaptainHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.2, 0.7)),
        ))
        .with_children(|hud| {
            hud.spawn((
                Text::new("COSMIC CAPTAIN"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(colors::CGA_BRIGHT_CYAN),
            ));

            hud.spawn((
                CaptainStatsText,
                Text::new("Shield: 12/12 | Weapon: No"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(colors::CGA_BRIGHT_YELLOW),
            ));
        });
}

#[derive(Component)]
struct CaptainStatsText;

// ─── Systems ───────────────────────────────────────────────────────

fn captain_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &Captain), With<Player>>,
) {
    for (mut vel, mut transform, captain) in &mut query {
        let mut move_dir = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) { move_dir -= 1.0; }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) { move_dir += 1.0; }

        transform.translation.x += move_dir * MOVE_SPEED * time.delta_secs();

        // Gravity
        vel.y += GRAVITY * time.delta_secs();
        transform.translation.y += vel.y * time.delta_secs();

        // Jump
        if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
            // Jump if on ground (simple threshold)
            if transform.translation.y <= -132.0 || transform.translation.y == -18.0 {
                 vel.y = if captain.has_boots { JUMP_FORCE * 1.3 } else { JUMP_FORCE };
            }
        }
    }
}

fn platform_collision(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    plat_query: Query<&Transform, (With<Platform>, Without<Player>)>,
) {
    for (mut p_transform, mut vel) in &mut query {
        for plat_transform in &plat_query {
            let dx = p_transform.translation.x - plat_transform.translation.x;
            let dy = p_transform.translation.y - plat_transform.translation.y;

            if dx.abs() < 24.0 && dy.abs() < 30.0 {
                // Land on top
                if vel.y < 0.0 && p_transform.translation.y > plat_transform.translation.y {
                    p_transform.translation.y = plat_transform.translation.y + 32.0;
                    vel.y = 0.0;
                }
            }
        }
    }
}

fn collect_items(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(Entity, &Transform, &Item)>,
    mut captain_query: Query<&mut Captain>,
) {
    let p_transform = match player_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };

    for (entity, i_transform, item) in &item_query {
        if (p_transform.translation - i_transform.translation).length() < 25.0 {
            commands.entity(entity).despawn();
            if let Ok(mut captain) = captain_query.single_mut() {
                match item.kind {
                    ItemKind::BlastolaCola => {
                        captain.can_shoot = true;
                        info!("Found Blastola Cola! Now you can shoot!");
                    },
                    ItemKind::Boots => {
                        captain.has_boots = true;
                        info!("Found Boots! Jump higher!");
                    },
                }
            }
        }
    }
}

fn enemy_ai() {
    // Placeholder for simple patrol enemies
}

fn update_hud(
    player_query: Query<(&Health, &Captain), With<Player>>,
    mut text_query: Query<&mut Text, With<CaptainStatsText>>,
) {
    if let Ok((hp, captain)) = player_query.single() {
        for mut text in &mut text_query {
            **text = format!(
                "Shield: {}/12 | Weapon: {}",
                hp.current,
                if captain.can_shoot { "READY" } else { "None" }
            );
        }
    }
}

fn handle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut results: ResMut<GameResults>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        results.game_name = "Cosmic Captain".to_string();
        next_state.set(GameState::Results);
    }
}

fn cleanup_captain(mut commands: Commands, query: Query<Entity, With<CaptainEntity>>) {
    for entity in &query { commands.entity(entity).despawn(); }
}
