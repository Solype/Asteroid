use bevy::prelude::*;


// Initialisation

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuSystemSet;

#[derive(Resource)]
pub struct SpawnMenuPlane;


// 3D components

#[derive(Component, Default)]
pub struct MenuPlane {
    pub width: f32,
    pub height: f32,
}

#[derive(Event, Default)]
pub struct MenuPlaneCursorCastEvent {
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub width: f32,
    pub height: f32,
}



// 2D menu elements

#[derive(Resource)]
pub struct MenuCameraTarget {
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct MenuCameraComponent;

#[derive(Component)]
pub struct MenuButton {
    pub action: MenuAction,
}

#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    Start,
    Quit,
}
