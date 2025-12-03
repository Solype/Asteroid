mod setup;
pub(crate) mod structs;
mod systems;

use crate::controller::structs::ControllerState;
use crate::controller::systems::free_look_system;
use crate::game_states::GameState;
use bevy::prelude::*;
use bevy::state::state::OnExit;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

#[derive(Component)]
pub struct DrivingUI;

fn grab_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    options.visible = false;
    options.grab_mode = CursorGrabMode::Locked;
}

fn leave_game_state(mut next_state: ResMut<NextState<ControllerState>>) {
    next_state.set(ControllerState::None);
}

fn enter_game_state(mut next_state: ResMut<NextState<ControllerState>>) {
    next_state.set(ControllerState::Driving);
}

pub fn plugin(app: &mut App) {
    app.init_state::<ControllerState>();

    app.add_systems(
        OnEnter(GameState::Game),
        (grab_mouse, enter_game_state, systems::setup_ui),
    );
    app.add_systems(OnExit(GameState::Game), leave_game_state);

    app.add_systems(
        OnEnter(ControllerState::Driving),
        systems::enter_driving_mod,
    );

    app.add_systems(
        Update,
        systems::player_system.run_if(in_state(GameState::Game)),
    );

    app.add_systems(
        OnEnter(ControllerState::FreeLook),
        systems::enter_free_look_mod,
    );
    app.add_systems(
        Update,
        free_look_system.run_if(in_state(ControllerState::FreeLook)),
    );

    app.add_systems(
        Update,
        systems::mouse_system.run_if(in_state(ControllerState::Driving)),
    );

    app.add_systems(
        Update,
        (
            systems::rotate_spaceship,
            systems::roll_spaceship,
            systems::move_player_system,
            systems::move_player
        )
            .run_if(in_state(GameState::Game)),
    );
}
