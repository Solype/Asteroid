use crate::asteroids::*;

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

pub fn asteroid_ammo_collision(
    mut commands: Commands,
    mut asteroids_query: Query<(Entity, &Asteroid, &Transform, &Velocity)>,
) {
    // todo
}
