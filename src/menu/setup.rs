use bevy::prelude::*;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages
};



use crate::menu::structs::*;

static SCREEN_WIDTH : u32 = 1280;
static SCREEN_HEIGHT : u32 = 512;

pub fn apply_texture_to_quad(mut commands: Commands, screens: Query<(&MenuPlane, Entity)>, menu_texture: Res<MenuCameraTarget>)
{
    for (_, entity) in screens.iter() {
        commands.entity(entity).insert(MeshMaterial3d(menu_texture.material.clone()));
        return;
    }
}

pub fn setup_texture_camera(
    mut commands: Commands, mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<crate::config::structs::GameConfig>,
) {
    let mut screen_size: Vec2 = Vec2::new(
        (config.ship.screen_center.tl - config.ship.screen_center.tr).length(),
        (config.ship.screen_center.tl - config.ship.screen_center.bl).length(),
    );

    if screen_size.x <= 0.0 {
        screen_size.x = SCREEN_WIDTH as f32;
    }
    if screen_size.y <= 0.0 {
        screen_size.y = SCREEN_HEIGHT as f32;
    }
    let ratio = screen_size.x / screen_size.y;
    let screen_height_scaled = (SCREEN_WIDTH as f32) / ratio;

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("menu_camera_target"),
            size: Extent3d { width: SCREEN_WIDTH, height: screen_height_scaled as u32, depth_or_array_layers: 1 },
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

    image.resize(Extent3d { width: SCREEN_WIDTH, height: screen_height_scaled as u32, depth_or_array_layers: 1 });
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

pub fn setup_sound_effect_and_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameconfig: Res<crate::config::structs::GameConfig>
) {
    let mut resource = MenuSounds::default();

    for path in gameconfig.ui.sounds.iter() {
        info!("path: {}", path);
        resource.button_bips.push(asset_server.load(path));
    }
    commands.insert_resource(resource);
}
