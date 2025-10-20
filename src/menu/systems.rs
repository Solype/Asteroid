use bevy::{
    // app::AppExit,
    prelude::*, window::{
        CursorGrabMode, CursorOptions,PrimaryWindow
    }
};
use crate::controller::PlayerCam;
// use crate::game_states::GameState;
use crate::menu::structs::*;



pub fn release_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>)
{
    options.grab_mode = CursorGrabMode::None;
    options.visible = true;
}

pub fn remove_focus_menu(mut command: Commands, entity: Single<Entity, With<PlayerCam>>)
{
    let player = entity.into_inner();

    command.entity(player).insert(SmoothCamMove {
        speed : Some(3.0),
        fov : Some(45.0_f32.to_radians()),
        position : Some(Vec3::new(0.0, 1.1, 0.3)),
        ..Default::default()
    });
}

pub fn focus_main_screen(mut command: Commands, player_entity: Single<Entity, With<PlayerCam>>)
{
    let player = player_entity.into_inner();
    let center = Vec3::new(0.0, 0.7087065, -0.29002798);
    let new_position = Vec3::new(0.0, 1.05, 0.27);

    command.entity(player).insert(SmoothCamMove {
        look_at: Some(center),
        position: Some(new_position),
        speed: Some(3.0),
        up: Some(Vec3::Y),
        fov: Some(20.0_f32.to_radians()),
        ..Default::default()
    });
}

// pub fn menu_button_collision_system(
//     mut commands : Commands,
//     mut events: MessageReader<MenuPlaneCursorCastEvent>,
//     buttons: Query<(Entity, &ComputedNode, &UiGlobalTransform, &MenuButton)>,
//     texture: Res<MenuCameraTarget>,
//     images: Res<Assets<Image>>,
//     mut next_state: ResMut<NextState<GameState>>,
//     mut exit: MessageWriter<AppExit>,
// ) {

//     let border_color_normal = BorderColor {
//         top: Color::srgba(0.0, 0.9, 1.0, 0.3),
//         bottom: Color::srgba(0.0, 0.6, 0.8, 0.3),
//         left: Color::srgba(0.0, 0.8, 1.0, 0.4),
//         right: Color::srgba(0.0, 0.8, 1.0, 0.4),
//     };

//     let border_color_hover = BorderColor {
//         top: Color::srgba(0.0, 1.0, 1.0, 0.9),
//         bottom: Color::srgba(0.0, 0.9, 1.0, 0.8),
//         left: Color::srgba(0.0, 1.0, 1.0, 1.0),
//         right: Color::srgba(0.0, 1.0, 1.0, 1.0),
//     };


//     for event in events.read() {
//         let Some(image) = images.get(&texture.image) else {
//             info!("No image found for MenuCameraTarget");
//             continue;
//         };

//         let cursor_cast = Vec2::new(
//             (event.cursor_coordinates.x / event.screen_dimensions.x) * image.width() as f32 + (image.width() as f32 / 2.0),
//             (image.height() as f32 / 2.0) - (event.cursor_coordinates.y / event.screen_dimensions.y) * image.height() as f32,
//         );

//         for (entity, computed_node, transform, button) in buttons.iter() {
//             if computed_node.contains_point(*transform, cursor_cast) {
//                 match button.action {
//                     MenuAction::Quit => {
//                         if event.event_type == CursorEventType::Click {
//                             exit.write(AppExit::Success);
//                         }
//                     }
//                     MenuAction::Start => {
//                         if event.event_type == CursorEventType::Click {
//                             next_state.set(GameState::Game);
//                         }
//                     }
//                     MenuAction::Options => {}
//                 }
//                 commands.entity(entity).insert(border_color_hover);
//             } else {
//                 commands.entity(entity).insert(border_color_normal);
//             }
//         }
//     }
// }
