use bevy::prelude::*;
use crate::controller::PlayerCam;
use crate::menu::structs::*;


pub fn cast_ray_from_click(
    mut writer: EventWriter<MenuPlaneCursorCastEvent>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<PlayerCam>>,
    mut planes: Query<(&GlobalTransform, &MenuPlane)>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return; };

    let Ok((camera, cam_transform)) = cameras.single() else { return; };
    let Some(ray) = ray_from_cursor(camera, cam_transform, cursor_pos) else { return; };

    for (plane_transform, menu_plane) in planes.iter_mut() {
        if let Some(world_point) =
            ray_plane_intersection(ray.origin, ray.direction.into(), plane_transform)
        {
            let local_point =
                plane_transform.compute_matrix().inverse().transform_point3(world_point);

            // émet l’event
            writer.write(MenuPlaneCursorCastEvent {
                cursor_x: local_point.x,
                cursor_y: local_point.y,
                width: menu_plane.width,
                height: menu_plane.height,
                menu_id: menu_plane.menu_id
            });
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// PRIVATE METHODE
/// 
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

fn ray_from_cursor(
    camera: &Camera,
    cam_transform: &GlobalTransform,
    cursor_pos: Vec2,
) -> Option<Ray3d> {
    camera.viewport_to_world(cam_transform, cursor_pos).ok()
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
