use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

#[derive(Component, Default)]
pub struct VirtualMouse {
    pub pos: Vec2
}

pub fn mouse_system(
    mouse: Single<(&mut Node, &mut VirtualMouse)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let (mut node, mut virtual_mouse) = mouse.into_inner();

    let mouse_delta = accumulated_mouse_motion.delta;
    virtual_mouse.pos += mouse_delta;

    node.left = Val::Px(virtual_mouse.pos.x);
    node.top = Val::Px(virtual_mouse.pos.y);
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    target_camera: Single<Entity, With<crate::controller::PlayerCam>>,
) {
    let parent = commands
        .spawn((
            DespawnOnExit(crate::GameState::Game),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .id();

    let mouse_node = commands.spawn((
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ImageNode {
            image: asset_server.load("niko.jpeg"),
            ..default()
        },
        VirtualMouse::default(),
        UiTargetCamera(target_camera.into_inner()),
    ))
    .id();

    commands.entity(parent).add_child(mouse_node);
}
