use bevy::prelude::{Resource, Vec3, Vec2};

#[derive(Debug, Clone, Default, Resource)]
pub struct GameConfig {
    pub window: Vec2,
    pub window_title: String,
    pub window_name: String,
    pub main_cam: CamConfig,
    pub ui: UIConfig,
    pub ship: ShipConfig,
}

#[derive(Debug, Clone, Default)]
pub struct CamConfig {
    pub position: Vec3,
    pub look_at_forward: Vec3,
    pub look_at_menu: Vec3,
}

#[derive(Debug, Clone, Default)]
pub struct UIConfig {
    pub background: String,
    pub font: String,
    pub dimension: Vec2,
    pub mouseasset: String
}

#[derive(Debug, Clone, Default)]
pub struct ShipConfig {
    pub asset: String,
    pub backcamera_position: Vec3,
    pub backcamera_look_at: Vec3,
    pub thruster_right: Vec3,
    pub thruster_left: Vec3,
    pub gun_right: Vec3,
    pub gun_left: Vec3,
    pub screen_right: ScreenQuad,
    pub screen_center: ScreenQuad,
    pub screen_left: ScreenQuad,
}

#[derive(Debug, Clone, Default)]
pub struct ScreenQuad {
    pub tr: Vec3,
    pub tl: Vec3,
    pub br: Vec3,
    pub bl: Vec3,
}
