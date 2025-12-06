use bevy::prelude::*;

use crate::{config::structs::GameConfig, game_states::GameState};
pub mod ammo;

#[derive(Component)]
pub struct PlayerHitBox {
    pub radius: f32,
}

#[derive(Component)]
pub struct Ammo;

#[derive(Resource)]
pub struct ShootSide {
    left: bool,
}

#[derive(Resource)]
pub struct AmmoAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
pub struct ShootSounds {
    pub shoot_pews: Vec<Handle<AudioSource>>,
}

pub const PLAYER_MASS: f32 = 216.0; //6³

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (ammo::shoot_ammo, ammo::clear_ammos).run_if(in_state(GameState::Game)),
        );
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    gameconfig: Res<GameConfig>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::hsl(280.0, 0.8, 0.6), // vivid purple
        emissive: LinearRgba::rgb(10.0, 0.0, 15.0),
        ..default()
    });
    let mesh = meshes.add(Sphere::new(0.05).mesh());

    commands.insert_resource(AmmoAssets {
        mesh: mesh,
        material: material,
    });

    commands.insert_resource(ShootSide { left: true });
    let mut resource = ShootSounds::default();
    for path in gameconfig.ship.ammo.sounds.iter() {
        resource.shoot_pews.push(asset_server.load(path))
    }

    commands.insert_resource(resource);
}
