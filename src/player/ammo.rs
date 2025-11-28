use bevy::audio::Volume;
use rand::seq::IndexedRandom;

use crate::globals_structs::{Keybinds, MusicVolume};
use crate::{asteroids::Velocity, controller::Player, player::*};

pub fn shoot_ammo(
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut shoot_side: ResMut<ShootSide>,
    player: Single<&Transform, With<Player>>,
    assets: Res<AmmoAssets>,
    audio: Res<ShootSounds>,
    master_volume: Res<MusicVolume>,
    mut commands: Commands,
) {
    if !keybinds.shoot.just_pressed(&keyboard, &mouse) {
        return;
    }

    let local_offset = Vec3::new(shoot_side.value * 0.9, 1.0, -2.5);
    shoot_side.value = -shoot_side.value;

    let spawn_pos = player.transform_point(local_offset);
    let laser_dir = (player.forward().normalize() * 60.0 - spawn_pos).normalize();

    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            range: 20.0,
            radius: 1.0,
            color: Color::srgb(1.0, 0.0, 0.5),
            shadows_enabled: false,
            ..default()
        },
        Transform {
            translation: spawn_pos,
            scale: Vec3::new(0.5, 0.5, 5.0), // ellipse shape
            rotation: Quat::from_rotation_arc(Vec3::Z, laser_dir),
            ..Default::default()
        },
        Ammo,
        Velocity(laser_dir * 10.0), // fast forward
        children![(
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.material.clone()),
        )],
    ));

    let mut rng = rand::rng();

    if let Some(handle) = audio.shoot_pews.choose(&mut rng) {
        commands.spawn((
            AudioPlayer::new(handle.clone()),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::Linear(master_volume.volume / 100.0_f32),
                ..Default::default()
            },
        ));
    }
}

pub fn move_ammos(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Ammo>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += **velocity * time.delta_secs();
    }
}

pub fn clear_ammos(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    mut query: Query<(Entity, &Transform), With<Ammo>>,
) {
    for (entity, transform) in &mut query {
        let distance = transform.translation.distance(player.translation);
        if distance > 50.0 {
            commands.entity(entity).despawn();
        }
    }
}
