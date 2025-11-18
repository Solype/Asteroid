use bevy::prelude::*;
use bevy::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages
};

use crate::back_camera::structs::{BackCameraComponent, BackCameraRenderTargetImage};
use crate::game_states::GameState;
use crate::menu::structs::{MenuPlane};

mod structs;

pub fn back_cam_plugin(app: &mut App)
{
    app.add_systems(Startup, setup_camera_3d_render_to_texture);
    app.add_systems(OnEnter(GameState::Game), display_renter_target);
}


fn display_renter_target(
    render_target: Res<BackCameraRenderTargetImage>,
    screens: Query<(&MenuPlane, Entity)>,
    mut commands: Commands
)
{
    for (_, entity) in screens.iter() {
        commands.entity(entity).insert(MeshMaterial3d(render_target.material.clone()));
    }
}


static SCREEN_WIDTH : u32 = 384;
static SCREEN_HEIGHT : u32 = 128;

fn setup_camera_3d_render_to_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("back_camera"),
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

    let image_handle: Handle<Image> = images.add(image);
    
    commands.spawn((
        Projection::from(PerspectiveProjection::default()),
        Camera3d::default(),
        Camera {
            target: RenderTarget::Image(image_handle.clone().into()),
            is_active: true,
            ..Default::default()
        },
        BackCameraComponent,
        Transform::from_xyz(0.0, 4.1, 5.3)
                .looking_at(Vec3::new(0.216544, -0.777080, 0.318808), Vec3::Y),
    ));

    let mat_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    commands.insert_resource(BackCameraRenderTargetImage{material: mat_handle.clone()});

}
