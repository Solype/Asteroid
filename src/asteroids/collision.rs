use crate::asteroids::{utils::f, *};
use crate::globals_structs::{MusicVolume, Score};
use crate::player::{Ammo, PlayerHitBox, PLAYER_MASS};
use crate::spritesheet::{AnimationDuration, AnimationTimer};

use bevy::audio::Volume;
use rand::seq::IndexedRandom;
use rand::Rng;

struct CollisionBody {
    tr: Vec3,
    vel: Vec3,
    radius: f32,
    mass: f32,
}

fn mass_collision(a_body: &mut CollisionBody, b_body: &mut CollisionBody, dist: f32) {
    let delta = b_body.tr - a_body.tr;
    let n = delta / dist;

    let overlap = (a_body.radius + b_body.radius) - dist;
    let correction = n * (overlap / (a_body.mass + b_body.mass));
    a_body.tr -= correction * b_body.mass; // lighter one moves more
    b_body.tr += correction * a_body.mass;

    let v_rel = a_body.vel - b_body.vel;
    let vel_along_normal = v_rel.dot(n);

    let impulse_mag = -(2.0 * vel_along_normal) / (1.0 / a_body.mass + 1.0 / b_body.mass);
    let impulse = impulse_mag * n;

    a_body.vel += impulse / a_body.mass;
    b_body.vel -= impulse / b_body.mass;
}

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
            let a_body = &mut CollisionBody {
                tr: a_tf.translation,
                vel: a_vel.0,
                radius: a_radius,
                mass: a_ast.size.powi(3),
            };
            let b_body = &mut CollisionBody {
                tr: b_tf.translation,
                vel: b_vel.0,
                radius: b_radius,
                mass: b_ast.size.powi(3),
            };

            mass_collision(a_body, b_body, dist);

            a_tf.translation = a_body.tr;
            b_tf.translation = b_body.tr;

            a_vel.0 = a_body.vel;
            b_vel.0 = b_body.vel;
        }
    }
}

pub fn asteroid_player_collision(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    player_hitboxes: Query<(&Transform, &PlayerHitBox), Without<Asteroid>>,
    //todo mut player_velocity: Single<&mut Velocity, (With<Player>, Without<Asteroid>)>,
    mut asteroids_query: Query<(Entity, &mut Transform, &Asteroid, &mut Velocity)>,
) {
    let player_velocity = Vec3::ZERO;
    for (hb_transform, player_hitbox) in &player_hitboxes {
        for (asteroid_entity, mut asteroid_transform, asteroid, mut asteroid_velocity) in
            &mut asteroids_query
        {
            let dist = hb_transform
                .translation
                .distance(asteroid_transform.translation);

            if dist > player_hitbox.radius + asteroid.size {
                continue;
            }

            let a_body = &mut CollisionBody {
                tr: hb_transform.translation,
                vel: player_velocity,
                radius: player_hitbox.radius,
                mass: PLAYER_MASS,
            };
            let b_body = &mut CollisionBody {
                tr: asteroid_transform.translation,
                vel: asteroid_velocity.0,
                radius: asteroid.size,
                mass: asteroid.size.powi(3),
            };

            mass_collision(a_body, b_body, dist);

            asteroid_transform.translation = b_body.tr;

            asteroid_velocity.0 = b_body.vel;

            // next_state.set(GameState::GameOver);
            return;
        }
    }
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
    audio: Res<BoomSounds>,
    master_volume: Res<MusicVolume>,
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

            let texture_atlas = TextureAtlas {
                layout: assets.explosion_layout.clone(),
                index: 0,
            };

            let mut rng = rand::rng();

            if let Some(handle) = audio.booms.choose(&mut rng) {
                commands.spawn((
                    Sprite {
                        image: assets.explosion_sheet.clone(),
                        texture_atlas: Some(texture_atlas),
                        ..default()
                    },
                    Sprite3d {
                        pixels_per_metre: 360.,
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    },
                    AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
                    AnimationDuration { frame_left: 7 },
                    Transform {
                        translation: ammo_transform.translation,
                        rotation: ammo_transform.rotation,
                        scale: asteroid_transform.scale,
                        ..Default::default()
                    },
                    children![(
                        AudioPlayer::new(handle.clone()),
                        PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Despawn,
                            volume: Volume::Linear(master_volume.volume / 100.0_f32),
                            spatial: true,
                            ..Default::default()
                        },
                        Transform::default(),
                    )],
                ));

            }
            return;
        }
    }
}
