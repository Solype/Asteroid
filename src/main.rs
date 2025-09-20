use bevy::{
    color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster,
    prelude::*, render::view::RenderLayers,
    window::PrimaryWindow
};

use bevy::render::camera::Viewport;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use std::f32::consts::FRAC_PI_2;



#[derive(Resource)]
struct RockIndex(usize);

#[derive(Component)]
struct Rock;

#[derive(Resource)]
struct RockAssets {
    scenes: Vec<Handle<Scene>>,
    current: usize,
}




#[derive(Component)]
struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RockIndex(0))
        .add_systems(Startup, setup)
        .add_systems(Startup, grab_mouse)
        // .add_systems(Startup, setup_rocks)
        // .add_systems(Update, cycle_rocks)
        .add_systems(Update, player_movement)
        .run();
}






fn setup_rocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut scenes = Vec::new();
    for i in 0..10 {
        let path = format!("Rock{}.glb#Scene0", i);
        scenes.push(asset_server.load(path));
    }

    let first_scene = scenes[0].clone();
    commands.insert_resource(RockAssets { scenes, current: 0 });

    commands.spawn((
        SceneRoot(first_scene),
        Transform::from_xyz(0.0, 2.0, 0.0),
        GlobalTransform::default(),
        Rock,
    ));
}

fn cycle_rocks(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut assets: ResMut<RockAssets>,
    query: Query<Entity, With<Rock>>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        // supprimer l’ancien
        for entity in &query {
            commands.entity(entity).despawn();
        }

        // passer au suivant
        assets.current = (assets.current + 1) % assets.scenes.len();
        let scene = assets.scenes[assets.current].clone();

        // spawn instantané
        commands.spawn((
            SceneRoot(scene),
            Transform::from_xyz(0.0, 2.0, 0.0),
            GlobalTransform::default(),
            Rock,
        ));
    }
}






fn grab_mouse(mut window: Single<&mut Window>) {
    window.cursor_options.visible = !window.cursor_options.visible;
    window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
    };
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Camera { order: 0, ..default() },
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Player,
        CameraSensitivity::default(),
    ));

    // commands.spawn((
    //     Camera3d::default(),
    //     Camera {
    //         order: 1,
    //         viewport: Some(Viewport {
    //             physical_position: UVec2::new(50, 50),
    //             physical_size: UVec2::new(300, 200),
    //             ..default()
    //         }),
    //         ..default()
    //     },
    //     Transform::from_xyz(0.0, 2000.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

    commands.spawn((
        DirectionalLight { illuminance: 20_000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler( EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0, )),
        GlobalTransform::default(),
    ));

    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 20.0),
        GlobalTransform::default(),
    ));

    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(-10.0, 0.0, 10.0),
        GlobalTransform::default(),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(10.0, 0.0, 10.0),
        GlobalTransform::default(),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(0.0, -10.0, 10.0),
        GlobalTransform::default(),
    ));
    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(0.0, 10.0, 10.0),
        GlobalTransform::default(),
    ));
}


const SPEED: f32 = 2.0;
const MOUSE_SENSITIVITY: f32 = 0.002;

fn player_movement(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
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

        if let Ok(mut window) = windows.get_single_mut() {
            let center = window.resolution.size() / 2.0;
            let _ = window.set_cursor_position(Some(center));
        }
    }
}



