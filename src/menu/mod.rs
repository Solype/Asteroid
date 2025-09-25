use bevy::prelude::*;
use crate::game_states::GameState;


mod structs;
mod plane_cast;
mod systems;
mod setup;

use plane_cast::*;
use structs::*;
use systems::*;
use setup::*;

pub fn menu_plugin(app: &mut App)
{
    app.add_systems(OnEnter(GameState::Menu), setup_menu);
    app.add_event::<MenuPlaneCursorCastEvent>();
    app.add_systems(
        Update,
        (menu_system, spawn_menu_plane, cast_ray_from_click, menu_button_collision_system).in_set(MenuSystemSet).run_if(in_state(GameState::Menu)),
    );
    app.add_systems(
    OnExit(GameState::Menu),
    menu_cleanup
    );
    app.add_systems(Update, print_all_entities.run_if(in_state(GameState::Game)));
}



