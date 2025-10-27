use bevy::prelude::*;
use bevy::picking::PickingSystems;
use crate::game_states::GameState;


pub mod structs;
mod plane_cast;
mod systems;
mod setup;
mod camera_manipulation;
mod scenes;


use plane_cast::*;
use systems::*;
use setup::*;
use camera_manipulation::*;
use scenes::*;

pub fn menu_plugin(app: &mut App)
{
    app.add_systems(Startup, (setup_texture_camera, setup_cube_ptr));
    app.add_systems(PostStartup, (setup_menu, apply_texture_to_quad));
    app.add_systems(OnEnter(GameState::Menu), (focus_main_screen, create_main_menu_scene, release_mouse));
    app.add_systems(OnExit(GameState::Menu), (remove_focus_menu, cleanup_menu_cam));
    app.add_systems(First, drive_diegetic_pointer
        .run_if(in_state(GameState::Menu))
        .in_set(PickingSystems::Input));
    app.add_systems(Update, smooth_look_at_system);
}



