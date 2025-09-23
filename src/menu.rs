use bevy::prelude::*;
use crate::controller::PlayerCam;
use crate::game_states::GameState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuSystemSet;

#[derive(Resource)]
struct SpawnMenuPlane;

#[derive(Component)]
struct MenuPlane;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup_menu);
    app.add_systems(
        Update,
        (menu_system, spawn_menu_plane, cast_ray_from_click).in_set(MenuSystemSet).run_if(in_state(GameState::Menu)),
    );
}

fn setup_menu(mut commands: Commands) {
    commands.insert_resource(SpawnMenuPlane);
}

fn ray_plane_intersection(
    ray_origin: Vec3,
    ray_dir: Vec3,
    plane_transform: &GlobalTransform,
) -> Option<Vec3> {
    let plane_pos = plane_transform.translation();
    let plane_normal = plane_transform.rotation() * Vec3::Y; // normal du plane (avant rot c'est Y)

    let denom = ray_dir.dot(plane_normal);
    if denom.abs() < 1e-6 {
        return None; // parallèle
    }

    let t = (plane_pos - ray_origin).dot(plane_normal) / denom;
    if t < 0.0 {
        return None; // intersection derrière la caméra
    }

    Some(ray_origin + t * ray_dir)
}

/// Système de picking avec clic gauche
fn cast_ray_from_click(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<PlayerCam>>,
    planes: Query<&GlobalTransform, With<MenuPlane>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single().expect("Pas de fenêtre unique !");
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok((camera, cam_transform)) = cameras.single() {
                if let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) {
                    let ray_origin = ray.origin;
                    let ray_dir = ray.direction;

                    // Test sur chaque plane du menu
                    for plane_transform in planes.iter() {
                        if let Some(world_point) =
                            ray_plane_intersection(ray_origin, ray_dir.into(), plane_transform)
                        {
                            // Convertir en coords locales
                            let local_point = plane_transform
                                .compute_matrix()
                                .inverse()
                                .transform_point3(world_point);

                            info!(
                                "Clic sur plane local coords: x={} z={}",
                                local_point.x, local_point.z
                            );

                            // Vérifier si c'est bien dans le rectangle (4x2)
                            if local_point.x.abs() <= 2.0 && local_point.z.abs() <= 1.0 {
                                info!("✅ Clic DANS le menu !");
                            } else {
                                info!("❌ Clic hors du menu.");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn spawn_menu_plane(
    mut commands: Commands,
    player_cam_query: Query<&GlobalTransform, With<PlayerCam>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            base_color: Color::srgb(0.8, 0.2, 0.2),
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


fn menu_system(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        next_state.set(GameState::Game);
    }
}
