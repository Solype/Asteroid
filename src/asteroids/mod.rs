use bevy::{color::palettes::css::WHITE, platform::collections::HashMap, prelude::*};

use crate::game_states::GameState;

pub mod collision;
pub mod movement;
pub mod spawn;
pub mod utils;

#[derive(Component)]
pub struct Asteroid {
    pub size: f32, // 1.0 - 10.0
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct RotationVelocity(Vec3);

#[derive(Resource)]
pub struct AsteroidAssets {
    meshes: HashMap<String, Vec<Handle<Mesh>>>,
    materials: HashMap<String, Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct AsteroidConfig {
    max_asteroid: usize,
    size_range: (f32, f32),
    spawn_range: f32,
    despawn_range: f32,
}
const ASTEROID_SIZE_TYPES_LEN: usize = 6;
const ASTEROID_SIZE_TYPES: [&str; ASTEROID_SIZE_TYPES_LEN] = ["XS", "S", "M", "L", "XL", "XXL"];

const ANIMATION_DURATION: f32 = 0.5;

const SUN_SCALE: f32 = 500.0;

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
                    movement::rotate_asteroids,
                    collision::asteroid_asteroid_collision,
                    collision::asteroid_player_collision,
                    collision::asteroid_ammo_collision,
                    spawn::asteroid_wave,
                    spawn::animate_spawn,
                    spawn::animate_despawn,
                    spawn::clear_asteroid,
                ).run_if(in_state(GameState::Game)),
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
    asset_server: Res<AssetServer>,
) {
    let mut asteroid_meshes: HashMap<String, Vec<Handle<Mesh>>> = Default::default();
    let mut asteroid_materials: HashMap<String, Handle<StandardMaterial>> = Default::default();
    for asteroid_type in ASTEROID_SIZE_TYPES {
        for n in 0..4 {
            let mesh =
                asset_server.load(format!("asteroids/{asteroid_type}{n}.glb#Mesh0/Primitive0"));
            asteroid_meshes
                .entry(asteroid_type.to_string()) // ensure key exists
                .or_default()
                .push(mesh);
        }
        let material = match asteroid_type {
            "XS" => materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.3, 0.6),
                emissive: Color::srgb(0.7, 0.2, 0.2).to_linear() * 0.3,
                metallic: 0.5,
                perceptual_roughness: 0.5,
                ..default()
            }),
            "S" | "M" => materials.add(StandardMaterial {
                base_color: Color::srgb(0.45, 0.4, 0.35),
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            }),
            "L" | "XL" | "XXL" => materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.3, 0.3),
                metallic: 0.1,
                perceptual_roughness: 0.9,
                ..default()
            }),
            _ => materials.add(StandardMaterial::default()),
        };
        asteroid_materials.insert(asteroid_type.to_string(), material);
    }

    commands.insert_resource(AsteroidAssets {
        meshes: asteroid_meshes,
        materials: asteroid_materials,
    });

    let sun_translation = Vec3::new(-1000.0, 1000.0, 0.0);
    let sun_scale = Vec3::new(SUN_SCALE, SUN_SCALE, SUN_SCALE);
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.988, 0.482, 0.0667),
        emissive: Color::srgb(0.988, 0.482, 0.0667).to_linear() * 2.0,
        unlit: false,
        ..Default::default()
    });
    commands.spawn((
        Mesh3d(asset_server.load("Sun.glb#Mesh0/Primitive0")),
        Transform{
            translation: sun_translation,
            scale: sun_scale,
            ..default()
        },
        MeshMaterial3d(sun_material),
        children![(
            PointLight {
                intensity: 1_000_000_000_000.0,
                range: 50000.0,
                radius: 500.0,
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },
        )],
    ));
    commands.spawn((
        Mesh3d(asset_server.load("Sun.glb#Mesh0/Primitive0")), // slightly larger
        Transform{
            translation: sun_translation,
            scale: sun_scale * 1.1,
            ..default()
        },
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(1.0, 0.8, 0.2, 0.2), // faint orange glow
            emissive: Color::srgb(1.0, 0.7, 0.2).to_linear() * 5.0,
            unlit: true,
            alpha_mode: AlphaMode::Add, // additive blending for glow effect
            ..Default::default()
        })),
    ));
}
