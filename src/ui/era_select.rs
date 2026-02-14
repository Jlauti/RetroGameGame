use bevy::prelude::*;

use crate::core::progression::PlayerProgress;
use crate::core::states::{Era, GameState, MiniGameId, PlayingState};
use super::colors;

/// Plugin for the era selection screen (mini-game picker).
pub struct EraSelectPlugin;

impl Plugin for EraSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentEra>()
            .add_systems(OnEnter(GameState::EraSelect), setup_era_select)
            .add_systems(
                Update,
                (era_select_input, game_card_interaction, game_card_action)
                    .run_if(in_state(GameState::EraSelect)),
            )
            .add_systems(OnExit(GameState::EraSelect), cleanup_era_select);
    }
}

// ─── Resources ─────────────────────────────────────────────────────

#[derive(Resource)]
pub struct CurrentEra {
    pub era: Era,
}

impl Default for CurrentEra {
    fn default() -> Self {
        Self { era: Era::The80s }
    }
}

// ─── Components ────────────────────────────────────────────────────

#[derive(Component)]
struct EraSelectRoot;

#[derive(Component)]
struct GameCard {
    game: MiniGameId,
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_era_select(
    mut commands: Commands,
    current_era: Res<CurrentEra>,
    progress: Res<PlayerProgress>,
) {
    let era = current_era.era;
    let games = get_era_games(era);

    commands
        .spawn((
            EraSelectRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.02, 0.08)),
        ))
        .with_children(|parent| {
            // Era header
            parent.spawn((
                Text::new(era.display_name().to_string()),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_YELLOW),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Era description
            let desc = match era {
                Era::The80s => "EGA graphics • PC speaker beeps • Chunky pixels",
                Era::The90s => "16-bit pixel art • Chiptune melodies • Parallax scrolling",
                _ => "Coming soon...",
            };
            parent.spawn((
                Text::new(desc.to_string()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Game cards
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(25.0),
                    ..default()
                })
                .with_children(|cards| {
                    for game_id in &games {
                        let game = *game_id;
                        let unlocked = progress.is_game_unlocked(game);
                        let completed = progress.is_game_completed(game);
                        let high_score = progress.high_score(game);

                        let bg = if completed {
                            Color::srgb(0.1, 0.2, 0.15)
                        } else if unlocked {
                            colors::BUTTON_NORMAL
                        } else {
                            colors::BUTTON_LOCKED
                        };

                        let border = if completed {
                            colors::EGA_BRIGHT_GREEN
                        } else if unlocked {
                            colors::PANEL_BORDER
                        } else {
                            Color::srgb(0.1, 0.1, 0.15)
                        };

                        cards
                            .spawn((
                                GameCard { game },
                                Button,
                                Node {
                                    width: Val::Px(260.0),
                                    height: Val::Px(300.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    padding: UiRect::all(Val::Px(20.0)),
                                    border: UiRect::all(Val::Px(2.0)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(bg),
                                BorderColor::all(border),
                            ))
                            .with_children(|card| {
                                // Game icon area (placeholder — will be replaced by sprite)
                                card.spawn((
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Px(120.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.08, 0.08, 0.15)),
                                    BorderColor::all(colors::PANEL_BORDER),
                                ))
                                .with_children(|icon_area| {
                                    let icon = match (game.era, game.index) {
                                        (Era::The80s, 0) => "⛏️",
                                        (Era::The80s, 1) => "🚀",
                                        (Era::The80s, 2) => "🦆",
                                        (Era::The90s, 0) => "🐛",
                                        (Era::The90s, 1) => "🏒",
                                        (Era::The90s, 2) => "💀",
                                        _ => "❓",
                                    };
                                    icon_area.spawn((
                                        Text::new(icon.to_string()),
                                        TextFont {
                                            font_size: 48.0,
                                            ..default()
                                        },
                                        TextColor(if unlocked {
                                            colors::TEXT_PRIMARY
                                        } else {
                                            colors::TEXT_SECONDARY
                                        }),
                                    ));
                                });

                                // Game name
                                card.spawn((
                                    Text::new(game.display_name().to_string()),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(if unlocked {
                                        colors::EGA_BRIGHT_YELLOW
                                    } else {
                                        colors::TEXT_SECONDARY
                                    }),
                                ));

                                // Description
                                card.spawn((
                                    Text::new(game.description().to_string()),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(colors::TEXT_SECONDARY),
                                ));

                                // Status line
                                let status = if completed {
                                    format!("✅ COMPLETED • High: {}", high_score)
                                } else if unlocked {
                                    if high_score > 0 {
                                        format!("Best: {}", high_score)
                                    } else {
                                        "Ready to play!".to_string()
                                    }
                                } else {
                                    "🔒 LOCKED".to_string()
                                };

                                card.spawn((
                                    Text::new(status),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(if completed {
                                        colors::EGA_BRIGHT_GREEN
                                    } else if unlocked {
                                        colors::TEXT_ACCENT
                                    } else {
                                        Color::srgb(0.5, 0.3, 0.3)
                                    }),
                                ));
                            });
                    }
                });

            // Footer
            parent.spawn((
                Text::new("Enter  Play  |  Esc  Back to Timeline"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

// spawn_game_card removed

// ─── Interaction ───────────────────────────────────────────────────

fn era_select_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Timeline);
    }
}

fn game_card_interaction(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &GameCard),
        Changed<Interaction>,
    >,
    progress: Res<PlayerProgress>,
) {
    for (interaction, mut bg, mut border, card) in &mut query {
        if !progress.is_game_unlocked(card.game) {
            continue;
        }

        let completed = progress.is_game_completed(card.game);

        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(colors::BUTTON_PRESSED);
                *border = BorderColor::all(colors::TEXT_ACCENT);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(colors::BUTTON_HOVER);
                *border = BorderColor::all(colors::EGA_BRIGHT_CYAN);
            }
            Interaction::None => {
                *bg = BackgroundColor(if completed {
                    Color::srgb(0.1, 0.2, 0.15)
                } else {
                    colors::BUTTON_NORMAL
                });
                *border = BorderColor::all(if completed {
                    colors::EGA_BRIGHT_GREEN
                } else {
                    colors::PANEL_BORDER
                });
            }
        }
    }
}

fn game_card_action(
    query: Query<(&Interaction, &GameCard), Changed<Interaction>>,
    progress: Res<PlayerProgress>,
    mut game_state: ResMut<NextState<GameState>>,
    mut playing_state: ResMut<NextState<PlayingState>>,
) {
    for (interaction, card) in &query {
        if *interaction == Interaction::Pressed && progress.is_game_unlocked(card.game) {
            playing_state.set(card.game.playing_state());
            game_state.set(GameState::Playing);
        }
    }
}

// ─── Helpers ───────────────────────────────────────────────────────

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
