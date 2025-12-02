use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Component, Deref, DerefMut, States};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCam;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec3);

impl CameraSensitivity {
    pub(crate) fn default() -> Self {
        Self(Vec3::new(0.003, 0.002, 0.002))
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct RotationalVelocity(Vec3);

#[derive(Component, Default)]
pub struct VirtualMouse {
    pub pos: Vec2
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ControllerState {
    FreeLook,
    Driving,
    #[default]
    None,
}
