use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game_states::GameState;
use std::f32::consts::FRAC_PI_2;
use crate::globals_structs::Keybinds;
use crate::direction_controller;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), grab_mouse);
    app.add_systems(
        Update,
        (player_system,
         direction_controller::mouse_system,
         direction_controller::rotate_spaceship,
         direction_controller::roll_spaceship,
         direction_controller::trans_spaceship
        )
            .in_set(GameSystemSet)
            .run_if(in_state(GameState::Game)),
    );
    app.add_systems(OnEnter(GameState::Game), direction_controller::setup_ui);
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameSystemSet;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCam;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct TranslationalVelocity(Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct RotationalVelocity(Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct MouseVector(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec3::new(0.003, 0.002, 0.002))
    }
}

impl Default for TranslationalVelocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

impl Default for RotationalVelocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

impl Default for MouseVector {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

fn grab_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>)
{
    options.visible = false;
    options.grab_mode = CursorGrabMode::Locked;
}

fn player_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<PlayerCam>>,
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keybinds.menu.pressed(&keyboard, &mouse) {
        next_state.set(GameState::Menu);
    }
    if keybinds.free_look.pressed(&keyboard, &mouse) {
        let (mut transform, camera_sensitivity) = player.into_inner();
        let delta = accumulated_mouse_motion.delta;

        if delta != Vec2::ZERO {
            let delta_yaw = -delta.x * camera_sensitivity.x;
            let delta_pitch = -delta.y * camera_sensitivity.y;
            let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
            let mut yaw = yaw + delta_yaw;
            let mut pitch = pitch + delta_pitch;
            const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
            pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
            const YAW_LIMIT: f32 = FRAC_PI_2;
            yaw = yaw.clamp(-YAW_LIMIT, YAW_LIMIT);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}
