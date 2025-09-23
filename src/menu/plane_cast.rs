use bevy::prelude::*;
use crate::controller::PlayerCam;
use crate::menu::structs::*;


pub fn cast_ray_from_click(
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


////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
/// 
/// PRIVATE METHODE
/// 
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////



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
