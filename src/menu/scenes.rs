use bevy::{
    prelude::*,
};
use crate::game_states::GameState;
use crate::globals_structs::{Action, Keybinds, MusicVolume};
use crate::menu::structs::*;
use crate::globals_structs::UIRessources;

static BORDER_HOVER: BorderColor = BorderColor {
    top: Color::srgba(0.0, 1.0, 1.0, 0.9),
    bottom: Color::srgba(0.0, 0.9, 1.0, 0.8),
    left: Color::srgba(0.0, 1.0, 1.0, 1.0),
    right: Color::srgba(0.0, 1.0, 1.0, 1.0),
};

static BORDER_NORMAL: BorderColor = BorderColor {
    top: Color::srgba(0.0, 0.9, 1.0, 0.3),
    bottom: Color::srgba(0.0, 0.6, 0.8, 0.3),
    left: Color::srgba(0.0, 0.8, 1.0, 0.4),
    right: Color::srgba(0.0, 0.8, 1.0, 0.4),
};

static BORDER_RADIUS_SQUARE: BorderRadius = BorderRadius {
    top_right: Val::Px(4.0),
    bottom_left: Val::Px(4.0),
    top_left: Val::Px(4.0),
    bottom_right: Val::Px(4.0),
};

pub fn create_main_menu_scene(
    mut commands: Commands,
    camera_components: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    menu_ressources: Res<UIRessources>
) {
    let (cam_entity, mut camera) = camera_components.into_inner();
    camera.is_active = true;

    let font: Handle<Font> = menu_ressources.font.clone();
    let background : Handle<Image> = menu_ressources.bg.clone();

    let node = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    };

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ImageNode {
            image: background,
            ..default()
        },
        UiTargetCamera(cam_entity),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("STARSHIP DASHBOARD"),
            TextFont { font: font.clone(), font_size: 60.0, ..default() },
            TextColor(Color::srgb(0.0, 1.0, 1.0)),
            Node {
                margin: UiRect::all(Val::Px(30.0)),
                ..default()
            },
        ));
        parent.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                node.clone(),
                BORDER_NORMAL,
                BackgroundColor(Color::srgba(0.0, 0.2, 0.4, 0.8)), // dark blue transparent
                children![(
                    Text::new("LAUNCH MISSION"),
                    TextFont { font: font.clone(), font_size: 28.0, ..default() },
                    TextColor(Color::srgb(0.0, 1.0, 1.0)),
                )],
            )).observe(|over: On<Pointer<Over>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(over.entity).unwrap()) = BORDER_HOVER;
            }).observe(|out: On<Pointer<Out>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(out.entity).unwrap()) = BORDER_NORMAL;
            }).observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Game);
            });



            parent.spawn((
                node.clone(),
                BORDER_NORMAL,
                BackgroundColor(Color::srgba(0.0, 0.2, 0.0, 0.8)), // dark green transparent
                children![(
                    Text::new("SYSTEM SETTINGS"),
                    TextFont { font: font.clone(), font_size: 28.0, ..default() },
                    TextColor(Color::srgb(0.0, 1.0, 0.0)),
                )],
            )).observe(|over: On<Pointer<Over>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(over.entity).unwrap()) = BORDER_HOVER;
            }).observe(|out: On<Pointer<Out>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(out.entity).unwrap()) = BORDER_NORMAL;
            }).observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<MenuState>>| {
                next_state.set(MenuState::Options);
            });


            parent.spawn((
                node.clone(),
                BORDER_NORMAL,
                BackgroundColor(Color::srgba(0.4, 0.0, 0.0, 0.8)), // dark red transparent
                children![(
                    Text::new("EJECT"),
                    TextFont { font: font.clone(), font_size: 28.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.0, 0.0)),
                )],
            )).observe(|over: On<Pointer<Over>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(over.entity).unwrap()) = BORDER_HOVER;
            }).observe(|out: On<Pointer<Out>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(out.entity).unwrap()) = BORDER_NORMAL;
            }).observe(|_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success);
            });
        });
    });
}

pub fn create_options_menu_scene(
    mut commands: Commands,
    camera_components: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    menu_ressources: Res<UIRessources>,
    master_volume: Res<MusicVolume>,
    keybinds: Res<Keybinds>,
) {
    let (cam_entity, mut camera) = camera_components.into_inner();
    camera.is_active = true;

    let font = menu_ressources.font.clone();
    let background = menu_ressources.bg.clone();

    let node = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    };

    // === Racine du menu ===
    commands.spawn((
        DespawnOnExit(MenuState::Options),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ImageNode {
            image: background,
            ..default()
        },
        UiTargetCamera(cam_entity),
    ))
    .with_children(|parent| {
        // === Titre ===
        parent.spawn((
            Text::new("SYSTEM SETTINGS"),
            TextFont { font: font.clone(), font_size: 50.0, ..default() },
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
            Node {
                margin: UiRect::all(Val::Px(30.0)),
                ..default()
            },
        ));

        // === Scrollable container ===
        parent
            .spawn((
                Node {
                    align_self: AlignSelf::Stretch,
                    height: percent(45),
                    overflow: Overflow::scroll_y(),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
            ))
            .with_children(|scroll_root| {
                // === Contenu scrollable ===
                scroll_root.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                ))
                .with_children(|content| {
                    // === Master Volume ===
                    content.spawn((
                        Node {
                            width: Val::Px(450.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::NONE),
                        ButtonInfo { border_hover: BORDER_HOVER, border_normal: BorderColor::all(Color::NONE) },
                        BORDER_RADIUS_SQUARE,
                        children![
                            (
                                Text::new("Master Volume"),
                                TextFont { font: font.clone(), font_size: 26.0, ..default() },
                                TextColor(Color::WHITE),
                            ),
                            (
                                Text::new(format!("{}%", master_volume.volume as i32)),
                                TextFont { font: font.clone(), font_size: 24.0, ..default() },
                                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                                VolumeText
                            )
                        ],
                    )).observe(|_: On<Pointer<Click>>, mut master_volume: ResMut<MusicVolume>, mut texts: Query<&mut Text, With<VolumeText>>| {
                        master_volume.volume = (master_volume.volume - 10.0).rem_euclid(110.0);
                        for mut text in &mut texts {
                            *text = Text::new(format!("{}%", master_volume.volume as i32));
                        }
                    });

                    // === 4 Key Binds ===
                    let binds = [
                        ("Up", keybinds.up, Action::Up),
                        ("Down", keybinds.down, Action::Down),
                        ("Left", keybinds.left, Action::Left),
                        ("Right", keybinds.right, Action::Right),
                        ("Forward", keybinds.forward, Action::Forward),
                        ("Backward", keybinds.backward, Action::Backward),
                        ("Rotate_left", keybinds.rotate_left, Action::RotateLeft),
                        ("Rotate_right", keybinds.rotate_right, Action::RotateRight),
                        ("Free look", keybinds.free_look, Action::FreeLook),
                        ("Shoot", keybinds.shoot, Action::Shoot),
                        ("Menu", keybinds.menu, Action::Menu),
                    ];

                    for (label, key, action) in binds {
                        content.spawn((
                            Node {
                                width: Val::Px(450.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            action,
                            ButtonInfo { border_hover: BORDER_HOVER, border_normal: BorderColor::all(Color::NONE) },
                            BorderColor::all(Color::NONE),
                            BORDER_RADIUS_SQUARE,
                        )).with_children(|parent|{
                            parent.spawn((
                                Text::new(label),
                                TextFont { font: font.clone(), font_size: 26.0, ..default() },
                                TextColor(Color::WHITE),
                            ));

                            parent.spawn((
                                Text::new(key.to_str()),
                                TextFont { font: font.clone(), font_size: 24.0, ..default() },
                                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                                action
                            ));
                        }) .observe(|click: On<Pointer<Click>>, action: Query<&Action>, mut waiting: ResMut<WaitingForRebind>| {
                            if waiting.0 != None {
                                return;
                            }
                            if let Ok(act) = action.get(click.entity) {
                                waiting.0 = Some(*act);
                                info!("Now waiting for new key for {:?}", waiting.0);
                            }
                        });
                    }
                });
            });

        // === Bouton "Back" ===
        parent
            .spawn((
                node.clone(),
                BORDER_NORMAL,
                BackgroundColor(Color::srgba(0.0, 0.2, 0.4, 0.8)),
                children![(
                    Text::new("BACK"),
                    TextFont { font: font.clone(), font_size: 28.0, ..default() },
                    TextColor(Color::srgb(0.0, 1.0, 1.0)),
                )],
            ))
            .observe(|over: On<Pointer<Over>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(over.entity).unwrap()) = BORDER_HOVER;
            })
            .observe(|out: On<Pointer<Out>>, mut colors: Query<&mut BorderColor>| {
                *(colors.get_mut(out.entity).unwrap()) = BORDER_NORMAL;
            })
            .observe(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<MenuState>>| {
                next_state.set(MenuState::Main);
            });
    });
}
