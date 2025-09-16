use bevy::{
    // color::palettes::css::*,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::camera::{Exposure, PhysicalCameraParameters, Viewport},
};

#[derive(Component)]
struct Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mon jeu 3D avec 2 caméras".to_string(),
                resolution: (1024., 768.).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Caméra principale
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: 0, // priorité (affiche en premier)
            ..default()
        },
        ..default()
    });

    // Caméra secondaire (vue du dessus, mini-map)
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.01).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(20, 20),   // coin bas-gauche
                physical_size: UVec2::new(300, 200),    // taille mini-écran
                ..default()
            }),
            order: 1, // dessine après la caméra principale
            ..default()
        },
        ..default()
    });

    
    // Lumière directionnelle
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_rotation(Quat::from_euler(
    //         EulerRot::XYZ,
    //         -std::f32::consts::FRAC_PI_4,
    //         std::f32::consts::FRAC_PI_8,
    //         0.0,
    //     )),
    //     ..default()
    // });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10_000_000.0, // luminosité
            // range: .0,       // portée
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 4.0),
        ..default()
    });

    // Sphère jouable
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    ));

    // Sol
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}


const SPEED: f32 = 2.0;

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        for mut transform in query.iter_mut() {
            transform.translation += direction * SPEED * time.delta_seconds();
        }
    }
}
