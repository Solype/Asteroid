use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MetricPlane;

#[derive(Component)]
pub struct MetricCamComponent;

#[derive(Resource)]
pub struct MetricCameraTarget {
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct MetricText;

#[derive(Component)]
pub struct SpeedText;

#[derive(Resource)]
pub struct DistanceTimer(pub Timer);
