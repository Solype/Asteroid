mod setup;
pub(crate) mod structs;
mod systems;

use bevy::prelude::{in_state, App, IntoScheduleConfigs, OnEnter, Single, Update, With, NextState, ResMut};
use bevy::state::state::OnExit;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy::prelude::AppExtStates;
use crate::controller::structs::ControllerState;
use crate::controller::systems::free_look_system;
use crate::game_states::GameState;


fn grab_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>)
{
    options.visible = false;
    options.grab_mode = CursorGrabMode::Locked;
}

fn leave_game_state(
    mut next_state: ResMut<NextState<ControllerState>>,
) {
    next_state.set(ControllerState::None);
}

fn enter_game_state(
    mut next_state: ResMut<NextState<ControllerState>>,
) {
    next_state.set(ControllerState::Driving);
}

pub fn plugin(app: &mut App) {
    app.init_state::<ControllerState>();

    app.add_systems(OnEnter(GameState::Game), (grab_mouse, enter_game_state));
    app.add_systems(OnExit(GameState::Game), leave_game_state);

    app.add_systems(OnEnter(ControllerState::Driving), systems::setup_ui);

    app.add_systems(Update, systems::player_system.run_if(in_state(GameState::Game)));

    app.add_systems(Update, free_look_system.run_if(in_state(ControllerState::FreeLook)));

    app.add_systems(
        Update,
        (
            systems::mouse_system,
            systems::rotate_spaceship,
            systems::roll_spaceship,
            systems::move_player_system
        ).run_if(in_state(ControllerState::Driving)),
    );
}
