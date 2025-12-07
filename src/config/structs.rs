use bevy::{prelude::{Resource, Vec2, Vec3}};

#[derive(Debug, Clone, Default, Resource)]
pub struct GameConfig {
    pub window: Vec2,
    pub window_title: String,
    pub window_name: String,
    pub main_cam: MainCamConfig,
    pub ui: UIConfig,
    pub ship: ShipConfig,
    pub asteroids: AsteroidConfig,
}

#[derive(Debug, Clone, Default)]
pub struct AsteroidConfig {
    pub max_asteroid: usize,
    pub size_range: (f32, f32),
    pub spawn_range: f32,
    pub despawn_range: f32,
    pub speed: f32,
    pub rotationnal_speed: f32
}


#[derive(Debug, Clone, Default)]
pub struct MainCamConfig {
    pub speed_transition: f32,
    pub maxfov: f32,
    pub driving: CamConfig,
    pub menu: CamConfig
}

#[derive(Debug, Clone, Default)]
pub struct CamConfig {
    pub position: Vec3,
    pub look_at: Vec3,
    pub fov: f32
}

#[derive(Debug, Clone, Default)]
pub struct UIConfig {
    pub background: String,
    pub font: String,
    pub dimension: Vec2,
    pub sounds: Vec<String>,
    pub music: String,
}

#[derive(Debug, Clone, Default)]
pub struct AmmoConfig {
    pub speed: f32,
    pub distance_despawn: f32,
    pub color: Vec3,
    pub sounds: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ShipConfig {
    pub music: String,
    pub alarm: String,
    pub speed: f32,
    pub rotation_speed: f32,
    pub virtual_mouse_sensitivity: f32,
    pub thurst_modifier: f32,
    pub asset: String,
    pub backcamera_position: Vec3,
    pub backcamera_look_at: Vec3,
    pub thruster_right: Vec3,
    pub thruster_left: Vec3,
    pub gun_right: Vec3,
    pub gun_left: Vec3,
    pub ammo: AmmoConfig,
    pub screen_right: ScreenQuad,
    pub screen_center: ScreenQuad,
    pub screen_left: ScreenQuad,
    pub color_particules: (Vec3, Vec3)
}

#[derive(Debug, Clone, Default)]
pub struct ScreenQuad {
    pub tr: Vec3,
    pub tl: Vec3,
    pub br: Vec3,
    pub bl: Vec3,
}
