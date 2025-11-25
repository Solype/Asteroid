use bevy::prelude::*;

pub mod camera_controller;
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller::run_camera_controller);
    }
}
