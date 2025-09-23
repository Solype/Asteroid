use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::controller::PlayerCam;

use crate::menu::structs::*;


pub fn setup_menu(mut commands: Commands, images: ResMut<Assets<Image>>, menu_texture: Option<Res<MenuCameraTarget>>)
{
    commands.insert_resource(SpawnMenuPlane);
    let handle = setup_texture_camera(&mut commands, images, menu_texture);
    setup_menu_camera(commands, handle);
}

pub fn menu_cleanup(mut commands: Commands, query: Query<Entity, With<MenuCameraComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
        info!("MenuCamera supprimée avec tous ses enfants {:?}", entity);
    }
}


////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// PRIVATE METHODE
/// 
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////


fn setup_menu_camera(mut commands: Commands, image_handle: Handle<Image>)
{
    let root_cam = commands.spawn((
        Camera2d::default(),
        Camera {
            target: RenderTarget::Image(image_handle.clone().into()),
            ..default()
        },
        MenuCameraComponent
    )).id();


    let square = commands.spawn((
        Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(100.0, 100.0)), // taille du carré
                ..default()
        },
        CameraSquareElement
    )).id();

    commands.entity(root_cam).add_child(square);
}

pub fn spawn_menu_plane(
    mut commands: Commands,
    player_cam_query: Query<&GlobalTransform, With<PlayerCam>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    menu_texture: Res<MenuCameraTarget>,
    trigger: Option<Res<SpawnMenuPlane>>,
) {
    if trigger.is_none() {
        return;
    }
    if let Ok(cam_transform) = player_cam_query.single() {
        let distance = 3.0;
        let position = cam_transform.translation() + cam_transform.forward() * distance;
        let look_at = Quat::from_rotation_arc(
            Vec3::Y,
            (-cam_transform.forward()).into(),
        );
        let mesh = meshes.add(Plane3d::default().mesh().size(4.0, 2.0));
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(menu_texture.image.clone()),
            ..default()
        });
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform {
                translation: position,
                rotation: look_at,
                ..default()
            },
            MenuPlane
        ));
        commands.remove_resource::<SpawnMenuPlane>();
    }
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

