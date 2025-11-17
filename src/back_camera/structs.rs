use bevy::prelude::*;

#[derive(Resource)]
pub struct BackCameraRenderTargetImage{
    pub image: Handle<Image>
}

#[derive(Component)]
pub struct BackCameraComponent;