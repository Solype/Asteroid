use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::PrimaryWindow
};

// use bevy::render::camera::Viewport;
// use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use std::f32::consts::FRAC_PI_2;




#[derive(Component)]
struct Rock;

#[derive(Resource)]
struct RockAssets {
    scenes: Vec<Handle<Scene>>,
    current: usize,
}




#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCam;


#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, grab_mouse)
        .add_systems(Startup, setup_rocks)
        .add_systems(Update, cycle_rocks)
        .add_systems(Update, player_cam_system)
        .add_systems(Update, player_system)
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
        for entity in &query {
            commands.entity(entity).despawn();
        }

        assets.current = (assets.current + 1) % assets.scenes.len();
        let scene = assets.scenes[assets.current].clone();

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
    // mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // --- Création du cube parent ---
    // let cube_handle = meshes.add(Cuboid::new(5.0, 5.0, 5.0));
    let parent = commands
        .spawn((
            // Mesh3d(cube_handle),
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    // --- Création de la caméra enfant ---
    let camera = commands
        .spawn((
            Camera3d::default(),
            Camera { order: 0, ..default() },
            Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            PlayerCam,
            CameraSensitivity::default(),
        ))
        .id();

    // --- Lien parent → enfant ---
    commands.entity(parent).add_child(camera);

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


fn player_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let mut delta_yaw = 0.0;
    let mut delta_pitch = 0.0;

    // 🔹 Contrôles ZQSD
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
        // Facteur de vitesse (dépendant du temps)
        let delta_yaw = delta_yaw * camera_sensitivity.x * time.delta_secs();
        let delta_pitch = delta_pitch * camera_sensitivity.y * time.delta_secs();

        // Extraire angles actuels
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        let mut yaw = yaw + delta_yaw;
        let mut pitch = pitch + delta_pitch;

        // Limite verticale
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        pitch = pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);

        // Limite horizontale (si tu veux limiter à gauche/droite)
        const YAW_LIMIT: f32 = std::f32::consts::PI; // ici 180°
        yaw = yaw.clamp(-YAW_LIMIT, YAW_LIMIT);

        // Appliquer la nouvelle rotation
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}


fn player_cam_system(
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



