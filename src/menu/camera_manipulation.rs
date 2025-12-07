use crate::menu::structs::*;
use bevy::prelude::*;

pub fn smooth_look_at_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &mut SmoothCamMove, &mut Projection)>,
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
            compute_change_fov(&mut params.fov, proj, t);
            compute_change_aspect_ratio(&mut params.aspect_ratio, proj, t);
        }
    }
}

////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////
// PRIVATE
////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

fn compute_change_fov(fov: &mut Option<f32>, proj: &mut PerspectiveProjection, t: f32) {
    let current_fov = proj.fov;

    if let Some(target_fov) = fov {
        let new_fov = current_fov + (*target_fov - current_fov) * t;

        if (new_fov - *target_fov).abs() < 1e-5 {
            *fov = None;
        }
        proj.fov = new_fov;
    }
}

fn compute_change_aspect_ratio(
    aspect_ratio: &mut Option<f32>,
    proj: &mut PerspectiveProjection,
    t: f32,
) {
    let current_aspect_ratio = proj.aspect_ratio;

    if let Some(target_aspect_ratio) = aspect_ratio {
        let new_aspect_ratio =
            current_aspect_ratio + (*target_aspect_ratio - current_aspect_ratio) * t;

        if (new_aspect_ratio - *target_aspect_ratio).abs() < 1e-5 {
            *aspect_ratio = None;
        }
        proj.aspect_ratio = new_aspect_ratio;
    }
}

fn compute_change_pos(param: &mut Option<Vec3>, transform: &mut Transform, t: f32) {
    let target = match param {
        Some(pos) => *pos,
        None => return,
    };
    if (transform.translation - target).length_squared() < 1e-5 {
        *param = None;
        return;
    }

    transform.translation = transform.translation.lerp(target, t);
}

fn compute_change_look_at(param: &mut Option<Vec3>, transform: &mut Transform, up: Vec3, t: f32) {
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

fn end_of_camera_movement(target: &SmoothCamMove) -> bool {
    if target.fov == None
        && target.aspect_ratio == None
        && target.look_at == None
        && target.position == None
    {
        return true;
    }
    return false;
}
