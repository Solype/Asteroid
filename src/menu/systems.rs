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
