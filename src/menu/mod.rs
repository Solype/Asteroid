use bevy::prelude::*;
use crate::game_states::GameState;


pub mod structs;
mod plane_cast;
mod systems;
mod setup;

use plane_cast::*;
use structs::*;
use systems::*;
use setup::*;

pub fn menu_plugin(app: &mut App)
{
    app.add_systems(Startup, setup_texture_camera);
    app.add_systems(PostStartup, (setup_menu, apply_texture_to_quad));
    app.add_event::<MenuPlaneCursorCastEvent>();
    app.add_systems(
        OnEnter(GameState::Menu), 
        (on_enter_menu, release_mouse)
    );
    app.add_systems(
        Update,
        (
            cast_ray_from_cursor, menu_button_collision_system, smooth_look_at_system
        ).in_set(MenuSystemSet)
        .run_if(in_state(GameState::Menu)),
    );
    // app.add_systems(
    // OnExit(GameState::Menu),
    // menu_cleanup
    // );
}



