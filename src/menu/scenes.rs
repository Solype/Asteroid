use bevy::{prelude::*};
use crate::game_states::GameState;
use crate::menu::structs::*;

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

pub fn create_main_menu_scene(
    mut commands: Commands,
    camera_components: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    asset_server: Res<AssetServer>,
    menu_ressources: Option<ResMut<MainMenuRessources>>
) {
    let (cam_entity, mut camera) = camera_components.into_inner();
    camera.is_active = true;

    let font: Handle<Font>;
    let background : Handle<Image>;

    if let Some(ressources) = menu_ressources {
        font = ressources.font.clone();
        background = ressources.bg.clone();
    } else {
        font = asset_server.load("fonts/font.ttf");
        background = asset_server.load("menu_background.jpg");
        commands.insert_resource(MainMenuRessources {font: font.clone(), bg: background.clone()});
    }

    let border_radius = BorderRadius {
        top_right: Val::Px(20.0),
        bottom_left: Val::Px(20.0),
        top_left: Val::Px(4.0),
        bottom_right: Val::Px(4.0),
    };

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
                height: Val::Px(70.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        )).with_children(|parent| {


            parent.spawn((
                node.clone(),
                border_radius.clone(),
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
                border_radius.clone(),
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
                border_radius.clone(),
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




/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////


pub fn create_options_menu_scene(
    mut commands: Commands,
    camera_components: Single<(Entity, &mut Camera), With<MenuCameraComponent>>,
    menu_ressources: Res<MainMenuRessources>,
) {
    let (cam_entity, mut camera) = camera_components.into_inner();
    camera.is_active = true;

    let font = menu_ressources.font.clone();
    let background = menu_ressources.bg.clone();

    let border_radius = BorderRadius {
        top_right: Val::Px(20.0),
        bottom_left: Val::Px(20.0),
        top_left: Val::Px(4.0),
        bottom_right: Val::Px(4.0),
    };

    let node = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    };

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
        // Titre principal
        parent.spawn((
            Text::new("SYSTEM SETTINGS"),
            TextFont { font: font.clone(), font_size: 50.0, ..default() },
            TextColor(Color::srgb(0.0, 1.0, 0.0)),
            Node {
                margin: UiRect::all(Val::Px(30.0)),
                ..default()
            },
        ));


        // Section des options
        parent.spawn((
            Node {
                width: Val::Px(500.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Master Volume
            parent.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::new("Master Volume"),
                        TextFont { font: font.clone(), font_size: 26.0, ..default() },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new("100%"),
                        TextFont { font: font.clone(), font_size: 24.0, ..default() },
                        TextColor(Color::srgb(0.0, 1.0, 0.0)),
                    )
                ],
            ));

            // Bind 1
            parent.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::new("Thrust Key"),
                        TextFont { font: font.clone(), font_size: 26.0, ..default() },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new("W"),
                        TextFont { font: font.clone(), font_size: 24.0, ..default() },
                        TextColor(Color::srgb(0.0, 1.0, 1.0)),
                    )
                ],
            ));

            // Bind 2
            parent.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text::new("Fire Key"),
                        TextFont { font: font.clone(), font_size: 26.0, ..default() },
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new("Space"),
                        TextFont { font: font.clone(), font_size: 24.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.0, 0.0)),
                    )
                ],
            ));
        });

        // Bouton "Back"
        parent.spawn((
            node.clone(),
            border_radius.clone(),
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
