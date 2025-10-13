use bevy::prelude::*;
use crate::menu::structs::*;

pub fn smooth_look_at_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut SmoothCamMove, &mut Projection), With<Camera>>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut params, mut projection) in q.iter_mut() {
        let up = params.up.unwrap_or(Vec3::Y);
        let speed = params.speed.unwrap_or(1.0);
        let t = 1.0 - (-speed * dt).exp();

        if end_of_camera_movement(&params) {
            commands.entity(entity).remove::<SmoothCamMove>();
            continue;
        }
        compute_change_look_at(&mut params.look_at, &mut transform, up, t);
        compute_change_pos(&mut params.position, &mut transform, t);
        if let Projection::Perspective(proj) = projection.as_mut() {
            compute_change_dimensions(&mut params.view_rect,  proj, t);
        }
    }
}


////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////
// PRIVATE
////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

fn compute_change_dimensions(
    view_rect: &mut Option<ViewRect>,
    proj: &mut PerspectiveProjection,
    t: f32,
) {
    // Hauteur et largeur actuelles à la near plane
    let current_height = 2.0 * proj.near * (proj.fov * 0.5).tan();
    let current_width = current_height * proj.aspect_ratio;

    if let Some(vr) = view_rect {
        let target_height = vr.height * (proj.near / vr.distance);
        let target_width = vr.width * (proj.near / vr.distance);

        let new_height = current_height + (target_height - current_height) * t;
        let new_width = current_width + (target_width - current_width) * t;

        proj.aspect_ratio = new_width / new_height;
        proj.fov = 2.0 * (new_height / (2.0 * proj.near)).atan();

        if (new_height - target_height).abs() < 1e-6 && (new_width - target_width).abs() < 1e-6 {
            *view_rect = None;
        }
    }
}

fn compute_change_pos(param: &mut Option<Vec3>, transform: &mut Transform, t: f32) {
    let target = match param {
        Some(pos) => *pos,
        None => return,
    };
    if (transform.translation - target).length_squared() < 1e-6 {
        *param = None;
        return;
    }

    transform.translation = transform.translation.lerp(target, t);
}

fn compute_change_look_at(param : &mut Option<Vec3>, transform : &mut Transform, up : Vec3, t : f32)
{
    let look_at = match param {
        Some(target) => target,
        None => return,
    };

    let mut tmp_world = Transform::from_translation(transform.translation);
    tmp_world.look_at(*look_at, up);
    let look_at_rot = tmp_world.rotation;

    if transform.rotation.angle_between(look_at_rot) < 1e-3 {
        *param = None;
        return;
    }
    transform.rotation = transform.rotation.slerp(look_at_rot, t);
}

fn end_of_camera_movement(target : &SmoothCamMove) -> bool
{
    if target.view_rect == None 
    && target.look_at == None
    && target.position == None {
        return true;
    }
    return false;
}
