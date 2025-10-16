use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;

////////////////////////////////////////////////////
///
/// Initialisation
/// 
////////////////////////////////////////////////////


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuSystemSet;



////////////////////////////////////////////////////
/// 
/// 3D components
/// 
////////////////////////////////////////////////////

#[derive(Component, Default)]
pub struct MenuPlane {
    pub dimensions: Vec2,
    pub center: Vec3,
    pub normal: Vec3,
    pub menu_id: MenuTypes
}


#[derive(Component, Default)]
pub struct SmoothCamMove {
    pub look_at : Option<Vec3>,
    pub position : Option<Vec3>,
    pub speed : Option<f32>,
    pub up : Option<Vec3>,
    pub fov : Option<f32>,
    pub aspect_ratio : Option<f32>
}


#[derive(Resource)]
pub struct MenuCameraTarget {
    pub image: Handle<Image>,
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


////////////////////////////////////////////////////
/// 
/// Events
/// 
////////////////////////////////////////////////////

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorEventType {
    #[default]
    Move = 0,
    Click = 1
}

#[derive(Message, Default, Debug)]
pub struct MenuPlaneCursorCastEvent {
    pub menu_id: MenuTypes,
    pub event_type: CursorEventType,
    pub cursor_coordinates: Vec2,
    pub screen_dimensions: Vec2,
}


////////////////////////////////////////////////////
/// 
/// 2D menu elements
/// 
////////////////////////////////////////////////////

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
    Options,
}

#[derive(Resource)]
pub struct MainMenuRessources {
    pub bg : Handle<Image>,
    pub font : Handle<Font>,
}
