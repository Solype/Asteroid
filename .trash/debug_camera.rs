use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct DebugCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

/// Spawn a debug (fly) camera. Use this in your startup setup.
pub fn spawn_debug_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        // The `Camera` component is also needed for rendering.
        Camera::default(),
        Transform::from_xyz(0.0, 2.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        DebugCamera {
            speed: 5.0,
            sensitivity: 0.7,
        },
    ));
}

/// Control system for the debug camera. Should run each frame.
pub fn control_debug_camera(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&DebugCamera, &mut Transform)>,
) {
    for (dbg, mut transform) in &mut query {
        // --- Movement ---
        let mut dir = Vec3::ZERO;

        // To compute forward and right, derive them from transform.rotation
        // Bevy uses a right-handed coordinate system:
        // - The camera’s “forward” is its local -Z direction
        let forward = transform.rotation * Vec3::new(0.0, 0.0, -1.0);
        let right   = transform.rotation * Vec3::new(1.0, 0.0, 0.0);
        let up      = Vec3::Y;

        if keys.pressed(KeyCode::KeyW) {
            dir += forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            dir -= forward;
        }
        if keys.pressed(KeyCode::KeyD) {
            dir += right;
        }
        if keys.pressed(KeyCode::KeyA) {
            dir -= right;
        }
        if keys.pressed(KeyCode::Space) {
            dir += up;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            dir -= up;
        }

        if dir != Vec3::ZERO {
            let movement = dir.normalize() * dbg.speed * time.delta_secs();
            transform.translation += movement;
        }

        // --- Mouse look / rotation ---
        let mut delta = Vec2::ZERO;
        for ev in mouse_motion.read() {
            delta += ev.delta;
        }
        if delta == Vec2::ZERO {
            continue;
        }
        // Only rotate when Right mouse is pressed
        if !mouse_buttons.pressed(MouseButton::Right) {
            continue;
        }

        let dx = -delta.x * dbg.sensitivity * time.delta_secs();
        let dy = -delta.y * dbg.sensitivity * time.delta_secs();

        // Yaw around world Y axis
        let yaw = Quat::from_rotation_y(dx);
        // Pitch around camera’s local right axis
        let pitch = Quat::from_axis_angle(right, dy);

        transform.rotation = yaw * transform.rotation;
        transform.rotation = transform.rotation * pitch;
    }
}
