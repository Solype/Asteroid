use bevy::{prelude::*};
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
    menu_ressources: Option<ResMut<MainMenuRessources>>
) {
    let (cam_entity, mut camera) = camera_components.into_inner();
    camera.is_active = true;

    let menu_layer = MenuTypes::layer(MenuTypes::MainMenu);
    let font: Handle<Font>;
    let background : Handle<Image>;

    if let Some(ressources) = menu_ressources {
        font = ressources.font.clone();
        background = ressources.bg.clone();
    } else {
        font = asset_server.load("fonts/font.ttf");
        background = asset_server.load("menu_background.jpg");
        commands.insert_resource(MainMenuRessources {font: font.clone(), bg: background.clone()});
    }

    let border_radius = BorderRadius {
        top_right: Val::Px(20.0),
        bottom_left: Val::Px(20.0),
        top_left: Val::Px(4.0),
        bottom_right: Val::Px(4.0),
    };

    let node = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    };

    let border_color = BorderColor {
        top: Color::srgba(0.0, 0.9, 1.0, 0.3),
        bottom: Color::srgba(0.0, 0.6, 0.8, 0.3),
        left: Color::srgba(0.0, 0.8, 1.0, 0.4),
        right: Color::srgba(0.0, 0.8, 1.0, 0.4),
    };

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
        ImageNode {
            image: background,
            ..default()
        },
        UiTargetCamera(cam_entity),
        menu_layer.clone(),
        children![
            // --- Titre du tableau de bord ---
            (
                Text::new("STARSHIP DASHBOARD"),
                TextFont { font: font.clone(), font_size: 60.0, ..default() },
                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                Node {
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
                menu_layer.clone(),
            ),
            // --- Boutons ---
            (
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(70.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                children![
                    // --- Start Mission ---
                    (
                        node.clone(),
                        border_radius.clone(),
                        border_color.clone(),
                        BackgroundColor(Color::srgba(0.0, 0.2, 0.4, 0.8)), // dark blue transparent
                        MenuButton { action: MenuAction::Start },
                        children![(
                            Text::new("LAUNCH MISSION"),
                            TextFont { font: font.clone(), font_size: 28.0, ..default() },
                            TextColor(Color::srgb(0.0, 1.0, 1.0)),
                        )],
                        menu_layer.clone(),
                    ),
                    // --- Options ---
                    (
                        node.clone(),
                        border_radius.clone(),
                        border_color.clone(),
                        BackgroundColor(Color::srgba(0.0, 0.2, 0.0, 0.8)), // dark green transparent
                        MenuButton { action: MenuAction::Options },
                        children![(
                            Text::new("SYSTEM SETTINGS"),
                            TextFont { font: font.clone(), font_size: 28.0, ..default() },
                            TextColor(Color::srgb(0.0, 1.0, 0.0)),
                        )],
                        menu_layer.clone(),
                    ),
                    // --- Quit ---
                    (
                        node.clone(),
                        border_radius.clone(),
                        border_color.clone(),
                        BackgroundColor(Color::srgba(0.4, 0.0, 0.0, 0.8)), // dark red transparent
                        MenuButton { action: MenuAction::Quit },
                        children![(
                            Text::new("EJECT"),
                            TextFont { font: font.clone(), font_size: 28.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.0, 0.0)),
                        )],
                        menu_layer.clone(),
                    ),
                ],
                menu_layer.clone(),
            ),
        ],
    ));
}
