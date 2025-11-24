use bevy::prelude::*;

use crate::game_states::GameState;
pub mod ammo;

#[derive(Component)]
pub struct Ammo;

#[derive(Resource)]
pub struct ShootSide {
    value: f32,
}

#[derive(Resource)]
pub struct AmmoAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (ammo::shoot_ammo, ammo::move_ammos, ammo::clear_ammos)
                .run_if(in_state(GameState::Game)),
        );
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    commands.insert_resource(ShootSide { value: 1.0 });
}
