use crate::globals_structs::Action;
use bevy::prelude::*;

////////////////////////////////////////////////////
///
/// Initialisation
///
////////////////////////////////////////////////////

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Options,
    GameOver,
}

////////////////////////////////////////////////////
///
/// 3D components
///
////////////////////////////////////////////////////

#[derive(Component, Default)]
pub struct MenuPlane;

#[derive(Component, Default)]
pub struct SmoothCamMove {
    pub look_at: Option<Vec3>,
    pub position: Option<Vec3>,
    pub speed: Option<f32>,
    pub up: Option<Vec3>,
    pub fov: Option<f32>,
    pub aspect_ratio: Option<f32>,
}

#[derive(Resource)]
pub struct MenuCameraTarget {
    // pub image: Handle<Image>,
    pub material: Handle<StandardMaterial>,
}

////////////////////////////////////////////////////
///
/// 2D menu elements
///
////////////////////////////////////////////////////

#[derive(Component)]
pub struct MenuCameraComponent;

#[derive(Component)]
pub struct VolumeText;

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub struct Scroll {
    pub entity: Entity,
    pub delta: Vec2,
}

#[derive(Resource, Default)]
pub struct WaitingForRebind(pub Option<Action>);

#[derive(Component)]
pub struct ButtonInfo {
    pub border_normal: BorderColor,
    pub border_hover: BorderColor,
}

////////////////////////////////////////////////////
///
/// AUDIO
///
////////////////////////////////////////////////////

#[derive(Resource, Default)]
pub struct MenuSounds {
    pub button_bips: Vec<Handle<AudioSource>>,
}
