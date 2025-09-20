use bevy::{
    prelude::*,
};
use bevy::core_pipeline::Skybox;


mod controller;
mod rock;
use controller::*;
use rock::*;




#[derive(Resource)]
pub struct SkyCubeMap {
    pub image: Handle<Image>,
    pub loaded: bool,
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



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let parent = commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    let sky_image = asset_server.load("HDR_silver_and_gold_nebulae.hdr");

    commands.insert_resource(SkyCubeMap {
        image: sky_image.clone(),
        loaded: false,
    });

    let camera = commands
        .spawn((
            Camera3d::default(),
            Camera { order: 0, ..default() },
            Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            PlayerCam,
            CameraSensitivity::default(),
            Skybox {
                image: sky_image,
                brightness: 1000.0,
                rotation: Quat::IDENTITY,
            },
        ))
        .id();

    commands.entity(parent).add_child(camera);

    commands.spawn((
        DirectionalLight { illuminance: 20_000.0, shadows_enabled: true, ..default() },
        Transform::from_rotation(Quat::from_euler( EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0, )),
        GlobalTransform::default(),
    ));


    let array_pos : [[f32; 3]; 6] = [
        [0.0, 0.0, 20.0],
        [0.0, 0.0, 0.0],
        [-10.0, 0.0, 10.0],
        [10.0, 0.0, 10.0],
        [0.0, -10.0, 10.0],
        [0.0, 10.0, 10.0],
    ];
    for i in 0..array_pos.len() {
        commands.spawn((
            SceneRoot(asset_server.load("CockpitCentered.glb#Scene0")),
            Transform::from_xyz(array_pos[i][0], array_pos[i][1], array_pos[i][2]),
            GlobalTransform::default(),
        ));
    }

}
