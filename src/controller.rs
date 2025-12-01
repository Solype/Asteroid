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
            move_player_system
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
pub struct RotationalVelocity(Vec3);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec3::new(0.003, 0.002, 0.002))
    }
}

impl Default for RotationalVelocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
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

fn move_player_system(
    time: Res<Time>,
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Single<(&mut Transform, &mut crate::asteroids::Velocity), With<Player>>,
) {
    let mut speed_to_add = Vec3::default();
    let (mut transform, mut velocity) = player.into_inner();

    if keybinds.right.pressed(&keyboard, &mouse) {
        speed_to_add.x += 1.0_f32;
    }
    if keybinds.left.pressed(&keyboard, &mouse) {
        speed_to_add.x += -1.0_f32;
    }
    if keybinds.forward.pressed(&keyboard, &mouse) {
        speed_to_add.z += -1.0_f32;
    }
    if keybinds.backward.pressed(&keyboard, &mouse) {
        speed_to_add.z += 1.0_f32;
    }
    if keybinds.up.pressed(&keyboard, &mouse) {
        speed_to_add.y += 1.0_f32;
    }

    if keybinds.down.pressed(&keyboard, &mouse) {
        speed_to_add.y += -1.0_f32;
    }

    if speed_to_add.length_squared() == 0.0 {
        return;
    }
    speed_to_add = speed_to_add.normalize() * 2.0;
    speed_to_add *= time.delta_secs();
    speed_to_add = transform.rotation * speed_to_add;
    transform.translation += speed_to_add * time.delta_secs();
    velocity.0 += speed_to_add;
    if velocity.0.length() > 10.0_f32 {
        velocity.0 = velocity.0.normalize() * 10.0_f32;
    }
}