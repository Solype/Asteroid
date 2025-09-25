use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::game_states::GameState;
use crate::menu::structs::*;



pub fn menu_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if keyboard.just_pressed(KeyCode::KeyW) {
        next_state.set(GameState::Game);
    }
}

// pub fn 

pub fn print_all_entities(query: Query<Entity, With<MenuCameraComponent>>) {
    for entity in query.iter() {
        let mut comps = vec![];
        comps.push("MenuCameraComponent");
        info!("Entity {:?} components: {:?}", entity, comps);
    }
}







fn point_in_button(cursor_x: f32, cursor_y: f32, pos: Vec3, size: Vec2) -> bool
{
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;

    let in_x = cursor_x >= pos.x - half_w && cursor_x <= pos.x + half_w;
    let in_y = cursor_y >= pos.y - half_h && cursor_y <= pos.y + half_h;

    in_x && in_y
}

fn check_button_collision(
    cursor_x: f32,
    cursor_y: f32,
    transform: &Transform,
    sprite: &Sprite,
    button: &MenuButton,
) {
    if let Some(size) = sprite.custom_size {
        if point_in_button(cursor_x, cursor_y, transform.translation, size) {
            info!("✅ Collision avec bouton {:?}", button.action);
            match button.action {
                MenuAction::Start => info!("🚀 Lancer le jeu !"),
                MenuAction::Quit => info!("👋 Quitter le jeu !"),
            }
        }
    }
}



pub fn menu_button_collision_system(
    mut events: EventReader<MenuPlaneCursorCastEvent>,
    buttons: Query<(&Transform, &Sprite, &MenuButton, &RenderLayers)>,
) {
    for event in events.read() {
        let cursor_x = event.cursor_x;
        let cursor_y = event.cursor_y;

        for (transform, sprite, button, layer) in buttons.iter() {
            // Filtrer sur le layer correspondant à ce menu
            let event_layer = MenuTypes::layer(event.menu_id);
            if !layer.intersects(&event_layer) {
                info!("No layer intersects");
                continue; // ignorer les boutons d'un autre menu
            }
            check_button_collision(cursor_x, cursor_y, transform, sprite, button);
        }
    }
}

