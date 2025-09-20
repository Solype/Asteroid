use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- Camera ---
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // --- Point Light ---
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000000.0, // luminosité
            // range: .0,       // portée
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // --- Sphere (player) ---
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.5).mesh().uv(32, 18)),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    ));
}

