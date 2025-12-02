use bevy::prelude::*;

use crate::{
    controller::Player, game_states::GameState, menu::structs::MenuState,
    menu::systems::focus_main_screen,
};

mod drift;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameOverState {
    #[default]
    None,
    Drift,
    Menu,
}

#[derive(Resource)]
pub struct DriftTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct BlinkingLight {
    pub timer: Timer,
    pub on_intensity: f32,
    pub off_intensity: f32,
    pub is_on: bool,
}
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameOverState>()
            .add_systems(OnEnter(GameOverState::Drift), setup)
            .add_systems(
                Update,
                blink_lights.run_if(not(in_state(GameOverState::None))),
            )
            .add_systems(
                Update,
                drift::run_drift.run_if(in_state(GameOverState::Drift)),
            )
            .add_systems(OnEnter(GameOverState::Menu), focus_main_screen)
            .add_systems(OnExit(MenuState::GameOver), reset_gameover_state);
    }
}

fn setup(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands.insert_resource(DriftTimer {
        timer: Timer::from_seconds(3.0, TimerMode::Once),
    });

    commands.entity(*player).with_children(|parent_builder| {
        parent_builder.spawn((
            DespawnOnExit(MenuState::GameOver),
            PointLight {
                intensity: 1_000_000.0,
                range: 10.0,
                radius: 10.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                shadows_enabled: true,
                ..default()
            },
            Transform {
                translation: Vec3::new(1.0, 0.5, 0.5),
                ..default()
            },
            BlinkingLight {
                timer: Timer::from_seconds(0.7, TimerMode::Repeating),
                on_intensity: 1_000_000.0,
                off_intensity: 0.0,
                is_on: true,
            },
        ));
        parent_builder.spawn((
            DespawnOnExit(MenuState::GameOver),
            PointLight {
                intensity: 0.0,
                range: 10.0,
                radius: 10.0,
                color: Color::srgb(1.0, 0.0, 0.0),
                shadows_enabled: true,
                ..default()
            },
            Transform {
                translation: Vec3::new(-1.0, 0.5, 0.5),
                ..default()
            },
            BlinkingLight {
                timer: Timer::from_seconds(0.7, TimerMode::Repeating),
                on_intensity: 1_000_000.0,
                off_intensity: 0.0,
                is_on: false,
            },
        ));
    });
}

fn blink_lights(time: Res<Time>, mut lights: Query<(&mut PointLight, &mut BlinkingLight)>) {
    for (mut light, mut blinking) in &mut lights {
        blinking.timer.tick(time.delta());

        if blinking.timer.just_finished() {
            blinking.is_on = !blinking.is_on;

            light.intensity = if blinking.is_on {
                blinking.on_intensity
            } else {
                blinking.off_intensity
            };
        }
    }
}

fn reset_gameover_state(mut next_gameover_state: ResMut<NextState<GameOverState>>) {
    next_gameover_state.set(GameOverState::None);
}
