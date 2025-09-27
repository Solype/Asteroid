use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;


mod controller;
// mod rock;
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
            // rock::plugin
        ))
        .init_state::<GameState>()

        // .add_systems(Update, print_state)
        .run();
}

// fn create_quad(
//     top_left: Vec3,
//     top_right: Vec3,
//     bottom_right: Vec3,
//     bottom_left: Vec3,
// ) -> (Plane3d, Transform) {
//     let bottom = (bottom_left - bottom_right).length();
//     let top = (top_left - top_right).length();
//     let right = (top_right - bottom_right).length();
//     let left = (top_left - bottom_left).length();
//     let mut center = (top_left + top_right + bottom_right + bottom_left) / 4.0;

//     let width = top.max(bottom);
//     let height = left.max(right);

//     let normal = -(top_right - top_left).cross(bottom_left - top_left).normalize();

//     let mut plane = Plane3d::default();
//     plane.half_size = Vec2::new(width / 2.0, height / 2.0);
//     plane.normal = Dir3::new(normal).unwrap();

//     // Décaler légèrement le plan le long de la normale
//     let offset = 0.01; // ajuster selon ton besoin
//     center += normal * offset;

//     let rotation = Quat::default();
//     let mut transform = Transform::from_translation(center).with_rotation(rotation);

//     // Axe local du plane qui correspond à son "haut"
//     let local_up = transform.rotation * Vec3::Y; // ici Y au lieu de X
//     let mut target_up = (top_right - top_left).normalize();
//     target_up -= normal * target_up.dot(normal); // projection sur le plan

//     let cos = local_up.dot(target_up).clamp(-1.0, 1.0);
//     let angle = cos.acos();
//     let sign = local_up.cross(target_up).dot(normal);
//     let signed_angle = if sign < 0.0 { -angle } else { angle };

//     transform.rotate_local_axis(plane.normal, signed_angle);

//     (plane, transform)
// }

use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;

fn create_quad(
    top_left: Vec3,
    top_right: Vec3,
    bottom_right: Vec3,
    bottom_left: Vec3,
) -> Mesh {
    let normal = (top_right - top_left).cross(bottom_left - top_left).normalize();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::all(),
    );

    let positions = vec![ top_left, top_right, bottom_right, bottom_left ];

    let normals = vec![normal; 4];
    let indices = Indices::U32(vec![ 0, 2, 1,    0, 3, 2 ]);
    let uvs = vec![ [0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0] ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);
    return mesh
}


fn setup_left_screen(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) -> (Entity, Entity, Entity)
{
    let left_points: Vec<Vec3> = vec![
        Vec3::new(-0.610449, 0.755574, -0.205797),
        Vec3::new(-0.502950, 0.752438, -0.251174),
        Vec3::new(-0.502971, 0.657055, -0.211015),
        Vec3::new(-0.610428, 0.681590, -0.174664),
    ];
    let right_points : Vec<Vec3> = vec![
        Vec3::new(0.502982, 0.752438, -0.251174),
        Vec3::new(0.610481, 0.755575, -0.205797),
        Vec3::new(0.610460, 0.681590, -0.174664),
        Vec3::new(0.503003, 0.657055, -0.211015),
    ];
    let middle_points : Vec<Vec3> = vec![
        Vec3::new(-0.216544, 0.777080, -0.318808),
        Vec3::new(0.216575, 0.777080, -0.318808),
        Vec3::new(0.216575, 0.640333, -0.261248),
        Vec3::new(-0.216544, 0.640333, -0.261248),
    ];
    
    let left_mesh = create_quad(left_points[0], left_points[1], left_points[2], left_points[3]);
    let middle_mesh = create_quad(middle_points[0], middle_points[1], middle_points[2], middle_points[3]);
    let right_mesh = create_quad(right_points[0], right_points[1], right_points[2], right_points[3]);

    let left_id = commands.spawn((
        Mesh3d(meshes.add(Mesh::from(left_mesh))),
        menu::structs::MenuPlane { width: 3.0, height: 2.0, menu_id: menu::structs::MenuTypes::MainMenu }
    )).id();
    let middle_id = commands.spawn((Mesh3d(meshes.add(Mesh::from(middle_mesh))))).id();
    let right_id = commands.spawn((Mesh3d(meshes.add(Mesh::from(right_mesh))))).id();

    return (left_id, middle_id, right_id);
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_entity = commands
        .spawn((
            SceneRoot(asset_server.load("Spaceship.glb#Scene0")),
            controller::Player,
            controller::CameraSensitivity::default(),
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Camera { order: 0, ..default() },
            Transform::from_xyz(0.0, 1.1, 0.3).looking_at(Vec3::new(-0.216544, 0.777080, -0.318808), Vec3::Y),
            controller::PlayerCam,
            controller::CameraSensitivity::default(),
        ))
        .id();


    let (left_screen, middle_screen, right_screen) = setup_left_screen(&mut commands, meshes);
    commands.entity(player_entity).add_children(&[camera_entity, left_screen, middle_screen, right_screen]);
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
}
