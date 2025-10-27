use crate::asteroids::utils::*;
use crate::asteroids::*;
use rand::Rng;

use crate::camera_controller::CameraController;

pub fn asteroid_wave(
    mut commands: Commands,
    config: ResMut<AsteroidConfig>,
    query: Query<Entity, With<Asteroid>>,
    assets: Res<AsteroidAssets>,
    player: Single<&Transform, With<CameraController>>,
) {
    let current = query.iter().count();
    if current >= config.max_asteroid {
        return;
    }

    let mut rng = rand::rng();

    let to_spawn = config.max_asteroid - current;
    for _ in 0..to_spawn {
        // Random direction on range sphere
        let theta = rng.random_range(0.0..std::f32::consts::TAU);
        let phi = rng.random_range(0.0..std::f32::consts::PI);

        let x = config.spawn_range * phi.sin() * theta.cos();
        let y = config.spawn_range * phi.cos();
        let z = config.spawn_range * phi.sin() * theta.sin();
        let position = Vec3::new(x, y, z) + player.translation;

        // Random inward velocity
        let random_dir = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize();

        let size = sample_truncated_norm(
            (config.size_range.0 + config.size_range.1) / 2.0,
            3.0,
            config.size_range.0,
            config.size_range.1,
            &mut rng,
        )
        .floor();
        let velocity = -(position.normalize() + random_dir * 0.3).normalize() * f(size);
        commands.spawn((
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.materials[size as usize].clone()),
            Asteroid { size: size },
            Transform {
                translation: position,
                scale: Vec3::ZERO,
                ..Default::default()
            },
            GlobalTransform::default(),
            Velocity(velocity),
            SpawnAnimation {
                timer: Timer::from_seconds(ANIMATION_DURATION, TimerMode::Once),
            },
        ));
    }
}

pub fn animate_spawn(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut SpawnAnimation, &mut Asteroid)>,
) {
    for (entity, mut transform, mut anim, asteroid) in &mut query {
        anim.timer.tick(time.delta());
        let t = anim.timer.fraction();
        transform.scale = Vec3::splat(t * asteroid.size);
        if anim.timer.is_finished() {
            commands.entity(entity).remove::<SpawnAnimation>();
        }
    }
}

pub fn animate_despawn(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut DespawnAnimation, &mut Asteroid)>,
) {
    for (entity, mut transform, mut anim, asteroid) in &mut query {
        anim.timer.tick(time.delta());
        let t = 1.0 - anim.timer.fraction();
        transform.scale = Vec3::splat(t * asteroid.size);
        if anim.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn clear_asteroid(
    mut commands: Commands,
    config: ResMut<AsteroidConfig>,
    player: Single<&Transform, With<CameraController>>,
    mut query: Query<(Entity, &Transform), (With<Asteroid>, Without<DespawnAnimation>)>,
) {
    for (entity, transform) in &mut query {
        let distance = transform.translation.distance(player.translation);
        if distance > config.despawn_range {
            commands.entity(entity).insert(DespawnAnimation {
                timer: Timer::from_seconds(ANIMATION_DURATION, TimerMode::Once),
            });
        }
    }
}
