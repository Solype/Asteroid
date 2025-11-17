use bevy::prelude::*;


#[derive(Component, Default)]
pub struct ScorePlane;

#[derive(Component)]
pub struct ScoreCamComponent;

#[derive(Resource)]
pub struct ScoreCameraTarget {
    pub image: Handle<Image>,
}
