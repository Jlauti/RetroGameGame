use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::colors;
use crate::core::progression::PlayerProgress;
use crate::core::states::{Era, GameState};
use crate::ui::era_select::CurrentEra;

/// Plugin for the timeline hub screen.
pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedEra>()
            .add_systems(OnEnter(GameState::Timeline), setup_timeline)
            .add_systems(
                Update,
                (
                    timeline_input,
                    layout_timeline_carousel,
                    update_timeline_labels,
                )
                    .run_if(in_state(GameState::Timeline)),
            )
            .add_systems(OnExit(GameState::Timeline), cleanup_timeline);
    }
}

#[derive(Resource, Default)]
struct SelectedEra {
    index: usize,
}

#[derive(Component)]
struct TimelineRoot;

#[derive(Component)]
struct EraCarouselCard {
    era: Era,
    index: usize,
}

#[derive(Component)]
struct SelectedEraTitle;

#[derive(Component)]
struct SelectedEraSubtitle;

#[derive(Component)]
struct SelectedEraStatus;

const ERA_COUNT: usize = 5;

fn setup_timeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    progress: Res<PlayerProgress>,
    mut selected: ResMut<SelectedEra>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    selected.index = selected.index.min(ERA_COUNT - 1);

    if camera_query.is_empty() {
        commands.spawn(Camera2d);
    }

    commands
        .spawn((
            TimelineRoot,
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
                    top: Val::Px(32.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new("TIMELINE"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_CYAN),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(82.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new(format!("🪙 {} Tokens", progress.tokens)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::GOLD),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(128.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SelectedEraTitle,
                Text::new(""),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_YELLOW),
            ));

            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(170.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SelectedEraSubtitle,
                Text::new(""),
                TextFont {
                    font_size: 16.0,
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
                SelectedEraStatus,
                Text::new(""),
                TextFont {
                    font_size: 14.0,
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
                Text::new("← → Navigate Era  |  Enter Select  |  Esc Back"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            for (index, (era, title, subtitle, image_path)) in era_entries().iter().enumerate() {
                root.spawn((
                    EraCarouselCard { era: *era, index },
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Px(280.0),
                        height: Val::Px(220.0),
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BorderColor::all(colors::PANEL_BORDER),
                    BackgroundColor(colors::BUTTON_NORMAL),
                ))
                .with_children(|card| {
                    card.spawn((
                        ImageNode::new(asset_server.load(*image_path)),
                        Node {
                            width: Val::Percent(90.0),
                            height: Val::Percent(62.0),
                            ..default()
                        },
                    ));

                    card.spawn((
                        Text::new(title.to_string()),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));

                    card.spawn((
                        Text::new(subtitle.to_string()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                    ));
                });
            }
        });
}

fn timeline_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedEra>,
    mut current_era: ResMut<CurrentEra>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) {
        selected.index = (selected.index + 1) % ERA_COUNT;
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) {
        selected.index = (selected.index + ERA_COUNT - 1) % ERA_COUNT;
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        let (era, _, _, _) = era_entries()[selected.index];
        current_era.era = era;
        next_state.set(GameState::EraSelect);
    }
}

fn layout_timeline_carousel(
    selected: Res<SelectedEra>,
    progress: Res<PlayerProgress>,
    time: Res<Time>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<
        (
            &EraCarouselCard,
            &mut Node,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<EraCarouselCard>,
    >,
) {
    let Ok(window) = window_q.single() else {
        return;
    };

    let center_x = window.width() * 0.5;
    let base_y = window.height() * 0.50;
    let t = (time.delta_secs() * 12.0).clamp(0.0, 1.0);

    for (card, mut node, mut bg, mut border) in &mut query {
        let distance = card.index as i32 - selected.index as i32;
        let abs_distance = distance.abs() as f32;

        let width = (320.0 - abs_distance * 70.0).max(170.0);
        let height = (240.0 - abs_distance * 55.0).max(135.0);
        let x = center_x + distance as f32 * 255.0 - width * 0.5;
        let y = base_y + abs_distance * 14.0 - height * 0.5;

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

        let unlocked = progress.is_era_unlocked(card.era);

        if !unlocked {
            *bg = BackgroundColor(colors::BUTTON_LOCKED);
            *border = BorderColor::all(Color::srgb(0.1, 0.1, 0.15));
        } else if card.index == selected.index {
            *bg = BackgroundColor(colors::BUTTON_HOVER);
            *border = BorderColor::all(colors::EGA_BRIGHT_CYAN);
        } else {
            *bg = BackgroundColor(colors::BUTTON_NORMAL);
            *border = BorderColor::all(colors::PANEL_BORDER);
        }
    }
}

fn update_timeline_labels(
    selected: Res<SelectedEra>,
    progress: Res<PlayerProgress>,
    mut text_sets: ParamSet<(
        Query<&mut Text, With<SelectedEraTitle>>,
        Query<&mut Text, With<SelectedEraSubtitle>>,
        Query<&mut Text, With<SelectedEraStatus>>,
    )>,
) {
    let (era, title, subtitle, _) = era_entries()[selected.index];

    if let Ok(mut text) = text_sets.p0().single_mut() {
        **text = title.to_string();
    }
    if let Ok(mut text) = text_sets.p1().single_mut() {
        **text = subtitle.to_string();
    }
    if let Ok(mut text) = text_sets.p2().single_mut() {
        if progress.is_era_unlocked(era) {
            **text = "READY TO ENTER".to_string();
        } else {
            **text = "LOCKED".to_string();
        }
    }
}

fn cleanup_timeline(mut commands: Commands, query: Query<Entity, With<TimelineRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
}

fn era_entries() -> [(Era, &'static str, &'static str, &'static str); ERA_COUNT] {
    [
        (
            Era::The80s,
            "The 1980s",
            "The DOS Age",
            "ui/thumbnails/tunnel_miner.png",
        ),
        (
            Era::The90s,
            "The 1990s",
            "The Golden Age",
            "ui/thumbnails/worm_wars.png",
        ),
        (
            Era::The2000s,
            "The 2000s",
            "Coming Soon",
            "ui/thumbnails/ice_blitz.png",
        ),
        (
            Era::The2010s,
            "The 2010s",
            "Coming Soon",
            "ui/thumbnails/depths_of_doom.png",
        ),
        (
            Era::Future,
            "Future",
            "Nebula Bouncer",
            "ui/thumbnails/nebula_bouncer.png",
        ),
    ]
}
