use bevy::prelude::*;

////////////////////////////////////////////////////
///
/// Initialisation
/// 
////////////////////////////////////////////////////

////////////////////////////////////////////////////
/// 
/// 3D components
/// 
////////////////////////////////////////////////////

#[derive(Component, Default)]
pub struct MenuPlane;


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


////////////////////////////////////////////////////
/// 
/// 2D menu elements
/// 
////////////////////////////////////////////////////

#[derive(Component)]
pub struct MenuCameraComponent;

#[derive(Resource)]
pub struct MainMenuRessources {
    pub bg : Handle<Image>,
    pub font : Handle<Font>,
}
