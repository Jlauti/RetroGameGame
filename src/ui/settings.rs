use bevy::app::AppExit;
use bevy::prelude::*;

use super::colors;
use crate::core::settings::{DisplayMode, GameSettings, QuitBehavior};
use crate::core::states::GameState;

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SettingsPanelState>()
            .add_systems(
                PreUpdate,
                (
                    handle_escape_in_playing.run_if(in_state(GameState::Playing)),
                    handle_escape_in_menu.run_if(in_state(GameState::Menu)),
                ),
            )
            .add_systems(
                Update,
                (
                    sync_settings_panel_root,
                    settings_button_interaction,
                    settings_button_action,
                    refresh_settings_value_labels,
                    sync_time_pause_for_settings,
                ),
            );
    }
}

const RESOLUTION_PRESETS: &[(u32, u32)] = &[
    (1024, 768),
    (1280, 720),
    (1366, 768),
    (1600, 900),
    (1920, 1080),
    (1920, 1200),
    (2048, 1152),
    (2560, 1440),
    (2560, 1600),
    (3440, 1440),
    (3840, 2160),
];

const MUSIC_VOLUME_STEP: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsPanelSource {
    #[default]
    Menu,
    Playing,
}

#[derive(Resource, Debug, Clone)]
pub struct SettingsPanelState {
    pub open: bool,
    pub source: SettingsPanelSource,
    pub draft: GameSettings,
}

impl Default for SettingsPanelState {
    fn default() -> Self {
        Self {
            open: false,
            source: SettingsPanelSource::Menu,
            draft: GameSettings::default(),
        }
    }
}

impl SettingsPanelState {
    pub fn open_from(&mut self, source: SettingsPanelSource, current: &GameSettings) {
        self.open = true;
        self.source = source;
        self.draft = current.clone();
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}

#[derive(Component)]
struct SettingsPanelRoot;

#[derive(Component, Clone, Copy)]
struct SettingsActionButton(SettingsAction);

#[derive(Component, Clone, Copy)]
struct SettingsValueLabel(SettingsValueKind);

#[derive(Clone, Copy)]
enum SettingsAction {
    ResolutionPrev,
    ResolutionNext,
    DisplayModePrev,
    DisplayModeNext,
    MusicVolumeDown,
    MusicVolumeUp,
    QuitBehaviorToggle,
    Apply,
    Cancel,
    Quit,
}

#[derive(Clone, Copy)]
enum SettingsValueKind {
    Resolution,
    DisplayMode,
    MusicVolume,
    QuitBehavior,
}

fn handle_escape_in_playing(
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    mut panel_state: ResMut<SettingsPanelState>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    if panel_state.open && panel_state.source == SettingsPanelSource::Playing {
        panel_state.close();
    } else if !panel_state.open {
        panel_state.open_from(SettingsPanelSource::Playing, &settings);
    }

    keyboard.clear_just_pressed(KeyCode::Escape);
}

fn handle_escape_in_menu(
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut panel_state: ResMut<SettingsPanelState>,
) {
    if keyboard.just_pressed(KeyCode::Escape)
        && panel_state.open
        && panel_state.source == SettingsPanelSource::Menu
    {
        panel_state.close();
        keyboard.clear_just_pressed(KeyCode::Escape);
    }
}

fn sync_settings_panel_root(
    mut commands: Commands,
    game_state: Res<State<GameState>>,
    panel_state: Res<SettingsPanelState>,
    existing: Query<Entity, With<SettingsPanelRoot>>,
) {
    let can_show = matches!(game_state.get(), GameState::Menu | GameState::Playing);

    if !panel_state.open || !can_show {
        for entity in &existing {
            commands.entity(entity).despawn_children();
            commands.entity(entity).despawn();
        }
        return;
    }

    if !existing.is_empty() {
        return;
    }

    commands
        .spawn((
            SettingsPanelRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.78)),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(680.0),
                    height: Val::Px(480.0),
                    border: UiRect::all(Val::Px(2.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    row_gap: Val::Px(14.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(colors::PANEL_BG),
                BorderColor::all(colors::PANEL_BORDER),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("SETTINGS"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(colors::EGA_BRIGHT_CYAN),
                ));

                panel.spawn((
                    Text::new("Esc closes this panel during gameplay"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_SECONDARY),
                ));

                panel
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },))
                    .with_children(|settings_rows| {
                        spawn_setting_row(
                            settings_rows,
                            "Resolution",
                            SettingsValueKind::Resolution,
                            SettingsAction::ResolutionPrev,
                            SettingsAction::ResolutionNext,
                        );
                        spawn_setting_row(
                            settings_rows,
                            "Display Mode",
                            SettingsValueKind::DisplayMode,
                            SettingsAction::DisplayModePrev,
                            SettingsAction::DisplayModeNext,
                        );
                        spawn_setting_row(
                            settings_rows,
                            "Music Volume",
                            SettingsValueKind::MusicVolume,
                            SettingsAction::MusicVolumeDown,
                            SettingsAction::MusicVolumeUp,
                        );
                        spawn_setting_row(
                            settings_rows,
                            "Quit Action",
                            SettingsValueKind::QuitBehavior,
                            SettingsAction::QuitBehaviorToggle,
                            SettingsAction::QuitBehaviorToggle,
                        );
                    });

                panel
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(14.0)),
                        ..default()
                    },))
                    .with_children(|buttons| {
                        spawn_action_button(
                            buttons,
                            "APPLY",
                            SettingsAction::Apply,
                            Color::srgb(0.15, 0.35, 0.15),
                        );
                        spawn_action_button(
                            buttons,
                            "CANCEL",
                            SettingsAction::Cancel,
                            colors::BUTTON_NORMAL,
                        );
                        spawn_action_button(
                            buttons,
                            "QUIT",
                            SettingsAction::Quit,
                            Color::srgb(0.45, 0.12, 0.12),
                        );
                    });
            });
        });
}

fn spawn_setting_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    value_kind: SettingsValueKind,
    action_left: SettingsAction,
    action_right: SettingsAction,
) {
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(52.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(12.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Node {
                    width: Val::Px(170.0),
                    ..default()
                },
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));

            spawn_small_action_button(row, "<", action_left);

            row.spawn((
                Node {
                    width: Val::Px(260.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                SettingsValueLabel(value_kind),
                Text::new(""),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_ACCENT),
            ));

            spawn_small_action_button(row, ">", action_right);
        });
}

fn spawn_small_action_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: SettingsAction,
) {
    parent
        .spawn((
            Button,
            SettingsActionButton(action),
            Node {
                width: Val::Px(42.0),
                height: Val::Px(36.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(colors::BUTTON_NORMAL),
            BorderColor::all(colors::PANEL_BORDER),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
        });
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: SettingsAction,
    base_color: Color,
) {
    parent
        .spawn((
            Button,
            SettingsActionButton(action),
            Node {
                width: Val::Px(180.0),
                height: Val::Px(44.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(base_color),
            BorderColor::all(colors::PANEL_BORDER),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
            ));
        });
}

fn settings_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (
            Changed<Interaction>,
            With<SettingsActionButton>,
            With<Button>,
        ),
    >,
) {
    for (interaction, mut bg, mut border) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(colors::BUTTON_PRESSED);
                *border = BorderColor::all(colors::EGA_BRIGHT_CYAN);
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

fn settings_button_action(
    mut panel_state: ResMut<SettingsPanelState>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
    query: Query<(&Interaction, &SettingsActionButton), (Changed<Interaction>, With<Button>)>,
) {
    if !panel_state.open {
        return;
    }

    for (interaction, action_button) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action_button.0 {
            SettingsAction::ResolutionPrev => {
                let idx = find_resolution_index(panel_state.draft.resolution);
                let next_idx = if idx == 0 {
                    RESOLUTION_PRESETS.len() - 1
                } else {
                    idx - 1
                };
                panel_state.draft.resolution = RESOLUTION_PRESETS[next_idx];
            }
            SettingsAction::ResolutionNext => {
                let idx = find_resolution_index(panel_state.draft.resolution);
                let next_idx = (idx + 1) % RESOLUTION_PRESETS.len();
                panel_state.draft.resolution = RESOLUTION_PRESETS[next_idx];
            }
            SettingsAction::DisplayModePrev => {
                panel_state.draft.display_mode = prev_display_mode(panel_state.draft.display_mode);
            }
            SettingsAction::DisplayModeNext => {
                panel_state.draft.display_mode = next_display_mode(panel_state.draft.display_mode);
            }
            SettingsAction::MusicVolumeDown => {
                panel_state.draft.music_volume =
                    (panel_state.draft.music_volume - MUSIC_VOLUME_STEP).clamp(0.0, 1.0);
            }
            SettingsAction::MusicVolumeUp => {
                panel_state.draft.music_volume =
                    (panel_state.draft.music_volume + MUSIC_VOLUME_STEP).clamp(0.0, 1.0);
            }
            SettingsAction::QuitBehaviorToggle => {
                panel_state.draft.quit_behavior = match panel_state.draft.quit_behavior {
                    QuitBehavior::ToHub => QuitBehavior::ToDesktop,
                    QuitBehavior::ToDesktop => QuitBehavior::ToHub,
                };
            }
            SettingsAction::Apply => {
                *settings = panel_state.draft.clone();
                panel_state.close();
            }
            SettingsAction::Cancel => {
                panel_state.close();
            }
            SettingsAction::Quit => match panel_state.draft.quit_behavior {
                QuitBehavior::ToHub => {
                    panel_state.close();
                    next_state.set(GameState::Timeline);
                }
                QuitBehavior::ToDesktop => {
                    app_exit.write(AppExit::Success);
                }
            },
        }
    }
}

fn refresh_settings_value_labels(
    panel_state: Res<SettingsPanelState>,
    mut labels: Query<(&SettingsValueLabel, &mut Text)>,
) {
    if !panel_state.open {
        return;
    }

    for (kind, mut text) in &mut labels {
        **text = match kind.0 {
            SettingsValueKind::Resolution => format!(
                "{} x {}",
                panel_state.draft.resolution.0, panel_state.draft.resolution.1
            ),
            SettingsValueKind::DisplayMode => {
                display_mode_label(panel_state.draft.display_mode).to_string()
            }
            SettingsValueKind::MusicVolume => {
                format!("{:.0}%", panel_state.draft.music_volume * 100.0)
            }
            SettingsValueKind::QuitBehavior => {
                quit_behavior_label(panel_state.draft.quit_behavior).to_string()
            }
        };
    }
}

fn sync_time_pause_for_settings(
    panel_state: Res<SettingsPanelState>,
    game_state: Res<State<GameState>>,
    mut time: ResMut<Time<Virtual>>,
) {
    let should_pause = panel_state.open
        && panel_state.source == SettingsPanelSource::Playing
        && matches!(game_state.get(), GameState::Playing);

    if should_pause {
        time.pause();
    } else {
        time.unpause();
    }
}

fn find_resolution_index(resolution: (u32, u32)) -> usize {
    RESOLUTION_PRESETS
        .iter()
        .position(|preset| *preset == resolution)
        .unwrap_or(0)
}

fn next_display_mode(mode: DisplayMode) -> DisplayMode {
    match mode {
        DisplayMode::Windowed => DisplayMode::Borderless,
        DisplayMode::Borderless => DisplayMode::Fullscreen,
        DisplayMode::Fullscreen => DisplayMode::Windowed,
    }
}

fn prev_display_mode(mode: DisplayMode) -> DisplayMode {
    match mode {
        DisplayMode::Windowed => DisplayMode::Fullscreen,
        DisplayMode::Borderless => DisplayMode::Windowed,
        DisplayMode::Fullscreen => DisplayMode::Borderless,
    }
}

fn display_mode_label(mode: DisplayMode) -> &'static str {
    match mode {
        DisplayMode::Windowed => "Windowed",
        DisplayMode::Borderless => "Windowed Fullscreen",
        DisplayMode::Fullscreen => "Fullscreen",
    }
}

fn quit_behavior_label(mode: QuitBehavior) -> &'static str {
    match mode {
        QuitBehavior::ToHub => "Quit to Hub",
        QuitBehavior::ToDesktop => "Quit to Desktop",
    }
}
