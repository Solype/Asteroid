use bevy::prelude::*;


#[derive(Resource)]
pub struct MusicVolume
{
    pub volume: f32
}


#[derive(Clone, Copy)]
pub enum InputButton {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl InputButton {
    pub fn to_str(&self) -> String {
        match self {
            InputButton::Key(k) => format!("{:?}", k),
            InputButton::Mouse(b) => format!("Mouse {:?}", b),
        }
    }
}

#[derive(Resource)]
pub struct Keybinds
{
    // movement
    pub up: InputButton,
    pub down: InputButton,
    pub right: InputButton,
    pub left: InputButton,
    pub forward: InputButton,
    pub backward: InputButton,
    // rotation
    pub rotate_left: InputButton,
    pub rotate_right: InputButton,
    // other
    pub menu: InputButton,
    pub free_look: InputButton,
    pub shoot: InputButton
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            left: InputButton::Key(KeyCode::KeyQ),
            right: InputButton::Key(KeyCode::KeyD),
            forward: InputButton::Key(KeyCode::KeyZ),
            backward: InputButton::Key(KeyCode::KeyS),

            up: InputButton::Key(KeyCode::Space),
            down: InputButton::Key(KeyCode::ShiftLeft),


            rotate_left: InputButton::Key(KeyCode::KeyA),
            rotate_right: InputButton::Key(KeyCode::KeyE),

            // Autres actions
            menu: InputButton::Key(KeyCode::Escape),
            free_look: InputButton::Mouse(MouseButton::Right),
            shoot: InputButton::Mouse(MouseButton::Left)
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
    RotateLeft,
    RotateRight,
    FreeLook,
    Shoot,
    Menu,
}
