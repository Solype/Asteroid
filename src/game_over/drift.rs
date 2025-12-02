use crate::game_over::*;

pub fn run_drift(
    time: Res<Time>,
    mut timer: ResMut<DriftTimer>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_gameover_state: ResMut<NextState<GameOverState>>,
) {
    if !timer.timer.is_finished() {
        timer.timer.tick(time.delta());
        // todo Drift logic
    } else {
        next_menu_state.set(MenuState::GameOver);
        next_game_state.set(GameState::Menu);
        next_gameover_state.set(GameOverState::Menu);
    }
}