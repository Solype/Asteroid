use bevy::prelude::*;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages
};



use crate::menu::structs::*;

static SCREEN_WIDTH : u32 = 96 * 16;
static SCREEN_HEIGHT : u32 = 32 * 16;

pub fn setup_menu(mut commands: Commands, menu_texture: Res<MenuCameraTarget>)
{
    let handle = menu_texture.image.clone();

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: RenderTarget::Image(handle.clone().into()),
            ..default()
        },
        MenuCameraComponent,
    ));
}

pub fn apply_texture_to_quad(mut commands: Commands, screens: Query<(&MenuPlane, Entity)>, mut materials: ResMut<Assets<StandardMaterial>>, menu_texture: Res<MenuCameraTarget>)
{
    let mat_handler = materials.add(StandardMaterial {
        base_color_texture: Some(menu_texture.image.clone()),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    for (_, entity) in screens.iter() {
        commands.entity(entity).insert(MeshMaterial3d(mat_handler));
        info!("Texture applied");
        return;
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

pub fn setup_sound_effect_and_music(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.insert_resource(MenuSounds{
        button_bips: vec![
            asset_server.load("menu_bip1.wav"),
            asset_server.load("menu_bip2.wav")
        ]
    });
}
