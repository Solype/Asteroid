use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct RockIndex(usize);

#[derive(Component)]
struct Rock;

#[derive(Resource)]
struct RockAssets {
    scenes: Vec<Handle<Scene>>,
    current: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RockIndex(0))
        .add_systems(Startup, setup)
        .add_systems(Startup, grab_mouse)
        .add_systems(Startup, setup_rocks)
        .add_systems(Update, cycle_rocks)
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

fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Caméra principale
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Player
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            viewport: Some(Viewport {
                physical_position: UVec2::new(50, 50),
                physical_size: UVec2::new(300, 200),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 2000.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 20_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
        GlobalTransform::default(),
    ));

    // Charger un modèle glTF/GLB
    commands.spawn((
        SceneRoot(asset_server.load("Rock0.glb#Scene0")),
        Transform::default(),
        GlobalTransform::default(),
        Rock,
    ));

    commands.spawn((
        SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
        Transform::from_xyz(0.0, 5.0, 0.0),
        GlobalTransform::default(),
    ));
}


const SPEED: f32 = 2.0;
const MOUSE_SENSITIVITY: f32 = 0.002;

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut move_dir = Vec3::ZERO;
    let mut pitch = 0.0;
    let mut yaw = 0.0;

    // --- Clavier pour translation ---
    if keyboard_input.pressed(KeyCode::KeyW) {
        move_dir.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        move_dir.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        move_dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        move_dir.x += 1.0;
    }

    // --- Souris pour rotation ---
    for event in mouse_motion_events.read() {
        yaw -= event.delta.x * MOUSE_SENSITIVITY;   // gauche/droite
        pitch -= event.delta.y * MOUSE_SENSITIVITY; // haut/bas
    }

    for mut transform in query.iter_mut() {
        // Appliquer la rotation (yaw autour de Y, pitch autour de X)
        let yaw_rot = Quat::from_rotation_y(yaw);
        let pitch_rot = Quat::from_rotation_x(pitch);
        transform.rotation = yaw_rot * pitch_rot * transform.rotation;

        // Déplacement relatif à l'orientation
        if move_dir != Vec3::ZERO {
            let forward = transform.forward();
            let right = transform.right();
            let movement = (forward * move_dir.z + right * move_dir.x).normalize();
            transform.translation += movement * SPEED * time.delta_secs();
        }
    }
}


