use bevy::{
    prelude::*,
};
use crate::game_states::GameState;



#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuSystemSet;




pub fn menu_plugin(app: &mut App)
{
    app.add_systems(OnEnter(GameState::Menu), setup_menu);
    app.add_systems(Update, 
        (menu_system).in_set(MenuSystemSet).run_if(in_state(GameState::Menu))
    );
}

fn setup_menu(/*mut commands: Commands*/)
{
}

fn menu_system(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>)
{
    if keyboard.just_pressed(KeyCode::KeyW) {
        next_state.set(GameState::Game);
    }
}
