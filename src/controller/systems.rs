use crate::controller::structs::{Player, RotationalVelocity, VirtualMouse};
use std::f32::consts::FRAC_PI_2;
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::log::info;
use bevy::math::{EulerRot, Quat, Vec2, Vec3};
use bevy::prelude::{default, Commands, DespawnOnExit, Entity, ImageNode, KeyCode, MouseButton, NextState, Node, ParamSet, PositionType, Res, ResMut, Single, Time, Transform, UiTargetCamera, Val, Window, With};
use bevy::window::PrimaryWindow;
use crate::controller::structs::{CameraSensitivity, PlayerCam};
use crate::game_states::GameState;
use crate::globals_structs::Keybinds;

pub fn player_system(
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
            info!("yaw: {}", yaw);
            info!("pitch: {}", pitch);
            info!("roll: {}", roll);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }
}

pub fn move_player_system(
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

pub fn mouse_system(
    mut params: ParamSet<(
        Single<(&mut Node, &mut VirtualMouse)>,
        Single<&Window, With<PrimaryWindow>>,
    )>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let win_dim: Vec2 = Vec2 {
        x: params.p1().width(),
        y: params.p1().height(),
    };

    let center = Vec2::new(win_dim.x / 2.0, win_dim.y / 2.0);
    let radius = 150.0;

    let (mut node, mut virtual_mouse) = params.p0().into_inner();

    virtual_mouse.pos += accumulated_mouse_motion.delta;

    // Clamp the virtual_mouse.pos to the circle
    let pos_from_center = virtual_mouse.pos;
    let dist_from_center = pos_from_center.length();

    if dist_from_center > radius {
        // Normalize and scale to radius
        let direction = pos_from_center.normalize_or_zero();
        virtual_mouse.pos = direction * radius;
    }

    let ui_pos = center + virtual_mouse.pos;
    node.left = Val::Px(ui_pos.x);
    node.top = Val::Px(ui_pos.y);
}

pub fn rotate_spaceship(
    mut params: ParamSet<(
        Single<&mut Transform, With<Player>>,
        Single<&VirtualMouse>
    )>,
    time: Res<Time>,
) {
    let mouse_pos: Vec2;
    { mouse_pos = params.p1().pos; }
    let mut transform = params.p0().into_inner();

    let mouse_offset = mouse_pos;

    if mouse_offset.length_squared() > 0.03 {
        let speed: f32 = 0.005;

        let target_angle_y = -mouse_offset.x * speed;
        transform.rotate_local_y(target_angle_y * time.delta_secs());

        let target_angle_x = -mouse_offset.y * speed;
        transform.rotate_local_x(target_angle_x * time.delta_secs());
    }

}

pub fn roll_spaceship(
    time: Res<Time>,
    player: Single<(&mut Transform, &mut RotationalVelocity), With<Player>>,
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let (mut transform, mut rota_velocity) = player.into_inner();

    let base_speed = 200.0_f32.to_radians(); // ≈3.49 rad/s
    let dt = time.delta_secs();

    let mut accel_roll  = 0.0;

    if keybinds.rotate_left.pressed(&keyboard, &mouse) {
        accel_roll += base_speed;
    }
    if keybinds.rotate_right.pressed(&keyboard, &mouse) {
        accel_roll -= base_speed;
    }

    // Apply input acceleration to angular velocity
    rota_velocity.z += accel_roll * dt;

    const DAMPING: f32 = 0.99f32; // 1.0 = no damping
    if rota_velocity.z.abs() < 2.0 { rota_velocity.z *= DAMPING; }

    rota_velocity.z = rota_velocity.z.clamp(-5.0, 5.0);

    let delta_roll  = rota_velocity.z * dt;

    // Build a small rotation from the angular increments
    let delta_rot = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, delta_roll);
    transform.rotation *= delta_rot;
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    target_camera: Single<Entity, With<PlayerCam>>,
) {
    let parent = commands
        .spawn((
            DespawnOnExit(crate::GameState::Game),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .id();

    let mouse_node = commands.spawn((
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ImageNode {
            image: asset_server.load("niko.jpeg"),
            ..default()
        },
        VirtualMouse::default(),
        UiTargetCamera(target_camera.into_inner()),
    ))
        .id();

    commands.entity(parent).add_child(mouse_node);
}
