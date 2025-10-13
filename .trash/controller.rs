use bevy::{
    prelude::*,
    window::{PrimaryWindow, CursorGrabMode},
    input::mouse::AccumulatedMouseMotion,
};

use std::f32::consts::FRAC_PI_2;



#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCam;



#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}




pub fn grab_mouse(mut window: Single<&mut Window>) {
    window.cursor_options.visible = !window.cursor_options.visible;
    window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
    };
}


pub fn player_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let mut delta_yaw = 0.0;
    let mut delta_pitch = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        delta_pitch -= 20.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta_pitch += 20.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        delta_yaw += 20.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta_yaw -= 20.0;
    }

    if delta_yaw != 0.0 || delta_pitch != 0.0 {
        let delta_yaw = delta_yaw * camera_sensitivity.x * time.delta_secs();
        let delta_pitch = delta_pitch * camera_sensitivity.y * time.delta_secs();
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let mut yaw = yaw + delta_yaw;
        let mut pitch = pitch + delta_pitch;
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
        const YAW_LIMIT: f32 = std::f32::consts::PI;
        yaw = yaw.clamp(-YAW_LIMIT, YAW_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}


pub fn player_cam_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
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
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
        const YAW_LIMIT: f32 = FRAC_PI_2;
        yaw = yaw.clamp(-YAW_LIMIT, YAW_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        if let Ok(mut window) = windows.single_mut() {
            let center = window.resolution.size() / 2.0;
            let _ = window.set_cursor_position(Some(center));
        }
    }
}
