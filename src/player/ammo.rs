use bevy::audio::Volume;
use rand::seq::IndexedRandom;

use crate::config::structs::GameConfig;
use crate::globals_structs::{Keybinds, MusicVolume};
use crate::physics::{RotationVelocity, Velocity};
use crate::{controller::structs::Player, player::*};

pub fn shoot_ammo(
    game_config: Res<GameConfig>,
    mut commands: Commands,
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut shoot_side: ResMut<ShootSide>,
    player: Single<(&Transform, &Velocity, &RotationVelocity), With<Player>>,
    assets: Res<AmmoAssets>,
    audio: Res<ShootSounds>,
    master_volume: Res<MusicVolume>,
) {
    if !keybinds.shoot.just_pressed(&keyboard, &mouse) {
        return;
    }

    let (player_tr, player_vel, player_rot) = player.into_inner();

    let local_offset = Vec3::from(if shoot_side.left {
        game_config.ship.gun_left
    } else {
        game_config.ship.gun_right
    });
    shoot_side.left = !shoot_side.left;

    let spawn_pos = player_tr.transform_point(local_offset);
    let laser_dir = (player_tr.forward().normalize() * 60.0 - local_offset).normalize();

    let world_offset = player_tr.rotation * local_offset;
    let tangential_vel = player_rot.cross(world_offset);

    let final_vel = player_vel.0 // inherit ship movement
              + tangential_vel// inherit rotational motion
              + laser_dir * game_config.ship.ammo.speed; // base speed

    let color_vec3: Vec3 = game_config.ship.ammo.color;
    let color: Color = Color::srgb(color_vec3.x, color_vec3.y, color_vec3.z);

    let mut rng = rand::rng();
    if let Some(handle) = audio.shoot_pews.choose(&mut rng) {
        commands.spawn((
            PointLight {
                intensity: 100_000.0,
                range: 20.0,
                radius: 1.0,
                color: color,
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
            Velocity(final_vel), // fast forward
            children![
                (
                    Mesh3d(assets.mesh.clone()),
                    MeshMaterial3d(assets.material.clone()),
                ),
                (
                    AudioPlayer::new(handle.clone()),
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::Linear(master_volume.volume / 100.0_f32),
                        spatial: true,
                        ..Default::default()
                    },
                    Transform::default(),
                )
            ],
        ));
    }
}

pub fn clear_ammos(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    mut query: Query<(Entity, &Transform), With<Ammo>>,
    game_config: Res<GameConfig>,
) {
    for (entity, transform) in &mut query {
        let distance = transform.translation.distance(player.translation);
        if distance > game_config.ship.ammo.distance_despawn {
            commands.entity(entity).despawn();
        }
    }
}
