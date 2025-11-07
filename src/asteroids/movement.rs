use crate::asteroids::*;

pub fn move_asteroids(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += **velocity * time.delta_secs();
    }
}

pub fn rotate_asteroids(
    time: Res<Time>,
    mut query: Query<(&mut Transform , &RotationVelocity)>,
) {
    for (mut transform, rotation_velocity) in &mut query {
        // compute delta rotation from angular velocity * delta time
        let delta_rotation = Quat::from_euler(
            EulerRot::XYZ,
            rotation_velocity.0.x * time.delta_secs(),
            rotation_velocity.0.y * time.delta_secs(),
            rotation_velocity.0.z * time.delta_secs(),
        );

        // compose rotations using multiplication
        transform.rotation *= delta_rotation;
    }
}
