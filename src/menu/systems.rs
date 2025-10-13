use crate::controller::PlayerCam;
use crate::game_states::GameState;
use crate::menu::structs::*;
use bevy::app::AppExit;
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

pub fn release_mouse(mut windows: Query<(&Window, &mut CursorOptions)>) {
    for (window, mut cursor_options) in &mut windows {
        if !window.focused {
            continue;
        }
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

pub fn on_enter_menu(mut command: Commands, entity: Single<Entity, With<PlayerCam>>) {
    let player = entity.into_inner();

    command.entity(player).insert(SmoothLookAt {
        target_world: Vec3 {
            x: 0.0,
            y: 0.7087065,
            z: -0.29002798,
        },
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

pub fn menu_button_collision_system(
    mut events: MessageReader<MenuPlaneCursorCastEvent>,
    buttons: Query<(&Transform, &Sprite, &MenuButton, &RenderLayers)>,
    texture: Res<MenuCameraTarget>,
    images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in events.read() {
        let Some(image) = images.get(&texture.image) else {
            continue;
        };

        for (transform, sprite, button, layer) in buttons.iter() {
            let event_layer = MenuTypes::layer(event.menu_id);
            if !layer.intersects(&event_layer) {
                continue;
            }
            let cursor_cast = Vec2::new(
                (event.cursor_coordinates.x / event.screen_dimensions.x) * image.width() as f32,
                (event.cursor_coordinates.y / event.screen_dimensions.y) * image.height() as f32,
            );

            let Some(action) = check_button_collision(cursor_cast, transform, sprite, button)
            else {
                continue;
            };
            match action {
                MenuAction::Quit => {
                    if event.event_type == CursorEventType::Click {
                        info!("FIN DU JEU !");
                        exit.write(AppExit::Success);
                    }
                }
                MenuAction::Start => {
                    if event.event_type == CursorEventType::Click {
                        next_state.set(GameState::Game);
                    }
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////
///
/// PRIVATE METHODE
///
////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

fn point_in_button(cursor_x: f32, cursor_y: f32, pos: Vec3, size: Vec2) -> bool {
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;

    let in_x = cursor_x >= pos.x - half_w && cursor_x <= pos.x + half_w;
    let in_y = cursor_y >= pos.y - half_h && cursor_y <= pos.y + half_h;

    in_x && in_y
}

fn check_button_collision(
    cursor: Vec2,
    transform: &Transform,
    sprite: &Sprite,
    button: &MenuButton,
) -> Option<MenuAction> {
    let Some(size) = sprite.custom_size else {
        return None;
    };
    info!(
        "cursor: {}; dimention bouton : coordinate : {}; size : {}",
        cursor, transform.translation, size
    );
    if !point_in_button(cursor.x, cursor.y, transform.translation, size) {
        return None;
    }
    return Some(button.action);
}
