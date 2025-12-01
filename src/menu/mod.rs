use bevy::prelude::*;
use bevy::picking::PickingSystems;
use crate::game_states::GameState;
use crate::menu::structs::{MenuState, WaitingForRebind};


pub mod structs;
mod plane_cast;
pub mod systems;
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
    // Initialisation of the ressources
    app.add_systems(Startup, (setup_texture_camera, setup_cube_ptr, setup_sound_effect_and_music));
    app.add_systems(PostStartup, apply_texture_to_quad);

    // Initialisation of the systems to enter Menu state of the game
    app.add_systems(OnEnter(GameState::Menu), (focus_main_screen, enter_menu_state, release_mouse));
    app.add_systems(OnExit(GameState::Menu), leave_menu_state);


    // Init the scene after entering into a specific menu state
    app.add_systems(OnEnter(MenuState::Main), create_main_menu_scene);
    app.add_systems(OnEnter(MenuState::Options), create_options_menu_scene);
    app.add_systems(OnEnter(MenuState::GameOver), create_gameover_menu_scene);

    app.add_systems(Update, (send_scroll_events, rebind_key, play_click_sound_system).run_if(in_state(GameState::Menu)));
    app.add_observer(on_scroll_handler);
    app.insert_resource(WaitingForRebind(None));

    // Overall modifications
    app.add_systems(First, drive_diegetic_pointer
        .run_if(in_state(GameState::Menu))
        .in_set(PickingSystems::Input));
    app.add_systems(Update, smooth_look_at_system);
    app.init_state::<MenuState>();
}



