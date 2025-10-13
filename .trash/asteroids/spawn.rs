use bevy::prelude::*;
use rand::Rng;


#[derive(Component)]
pub struct Rock;

#[derive(Resource)]
pub struct RockAssets {
    scenes: Vec<Handle<Scene>>,
    current: usize,
}


pub fn setup_rocks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut scenes = Vec::new();
    for i in 0..10 {
        let path = format!("Rock{}.glb#Scene0", i);
        scenes.push(asset_server.load(path));
    }

    let first_scene = scenes[0].clone();
    commands.insert_resource(RockAssets { scenes, current: 0 });

    commands.spawn((
        SceneRoot(first_scene),
        Transform::from_xyz(0.0, 2.0, 0.0),
        GlobalTransform::default(),
        Rock,
    ));
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec3);

pub fn rock_wave(
    mut commands: Commands,
    query: Query<Entity, With<Rock>>,
    mut asset_server: ResMut<RockAssets>,
) {
    let current = query.iter().count();
    let target = 10;

    if current >= target {
        return;
    }

    let to_spawn = target - current;
    let radius = 300.0;

    let mut rng = rand::rng();

    for _ in 0..to_spawn {
        // Random direction on unit sphere
        let theta = rng.random_range(0.0..std::f32::consts::TAU);
        let phi = rng.random_range(0.0..std::f32::consts::PI);

        let x = radius * phi.sin() * theta.cos();
        let y = radius * phi.cos();
        let z = radius * phi.sin() * theta.sin();

        let position = Vec3::new(x, y, z);

        // Random inward velocity (not always exactly toward center)
        let random_dir = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize();

        let velocity = -(position.normalize() + random_dir * 0.3).normalize() * 50.0;

        commands.spawn((
            SceneRoot(asset_server.scenes[asset_server.current].clone()),
            Transform::from_translation(position),
            GlobalTransform::default(),
            Rock,
            Velocity(velocity),
        ));
    }
}

// pub fn cycle_rocks(
//     mut commands: Commands,
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut assets: ResMut<RockAssets>,
//     query: Query<Entity, With<Rock>>,
// ) {
//     // if keyboard.just_pressed(KeyCode::KeyQ) {
//     //     for entity in &query {
//     //         commands.entity(entity).despawn();
//     //     }

//     //     assets.current = (assets.current + 1) % assets.scenes.len();
//     //     let scene = assets.scenes[assets.current].clone();

//     //     commands.spawn((
//     //         SceneRoot(scene),
//     //         Transform::from_xyz(0.0, 2.0, 0.0),
//     //         GlobalTransform::default(),
//     //         Rock,
//     //     ));
//     // }
// }
