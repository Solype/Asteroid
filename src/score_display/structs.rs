use bevy::prelude::*;


#[derive(Component, Default)]
pub struct ScorePlane;

#[derive(Component)]
pub struct ScoreCamComponent;

#[derive(Resource)]
pub struct ScoreCameraTarget {
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct ScoreCamTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct ScoreText;
