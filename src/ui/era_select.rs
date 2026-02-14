use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::colors;
use crate::core::progression::PlayerProgress;
use crate::core::states::{Era, GameState, MiniGameId, PlayingState};

/// Plugin for the era selection screen (mini-game picker).
pub struct EraSelectPlugin;

impl Plugin for EraSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentEra>()
            .init_resource::<SelectedGame>()
            .add_systems(OnEnter(GameState::EraSelect), setup_era_select)
            .add_systems(
                Update,
                (era_select_input, layout_game_carousel, update_game_labels)
                    .run_if(in_state(GameState::EraSelect)),
            )
            .add_systems(OnExit(GameState::EraSelect), cleanup_era_select);
    }
}

#[derive(Resource)]
pub struct CurrentEra {
    pub era: Era,
}

impl Default for CurrentEra {
    fn default() -> Self {
        Self { era: Era::The80s }
    }
}

#[derive(Resource, Default)]
struct SelectedGame {
    index: usize,
}

#[derive(Component)]
struct EraSelectRoot;

#[derive(Component)]
struct GameCarouselCard {
    game: MiniGameId,
    index: usize,
}

#[derive(Component)]
struct SelectedGameTitle;

#[derive(Component)]
struct SelectedGameDescription;

#[derive(Component)]
struct SelectedGameStatus;

fn setup_era_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_era: Res<CurrentEra>,
    mut selected_game: ResMut<SelectedGame>,
) {
    let games = get_era_games(current_era.era);
    if games.is_empty() {
        selected_game.index = 0;
    } else {
        selected_game.index = selected_game.index.min(games.len() - 1);
    }

    commands
        .spawn((
            EraSelectRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.04, 0.04, 0.08)),
        ))
        .with_children(|root| {
            root.spawn((
                ImageNode::new(asset_server.load("ui/main_menu_bg_v2.png")),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new(current_era.era.display_name().to_string()),
                TextFont {
                    font_size: 34.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_YELLOW),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(114.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SelectedGameTitle,
                Text::new(""),
                TextFont {
                    font_size: 44.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_CYAN),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(168.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SelectedGameDescription,
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(198.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SelectedGameStatus,
                Text::new(""),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(colors::TEXT_ACCENT),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(22.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new("← → Navigate Game  |  Enter Play  |  Esc Back"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            for (index, game) in games.iter().enumerate() {
                root.spawn((
                    EraSelectRoot,
                    GameCarouselCard {
                        game: *game,
                        index,
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(300.0),
                        height: Val::Px(250.0),
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    BorderColor::all(colors::PANEL_BORDER),
                    BackgroundColor(colors::BUTTON_NORMAL),
                ))
                .with_children(|card| {
                    card.spawn((
                        ImageNode::new(asset_server.load(game_thumbnail_path(*game))),
                        Node {
                            width: Val::Percent(92.0),
                            height: Val::Percent(65.0),
                            ..default()
                        },
                    ));

                    card.spawn((
                        Text::new(game.display_name().to_string()),
                        TextFont {
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            }
        });
}

fn era_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_era: Res<CurrentEra>,
    progress: Res<PlayerProgress>,
    mut selected_game: ResMut<SelectedGame>,
    mut game_state: ResMut<NextState<GameState>>,
    mut playing_state: ResMut<NextState<PlayingState>>,
) {
    let games = get_era_games(current_era.era);

    if keyboard.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Timeline);
        return;
    }

    if games.is_empty() {
        return;
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        selected_game.index = (selected_game.index + 1) % games.len();
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        selected_game.index = (selected_game.index + games.len() - 1) % games.len();
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        let game = games[selected_game.index];
        if progress.is_game_unlocked(game) {
            playing_state.set(game.playing_state());
            game_state.set(GameState::Playing);
        }
    }
}

fn layout_game_carousel(
    current_era: Res<CurrentEra>,
    selected_game: Res<SelectedGame>,
    progress: Res<PlayerProgress>,
    time: Res<Time>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<
        (
            &GameCarouselCard,
            &mut Node,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<EraSelectRoot>,
    >,
) {
    let games = get_era_games(current_era.era);
    if games.is_empty() {
        return;
    }

    let Ok(window) = window_q.single() else {
        return;
    };

    let center_x = window.width() * 0.5;
    let base_y = window.height() * 0.52;
    let t = (time.delta_secs() * 12.0).clamp(0.0, 1.0);

    for (card, mut node, mut bg, mut border) in &mut query {
        let distance = card.index as i32 - selected_game.index as i32;
        let abs_distance = distance.abs() as f32;

        let width = (330.0 - abs_distance * 72.0).max(170.0);
        let height = (255.0 - abs_distance * 62.0).max(130.0);
        let x = center_x + distance as f32 * 260.0 - width * 0.5;
        let y = base_y + abs_distance * 16.0 - height * 0.5;

        let current_left = match node.left {
            Val::Px(v) => v,
            _ => x,
        };
        let current_top = match node.top {
            Val::Px(v) => v,
            _ => y,
        };
        let current_width = match node.width {
            Val::Px(v) => v,
            _ => width,
        };
        let current_height = match node.height {
            Val::Px(v) => v,
            _ => height,
        };

        node.left = Val::Px(current_left + (x - current_left) * t);
        node.top = Val::Px(current_top + (y - current_top) * t);
        node.width = Val::Px(current_width + (width - current_width) * t);
        node.height = Val::Px(current_height + (height - current_height) * t);

        let unlocked = progress.is_game_unlocked(card.game);
        let completed = progress.is_game_completed(card.game);

        if !unlocked {
            *bg = BackgroundColor(colors::BUTTON_LOCKED);
            *border = BorderColor::all(Color::srgb(0.1, 0.1, 0.15));
        } else if card.index == selected_game.index {
            *bg = BackgroundColor(colors::BUTTON_HOVER);
            *border = BorderColor::all(colors::EGA_BRIGHT_CYAN);
        } else if completed {
            *bg = BackgroundColor(Color::srgb(0.1, 0.2, 0.15));
            *border = BorderColor::all(colors::EGA_BRIGHT_GREEN);
        } else {
            *bg = BackgroundColor(colors::BUTTON_NORMAL);
            *border = BorderColor::all(colors::PANEL_BORDER);
        }
    }
}

fn update_game_labels(
    current_era: Res<CurrentEra>,
    selected_game: Res<SelectedGame>,
    progress: Res<PlayerProgress>,
    mut text_sets: ParamSet<(
        Query<&mut Text, With<SelectedGameTitle>>,
        Query<&mut Text, With<SelectedGameDescription>>,
        Query<&mut Text, With<SelectedGameStatus>>,
    )>,
) {
    let games = get_era_games(current_era.era);
    if games.is_empty() {
        if let Ok(mut text) = text_sets.p0().single_mut() {
            **text = "No Games Available".to_string();
        }
        if let Ok(mut text) = text_sets.p1().single_mut() {
            **text = "".to_string();
        }
        if let Ok(mut text) = text_sets.p2().single_mut() {
            **text = "".to_string();
        }
        return;
    }

    let game = games[selected_game.index.min(games.len() - 1)];

    if let Ok(mut text) = text_sets.p0().single_mut() {
        **text = game.display_name().to_string();
    }
    if let Ok(mut text) = text_sets.p1().single_mut() {
        **text = game.description().to_string();
    }
    if let Ok(mut text) = text_sets.p2().single_mut() {
        if progress.is_game_completed(game) {
            **text = format!("COMPLETED  |  High Score: {}", progress.high_score(game));
        } else if progress.is_game_unlocked(game) {
            let high = progress.high_score(game);
            if high > 0 {
                **text = format!("READY  |  Best: {}", high);
            } else {
                **text = "READY TO PLAY".to_string();
            }
        } else {
            **text = "LOCKED".to_string();
        }
    }
}

fn game_thumbnail_path(game: MiniGameId) -> &'static str {
    match (game.era, game.index) {
        (Era::The80s, 0) => "ui/thumbnails/tunnel_miner.png",
        (Era::The80s, 1) => "ui/thumbnails/cosmic_captain.png",
        (Era::The80s, 2) => "ui/thumbnails/star_goose.png",
        (Era::The90s, 0) => "ui/thumbnails/worm_wars.png",
        (Era::The90s, 1) => "ui/thumbnails/ice_blitz.png",
        (Era::The90s, 2) => "ui/thumbnails/depths_of_doom.png",
        _ => "ui/thumbnails/tunnel_miner.png",
    }
}

fn get_era_games(era: Era) -> Vec<MiniGameId> {
    match era {
        Era::The80s => vec![
            MiniGameId { era, index: 0 },
            MiniGameId { era, index: 1 },
            MiniGameId { era, index: 2 },
        ],
        Era::The90s => vec![
            MiniGameId { era, index: 0 },
            MiniGameId { era, index: 1 },
            MiniGameId { era, index: 2 },
        ],
        _ => vec![],
    }
}

fn cleanup_era_select(mut commands: Commands, query: Query<Entity, With<EraSelectRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
