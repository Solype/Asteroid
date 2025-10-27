use bevy::asset::RenderAssetUsages;
use bevy::light::DirectionalLightShadowMap;
use bevy::mesh::primitives::SphereMeshBuilder;
use bevy::mesh::{Indices, Mesh};
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

mod asteroids;
#[path = "./helpers/camera_controller.rs"]
mod camera_controller;
mod controller;
mod game_states;
mod menu;
mod skybox;

use game_states::GameState;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins((
            // menu::menu_plugin,
            skybox::plugin,
            controller::plugin,
            asteroids::AsteroidPlugin,
            camera_controller::CameraControllerPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(
            Update,
            start_after_startup.run_if(in_state(GameState::Startup)),
        )
        .run();
}

fn start_after_startup(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}

fn create_quad(
    top_left: Vec3,
    top_right: Vec3,
    bottom_right: Vec3,
    bottom_left: Vec3,
) -> (Mesh, Vec3, Vec3) {
    let normal = (top_right - top_left)
        .cross(bottom_left - top_left)
        .normalize();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());

    let epsilon = 0.001;
    let offset = normal * epsilon;

    let positions = vec![
        top_left - offset,
        top_right - offset,
        bottom_right - offset,
        bottom_left - offset,
    ];

    let u_axis = (top_right - top_left).normalize();
    let v_axis = (bottom_left - top_left).normalize();

    let width = (top_right - top_left).length();
    let height = (bottom_left - top_left).length();

    let uvs: Vec<[f32; 2]> = positions
        .iter()
        .map(|p| {
            let local = *p - top_left;
            [local.dot(u_axis) / width, local.dot(v_axis) / height]
        })
        .collect();

    let normals = vec![normal; 4];
    let center = (top_left + top_right + bottom_right + bottom_left) / 4.0;
    let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    return (mesh, normal, center);
}

fn setup_left_screen(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) -> (Entity, Entity, Entity) {
    let left_points: Vec<Vec3> = vec![
        Vec3::new(-0.610449, 0.755574, -0.205797),
        Vec3::new(-0.502950, 0.752438, -0.251174),
        Vec3::new(-0.502971, 0.657055, -0.211015),
        Vec3::new(-0.610428, 0.681590, -0.174664),
    ];
    let right_points: Vec<Vec3> = vec![
        Vec3::new(0.502982, 0.752438, -0.251174),
        Vec3::new(0.610481, 0.755575, -0.205797),
        Vec3::new(0.610460, 0.681590, -0.174664),
        Vec3::new(0.503003, 0.657055, -0.211015),
    ];
    let middle_points: Vec<Vec3> = vec![
        Vec3::new(-0.216544, 0.777080, -0.318808),
        Vec3::new(0.216575, 0.777080, -0.318808),
        Vec3::new(0.216575, 0.640333, -0.261248),
        Vec3::new(-0.216544, 0.640333, -0.261248),
    ];

    let (left_mesh, _left_normal, _left_center) = create_quad(
        left_points[0],
        left_points[1],
        left_points[2],
        left_points[3],
    );
    let (middle_mesh, middle_normal, middle_center) = create_quad(
        middle_points[0],
        middle_points[1],
        middle_points[2],
        middle_points[3],
    );
    let (right_mesh, _right_normal, _right_center) = create_quad(
        right_points[0],
        right_points[1],
        right_points[2],
        right_points[3],
    );

    let left_id = commands
        .spawn((Mesh3d(meshes.add(Mesh::from(left_mesh))),))
        .id();

    let middle_id = commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(middle_mesh))),
            menu::structs::MenuPlane {
                dimensions: Vec2 {
                    x: (middle_points[0] - middle_points[1]).length(),
                    y: (middle_points[1] - middle_points[2]).length(),
                },
                normal: middle_normal,
                center: middle_center,
                menu_id: menu::structs::MenuTypes::MainMenu,
            },
        ))
        .id();

    let right_id = commands
        .spawn((Mesh3d(meshes.add(Mesh::from(right_mesh))),))
        .id();

    return (left_id, middle_id, right_id);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let player_entity = commands
    // .spawn((
    //     SceneRoot(asset_server.load("Spaceship.glb#Scene0")),
    //     controller::Player,
    //     controller::CameraSensitivity::default(),
    //     Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ))
    // .id();

    // let camera_entity = commands
    //     .spawn((
    //         Camera3d::default(),
    //         Camera { order: 0, ..default() },
    //         Transform::from_xyz(0.0, 1.1, 0.3).looking_at(Vec3::new(-0.216544, 0.777080, -0.318808), Vec3::Y),
    //         controller::PlayerCam,
    //         controller::CameraSensitivity::default(),
    //     ))
    //     .id();
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 1.5, 6.).looking_at(Vec3::ZERO, Vec3::Y),
            camera_controller::CameraController::default(),
        ))
        .id();

    // let (left_screen, middle_screen, right_screen) = setup_left_screen(&mut commands, meshes);
    // commands.entity(player_entity).add_children(&[camera_entity, left_screen, middle_screen, right_screen]);
    commands.insert_resource(skybox::CameraHolder(camera_entity));


    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.9, 0.0),
        emissive: Color::srgb(1.0, 0.9, 0.0).to_linear() * 8.0,
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
        Transform::from_xyz(-1000.0, 1000.0, 0.0),
        children![(
            Mesh3d(meshes.add(Sphere::new(500.0).mesh().uv(64, 64))),
            MeshMaterial3d(sun_material),
        )],
    ));
    commands.insert_resource(AmbientLight {
        brightness: 100.0,
        color: Color::srgb(1.0, 0.9, 0.0),
        ..default()
    });
}
