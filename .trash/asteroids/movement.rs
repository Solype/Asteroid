use bevy::prelude::*;
use crate::spawn::*;

pub fn move_rocks(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<Rock>>,
) {
    for (entity, mut transform, velocity) in &mut query {
        transform.translation += **velocity * time.delta_secs();

        if transform.translation.length() > 350.0 {
            commands.entity(entity).despawn();
        }
    }
}
