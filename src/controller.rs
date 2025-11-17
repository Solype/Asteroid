use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::game_states::GameState;
use std::f32::consts::FRAC_PI_2;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), grab_mouse);
    app.add_systems(
        Update,
        (player_cam_system, player_system)
            .in_set(GameSystemSet)
            .run_if(in_state(GameState::Game)),
    );
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

fn grab_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>)
{
    options.visible = false;
    options.grab_mode = CursorGrabMode::Locked;
    // options.grab_mode = match cfg!(target_os = "macos") {
    //     true => CursorGrabMode::Locked,
    //     false => CursorGrabMode::Confined
    // }
}

fn player_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &CameraSensitivity, &mut TranslationalVelocity, &mut RotationalVelocity), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let (mut transform, _camera_sensitivity , _trans_velocity, mut rota_velocity) = player.into_inner();

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Menu);
    }

    let base_speed = 200.0_f32.to_radians(); // ≈3.49 rad/s
    let dt = time.delta_secs();

    let mut accel_yaw   = 0.0;
    let mut accel_pitch = 0.0;
    let mut accel_roll  = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        accel_pitch -= base_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        accel_pitch += base_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        accel_yaw += base_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        accel_yaw -= base_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        accel_roll += base_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        accel_roll -= base_speed;
    }

    // Apply input acceleration to angular velocity
    rota_velocity.x += accel_yaw * dt;
    rota_velocity.y += accel_pitch * dt;
    rota_velocity.z += accel_roll * dt;

    const DAMPING: f32 = 0.99f32; // 1.0 = no damping
    rota_velocity.x *= DAMPING;
    rota_velocity.y *= DAMPING;
    rota_velocity.z *= DAMPING;

    let delta_yaw   = rota_velocity.x * dt;
    let delta_pitch = rota_velocity.y * dt;
    let delta_roll  = rota_velocity.z * dt;

    // Build a small rotation from the angular increments
    let delta_rot = Quat::from_euler(EulerRot::YXZ, delta_yaw, delta_pitch, delta_roll);
    println!("{:?}", transform.translation.x);
    println!("{:?}", transform.translation.y);
    println!("{:?}", transform.translation.z);
    // Apply to the transform
    transform.rotation *= delta_rot;
}

fn player_cam_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<PlayerCam>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let mut yaw = yaw + delta_yaw;
        let mut pitch = pitch + delta_pitch;
        // const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        // pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
        // const YAW_LIMIT: f32 = FRAC_PI_2;
        // yaw = yaw.clamp(-YAW_LIMIT, YAW_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
