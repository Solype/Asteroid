use bevy::prelude::*;
use bevy::render::view::RenderLayers;


// Initialisation

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuSystemSet;

// 3D components

#[derive(Component, Default)]
pub struct MenuPlane {
    pub width: f32,
    pub height: f32,
    pub menu_id: MenuTypes
}

#[derive(Event, Default)]
pub struct MenuPlaneCursorCastEvent {
    pub menu_id: MenuTypes,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuTypes {
    MainMenu = 1,
    // StatMenu = 2
}

impl Default for MenuTypes {
    fn default() -> Self {
        MenuTypes::MainMenu
    }
}
impl MenuTypes {
    pub fn layer(self) -> RenderLayers {
        RenderLayers::layer(self as usize)
    }
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
