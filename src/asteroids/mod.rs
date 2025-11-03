use bevy::prelude::*;

pub mod collision;
pub mod movement;
pub mod spawn;
pub mod utils;

#[derive(Component)]
pub struct Asteroid {
    pub size: f32, // 1.0 - 10.0
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Resource)]
pub struct AsteroidAssets {
    mesh: Handle<Mesh>,
    materials: Vec<Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct AsteroidConfig {
    max_asteroid: usize,
    size_range: (f32, f32),
    spawn_range: f32,
    despawn_range: f32,
}

const ANIMATION_DURATION: f32 = 0.5;

#[derive(Component)]
pub struct SpawnAnimation {
    timer: Timer,
}

#[derive(Component)]
pub struct DespawnAnimation {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    movement::move_asteroids,
                    collision::asteroid_asteroid_collision,
                    spawn::asteroid_wave,
                    spawn::animate_spawn,
                    spawn::animate_despawn,
                    spawn::clear_asteroid, // collision::asteroid_collision_system,
                ),
            )
            .insert_resource(AsteroidConfig {
                max_asteroid: 50,
                size_range: (1.0, 10.0),
                spawn_range: 300.0,
                despawn_range: 350.0,
            });
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(1.0).mesh().uv(32, 32));

    let materials_vec = (0..10)
        .map(|i| {
            materials.add(StandardMaterial {
                base_color: Color::hsl(i as f32 * 36.0, 0.8, 0.6),
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            })
        })
        .collect::<Vec<_>>();

    commands.insert_resource(AsteroidAssets {
        mesh: sphere_mesh,
        materials: materials_vec,
    });

    let sun_position = Vec3::new(-1000.0, 1000.0, 0.0);
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.988, 0.482, 0.0667),
        emissive: Color::srgb(0.988, 0.482, 0.0667).to_linear() * 2.0,
        unlit: false,
        ..Default::default()
    });
    commands.spawn((
        PointLight {
            intensity: 1_000_000_000_000.0,
            range: 50000.0,
            radius: 500.0,
            color: Color::srgb(1.0, 0.9, 0.0),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(sun_position),
        children![(
            Mesh3d(meshes.add(Sphere::new(500.0).mesh().uv(64, 64))),
            MeshMaterial3d(sun_material),
        )],
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(520.0).mesh().uv(64, 64))), // slightly larger
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.8, 0.2, 0.2), // faint orange glow
            emissive: Color::srgb(1.0, 0.7, 0.2).to_linear() * 5.0,
            unlit: true,
            alpha_mode: AlphaMode::Add, // additive blending for glow effect
            ..Default::default()
        })),
        Transform::from_translation(sun_position),
    ));
}
