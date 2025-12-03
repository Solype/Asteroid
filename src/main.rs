use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, Mesh},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

mod asteroids;
mod back_camera;
mod background_musics;
mod controller;
mod game_states;
mod globals_structs;
mod helpers;
mod menu;
mod particules;
mod player;
mod score_display;
mod skybox;
mod spritesheet;
mod game_over;
mod config;

use bevy_hanabi::HanabiPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use game_states::GameState;
use globals_structs::*;

use crate::asteroids::Velocity;
use crate::player::PlayerHitBox;


fn main() {
    let gameconfig = config::load_game_config("assets/manifest.xml");

    let width = if gameconfig.window.x > 0.0 { gameconfig.window.x as u32 } else { 1280 };
    let height = if gameconfig.window.y > 0.0 { gameconfig.window.y as u32 } else { 720 };

    println!("Size of the window: {} {}", width, height);

    let mut app = App::new();

    app.insert_resource(gameconfig.clone());

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }).set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: gameconfig.window_title.into(),
                    name: Some(gameconfig.window_name.into()),
                    resolution: (width, height).into(),
                    ..default()
                }),
                ..default()
            } ))
        .add_systems(Startup, (setup, setup_ui_ressource))
        .add_plugins((
            HanabiPlugin,
            Sprite3dPlugin,
            menu::menu_plugin,
            skybox::plugin,
            controller::plugin,
            asteroids::AsteroidPlugin,
            score_display::score_display_plugin,
            player::PlayerPlugin,
            back_camera::back_cam_plugin,
            helpers::CameraControllerPlugin,
            particules::ParticlesPlugin,
            spritesheet::SpriteSheetPlugin,
            background_musics::BackgroundMusicPlugin,
            game_over::GameOverPlugin
        ))
        .init_state::<GameState>()
        .insert_resource(MusicVolume { volume: 100.0_f32 })
        .insert_resource(Keybinds::default())
        .insert_resource(Score::default())
        .add_systems(
            Update,
            (start_after_startup).run_if(in_state(GameState::Startup)),
        )
        .run();
}

fn setup_ui_ressource(mut command: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("font.ttf");
    let background = asset_server.load("menu_bg.jpg");
    command.insert_resource(UIRessources {
        font: font.clone(),
        bg: background.clone(),
    });
}


fn start_after_startup(
    mut next_state: ResMut<NextState<GameState>>,
    mut frame_count: Local<u32>,
) {
    *frame_count += 1;
    if *frame_count < 10 { // wait one frame
        return;
    }
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

    let epsilon = 0.0;
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
    gameconfig: Res<config::structs::GameConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> (Entity, Entity, Entity) {

    let left_points: Vec<Vec3> = vec![
        gameconfig.ship.screen_left.tl,
        gameconfig.ship.screen_left.tr,
        gameconfig.ship.screen_left.br,
        gameconfig.ship.screen_left.bl,
    ];
    let right_points: Vec<Vec3> = vec![
        gameconfig.ship.screen_right.tl,
        gameconfig.ship.screen_right.tr,
        gameconfig.ship.screen_right.br,
        gameconfig.ship.screen_right.bl,
    ];
    let middle_points: Vec<Vec3> = vec![
        gameconfig.ship.screen_center.tl,
        gameconfig.ship.screen_center.tr,
        gameconfig.ship.screen_center.br,
        gameconfig.ship.screen_center.bl,
    ];
    println!("Left screen points:");
    for p in &left_points {
        println!("  {:?}", p);
    }

    println!("Right screen points:");
    for p in &right_points {
        println!("  {:?}", p);
    }

    println!("Middle screen points:");
    for p in &middle_points {
        println!("  {:?}", p);
    }

    let (left_mesh, _left_normal, _left_center) = create_quad(
        left_points[0],
        left_points[1],
        left_points[2],
        left_points[3],
    );
    let (middle_mesh, _middle_normal, _middle_center) = create_quad(
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
            menu::structs::MenuPlane,
        ))
        .id();

    let right_id = commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(right_mesh))),
            score_display::structs::ScorePlane,
        ))
        .id();

    return (left_id, middle_id, right_id);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameconfig: Res<config::structs::GameConfig>,
    meshes: ResMut<Assets<Mesh>>,
) {
    let player_entity = commands
        .spawn((
            SceneRoot(asset_server.load(gameconfig.ship.asset.clone())),
            controller::structs::Player,
            Velocity(Vec3::default()),
            controller::structs::RotationalVelocity::default(),
            Transform::default(),
            children![
                (
                    PlayerHitBox { radius: 1.0 },
                    Transform::from_xyz(0.0, 0.5, -2.0),
                ),
                (
                    PlayerHitBox { radius: 1.5 },
                    Transform::from_xyz(0.0, 1.0, 0.5),
                ),
                (
                    PlayerHitBox { radius: 1.25 },
                    Transform::from_xyz(0.0, 0.5, 3.5),
                ),
            ],
        ))
        .id();


    let camera_entity = commands
        .spawn((
            Projection::from(PerspectiveProjection::default()),
            Camera3d::default(),
            Camera {
                order: 0,
                ..default()
            },
            SpatialListener::new(1.0),
            GlobalTransform::default(),
            Transform::from_xyz(0.0, 1.1, 0.3)
                .looking_at(Vec3::new(-0.216544, 0.777080, -0.318808), Vec3::Y),
            controller::structs::PlayerCam,
            controller::structs::CameraSensitivity::default(),
        ))
        .id();

    let (left_screen, middle_screen, right_screen) = setup_left_screen(&mut commands, gameconfig, meshes);
    commands.entity(player_entity).add_children(&[
        camera_entity,
        left_screen,
        middle_screen,
        right_screen,
    ]);
    // commands.insert_resource(skybox::CameraHolder(camera_entity));
}
