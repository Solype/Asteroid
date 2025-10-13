use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::controller::PlayerCam;
use crate::menu::structs::*;


pub fn cast_ray_from_cursor(
    mut writer: MessageWriter<MenuPlaneCursorCastEvent>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<PlayerCam>>,
    planes: Query<(&GlobalTransform, &MenuPlane)>,
    inputs: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
) {
    let pressed = inputs.just_pressed(MouseButton::Left);

    if mouse_motion.is_empty() && !pressed {
        return;
    }
    mouse_motion.clear();

    let window = windows.single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else { return; };

    let Ok((camera, cam_transform)) = cameras.single() else { return; };
    let Some(ray) = ray_from_cursor(camera, cam_transform, cursor_pos) else { return; };

    for (plane_transform, menu_plane) in planes.iter() {
        if let Some(world_point) = ray_plane_intersection(ray.origin, ray.direction.into(), menu_plane.center, menu_plane.normal, plane_transform) {
            let local_point = world_to_plane_coords(world_point, menu_plane.normal, menu_plane.center, plane_transform);
            info!("local_point {}", local_point);
            writer.write(MenuPlaneCursorCastEvent {
                cursor_coordinates: local_point,
                screen_dimensions: menu_plane.dimensions,
                menu_id: menu_plane.menu_id,
                event_type: if pressed { CursorEventType::Click } else { CursorEventType::Move }
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

fn world_to_plane_coords(
    world_point: Vec3,
    normal: Vec3,
    center: Vec3,
    plane_transform: &GlobalTransform,
) -> Vec2 {
    let tmp_point = center + normal;
    let tmp_point_transformed = plane_transform.transform_point(tmp_point);
    let plane_origin = plane_transform.transform_point(center);

    let plane_normal = (tmp_point_transformed - plane_origin).normalize();

    let rel_point = world_point - plane_origin;

    let arbitrary = if plane_normal.abs_diff_eq(Vec3::Y, 1e-3) {
        Vec3::X
    } else {
        Vec3::Y
    };

    let right = plane_normal.cross(arbitrary).normalize();
    let up = right.cross(plane_normal).normalize();

    let u = rel_point.dot(right);
    let v = rel_point.dot(up);

    Vec2::new(u, v)
}


fn ray_from_cursor(
    camera: &Camera,
    cam_transform: &GlobalTransform,
    cursor_pos: Vec2,
) -> Option<Ray3d> {
    camera.viewport_to_world(cam_transform, cursor_pos).ok()
}


fn ray_plane_intersection(
    ray_origin: Vec3,
    ray_dir: Vec3,
    plane_center_local: Vec3,
    plane_normal_local: Vec3,
    plane_transform: &GlobalTransform,
) -> Option<Vec3>
{
    let plane_pos = plane_transform.transform_point(plane_center_local);
    let plane_normal = plane_transform.transform_point(plane_normal_local).normalize();

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
