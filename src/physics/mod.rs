use bevy::prelude::*;

use crate::game_states::GameState;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct RotationVelocity(pub Vec3);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_body, rotate_body).run_if(in_state(GameState::Game)),
        );
    }
}


fn move_body(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += **velocity * time.delta_secs();
    }
}

fn rotate_body(
    time: Res<Time>,
    mut query: Query<(&mut Transform , &RotationVelocity)>,
) {
    for (mut transform, rotation_velocity) in &mut query {
        let delta_rotation = Quat::from_euler(
            EulerRot::XYZ,
            rotation_velocity.0.x * time.delta_secs(),
            rotation_velocity.0.y * time.delta_secs(),
            rotation_velocity.0.z * time.delta_secs(),
        );

        transform.rotation *= delta_rotation;
    }
}
