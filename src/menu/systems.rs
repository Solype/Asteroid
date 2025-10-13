use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::render::view::RenderLayers;
use bevy::window::CursorGrabMode;
use crate::controller::PlayerCam;
use crate::game_states::GameState;
use crate::menu::structs::*;



pub fn release_mouse(mut window: Single<&mut Window>)
{
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

pub fn on_enter_menu(mut command: Commands, entity: Single<Entity, With<PlayerCam>>)
{
    let player = entity.into_inner();
    // let corners = [
    //         Vec3::new(-0.216544, 0.777080, -0.318808),
    //         Vec3::new(0.216575, 0.777080, -0.318808),
    //         Vec3::new(0.216575, 0.640333, -0.261248),
    //         Vec3::new(-0.216544, 0.640333, -0.261248),
    // ];
    let tmp = Vec3 { x: 0.0, y: 0.7087065, z: -0.29002798 } - Vec3 { x: 0.0, y: 1.1, z: 0.3 };
    let distance = tmp.length();

    command.entity(player).insert(SmoothCamMove {
        look_at: Some(Vec3 { x: 0.0, y: 0.7087065, z: -0.29002798 }),
        speed: Some(1.0),
        up: Some(Vec3::Y),
        view_rect : Some(ViewRect { width: 1.0, height: 1.5, distance: distance }),
        ..Default::default()
    });
}

pub fn menu_button_collision_system(
    mut events: EventReader<MenuPlaneCursorCastEvent>,
    buttons: Query<(&Transform, &Sprite, &MenuButton, &RenderLayers)>,
    texture: Res<MenuCameraTarget>,
    images: Res<Assets<Image>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>
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
                (event.cursor_coordinates.y / event.screen_dimensions.y) * image.height() as f32
            );

            let Some(action) = check_button_collision(cursor_cast, transform, sprite, button) else {
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

fn point_in_button(cursor_x: f32, cursor_y: f32, pos: Vec3, size: Vec2) -> bool
{
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
    let Some(size) = sprite.custom_size else { return None; };
    if !point_in_button(cursor.x, cursor.y, transform.translation, size) { return None; }
    return Some(button.action);
}



