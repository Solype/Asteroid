use bevy::prelude::*;

pub mod spawn;
pub mod movement;
pub mod collision;

#[derive(Component)]
pub struct Asteroid {
    pub size: f32,
    pub split_level: u8,
}

// pub struct AsteroidPlugin;

// impl Plugin for AsteroidPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_systems(Startup, spawn::setup)
//             .add_systems(Update, (
//                 movement::asteroid_movement_system,
//                 collision::asteroid_collision_system,
//             ));
//     }
// }
