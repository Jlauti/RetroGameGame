use bevy::prelude::*;
use crate::core::states::{GameState, PlayingState};
use crate::shared::components::{Health, Player, Score, Velocity};
use crate::ui::colors;
use crate::ui::results::GameResults;

/// Star Goose — inspired by Star Goose (1988).
/// Vertical-scrolling shooter with resource management.
pub struct StarGoosePlugin;

impl Plugin for StarGoosePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarGooseResources>()
            .add_systems(
                OnEnter(GameState::Playing),
                setup_star_goose.run_if(in_state(PlayingState::StarGoose)),
            )
            .add_systems(
                Update,
                (
                    ship_movement,
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

const SCROLL_SPEED: f32 = 100.0;
const SHIP_SPEED: f32 = 250.0;

// ─── Components ───────────────────────────────────────────────────

#[derive(Component)]
struct StarGooseEntity;

#[derive(Component)]
struct Crystal;

#[derive(Component)]
struct FuelPod;

#[derive(Component)]
struct StarGooseHud;

// ─── Resources ─────────────────────────────────────────────────────

#[derive(Resource)]
struct StarGooseResources {
    fuel: f32,
    ammo: u32,
    crystals: u32,
}

impl Default for StarGooseResources {
    fn default() -> Self {
        Self {
            fuel: 100.0,
            ammo: 100,
            crystals: 0,
        }
    }
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_star_goose(mut commands: Commands) {
    info!("Setting up Star Goose...");

    // Reset resources
    commands.insert_resource(StarGooseResources::default());

    // Terrain (Scrolling background)
    commands.spawn((
        StarGooseEntity,
        Sprite {
            color: Color::srgb(0.1, 0.05, 0.2), // Dark purple terrain
            custom_size: Some(Vec2::new(600.0, 2000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Player Ship
    commands.spawn((
        StarGooseEntity,
        Player,
        Velocity::default(),
        Health::new(100),
        Sprite {
            color: colors::CGA_BRIGHT_CYAN,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -250.0, 1.0),
    ));

    // Spawn some initial crystals
    for i in 0..10 {
        let rx = (rand::random::<f32>() - 0.5) * 400.0;
        let ry = (rand::random::<f32>() * 800.0) - 200.0;
        commands.spawn((
            StarGooseEntity,
            Crystal,
            Sprite {
                color: colors::CGA_BRIGHT_GREEN,
                custom_size: Some(Vec2::new(12.0, 12.0)),
                ..default()
            },
            Transform::from_xyz(rx, ry, 0.5),
        ));
    }

    spawn_goose_hud(&mut commands);
}

fn spawn_goose_hud(commands: &mut Commands) {
    commands
        .spawn((
            StarGooseEntity,
            StarGooseHud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|hud| {
            hud.spawn((
                FuelText,
                Text::new("Fuel: 100%"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(colors::CGA_YELLOW),
            ));
            hud.spawn((
                CrystalText,
                Text::new("Crystals: 0/6"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(colors::CGA_BRIGHT_GREEN),
            ));
        });
}

#[derive(Component)]
struct FuelText;

#[derive(Component)]
struct CrystalText;

// ─── Systems ───────────────────────────────────────────────────────

fn ship_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut delta = Vec2::ZERO;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) { delta.x -= 1.0; }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) { delta.x += 1.0; }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) { delta.y += 1.0; }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) { delta.y -= 1.0; }

        transform.translation += delta.extend(0.0) * SHIP_SPEED * time.delta_secs();

        // Boundaries
        transform.translation.x = transform.translation.x.clamp(-280.0, 280.0);
        transform.translation.y = transform.translation.y.clamp(-350.0, 350.0);
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
    crystal_query: Query<(Entity, &Transform), With<Crystal>>,
    mut resources: ResMut<StarGooseResources>,
) {
    let p_pos = match player_query.single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (entity, transform) in &crystal_query {
        if (p_pos - transform.translation).length() < 25.0 {
            commands.entity(entity).despawn();
            resources.crystals += 1;
            info!("Crystal collected! Total: {}", resources.crystals);
        }
    }
}

fn enemy_system(
    // Placeholder for enemy wave spawning and movement
) {
}

fn update_hud(
    resources: Res<StarGooseResources>,
    mut fuel_text: Query<&mut Text, (With<FuelText>, Without<CrystalText>)>,
    mut crystal_text: Query<&mut Text, (With<CrystalText>, Without<FuelText>)>,
) {
    for mut text in &mut fuel_text {
        **text = format!("Fuel: {:.0}%", resources.fuel);
    }
    for mut text in &mut crystal_text {
        **text = format!("Crystals: {}/6", resources.crystals);
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
    for entity in &query { commands.entity(entity).despawn(); }
}
