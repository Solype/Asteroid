use bevy::{audio::Volume, prelude::*};

use crate::{game_over::GameOverState, game_states::GameState, globals_structs::MusicVolume, menu::structs::MenuState};

#[derive(Component)]
pub struct BackgroundMusic;

pub struct BackgroundMusicPlugin;

impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnEnter(MenuState::Main), start_menu_music)
            .add_systems(OnEnter(GameState::Game), start_game_music)
            .add_systems(OnEnter(GameOverState::Drift), start_gameover_music);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((BackgroundMusic,));
}

fn start_menu_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music: Single<(Entity, Option<&AudioSink>), With<BackgroundMusic>>,
    master_volume: Res<MusicVolume>,
) {
    if let Some(sink) = music.1 {
        sink.stop();
    }
    commands.entity(music.0).despawn();

    commands.spawn((
        BackgroundMusic,
        AudioPlayer::new(asset_server.load("sounds/menu.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(master_volume.volume / 100.0_f32)),
    ));
}

fn start_game_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music: Single<(Entity, Option<&AudioSink>), With<BackgroundMusic>>,
    master_volume: Res<MusicVolume>,
) {
    if let Some(sink) = music.1 {
        sink.stop();
    }
    commands.entity(music.0).despawn();

    commands.spawn((
        BackgroundMusic,
    AudioPlayer::new(asset_server.load("sounds/game.wav")),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(master_volume.volume / 100.0_f32)),
    ));
}

fn start_gameover_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    music: Single<(Entity, Option<&AudioSink>), With<BackgroundMusic>>,
    master_volume: Res<MusicVolume>,
) {
    if let Some(sink) = music.1 {
        sink.stop();
    }
    commands.entity(music.0).despawn();

    commands.spawn((BackgroundMusic,));
}
