use bevy::{audio::Volume, prelude::*};

use crate::{game_over::GameOverState, game_states::GameState, globals_structs::MusicVolume, menu::structs::MenuState};

#[derive(Component)]
pub struct BackgroundMusic;

pub struct BackgroundMusicPlugin;

#[derive(Resource, Default)]
struct MusicResources {
    pub menu_music: Handle<AudioSource>,
    pub game_music: Handle<AudioSource>,
    pub gameover_music: Handle<AudioSource>
}

impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(MenuState::Main), start_menu_music)
            .add_systems(OnEnter(GameState::Game), start_game_music)
            .add_systems(OnEnter(GameOverState::Drift), start_gameover_music);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameconfig: Res<crate::config::structs::GameConfig>
) {
    commands.spawn((BackgroundMusic,));
    let mut resource = MusicResources::default();

    resource.menu_music = asset_server.load(gameconfig.ui.music.clone());
    resource.game_music = asset_server.load(gameconfig.ship.music.clone());
    resource.gameover_music = asset_server.load(gameconfig.ship.alarm.clone());
    commands.insert_resource(resource);
}

fn start_menu_music(
    mut commands: Commands,
    music_resource: Res<MusicResources>,
    music: Single<(Entity, Option<&AudioSink>), With<BackgroundMusic>>,
    master_volume: Res<MusicVolume>,
) {
    if let Some(sink) = music.1 {
        sink.stop();
    }
    commands.entity(music.0).despawn();

    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(music_resource.menu_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(master_volume.volume / 100.0_f32)),
    ));
}

fn start_game_music(
    mut commands: Commands,
    music_resource: Res<MusicResources>,
    music: Single<(Entity, Option<&AudioSink>), With<BackgroundMusic>>,
    master_volume: Res<MusicVolume>,
) {
    if let Some(sink) = music.1 {
        sink.stop();
    }
    commands.entity(music.0).despawn();

    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(music_resource.game_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(master_volume.volume / 100.0_f32)),
    ));
}

fn start_gameover_music(
    mut commands: Commands,
    music_resource: Res<MusicResources>,
    music: Single<(Entity, Option<&AudioSink>), With<BackgroundMusic>>,
    master_volume: Res<MusicVolume>,
) {
    if let Some(sink) = music.1 {
        sink.stop();
    }
    commands.entity(music.0).despawn();

    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(music_resource.gameover_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(master_volume.volume / 100.0_f32)),
    ));
}
