use rand::Rng;

use crate::controller::structs::ControllerState;
use crate::game_over::*;
use crate::physics::{RotationVelocity, Velocity};

pub fn setup_drift(player: Single<(&mut Velocity, &mut RotationVelocity), With<Player>>) {
    let (mut vel, mut rot) = player.into_inner();

    let mut rng = rand::rng();

    **vel *= 10.;
    *rot = RotationVelocity(Vec3::new(
        rng.random_range(-2.0..2.0),
        rng.random_range(-2.0..2.0),
        rng.random_range(-2.0..2.0),
    ))
}

pub fn run_drift(
    time: Res<Time>,
    mut timer: ResMut<DriftTimer>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_gameover_state: ResMut<NextState<GameOverState>>,
    mut next_controller_state: ResMut<NextState<ControllerState>>,
) {
    if !timer.timer.is_finished() {
        timer.timer.tick(time.delta());
    } else {
        next_menu_state.set(MenuState::GameOver);
        next_game_state.set(GameState::Menu);
        next_gameover_state.set(GameOverState::Menu);
        next_controller_state.set(ControllerState::None);
    }
}
