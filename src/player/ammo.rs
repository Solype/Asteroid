use crate::globals_structs::Keybinds;
use crate::{asteroids::Velocity, controller::structs::Player, player::*};

pub fn shoot_ammo(
    keybinds: Res<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Single<&Transform, With<Player>>,
    assets: Res<AmmoAssets>,
    mut commands: Commands,
) {
    if !keybinds.shoot.just_pressed(&keyboard, &mouse) {
        return;
    }

    let spawn_pos = player.translation;

    let laser_dir = player.forward().normalize();

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
