use bevy::{prelude::*};
use crate::menu::structs::*;

static SCREEN_WIDTH : u32 = 512;
static SCREEN_HEIGHT : u32 = 256;

pub fn cleanup_menu_cam(mut commands: Commands, entity: Single<(&mut Camera, &Children), With<MenuCameraComponent>>)
{
    let (mut camera, children) = entity.into_inner();
    
    camera.is_active = false;
    for child in children.iter() {
        commands.entity(child).despawn();
    }
}

pub fn create_main_menu_scene(
    mut commands: Commands,
    entity: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    asset_server: Res<AssetServer>,
    existing_image: Option<Res<MenuBackgroundImage>>,
)
{
    let handler : Handle<Image>;

    if existing_image.is_none() {
        handler = asset_server.load("menu_background.jpg");
        commands.insert_resource(MenuBackgroundImage { image: handler.clone() });
    } else {
        handler = existing_image.unwrap().image.clone();
    }

    let (camera_entity, mut camera) = entity.into_inner();
    let menu_layer = MenuTypes::layer(MenuTypes::MainMenu);
    let background_texture: Handle<Image> = handler;

    let background = commands
        .spawn((
            Sprite {
                image: background_texture,
                custom_size: Some(Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)),
                ..default()
            },
            menu_layer.clone(),
        ))
        .id();

    let start_button = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.2),
                custom_size: Some(Vec2::new(150.0, 50.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 40.0, 1.0)),
            MenuButton {
                action: MenuAction::Start,
            },
            menu_layer.clone(),
        ))
        .id();

    let quit_button = commands
        .spawn((
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::new(150.0, 50.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, -40.0, 1.0)),
            MenuButton {
                action: MenuAction::Quit,
            },
            menu_layer.clone(),
        ))
        .id();

    commands.entity(camera_entity).add_children(&[background, start_button, quit_button]);
    camera.is_active = true;
}
