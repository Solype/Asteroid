use crate::asteroids::{utils::f, *};
use crate::globals_structs::Score;
use crate::player::Ammo;

use rand::{prelude::IteratorRandom, Rng};

pub fn asteroid_asteroid_collision(
    mut asteroids_query: Query<(Entity, &Asteroid, &mut Transform, &mut Velocity)>,
) {
    let entity_ids: Vec<Entity> = asteroids_query
        .iter()
        .map(|(e, _, _, _)| e.entity())
        .collect(); // just to get count
    let len = entity_ids.len();

    for i in 0..len {
        for j in (i + 1)..len {
            let [a, b] = asteroids_query
                .get_many_mut([entity_ids[i], entity_ids[j]])
                .unwrap();

            let (_, a_ast, mut a_tf, mut a_vel) = a;
            let (_, b_ast, mut b_tf, mut b_vel) = b;
            let dist = a_tf.translation.distance(b_tf.translation);

            let a_radius = a_ast.size;
            let b_radius = b_ast.size;
            if dist > a_radius + b_radius {
                continue;
            }

            let delta = b_tf.translation - a_tf.translation;
            let n = delta / dist;

            let a_mass = a_ast.size.powi(3);
            let b_mass = b_ast.size.powi(3);

            let overlap = (a_radius + b_radius) - dist;
            let correction = n * (overlap / (a_mass + b_mass));
            a_tf.translation -= correction * b_mass; // lighter one moves more
            b_tf.translation += correction * a_mass;

            let v_rel = a_vel.0 - b_vel.0;
            let vel_along_normal = v_rel.dot(n);

            let impulse_mag = -(2.0 * vel_along_normal) / (1.0 / a_mass + 1.0 / b_mass);
            let impulse = impulse_mag * n;

            a_vel.0 += impulse / a_mass;
            b_vel.0 -= impulse / b_mass;
        }
    }
}

pub fn asteroid_player_collision(
    mut commands: Commands,
    mut asteroids_query: Query<(Entity, &Asteroid, &Transform, &Velocity)>,
) {
    // todo
}

pub fn get_score(size_type: &str) -> u32 {
    return match size_type {
        "XS" => 50,
        "S" => 30,
        "M" => 10,
        "L" => 5,
        "XL" => 1,
        "XXL" => 1,
        _ => 0,
    };
}

pub fn asteroid_ammo_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    assets: Res<AsteroidAssets>,
    asteroids_query: Query<
        (Entity, &Asteroid, &Transform),
        (Without<SpawnAnimation>, Without<DespawnAnimation>),
    >,
    ammos_query: Query<(Entity, &Transform), With<Ammo>>,
) {
    let mut rng = rand::rng();

    for (ammo_entity, ammo_transform) in &ammos_query {
        for (asteroid_entity, asteroid, asteroid_transform) in &asteroids_query {
            let dist = ammo_transform
                .translation
                .distance(asteroid_transform.translation);

            let ammo_radius = 1.0;
            if dist > ammo_radius + asteroid.size {
                continue;
            }

            let size_type = ASTEROID_SIZE_TYPES
                [(asteroid.size / (ASTEROID_SIZE_TYPES_LEN as f32)).round() as usize];

            score.value += get_score(size_type);

            commands.entity(asteroid_entity).insert(DespawnAnimation {
                timer: Timer::from_seconds(ANIMATION_DURATION, TimerMode::Once),
            });
            commands.entity(ammo_entity).despawn();
            if asteroid.size < 2.0 {
                return;
            }

            let new_size = asteroid.size / 2.0;

            let new_size_type =
                ASTEROID_SIZE_TYPES[(new_size / (ASTEROID_SIZE_TYPES_LEN as f32)).round() as usize];

            let new_size_rounded = new_size.round();

            let fw = ammo_transform.forward().normalize();
            let helper = if fw.abs().z < 0.9 { Vec3::Z } else { Vec3::Y };

            let u = fw.cross(helper).normalize();
            let v = fw.cross(u).normalize();

            let angle = rand::random::<f32>() * core::f32::consts::TAU;
            let new_dir = (u * angle.cos() + v * angle.sin()).normalize();
            let new_velocity = (new_dir * 0.3).normalize() * f(new_size);
            let new_velocity_neg = (-new_dir * 0.3).normalize() * f(new_size);

            let new_rotation_velocity = Vec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            )
            .normalize()
                * f(new_size)
                * 0.3;

            commands.spawn_batch([
                (
                    Mesh3d(
                        assets.meshes.get(new_size_type).unwrap()[rng.random_range(0..4)].clone(),
                    ),
                    MeshMaterial3d(assets.materials.get(new_size_type).unwrap().clone()),
                    Asteroid {
                        size: new_size_rounded,
                    },
                    Transform {
                        translation: asteroid_transform.translation + new_velocity * 0.1,
                        scale: Vec3 {
                            x: new_size_rounded,
                            y: new_size_rounded,
                            z: new_size_rounded,
                        },
                        rotation: Quat::from_rng(&mut rng),
                        ..Default::default()
                    },
                    Velocity(new_velocity),
                    RotationVelocity(new_rotation_velocity),
                ),
                (
                    Mesh3d(
                        assets.meshes.get(new_size_type).unwrap()[rng.random_range(0..4)].clone(),
                    ),
                    MeshMaterial3d(assets.materials.get(new_size_type).unwrap().clone()),
                    Asteroid {
                        size: new_size_rounded,
                    },
                    Transform {
                        translation: asteroid_transform.translation + new_velocity_neg * 0.1,
                        scale: Vec3 {
                            x: new_size_rounded,
                            y: new_size_rounded,
                            z: new_size_rounded,
                        },
                        rotation: Quat::from_rng(&mut rng),

                        ..Default::default()
                    },
                    Velocity(new_velocity_neg),
                    RotationVelocity(-new_rotation_velocity),
                ),
            ]);
            return;
        }
    }
}
