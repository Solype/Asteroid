use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::CursorGrabMode;
use crate::controller::PlayerCam;
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

pub fn release_mouse(mut window: Single<&mut Window>)
{
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

pub fn on_enter_menu(mut command: Commands, entity: Single<Entity, With<PlayerCam>>)
{
    let player = entity.into_inner();

    command.entity(player).insert(SmoothLookAt {
        target_world: Vec3 { x: 0.0, y: 0.7087065, z: -0.29002798 },
        speed: 1.0,
        up: Vec3::Y,
    });


}

pub fn smooth_look_at_system(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &SmoothLookAt), With<Camera>>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, params) in q.iter_mut() {
        let to_target = params.target_world - transform.translation;
        if to_target.length_squared() < 1e-8 {
            continue;
        }

        let mut tmp_world = Transform::from_translation(transform.translation);
        tmp_world.look_at(params.target_world, params.up);
        let target_world_rot = tmp_world.rotation;

        let t = 1.0 - (-params.speed * dt).exp();
        transform.rotation = transform.rotation.slerp(target_world_rot, t);

        let angle = transform.rotation.angle_between(target_world_rot);
        if angle < 1e-3 {
            transform.rotation = target_world_rot;
            commands.entity(entity).remove::<SmoothLookAt>();
        }
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
        let cursor_x = event.cursor_coordinates.x;
        let cursor_y = event.cursor_coordinates.y;
        let Some(image) = images.get(&texture.image) else {
            continue;
        };

        for (transform, sprite, button, layer) in buttons.iter() {
            let event_layer = MenuTypes::layer(event.menu_id);
            if !layer.intersects(&event_layer) {
                info!("No layer intersects");
                continue;
            }
            let cursor_px = (cursor_x / event.screen_dimensions.x) * image.width() as f32;
            let cursor_py = (cursor_y / event.screen_dimensions.y) * image.height() as f32;
            info!("event.screen_dimensions.y {}, cursor_y {}, image.height() {}", event.screen_dimensions.y, cursor_y, image.height());
            info!("event.screen_dimensions.x {}, cursor_x {}, image.width() {}", event.screen_dimensions.x, cursor_x, image.width());
            info!("px {} py {}", cursor_px, cursor_py);
            check_button_collision(cursor_px, cursor_py, transform, sprite, button);
        }
    }
}

