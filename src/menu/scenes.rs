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

    if let Some (ressources) = menu_ressources {
        font = ressources.font.clone();
        background = ressources.bg.clone();
    } else {
        font = asset_server.load("fonts/Orbitron-Regular.otf");
        background = asset_server.load("menu_background.jpg");
        commands.insert_resource(MainMenuRessources {font: font.clone(), bg: background.clone()});
    }

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
