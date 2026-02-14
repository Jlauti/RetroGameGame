use bevy::prelude::*;
use bevy::app::AppExit;

use crate::core::states::GameState;
use super::colors;

/// Plugin for the main menu screen.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<ScreenTransitionEvent>()
        //     .add_systems(Update, (start_transition, update_transition));
        app.add_systems(OnEnter(GameState::Boot), setup_boot)
            .add_systems(Update, boot_timer.run_if(in_state(GameState::Boot)))
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (button_interaction, menu_action).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

// ─── Components ────────────────────────────────────────────────────

#[derive(Component)]
struct BootScreen;

#[derive(Component)]
struct BootTimer(Timer);

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
enum MenuButton {
    Play,
    Settings,
    Credits,
    Quit,
}

// ─── Boot screen ───────────────────────────────────────────────────

fn setup_boot(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Boot/splash screen
    commands
        .spawn((
            BootScreen,
            BootTimer(Timer::from_seconds(2.0, TimerMode::Once)),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(colors::EGA_BLACK),
        ))
        .with_children(|parent| {
            // DOS-style boot text
            parent.spawn((
                Text::new("C:\\>RETROGAMEGAME.EXE"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_GREEN),
            ));

            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::EGA_GREEN),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Blinking cursor
            parent.spawn((
                Text::new("█"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(colors::EGA_BRIGHT_GREEN),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn boot_timer(
    time: Res<Time>,
    mut query: Query<&mut BootTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    boot_screen: Query<Entity, With<BootScreen>>,
    mut commands: Commands,
) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // Clean up boot screen
            for entity in &boot_screen {
                commands.entity(entity).despawn();
            }
            next_state.set(GameState::Menu);
        }
    }
}

// ─── Main menu ─────────────────────────────────────────────────────

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MenuRoot,
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
            // Title (Graphic)
            parent.spawn((
                ImageNode::new(asset_server.load::<Image>("sprites/menu/title_logo.png")),
                Node {
                    width: Val::Px(512.0),
                    height: Val::Px(256.0),
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("A Journey Through Gaming History"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Buttons container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.0),
                    ..default()
                })
                .with_children(|buttons| {
                    let menu_items = [
                        ("▶  PLAY", MenuButton::Play),
                        ("⚙  SETTINGS", MenuButton::Settings),
                        ("★  CREDITS", MenuButton::Credits),
                        ("✕  QUIT", MenuButton::Quit),
                    ];

                    for (label, action) in menu_items {
                        buttons.spawn((
                            action,
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(50.0),
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
                                Text::new(label),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_PRIMARY),
                            ));
                        });
                    }
                });

            // Footer
            parent.spawn((
                Text::new("v0.1.0 — Press Enter to select"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}

// spawn_menu_button removed

// ─── Interaction systems ───────────────────────────────────────────

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg, mut border) in &mut interaction_query {
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

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    // mut exit: ResMut<Events<AppExit>>,
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::Play => {
                    next_state.set(GameState::Timeline);
                }
                MenuButton::Settings => {
                    // TODO: Settings
                }
                MenuButton::Credits => {
                    // TODO: Credits
                }
                MenuButton::Quit => {
                    // exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
