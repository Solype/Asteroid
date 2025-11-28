use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::PrimaryWindow
};

#[derive(Component, Default)]
pub struct VirtualMouse {
    pub pos: Vec2
}

pub fn mouse_system(
    mut params: ParamSet<(
        Single<(&mut Node, &mut VirtualMouse)>,
        Single<&Window, With<PrimaryWindow>>,
    )>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let win_dim;
    {
        let dim: Vec2 = Vec2{x: params.p1().width(), y: params.p1().height()};
        win_dim = dim.clone();
    }
    let (mut node, mut virtual_mouse) = params.p0().into_inner();
    virtual_mouse.pos += accumulated_mouse_motion.delta;
        
    let center = Vec2::new(win_dim.x / 2.0, win_dim.y / 2.0);
    let ui_pos = center + virtual_mouse.pos;
    node.left = Val::Px(ui_pos.x);
    node.top = Val::Px(ui_pos.y);
}


pub fn rotate_spaceship(
    mut params: ParamSet<(
        Single<&mut Transform, With<crate::controller::Player>>,
        Single<&VirtualMouse>
    )>,
    time: Res<Time>,
) {
    let mouse_pos: Vec2;
    { mouse_pos = params.p1().pos; }
    let mut transform = params.p0().into_inner();

    let mouse_offset = mouse_pos;
    
    if mouse_offset.length_squared() > 0.03 {
        let speed: f32 = 0.0005;

        let target_angle_y = -mouse_offset.x * speed;
        transform.rotate_local_y(target_angle_y * time.delta_secs());

        let target_angle_x = -mouse_offset.y * speed;
        transform.rotate_local_x(target_angle_x * time.delta_secs());
    }

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
