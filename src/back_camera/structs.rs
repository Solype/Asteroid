use bevy::prelude::*;

#[derive(Resource)]
pub struct BackCameraRenderTargetImage {
    pub material: Handle<StandardMaterial>, // pub image: Handle<Image>
}

#[derive(Component)]
pub struct BackCameraComponent;
