use bevy::{prelude::*};
// use bevy::state::state_scoped::*;
// use bevy::sprite::{MeshMaterial2d};
// use bevy::prelude::Mesh2d;
use crate::game_states::GameState;
use crate::menu::structs::*;

pub fn cleanup_menu_cam(entity: Single<&mut Camera, With<MenuCameraComponent>>)
{
    let mut camera = entity.into_inner();
    camera.is_active = false;
}

pub fn create_main_menu_scene(
    mut commands: Commands,
    camera_components: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    asset_server: Res<AssetServer>,
) {
    let (cam_entity, mut camera) = camera_components.into_inner();
    camera.is_active = true;

    let menu_layer = MenuTypes::layer(MenuTypes::MainMenu);
    let font: Handle<Font> = asset_server.load("fonts/Orbitron-Regular.otf");

    // --- Conteneur principal du menu ---
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        UiTargetCamera(cam_entity),
        menu_layer.clone(),
        children![
            // --- Titre du menu ---
            (
                Text::new("Main Menu"),
                TextFont { font: font.clone(), font_size: 48.0, ..default() },
                TextColor(Color::srgb(0.8, 1.0, 1.0)),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                menu_layer.clone(),
            ),
            // --- Bouton Start ---
            (
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.1, 0.8, 0.4)),
                MenuButton {
                    action: MenuAction::Start,
                },
                children![(
                    Text::new("Start"),
                    TextFont { font: font.clone(), font_size: 28.0, ..default() },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                )],
                menu_layer.clone(),
            ),
            // --- Bouton Quit ---
            (
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.9, 0.2, 0.2)),
                MenuButton {
                    action: MenuAction::Quit,
                },
                children![(
                    Text::new("Quit"),
                    TextFont { font: font.clone(), font_size: 28.0, ..default() },
                    TextColor(Color::srgb(0.0, 0.0, 0.0)),
                )],
                menu_layer.clone(),
            ),
        ],
    ));
}
/*
    let background = commands.spawn((
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

    let glow_mesh = meshes.add(Circle::new(400.0));
    let glow_mat = materials.add(ColorMaterial::from(Color::srgba(0.2, 0.8, 1.0, 0.05)));

    let glow = commands.spawn((
            DespawnOnExit(GameState::Menu),
            Mesh2d(glow_mesh),
            MeshMaterial2d(glow_mat),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            menu_layer.clone(),
        )).id();

    let start_button = commands.spawn((
            DespawnOnExit(GameState::Menu),
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
        )).id();

    let quit_button = commands.spawn((
            DespawnOnExit(GameState::Menu),
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
        )).id();
 */
