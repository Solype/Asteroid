use bevy::prelude::*;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages
};



use crate::menu::structs::*;

static SCREEN_WIDTH : u32 = 96 * 16;
static SCREEN_HEIGHT : u32 = 32 * 16;

pub fn apply_texture_to_quad(mut commands: Commands, screens: Query<(&MenuPlane, Entity)>, menu_texture: Res<MenuCameraTarget>)
{
    for (_, entity) in screens.iter() {
        commands.entity(entity).insert(MeshMaterial3d(menu_texture.material.clone()));
        info!("Texture applied");
        return;
    }
}

pub fn setup_texture_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut materials: ResMut<Assets<StandardMaterial>>)
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
    let handler = images.add(image);

    let mat_handler = materials.add(StandardMaterial {
        base_color_texture: Some(handler.clone()),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    commands.insert_resource(MenuCameraTarget { /* image: handler.clone(), */ material: mat_handler });

    commands.spawn((
        Camera2d::default(),
        Camera {
            target: RenderTarget::Image(handler.clone().into()),
            ..default()
        },
        MenuCameraComponent,
    ));
}

pub fn setup_sound_effect_and_music(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.insert_resource(MenuSounds{
        button_bips: vec![
            asset_server.load("sounds/menu_bip1.wav"),
            asset_server.load("sounds/menu_bip2.wav")
        ]
    });
}
