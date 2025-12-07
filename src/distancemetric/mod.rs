use bevy::prelude::*;

mod setup;
pub mod structs;
mod systems;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup::setup_texture_camera);
    app.add_systems(
        PostStartup,
        (setup::setup_metric_screen, setup::apply_texture_to_quad),
    );
    app.add_systems(Update, systems::get_distance_of_object);
}
