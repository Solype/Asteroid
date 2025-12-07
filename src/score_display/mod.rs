use bevy::prelude::*;

mod setup;
pub mod structs;
mod systems;

pub fn score_display_plugin(app: &mut App) {
    app.add_systems(Startup, setup::setup_texture_camera);
    app.add_systems(
        PostStartup,
        (setup::setup_score, setup::apply_texture_to_quad),
    );
    app.add_systems(Update, systems::toggle_screenshot_camera);
}
