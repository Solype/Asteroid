use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuSystemSet;

#[derive(Resource)]
pub struct SpawnMenuPlane;

#[derive(Component)]
pub struct MenuPlane;

#[derive(Resource)]
pub struct MenuCameraTarget {
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct MenuCameraComponent;

#[derive(Component)]
pub struct CameraSquareElement;

