use crate::distancemetric::structs::*;
use crate::{
    asteroids::Asteroid,
    controller::structs::{Player, PlayerCam, VirtualMouse},
    distancemetric::structs::MetricText,
    physics::Velocity,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn hit_sphere(center: Vec3, radius: f32, ray_pos: Vec3, ray_dir: Vec3) -> f32 {
    let oc = ray_pos - center;
    let a = ray_dir.dot(ray_dir);
    let b = 2.0 * oc.dot(ray_dir);
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    if t1 > 0.0 {
        t1
    } else {
        t2
    }
}

pub fn get_distance_of_object(
    asteroids: Query<(&GlobalTransform, &Asteroid)>,
    mut cameras: ParamSet<(
        Single<(&GlobalTransform, &Camera), With<PlayerCam>>,
        Single<&mut Camera, With<MetricCamComponent>>,
    )>,
    mouse: Single<&VirtualMouse>,
    mut text: ParamSet<(
        Single<&mut Text, With<MetricText>>,
        Single<&mut Text, With<SpeedText>>,
    )>,
    speed: Single<&Velocity, With<Player>>,
    window: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut timer: ResMut<DistanceTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        cameras.p1().into_inner().is_active = false;
        return;
    }
    cameras.p1().into_inner().is_active = true;

    {
        let speed = speed.into_inner().length();
        text.p1().0 = format!("{:.1}v", speed);
    }

    let (cam_transform, camera) = cameras.p0().into_inner();
    let window_dimension = Vec2::new(window.width() / 2., window.height() / 2.);

    let Ok(ray) = camera.viewport_to_world(cam_transform, mouse.pos + window_dimension) else {
        if text.p0().0 != "--m" {
            text.p0().0 = "--m".to_string();
        }
        return;
    };

    let ray_origin = cam_transform.transform_point(Vec3::ZERO);
    let ray_dir = ray.direction.normalize();

    let mut closest: Option<f32> = None;

    for (transform, asteroid) in &asteroids {
        let obj_pos = transform.translation();
        let radius = asteroid.size;
        let t = hit_sphere(obj_pos, radius, ray_origin, ray_dir);

        if t > 0.0 {
            if let Some(closest_val) = closest {
                if closest_val > t {
                    closest = Some(t);
                }
            } else {
                closest = Some(t);
            }
        }
    }

    text.p0().0 = match closest {
        Some(d) => format!("{:.0}m", d),
        None => "--m".into(),
    };
}
