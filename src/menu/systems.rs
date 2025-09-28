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
    texture: Res<MenuCameraTarget>,
    images: Res<Assets<Image>>
) {
    for event in events.read() {
        let cursor_x = event.cursor_x;
        let cursor_y = event.cursor_y;
        let Some(image) = images.get(&texture.image) else {
            continue;
        };

        for (transform, sprite, button, layer) in buttons.iter() {
            let event_layer = MenuTypes::layer(event.menu_id);
            if !layer.intersects(&event_layer) {
                info!("No layer intersects");
                continue;
            }
            let cursor_px = (cursor_x / event.width) * image.width() as f32;
            let cursor_py = (cursor_y / event.height) * image.height() as f32;
            info!("event.height {}, cursor_y {}, image.height() {}", event.height, cursor_y, image.height());
            info!("event.width {}, cursor_x {}, image.width() {}", event.width, cursor_x, image.width());
            info!("px {} py {}", cursor_px, cursor_py);
            check_button_collision(cursor_px, cursor_py, transform, sprite, button);
        }
    }
}

