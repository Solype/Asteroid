use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
// use crate::controller::Player;

use crate::menu::{structs::*};


pub fn setup_menu(mut commands: Commands, images: ResMut<Assets<Image>>, menu_texture: Option<Res<MenuCameraTarget>>)
{
    commands.insert_resource(SpawnMenuPlane);
    let handle = setup_texture_camera(&mut commands, images, menu_texture);
    let root_came = setup_menu_camera(&mut commands, handle);
    setup_2d_scene(&mut commands, MenuTypes::MainMenu, root_came);
}

pub fn menu_cleanup(mut commands: Commands, query: Query<Entity, With<MenuCameraComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}


////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// PRIVATE METHODE
/// 
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////


fn setup_2d_scene(commands: &mut Commands, menu_id: MenuTypes, camera_entity: Entity)
{
    let menu_layer = MenuTypes::layer(menu_id);

    // Fond du menu
    let background = commands
        .spawn((
            Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.8), // gris semi-transparent
                custom_size: Some(Vec2::new(400.0, 200.0)),
                ..default()
            },
            menu_layer.clone(),
        ))
        .id();

    // Bouton Start
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

    // Bouton Quit
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
}


fn setup_menu_camera(commands: &mut Commands, image_handle: Handle<Image>) -> Entity
{
    let menu_layer = MenuTypes::layer(MenuTypes::MainMenu);

    // Caméra qui rend dans la texture
    return commands
        .spawn((
            Camera2d::default(),
            Camera {
                target: RenderTarget::Image(image_handle.clone().into()),
                ..default()
            },
            MenuCameraComponent,
            menu_layer.clone(),
        ))
        .id();
}

fn setup_texture_camera(commands: &mut Commands, mut images: ResMut<Assets<Image>>, menu_texture: Option<Res<MenuCameraTarget>>) -> Handle<Image>
{
    if let Some(existing) = menu_texture {
        return existing.image.clone();
    }
    let x: u32 = 512;
    let y: u32 = 256;

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("menu_camera_target"),
            size: Extent3d { width: x, height: y, depth_or_array_layers: 1 },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(Extent3d { width: x, height: y, depth_or_array_layers: 1 });

    let image_handle = images.add(image);
    commands.insert_resource(MenuCameraTarget {
        image: image_handle.clone(),
    });
    return image_handle.clone();
}

