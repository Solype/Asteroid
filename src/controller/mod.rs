mod setup;
pub(crate) mod structs;
mod systems;

use bevy::prelude::{in_state, App, IntoScheduleConfigs, OnEnter, Single, Update, With};
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use crate::game_states::GameState;

fn grab_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>)
{
    options.visible = false;
    options.grab_mode = CursorGrabMode::Locked;
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), grab_mouse);
    app.add_systems(
        Update,
        (systems::player_system,
         systems::mouse_system,
         systems::rotate_spaceship,
         systems::roll_spaceship,
         systems::move_player_system
        )
            .in_set(structs::GameSystemSet)
            .run_if(in_state(GameState::Game)),
    );
    app.add_systems(OnEnter(GameState::Game), systems::setup_ui);
}
