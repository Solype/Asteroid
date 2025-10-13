use bevy::prelude::*;


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

pub fn cycle_rocks(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut assets: ResMut<RockAssets>,
    query: Query<Entity, With<Rock>>,
) {
    if keyboard.just_pressed(KeyCode::KeyQ) {
        for entity in &query {
            commands.entity(entity).despawn();
        }

        assets.current = (assets.current + 1) % assets.scenes.len();
        let scene = assets.scenes[assets.current].clone();

        commands.spawn((
            SceneRoot(scene),
            Transform::from_xyz(0.0, 2.0, 0.0),
            GlobalTransform::default(),
            Rock,
        ));
    }
}
