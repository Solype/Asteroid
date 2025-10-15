use bevy::{prelude::*};
// use bevy::state::state_scoped::*;
// use bevy::sprite::{MeshMaterial2d};
use bevy::prelude::Mesh2d;
use crate::game_states::GameState;
use crate::menu::structs::*;

pub fn cleanup_menu_cam(entity: Single<&mut Camera, With<MenuCameraComponent>>)
{
    let mut camera = entity.into_inner();
    camera.is_active = false;
}

pub fn create_main_menu_scene(
    mut commands: Commands,
    entity: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    asset_server: Res<AssetServer>,
    existing_image: Option<Res<MenuBackgroundImage>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // --- Charger ou réutiliser l’image de fond ---
    let handler: Handle<Image> = if let Some(existing) = existing_image {
        existing.image.clone()
    } else {
        let new_image = asset_server.load("menu_background.jpg");
        commands.insert_resource(MenuBackgroundImage {
            image: new_image.clone(),
        });
        new_image
    };

    let (camera_entity, mut camera) = entity.into_inner();
    let menu_layer = MenuTypes::layer(MenuTypes::MainMenu);

    // --- Image de fond ---
    let background = commands
        .spawn((
            DespawnOnExit(GameState::Menu),
            Sprite {
                image: handler,
                custom_size: Some(Vec2::new(1920.0, 1080.0)),
                ..default()
            },
            menu_layer.clone(),
            Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ))
        .id();

    // --- Effet lumineux central (sphère bleutée) ---
    let glow_mesh = meshes.add(Circle::new(400.0));
    let glow_mat = materials.add(ColorMaterial::from(Color::srgba(0.2, 0.8, 1.0, 0.05)));

    let glow = commands
        .spawn((
            Mesh2d(glow_mesh),
            MeshMaterial2d(glow_mat),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            menu_layer.clone(),
        ))
        .id();

    // --- Bouton "Start" ---
    let start_button = commands
        .spawn((
            Sprite {
                color: Color::srgba(0.1, 0.8, 0.4, 0.9),
                custom_size: Some(Vec2::new(220.0, 60.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 80.0, 1.0)),
            MenuButton {
                action: MenuAction::Start,
            },
            menu_layer.clone(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d("Start Game".to_string()),
                TextFont {
                    font: asset_server.load("fonts/Orbitron-Regular.otf"),
                    font_size: 38.0,
                    ..default()
                },
                TextColor(Color::srgba(0.0, 1.0, 1.0, 1.0)),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            ));
        //(
        //         Text::new("Start Game"),
        //         TextFont {
        //             font: asset_server.load("fonts/Orbitron-Regular.otf"),
        //             font_size: 38.0,
        //             ..default()
        //         },
        //         TextColor(Color::srgba(0.0, 1.0, 1.0, 1.0)),
        //     ));
        })
        .id();

    // --- Bouton "Quit" ---
    let quit_button = commands
        .spawn((
            Sprite {
                color: Color::srgba(0.9, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(220.0, 60.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, -80.0, 1.0)),
            MenuButton {
                action: MenuAction::Quit,
            },
            menu_layer.clone(),
        ))
        // .with_children(|parent| {
        //     parent.spawn((
        //         Text::new("Exit"),
        //         TextFont {
        //             font: asset_server.load("fonts/Orbitron-Regular.otf"),
        //             font_size: 38.0,
        //             ..default()
        //         },
        //         TextColor(Color::srgba(1.0, 0.4, 0.4, 1.0)),
        //     ));
        // })
        .id();

    commands.entity(camera_entity).add_children(&[background, glow, start_button, quit_button]);
    camera.is_active = true;
}

// pub fn create_main_menu_scene(
//     mut commands: Commands,
//     entity: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
//     asset_server: Res<AssetServer>,
//     existing_image: Option<Res<MenuBackgroundImage>>,
// )
// {
//     let handler : Handle<Image>;

//     if existing_image.is_none() {
//         handler = asset_server.load("menu_background.jpg");
//         commands.insert_resource(MenuBackgroundImage { image: handler.clone() });
//     } else {
//         handler = existing_image.unwrap().image.clone();
//     }

//     let (camera_entity, mut camera) = entity.into_inner();
//     let menu_layer = MenuTypes::layer(MenuTypes::MainMenu);
//     let background_texture: Handle<Image> = handler;

//     let background = commands
//         .spawn((
//             Sprite {
//                 image: background_texture,
//                 custom_size: Some(Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)),
//                 ..default()
//             },
//             menu_layer.clone(),
//         ))
//         .id();

//     let start_button = commands
//         .spawn((
//             Sprite {
//                 color: Color::srgb(0.2, 0.8, 0.2),
//                 custom_size: Some(Vec2::new(150.0, 50.0)),
//                 ..default()
//             },
//             Transform::from_translation(Vec3::new(0.0, 40.0, 1.0)),
//             MenuButton {
//                 action: MenuAction::Start,
//             },
//             menu_layer.clone(),
//         ))
//         .id();

//     let quit_button = commands
//         .spawn((
//             Sprite {
//                 color: Color::srgb(0.8, 0.2, 0.2),
//                 custom_size: Some(Vec2::new(150.0, 50.0)),
//                 ..default()
//             },
//             Transform::from_translation(Vec3::new(0.0, -40.0, 1.0)),
//             MenuButton {
//                 action: MenuAction::Quit,
//             },
//             menu_layer.clone(),
//         ))
//         .id();

//     commands.entity(camera_entity).add_children(&[background, start_button, quit_button]);
//     camera.is_active = true;
// }
