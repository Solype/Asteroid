use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3d;

use crate::game_over::GameOverState;
use crate::game_states::GameState;
use crate::physics::Velocity;

pub mod collision;
pub mod spawn;
pub mod utils;

#[derive(Component)]
pub struct Asteroid {
    pub size: f32, // 1.0 - 10.0
}
#[derive(Component)]
pub struct Sun {
    pub size: f32, // 1.0
}

#[derive(Resource)]
pub struct AsteroidAssets {
    meshes: HashMap<String, Vec<Handle<Mesh>>>,
    materials: HashMap<String, Handle<StandardMaterial>>,
    explosion_sheet: Handle<Image>,
    explosion_layout: Handle<TextureAtlasLayout>,
    sun_meshes: [Handle<Mesh>; 2],
    sun_materials: [Handle<StandardMaterial>; 3],
}

#[derive(Resource)]
pub struct BoomSounds {
    pub booms: Vec<Handle<AudioSource>>,
}

const ASTEROID_SIZE_TYPES_LEN: usize = 6;
const ASTEROID_SIZE_TYPES: [&str; ASTEROID_SIZE_TYPES_LEN] = ["XS", "S", "M", "L", "XL", "XXL"];

const ANIMATION_DURATION: f32 = 0.5;

const SUN_SIZE: f32 = 500.0;

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
                    spawn::animate_spawn,
                    spawn::animate_despawn,
                    spawn::animate_despawn_sun,
                ),
            )
            .add_systems(
                Update,
                (
                    collision::asteroid_asteroid_collision,
                    collision::asteroid_ammo_collision,
                    spawn::asteroid_wave,
                    spawn::clear_asteroid,
                )
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (
                    collision::asteroid_player_collision,
                    collision::sun_player_collision,
                )
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(GameOverState::None)),
            );
    }
}

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
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

    let sun_translation = Vec3::new(-1000.0, 1000.0, 0.0);
    let sun_scale = Vec3::new(SUN_SIZE, SUN_SIZE, SUN_SIZE);

    let sun_mesh = asset_server.load("Sun.glb#Mesh0/Primitive0");
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.988, 0.482, 0.0667),
        emissive: Color::srgb(0.988, 0.482, 0.0667).to_linear() * 2.0,
        unlit: false,
        ..Default::default()
    });
    let sun_aura_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.8, 0.2, 0.2), // faint orange glow
        emissive: Color::srgb(1.0, 0.7, 0.2).to_linear() * 5.0,
        unlit: true,
        alpha_mode: AlphaMode::Add, // additive blending for glow effect
        ..Default::default()
    });

    let wireframe_mesh = asset_server.load("sun_wireframe.glb#Mesh0/Primitive0");
    let wireframe_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.1),
        unlit: true,
        ..Default::default()
    });
    commands.spawn((
        Sun { size: SUN_SIZE },
        Mesh3d(sun_mesh.clone()),
        Transform {
            translation: sun_translation,
            scale: sun_scale,
            ..default()
        },
        Velocity(Vec3::ZERO),
        MeshMaterial3d(sun_material.clone()),
        children![
            (PointLight {
                intensity: 2. * SUN_SIZE * 1_000_000_000.0,
                range: SUN_SIZE * 100.0,
                radius: SUN_SIZE,
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },),
            (
                Mesh3d(wireframe_mesh.clone()), // wireframe
                Transform {
                    scale: Vec3::new(1.001, 1.001, 1.001),
                    ..default()
                },
                MeshMaterial3d(wireframe_material.clone()),
            ),
            (
                Mesh3d(sun_mesh.clone()), // slightly larger
                Transform {
                    scale: Vec3::new(1.1, 1.1, 1.1),
                    ..default()
                },
                MeshMaterial3d(sun_aura_material.clone()),
            )
        ],
    ));

    commands.insert_resource(AsteroidAssets {
        meshes: asteroid_meshes,
        materials: asteroid_materials,
        explosion_sheet: asset_server.load("explosion_sheet.png"),
        explosion_layout: texture_atlases.add(TextureAtlasLayout::from_grid(
            UVec2::new(232, 232),
            7,
            1,
            None,
            None,
        )),
        sun_meshes: [sun_mesh, wireframe_mesh],
        sun_materials: [sun_material, sun_aura_material, wireframe_material],
    });
    commands.insert_resource(BoomSounds {
        booms: vec![
            asset_server.load("sounds/boom1.wav"),
            asset_server.load("sounds/boom2.wav"),
        ],
    });
}
