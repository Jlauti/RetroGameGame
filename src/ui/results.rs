use bevy::prelude::*;

use crate::core::states::GameState;
use super::colors;

/// Plugin for the results / score screen shown after a mini-game.
pub struct ResultsPlugin;

impl Plugin for ResultsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameResults>()
            .add_systems(OnEnter(GameState::Results), setup_results)
            .add_systems(
                Update,
                (results_input, results_button_interaction, results_button_action)
                    .run_if(in_state(GameState::Results)),
            )
            .add_systems(OnExit(GameState::Results), cleanup_results);
    }
}

// ─── Resources ─────────────────────────────────────────────────────

/// Data to display on the results screen.
#[derive(Resource, Default)]
pub struct GameResults {
    pub game_name: String,
    pub score: u64,
    pub high_score: u64,
    pub is_new_high: bool,
    pub tokens_earned: u64,
    pub completed: bool,
    pub newly_completed: bool,
}

// ─── Components ────────────────────────────────────────────────────

#[derive(Component)]
struct ResultsRoot;

#[derive(Component)]
enum ResultsButton {
    Retry,
    BackToEra,
    BackToTimeline,
}

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_results(mut commands: Commands, results: Res<GameResults>) {
    commands
        .spawn((
            ResultsRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.02, 0.08)),
        ))
        .with_children(|parent| {
            // Panel
            parent
                .spawn(Node {
                        width: Val::Px(500.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        row_gap: Val::Px(15.0),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    })
                    .insert(BackgroundColor(colors::PANEL_BG))
                    .insert(BorderColor::all(colors::PANEL_BORDER))
                .with_children(|panel| {
                    // Title
                    let title = if results.newly_completed {
                        "🎉 GAME COMPLETED! 🎉"
                    } else {
                        "GAME OVER"
                    };
                    panel.spawn((
                        Text::new(title.to_string()),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(if results.newly_completed {
                            colors::GOLD
                        } else {
                            colors::TEXT_PRIMARY
                        }),
                    ));

                    // Game name
                    panel.spawn((
                        Text::new(results.game_name.clone()),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_ACCENT),
                    ));

                    // Divider
                    panel.spawn((
                        Node {
                            width: Val::Percent(80.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(colors::PANEL_BORDER),
                    ));

                    // Score
                    panel.spawn((
                        Text::new(format!("Score: {}", results.score)),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(colors::EGA_BRIGHT_YELLOW),
                    ));

                    // High score
                    if results.is_new_high {
                        panel.spawn((
                            Text::new("★ NEW HIGH SCORE! ★"),
                            TextFont {
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(colors::GOLD),
                        ));
                    } else {
                        panel.spawn((
                            Text::new(format!("High Score: {}", results.high_score)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                    }

                    // Tokens earned
                    panel.spawn((
                        Text::new(format!("🪙 +{} Tokens", results.tokens_earned)),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(colors::GOLD),
                    ));

                    // Divider
                    panel.spawn((
                        Node {
                            width: Val::Percent(80.0),
                            height: Val::Px(2.0),
                            margin: UiRect::vertical(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(colors::PANEL_BORDER),
                    ));

                    // Buttons
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|buttons| {
                            // Retry Button
                            buttons
                                .spawn((
                                    ResultsButton::Retry,
                                    Button,
                                    Node {
                                        width: Val::Px(250.0),
                                        height: Val::Px(44.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::BUTTON_NORMAL),
                                    BorderColor::all(colors::PANEL_BORDER),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("🔄  RETRY"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_PRIMARY),
                                    ));
                                });

                            // Back to Era Button
                            buttons
                                .spawn((
                                    ResultsButton::BackToEra,
                                    Button,
                                    Node {
                                        width: Val::Px(250.0),
                                        height: Val::Px(44.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::BUTTON_NORMAL),
                                    BorderColor::all(colors::PANEL_BORDER),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("📋  BACK TO ERA"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_PRIMARY),
                                    ));
                                });

                            // Back to Timeline Button
                            buttons
                                .spawn((
                                    ResultsButton::BackToTimeline,
                                    Button,
                                    Node {
                                        width: Val::Px(250.0),
                                        height: Val::Px(44.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(colors::BUTTON_NORMAL),
                                    BorderColor::all(colors::PANEL_BORDER),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("🗺️  TIMELINE"),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_PRIMARY),
                                    ));
                                });
                        });
                });
        });
}

// spawn_results_button removed

// ─── Interaction ───────────────────────────────────────────────────

fn results_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::EraSelect);
    }
}

fn results_button_interaction(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ResultsButton>),
    >,
) {
    for (interaction, mut bg, mut border) in &mut query {
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
                *bg = BackgroundColor(colors::BUTTON_NORMAL);
                *border = BorderColor::all(colors::PANEL_BORDER);
            }
        }
    }
}

fn results_button_action(
    query: Query<(&Interaction, &ResultsButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in &query {
        if *interaction == Interaction::Pressed {
            match button {
                ResultsButton::Retry => {
                    next_state.set(GameState::Playing);
                }
                ResultsButton::BackToEra => {
                    next_state.set(GameState::EraSelect);
                }
                ResultsButton::BackToTimeline => {
                    next_state.set(GameState::Timeline);
                }
            }
        }
    }
}

fn cleanup_results(mut commands: Commands, query: Query<Entity, With<ResultsRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
}
