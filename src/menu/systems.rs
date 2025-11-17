use crate::menu::structs::*;
use crate::{
    controller::PlayerCam,
    globals_structs::{Action, InputButton, Keybinds},
};
use bevy::{
    // app::AppExit,
    audio::Volume, input::mouse::{MouseScrollUnit, MouseWheel}, picking::hover::HoverMap, prelude::*, window::{
        CursorGrabMode, CursorOptions,PrimaryWindow
    }
};
use crate::{controller::PlayerCam, globals_structs::{Action, InputButton, Keybinds, MusicVolume}};
use crate::menu::structs::*;
use rand::seq::SliceRandom;



pub fn enter_menu_state(mut next_state: ResMut<NextState<MenuState>>) {
    next_state.set(MenuState::Main);
}

pub fn leave_menu_state(
    mut next_state: ResMut<NextState<MenuState>>,
    entity: Single<&mut Camera, With<MenuCameraComponent>>,
) {
    entity.into_inner().is_active = false;
    next_state.set(MenuState::None);
}

pub fn release_mouse(mut options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    options.grab_mode = CursorGrabMode::None;
    options.visible = true;
}

pub fn remove_focus_menu(mut command: Commands, entity: Single<Entity, With<PlayerCam>>) {
    let player = entity.into_inner();

    command.entity(player).insert(SmoothCamMove {
        speed: Some(3.0),
        fov: Some(45.0_f32.to_radians()),
        position: Some(Vec3::new(0.0, 1.1, 0.3)),
        ..Default::default()
    });
}

pub fn focus_main_screen(mut command: Commands, player_entity: Single<Entity, With<PlayerCam>>) {
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

pub fn play_click_sound_system(
    mut over_reader : MessageReader<Pointer<Over>>,
    mut out_reader : MessageReader<Pointer<Out>>,
    mut click_reader : MessageReader<Pointer<Click>>,
    audio : Res<MenuSounds>,
    master_volume : Res<MusicVolume>, 
    mut commands : Commands,
    query : Query<(&ButtonInfo, Entity)>,
) {
    for over in over_reader.read() {
        let Ok((button_info, entity)) = query.get(over.entity) else {
            continue;
        };
        commands.entity(entity).insert(button_info.border_hover);
    }
    for out in out_reader.read() {
        let Ok((button_info, entity)) = query.get(out.entity) else {
            continue;
        };
        commands.entity(entity).insert(button_info.border_normal);
    }
    if !click_reader.is_empty() && master_volume.volume != 0.0_f32 {
        let mut rng = rand::thread_rng();
        if let Some(handle) = audio.button_bips.choose(&mut rng) {
            commands.spawn((
                AudioPlayer::new(handle.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume : Volume::Linear(master_volume.volume / 100.0_f32),
                    ..Default::default()
                }
            ));
        }
        click_reader.clear();
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//
//
// SCROLL
//
//////////////////////////////////////////////////////////////////////////////////////////////

const LINE_HEIGHT: f32 = 21.;

pub fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(crate::menu::structs::Scroll { entity, delta });
            }
        }
    }
}

pub fn on_scroll_handler(
    mut scroll: On<crate::menu::structs::Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            // Consume the Y portion of the scroll delta.
            delta.y = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////
//
//
// REBIND
//
//////////////////////////////////////////////////////////////////////////////////////////////

pub fn rebind_key(
    mut waiting: ResMut<WaitingForRebind>,
    mut keybinds: ResMut<Keybinds>,
    mut texts: Query<(&mut Text, &Action)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if let Some(action) = waiting.0 {
        if let Some(code) = keyboard.get_just_pressed().last() {
            let button = InputButton::Key(*code);
            set_bind(&mut keybinds, action, button);
            update_text(&mut texts, action, button);
            waiting.0 = None;
            info!("Has bind  with keyboard!");
            return;
        }
        if let Some(code) = mouse.get_just_pressed().last() {
            let button = InputButton::Mouse(*code);
            set_bind(&mut keybinds, action, button);
            update_text(&mut texts, action, button);
            waiting.0 = None;
            info!("Has bind  with mouse!");
            return;
        }
    }
}

fn set_bind(binds: &mut Keybinds, action: Action, button: InputButton) {
    match action {
        Action::Up => binds.up = button,
        Action::Down => binds.down = button,
        Action::Left => binds.left = button,
        Action::Right => binds.right = button,
        Action::Forward => binds.forward = button,
        Action::Backward => binds.backward = button,
        Action::RotateLeft => binds.rotate_left = button,
        Action::RotateRight => binds.rotate_right = button,
        Action::FreeLook => binds.free_look = button,
        Action::Shoot => binds.shoot = button,
        Action::Menu => binds.menu = button,
    }
}

fn update_text(texts: &mut Query<(&mut Text, &Action)>, action: Action, button: InputButton) {
    for (mut text, act) in texts.iter_mut() {
        if *act == action {
            *text = Text::new(button.to_str());
        }
    }
}
