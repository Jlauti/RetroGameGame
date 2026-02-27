use bevy::asset::LoadState;
use bevy::audio::{AudioSink, AudioSinkPlayback, Volume};
use bevy::prelude::*;

use crate::core::settings::GameSettings;
use crate::core::states::GameState;

/// Global music controller for hub screens.
pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hub_music)
            .add_systems(Update, control_hub_music);
    }
}

#[derive(Component)]
struct HubMusicEntity;

#[derive(Resource)]
struct HubMusicController {
    entity: Entity,
    handle: Handle<AudioSource>,
    missing_sink_frames: u32,
    load_wait_frames: u32,
    warned_failed_load: bool,
    warned_missing_sink: bool,
    logged_sink_ready: bool,
    tried_known_good_fallback: bool,
}

const HUB_MUSIC_PRIMARY: &str = "music/pixel_pathways.mp3";
const HUB_MUSIC_FALLBACK: &str = "music/Pixel Pathways.mp3";
const HUB_MUSIC_KNOWN_GOOD: &str = "music/Pixel Popcorn Rush.mp3";

/// Conversion policy: Map UI 0.0-1.0 to linear volume.
/// Original mix used 1.25 for this track.
fn get_volume_for_setting(setting: f32) -> Volume {
    Volume::Linear(setting * 1.25)
}

fn setup_hub_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    let music_path = choose_music_path();
    let handle: Handle<AudioSource> = asset_server.load(music_path);
    let entity = spawn_hub_music_entity(&mut commands, handle.clone(), settings.music_volume);

    info!(
        "Hub music controller initialized with track '{}'",
        music_path
    );

    commands.insert_resource(HubMusicController {
        entity,
        handle,
        missing_sink_frames: 0,
        load_wait_frames: 0,
        warned_failed_load: false,
        warned_missing_sink: false,
        logged_sink_ready: false,
        tried_known_good_fallback: false,
    });
}

fn control_hub_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    settings: Res<GameSettings>,
    controller: Option<ResMut<HubMusicController>>,
    music_entity_query: Query<(), With<HubMusicEntity>>,
    mut sink_query: Query<&mut AudioSink, With<HubMusicEntity>>,
) {
    let Some(mut controller) = controller else {
        return;
    };

    if music_entity_query.get(controller.entity).is_err() {
        controller.entity = spawn_hub_music_entity(
            &mut commands,
            controller.handle.clone(),
            settings.music_volume,
        );
    }

    let current_state = state.get();
    let should_play = matches!(
        current_state,
        GameState::Menu | GameState::Timeline | GameState::EraSelect
    );
    let load_state = asset_server.load_state(controller.handle.id());

    if let LoadState::Failed(err) = &load_state {
        if !controller.warned_failed_load {
            error!("Hub music failed to load: {err}");
            controller.warned_failed_load = true;
        }
        return;
    }

    if !matches!(load_state, LoadState::Loaded) {
        controller.load_wait_frames += 1;
        if controller.load_wait_frames == 180 {
            warn!("Hub music load state after 3s: {:?}", load_state);
        }
        if controller.load_wait_frames > 600 && !controller.tried_known_good_fallback {
            warn!(
                "Hub music did not reach Loaded state in time ({:?}). Switching to fallback track '{}'",
                load_state, HUB_MUSIC_KNOWN_GOOD
            );
            controller.handle = asset_server.load(HUB_MUSIC_KNOWN_GOOD);
            commands.entity(controller.entity).despawn();
            controller.entity = spawn_hub_music_entity(
                &mut commands,
                controller.handle.clone(),
                settings.music_volume,
            );
            controller.load_wait_frames = 0;
            controller.missing_sink_frames = 0;
            controller.warned_missing_sink = false;
            controller.warned_failed_load = false;
            controller.logged_sink_ready = false;
            controller.tried_known_good_fallback = true;
        }
        return;
    }
    controller.load_wait_frames = 0;

    if let Ok(mut sink) = sink_query.get_mut(controller.entity) {
        controller.missing_sink_frames = 0;
        controller.warned_missing_sink = false;

        if !controller.logged_sink_ready {
            info!("Hub music sink is active");
            controller.logged_sink_ready = true;
        }

        if should_play {
            if sink.is_paused() {
                info!("Playing hub music (state: {:?})", current_state);
                sink.play();
            }
            sink.unmute();
            sink.set_volume(get_volume_for_setting(settings.music_volume));

            if sink.empty() {
                warn!("Hub music sink is empty, respawning music entity");
                commands.entity(controller.entity).despawn();
                controller.entity = spawn_hub_music_entity(
                    &mut commands,
                    controller.handle.clone(),
                    settings.music_volume,
                );
                controller.logged_sink_ready = false;
            }
        } else if !sink.is_paused() {
            sink.pause();
        }
    } else {
        controller.missing_sink_frames += 1;

        if should_play && controller.missing_sink_frames > 120 && !controller.warned_missing_sink {
            warn!(
                "Hub music loaded but AudioSink unavailable. This usually means no audio output device was found."
            );
            controller.warned_missing_sink = true;
        }
    }
}

fn spawn_hub_music_entity(
    commands: &mut Commands,
    handle: Handle<AudioSource>,
    volume_setting: f32,
) -> Entity {
    commands
        .spawn((
            HubMusicEntity,
            AudioPlayer::new(handle),
            PlaybackSettings::LOOP
                .with_volume(get_volume_for_setting(volume_setting))
                .paused(),
        ))
        .id()
}

fn choose_music_path() -> &'static str {
    // Both files should exist in assets/music/.
    // We prefer the lowercase one as it's more standard for the project.
    HUB_MUSIC_PRIMARY
}
