use bevy::prelude::*;


mod controller;
mod rock;
mod menu;
mod game_states;
mod skybox;

use game_states::GameState;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((
            menu::menu_plugin,
            skybox::plugin,
            controller::plugin,
            rock::plugin
        ))
        .init_state::<GameState>()

        // .add_systems(Update, print_state)
        .run();
}


// fn print_state(current_state: Res<State<game_states::GameState>>) {
//     println!("État courant : {:?}", current_state.get());
// }

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_entity = commands
        .spawn((
            controller::Player,
            controller::CameraSensitivity::default(),
            Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Camera { order: 0, ..default() },
            Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            controller::PlayerCam,
            controller::CameraSensitivity::default(),
        ))
        .id();

    commands.entity(player_entity).add_child(camera_entity);
    commands.insert_resource(skybox::CameraHolder(camera_entity));

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
