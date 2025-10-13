use bevy::{prelude::*, render::camera::CameraProjection};
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
        let mut temp_width = params.width;
        let mut temp_height = params.height;

        if end_of_camera_movement(&params) {
            commands.entity(entity).remove::<SmoothCamMove>();
            continue;
        }
        compute_change_look_at(&mut params.look_at, &mut transform, up, t);
        compute_change_pos(&mut params.position, &mut transform, t);
        if let Projection::Perspective(ref mut proj) = *projection {
            compute_change_dimensions(&mut temp_width, &mut temp_height,  proj, t);
            params.width = temp_width;
            params.height = temp_height;
        }
    }
}


////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////
// PRIVATE
////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////

fn compute_change_dimensions(
    width: &mut Option<f32>,
    height: &mut Option<f32>,
    proj: &mut PerspectiveProjection,
    t: f32,
) {
    let current_height: f32 = 2.0 * proj.near * (proj.fov * 0.5).tan();
    let current_width: f32 = current_height * proj.aspect_ratio;

    let new_width = if let Some(target_width) = width {
        let tmp = current_width + (*target_width - current_width) * t;
        if (tmp - *target_width).abs() < 1e-6 {
            *width = None;
        }
        tmp
    } else {
        current_width
    };


    let new_height = if let Some(target_height) = height {
        let tmp = current_height + (*target_height - current_height) * t;
        if (tmp - *target_height).abs() < 1e-6 {
            *height = None;
        }
        tmp
    } else {
        current_height
    };

    proj.update(new_width, new_height);
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
    if target.height == None 
    && target.look_at == None
    && target.position == None
    && target.width == None
    && target.height == None {
        return true;
    }
    return false;
}
