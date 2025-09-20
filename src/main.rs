use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::TextureViewDimension,
    },
    image::{ImageSampler, ImageSamplerDescriptor},
};


mod controller;
mod rock;
use controller::*;
use rock::*;

/// Ressource pour suivre le chargement du cubemap
#[derive(Resource)]
pub struct SkyCubeMap {
    pub image: Handle<Image>,
    pub loaded: bool,
}

/// Ressource pour stocker l'entité caméra
#[derive(Resource)]
struct CameraHolder(Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, grab_mouse)
        .add_systems(Startup, setup_rocks)
        .add_systems(Update, reinterpret_cubemap)
        .add_systems(Update, cycle_rocks)
        .add_systems(Update, player_cam_system)
        .add_systems(Update, player_system)
        .run();
}

fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyCubeMap>,
    mut commands: Commands,
    camera_holder: Res<CameraHolder>,
) {
    if !cubemap.loaded
        && matches!(
            asset_server.get_load_state(&cubemap.image),
            Some(LoadState::Loaded)
        )
    {
        cubemap.loaded = true;

        let image = images.get_mut(&cubemap.image).unwrap();

        let layers = image.height() / image.width();
        if layers == 6 {
            image.reinterpret_stacked_2d_as_array(layers);
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
            image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });

            commands.entity(camera_holder.0).insert(Skybox {
                image: cubemap.image.clone(),
                brightness: 1000.0,
                rotation: Quat::IDENTITY,
            });
        } else {
            warn!(
                "Skybox image must be 6xN pixels. Got {} layers.",
                layers
            );
        }
    }
}

/// Setup initial : joueur, caméra, lumière, cockpit et cubemap
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Parent du joueur
    let player_entity = commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    // Charger l'image du skybox (6 faces verticales)
    let sky_image = asset_server.load("skybox.png");

    // Ressource pour suivre le chargement
    commands.insert_resource(SkyCubeMap {
        image: sky_image.clone(),
        loaded: false,
    });

    // Créer la caméra sans skybox pour l'instant
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Camera { order: 0, ..default() },
            Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            PlayerCam,
            CameraSensitivity::default(),
        ))
        .id();

    commands.entity(player_entity).add_child(camera_entity);
    commands.insert_resource(CameraHolder(camera_entity));

    // Lumière directionnelle
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

    // Cockpit / scènes
    let positions: [[f32; 3]; 6] = [
        [0.0, 0.0, 20.0],
        [0.0, 0.0, 0.0],
        [-10.0, 0.0, 10.0],
        [10.0, 0.0, 10.0],
        [0.0, -10.0, 10.0],
        [0.0, 10.0, 10.0],
    ];

    for pos in positions {
        commands.spawn((
            SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
            Transform::from_xyz(pos[0], pos[1], pos[2]),
            GlobalTransform::default(),
        ));
    }
}
