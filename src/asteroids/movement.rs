use crate::asteroids::*;

pub fn move_asteroids(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += **velocity * time.delta_secs();
    }
}
