use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::controller::PlayerCam;
use crate::game_states::GameState;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuSystemSet;

#[derive(Resource)]
struct SpawnMenuPlane;

#[derive(Component)]
struct MenuPlane;

#[derive(Resource)]
struct MenuCameraTarget {
    image: Handle<Image>,
}

#[derive(Component)]
struct MenuCameraComponent;

#[derive(Component)]
struct CameraSquareElement;



pub fn menu_plugin(app: &mut App)
{
    app.add_systems(OnEnter(GameState::Menu), setup_menu);
    app.add_systems(
        Update,
        (menu_system, spawn_menu_plane, cast_ray_from_click).in_set(MenuSystemSet).run_if(in_state(GameState::Menu)),
    );
    app.add_systems(
    OnExit(GameState::Menu),
    menu_cleanup
    );
    app.add_systems(Update, print_all_entities.run_if(in_state(GameState::Game)));
}


fn print_all_entities(query: Query<(Entity, &MenuCameraComponent)>) {
    for (entity, _) in query.iter() {
        let mut comps = vec![];
        comps.push("MenuCameraComponent");
        info!("Entity {:?} components: {:?}", entity, comps);
    }
}

fn menu_cleanup(mut commands: Commands, query: Query<Entity, With<MenuCameraComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
        info!("MenuCamera supprimée avec tous ses enfants {:?}", entity);
    }
}


fn setup_menu(mut commands: Commands, images: ResMut<Assets<Image>>, menu_texture: Option<Res<MenuCameraTarget>>)
{
    commands.insert_resource(SpawnMenuPlane);
    let handle = setup_texture_camera(&mut commands, images, menu_texture);
    setup_menu_camera(commands, handle);
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






fn menu_system(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        next_state.set(GameState::Game);
    }
}



fn ray_plane_intersection(ray_origin: Vec3, ray_dir: Vec3, plane_transform: &GlobalTransform) -> Option<Vec3>
{
    let plane_pos = plane_transform.translation();
    let plane_normal = plane_transform.rotation() * Vec3::Y;

    let denom = ray_dir.dot(plane_normal);
    if denom.abs() < 1e-6 {
        return None;
    }

    let t = (plane_pos - ray_origin).dot(plane_normal) / denom;
    if t < 0.0 {
        return None;
    }
    Some(ray_origin + t * ray_dir)
}



fn is_point_in_menu(local_point: Vec3, width: f32, height: f32) -> bool {
    local_point.x.abs() <= width / 2.0 && local_point.z.abs() <= height / 2.0
}

fn ray_from_cursor(
    camera: &Camera,
    cam_transform: &GlobalTransform,
    cursor_pos: Vec2,
) -> Option<Ray3d> {
    camera.viewport_to_world(cam_transform, cursor_pos).ok()
}

fn cast_ray_from_click(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<PlayerCam>>,
    planes: Query<&GlobalTransform, With<MenuPlane>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().expect("Pas de fenêtre unique !");
    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    let (camera, cam_transform) = match cameras.single() {
        Ok(data) => data,
        Err(_) => return,
    };

    let ray = match ray_from_cursor(camera, cam_transform, cursor_pos) {
        Some(ray) => ray,
        None => return,
    };

    for plane_transform in planes.iter() {
        if let Some(world_point) = ray_plane_intersection(ray.origin, ray.direction.into(), plane_transform) {
            let local_point = plane_transform.compute_matrix().inverse().transform_point3(world_point);
            info!("Clic sur plane local coords: x={} z={}", local_point.x, local_point.z);

            if is_point_in_menu(local_point, 4.0, 2.0) {
                info!("✅ Clic DANS le menu !");
            } else {
                info!("❌ Clic hors du menu.");
            }
        }
    }
}

fn spawn_menu_plane(
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

