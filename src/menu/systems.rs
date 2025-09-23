use bevy::prelude::*;
use crate::game_states::GameState;
use crate::menu::structs::*;


pub fn menu_system(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        next_state.set(GameState::Game);
    }
}

pub fn print_all_entities(query: Query<(Entity, &MenuCameraComponent)>) {
    for (entity, _) in query.iter() {
        let mut comps = vec![];
        comps.push("MenuCameraComponent");
        info!("Entity {:?} components: {:?}", entity, comps);
    }
}