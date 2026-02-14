use bevy::prelude::*;

use crate::core::progression::PlayerProgress;
use crate::core::states::{Era, GameState};
use super::colors;

/// Plugin for the timeline hub screen.
pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEra>()
            .add_systems(OnEnter(GameState::Timeline), setup_timeline)
            .add_systems(
                Update,
                (timeline_input, era_button_interaction, era_button_action)
                    .run_if(in_state(GameState::Timeline)),
            )
            .add_systems(OnExit(GameState::Timeline), cleanup_timeline);
    }
}

// ─── Resources ─────────────────────────────────────────────────────

#[derive(Resource, Default)]
struct SelectedEra {
    index: usize,
}

// ─── Components ────────────────────────────────────────────────────

#[derive(Component)]
struct TimelineRoot;

#[derive(Component)]
struct EraCard {
    era: Era,
    index: usize,
}

#[derive(Component)]
struct EraLabel;

#[derive(Component)]
struct TokenDisplay;

// ─── Setup ─────────────────────────────────────────────────────────

fn setup_timeline(mut commands: Commands, progress: Res<PlayerProgress>) {
    let eras = [
        (Era::The80s, "1980s", "The DOS Age", "🖥️"),
        (Era::The90s, "1990s", "The Golden Age", "🎮"),
        (Era::The2000s, "2000s", "Coming Soon", "💿"),
        (Era::The2010s, "2010s", "Coming Soon", "🕹️"),
    ];

    commands
        .spawn((
            TimelineRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.02, 0.02, 0.08)),
        ))
        .with_children(|parent| {
            // Header
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("TIMELINE"),
                        TextFont {
                            font_size: 36.0,
                            ..default()
                        },
                        TextColor(colors::EGA_BRIGHT_CYAN),
                    ));

                    // Token counter
                    header.spawn((
                        TokenDisplay,
                        Text::new(format!("🪙 {} Tokens", progress.tokens)),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(colors::GOLD),
                    ));
                });

            // Decorative timeline line
            parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(3.0),
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(colors::PANEL_BORDER),
            ));

            // Era cards container
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|cards| {
                    for (i, (era, decade, subtitle, icon)) in eras.iter().enumerate() {
                        let unlocked = progress.is_era_unlocked(*era);
                        // Inline spawn logic
                        let bg_color = if unlocked {
                            colors::BUTTON_NORMAL
                        } else {
                            colors::BUTTON_LOCKED
                        };

                        let border_color = if unlocked {
                            colors::PANEL_BORDER
                        } else {
                            Color::srgb(0.1, 0.1, 0.15)
                        };

                        cards
                            .spawn((
                                EraCard { era: *era, index: i },
                                Button,
                                Node {
                                    width: Val::Px(200.0),
                                    height: Val::Px(260.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::all(Val::Px(15.0)),
                                    border: UiRect::all(Val::Px(2.0)),
                                    row_gap: Val::Px(10.0),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                                BorderColor::all(border_color),
                            ))
                            .with_children(|card| {
                                // Icon
                                card.spawn((
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

                                // Decade label
                                card.spawn((
                                    Text::new(decade.to_string()),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(if unlocked {
                                        colors::EGA_BRIGHT_YELLOW
                                    } else {
                                        colors::TEXT_SECONDARY
                                    }),
                                ));

                                // Subtitle
                                card.spawn((
                                    Text::new(subtitle.to_string()),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(if unlocked {
                                        colors::TEXT_ACCENT
                                    } else {
                                        Color::srgb(0.3, 0.3, 0.35)
                                    }),
                                ));

                                // Lock indicator for locked eras
                                if !unlocked {
                                    card.spawn((
                                        Text::new("🔒 LOCKED"),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.5, 0.3, 0.3)),
                                        Node {
                                            margin: UiRect::top(Val::Px(10.0)),
                                            ..default()
                                        },
                                    ));
                                }
                            });
                    }
                });

            // Footer instructions
            parent.spawn((
                Text::new("← →  Navigate  |  Enter  Select  |  Esc  Back"),
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

// spawn_era_card removed

// ─── Interaction ───────────────────────────────────────────────────

fn timeline_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn era_button_interaction(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &EraCard),
        Changed<Interaction>,
    >,
    progress: Res<PlayerProgress>,
) {
    for (interaction, mut bg, mut border, card) in &mut query {
        let unlocked = progress.is_era_unlocked(card.era);
        if !unlocked {
            continue;
        }

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

use crate::ui::era_select::CurrentEra;

fn era_button_action(
    query: Query<(&Interaction, &EraCard), Changed<Interaction>>,
    progress: Res<PlayerProgress>,
    mut current_era: ResMut<CurrentEra>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, card) in &query {
        if *interaction == Interaction::Pressed && progress.is_era_unlocked(card.era) {
            current_era.era = card.era;
            next_state.set(GameState::EraSelect);
        }
    }
}

fn cleanup_timeline(mut commands: Commands, query: Query<Entity, With<TimelineRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
