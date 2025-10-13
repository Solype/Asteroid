// use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;
// use super::Asteroid;

// /// Détection des collisions et split si nécessaire
// pub fn asteroid_collision_system(
//     mut commands: Commands,
//     mut collision_events: EventReader<CollisionEvent>,
//     query: Query<(Entity, &Asteroid, &Transform)>,
// ) {
//     for event in collision_events.read() {
//         if let CollisionEvent::Started(e1, e2, _) = event {
//             if let Ok((entity, asteroid, transform)) = query.get(*e1).or_else(|_| query.get(*e2)) {
//                 // Ici tu pourrais tester si c'est un projectile vs astéroïde
//                 // Pour l'instant : tout split à la 1ère collision
//                 if asteroid.split_level > 0 {
//                     let new_size = asteroid.size * 0.5;
//                     let new_level = asteroid.split_level - 1;

//                     commands.entity(entity).despawn_recursive();

//                     // Spawn 2 morceaux
//                     for offset in [-1.0, 1.0] {
//                         commands.spawn((
//                             PbrBundle {
//                                 transform: Transform::from_translation(
//                                     transform.translation + Vec3::new(offset * new_size, 0.0, 0.0)
//                                 ),
//                                 ..default()
//                             },
//                             Asteroid { size: new_size, split_level: new_level },
//                             RigidBody::Dynamic,
//                             Collider::ball(new_size),
//                             Velocity::linear(Vec3::new(offset as f32, 1.0, 0.0)),
//                         ));
//                     }
//                 }
//             }
//         }
//     }
// }
