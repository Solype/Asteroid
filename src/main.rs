use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::Extent3d;
use bevy::render::camera::Viewport;
// use bevy::render::mesh::shape::{Cuboid, Plane3d};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct ScreenPlane;

#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // --- Texture pour la caméra secondaire ---
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[255, 255, 255, 255],
        bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
        Default::default(),
    );
    image.texture_descriptor.usage = bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT
        | bevy::render::render_resource::TextureUsages::TEXTURE_BINDING;

    let image_handle = images.add(image);

    // --- Cube ---
    let cube_handle = meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0)));
    let cube_material = materials.add(Color::rgb(0.8, 0.2, 0.2));

    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Cube,
    ));

    // --- Plan/écran ---
    let plane_handle = meshes.add(Mesh::from(Plane3d::default().mesh().size(5.0, 5.0)));
    let plane_material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        ..default()
    });

    // Plan vertical, comme un écran devant la caméra principale
    commands.spawn((
        PbrBundle {
            mesh: plane_handle,
            material: plane_material,
            transform: Transform::from_xyz(0.0, 4.0, -4.0),
            ..default()
        },
        ScreenPlane,
    ));

    // --- Lumière ---
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.0,
            range: 20.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 6.0, 4.0),
        ..default()
    });

    // --- Caméra principale ---
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        }, 
        Player
    ));

    // --- Caméra secondaire (rend dans la texture) ---
    commands.spawn(Camera3dBundle {
        camera: Camera {
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}


const SPEED: f32 = 1.5; // vitesse d'orbite

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        // Calculer l'angle autour du cube
        let mut angle = transform.translation.z.atan2(transform.translation.x);
        let radius = (transform.translation.x.powi(2) + transform.translation.z.powi(2)).sqrt();

        // Modifier l'angle selon l'entrée clavier
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            angle += SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            angle -= SPEED * time.delta_seconds();
        }

        // Optionnel : déplacer la caméra vers/loin du cube
        let mut height = transform.translation.y;
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            height += SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            height -= SPEED * time.delta_seconds();
        }

        // Recalculer la position de la caméra
        transform.translation.x = radius * angle.cos();
        transform.translation.z = radius * angle.sin();
        transform.translation.y = height;

        // Toujours regarder le cube
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
