use crate::controller::structs::{CameraSensitivity, PlayerCam};
use crate::controller::structs::{ControllerState, Player, RotationalVelocity, VirtualMouse};
use crate::controller::DrivingUI;
use crate::game_states::GameState;
use crate::globals_structs::Keybinds;
use crate::menu::structs::SmoothCamMove;
use bevy::asset::{AssetServer, Handle};
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec2, Vec3};
use bevy::prelude::*;
use bevy::ui::{BorderRadius, UiRect};
use bevy::window::PrimaryWindow;
use std::f32::consts::FRAC_PI_2;

pub fn enter_driving_mod(
    mut command: Commands,
    entity: Single<Entity, With<PlayerCam>>,
    mut ui_entities: Query<&mut Visibility, With<DrivingUI>>,
) {
    let player = entity.into_inner();

    command.entity(player).insert(SmoothCamMove {
        speed: Some(3.0),
        fov: Some(60.0_f32.to_radians()),
        position: Some(Vec3::new(0.0, 1.2, 0.3)),
        look_at: Some(Vec3::new(0.0, 1.2, 0.0)),
        ..Default::default()
    });

    for mut ui_visibility in &mut ui_entities {
        ui_visibility.toggle_visible_hidden();
    }
}

pub fn enter_free_look_mod(
    mut commands: Commands,
    cam_move_entity_option: Option<Single<Entity, With<SmoothCamMove>>>,
    mut ui_entities: Query<&mut Visibility, With<DrivingUI>>,
) {
    if let Some(cam_move_entity) = cam_move_entity_option {
        commands.entity(*cam_move_entity).remove::<SmoothCamMove>();
    }

    for mut ui_visibility in &mut ui_entities {
        ui_visibility.toggle_visible_hidden();
    }
}

pub fn player_system(
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut next_state_game: ResMut<NextState<GameState>>,
    mut next_state_gamemod: ResMut<NextState<ControllerState>>,
) {
    if keybinds.menu.just_pressed(&keyboard, &mouse) {
        next_state_game.set(GameState::Menu);
    }
    if keybinds.free_look.just_pressed(&keyboard, &mouse) {
        next_state_gamemod.set(ControllerState::FreeLook);
    }
    if keybinds.free_look.just_released(&keyboard, &mouse) {
        next_state_gamemod.set(ControllerState::Driving);
    }
}

pub fn free_look_system(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<PlayerCam>>,
    mut vm: Single<&mut VirtualMouse>,
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
    }

    let decay_speed = 2.0; // higher = faster return
    vm.pos = vm.pos.lerp(Vec2::ZERO, decay_speed * time.delta_secs());
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
    node.left = Val::Px(ui_pos.x - 16.);
    node.top = Val::Px(ui_pos.y - 16.);
}

pub fn rotate_spaceship(
    mut transform: Single<&mut Transform, With<Player>>,
    mut vm: Single<&mut VirtualMouse>,
    time: Res<Time>,
) {
    // --- Configurable values ---
    let dead_radius = 24.0; // No rotation inside this radius
    let max_radius = 150.0; // Where rotation reaches full speed
    let base_speed = 1.0;

    let offset = vm.pos;
    let dist = offset.length();

    let delta = time.delta_secs();
    if dist > dead_radius {
        let scaled = ((dist - dead_radius) / (max_radius - dead_radius)).clamp(0.0, 1.0);

        let effective_offset = offset.normalize() * scaled;

        transform.rotate_local_y(-effective_offset.x * base_speed * delta);
        transform.rotate_local_x(-effective_offset.y * base_speed * delta);
    } else if dist > 0.0 {
        let return_speed = 15.0; // higher = faster return

        let step = return_speed * delta;
        if dist <= step {
            vm.pos = Vec2::ZERO;
        } else {
            vm.pos -= offset.normalize() * step;
        }
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

    let mut accel_roll = 0.0;

    if keybinds.rotate_left.pressed(&keyboard, &mouse) {
        accel_roll += base_speed;
    }
    if keybinds.rotate_right.pressed(&keyboard, &mouse) {
        accel_roll -= base_speed;
    }

    // Apply input acceleration to angular velocity
    rota_velocity.z += accel_roll * dt;

    const DAMPING: f32 = 0.99f32; // 1.0 = no damping
    if rota_velocity.z.abs() < 2.0 {
        rota_velocity.z *= DAMPING;
    }

    rota_velocity.z = rota_velocity.z.clamp(-5.0, 5.0);

    let delta_roll = rota_velocity.z * dt;

    // Build a small rotation from the angular increments
    let delta_rot = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, delta_roll);
    transform.rotation *= delta_rot;
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut params: ParamSet<(
        Single<Entity, With<PlayerCam>>,
        Single<&Window, With<PrimaryWindow>>,
    )>,
) {
    let win_dim: Vec2;
    {
        win_dim = Vec2 {
            x: params.p1().width(),
            y: params.p1().height(),
        };
    }
    let target_camera = params.p0();

    let cursor: Handle<Image> = asset_server.load("cursor.png");
    let cursor_external: Handle<Image> = asset_server.load("cursor_external.png");
    let size = 300.0;

    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,
            ..default()
        },
        Visibility::Hidden,
        DrivingUI,
        children![
            (
                Node {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode {
                    image: cursor.clone(),
                    ..default()
                },
                VirtualMouse::default(),
                UiTargetCamera(target_camera.into_inner()),
            ),
            (
                Node {
                    left: Val::Px(win_dim.x / 2. - 24.),
                    top: Val::Px(win_dim.y / 2. - 24.),
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode {
                    image: cursor_external.clone(),
                    ..default()
                },
            ),
            (
                Node {
                    width: Val::Px(size),
                    height: Val::Px(size),
                    left: Val::Px(win_dim.x / 2. - size / 2.),
                    top: Val::Px(win_dim.y / 2. - size / 2.),
                    position_type: PositionType::Absolute,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderRadius::all(Val::Percent(50.0)),
            )
        ],
    ));
}
