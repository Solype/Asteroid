use bevy::prelude::*;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
// use crate::controller::Player;

use crate::menu::{structs::*};

static SCREEN_WIDTH : u32 = 512;
static SCREEN_HEIGHT : u32 = 256;

pub fn setup_menu(mut commands: Commands, menu_texture: Res<MenuCameraTarget>, asset_server: Res<AssetServer>)
{
    let handle = menu_texture.image.clone();
    let root_came = setup_menu_camera(&mut commands, handle);
    setup_2d_scene(&mut commands, MenuTypes::MainMenu, root_came, asset_server);
    info!("Menu setup !");
}

// pub fn menu_cleanup(mut commands: Commands, query: Query<Entity, With<MenuCameraComponent>>) {
//     for entity in query.iter() {
//         commands.entity(entity).despawn();
//     }
// }

pub fn apply_texture_to_quad(mut commands: Commands, screens: Query<(&MenuPlane, Entity)>, mut materials: ResMut<Assets<StandardMaterial>>, menu_texture: Res<MenuCameraTarget>)
{
    let mat_handler = materials.add(StandardMaterial {
        base_color_texture: Some(menu_texture.image.clone()),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    for (planes, entity) in screens.iter() {
        if planes.menu_id == MenuTypes::MainMenu {
            commands.entity(entity).insert(MeshMaterial3d(mat_handler));
            info!("Texture applied");
            return;
        }
    }
}

pub fn setup_texture_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>)
{
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("menu_camera_target"),
            size: Extent3d { width: SCREEN_WIDTH, height: SCREEN_HEIGHT, depth_or_array_layers: 1 },
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

    image.resize(Extent3d { width: SCREEN_WIDTH, height: SCREEN_HEIGHT, depth_or_array_layers: 1 });

    commands.insert_resource(MenuCameraTarget { image: images.add(image) });
    info!("Texture set !")
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// PRIVATE METHODE
/// 
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////


fn setup_2d_scene(commands: &mut Commands, menu_id: MenuTypes, camera_entity: Entity, asset_server: Res<AssetServer>)
{
    let menu_layer = MenuTypes::layer(menu_id);
    let background_texture: Handle<Image> = asset_server.load("menu_background.jpg");

    // Fond du menu
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


