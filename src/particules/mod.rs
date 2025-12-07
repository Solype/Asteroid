use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{game_states::GameState, physics::Velocity};

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_particles);
        app.add_systems(
            Update,
            enable_disable_rockets_particules.run_if(in_state(GameState::Game)),
        );
    }
}

fn lerp(a: Vec4, b: Vec4, t: f32) -> Vec4 {
    a + (b - a) * t
}

fn create_rocket_effect(v3color_start: Vec3, v3color_end: Vec3) -> EffectAsset {
    let color_start = Vec4::new(v3color_start.x, v3color_start.y, v3color_start.z, 1.0);
    let color_end = Vec4::new(v3color_end.x, v3color_end.y, v3color_end.z, 1.0);

    let writer = ExprWriter::new();

    let direction_handle = writer.add_property("direction", Vec3::Y.into());
    let speed_handle = writer.add_property("speed", 100.0.into());

    let direction = writer.prop(direction_handle);
    let speed = writer.prop(speed_handle);

    let velocity = (direction * speed).expr();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, velocity);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::X).expr(),
        radius: writer.lit(0.5).expr(),
        dimension: ShapeDimension::Volume,
    };

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let rgb = writer.rand(VectorType::VEC3F) * writer.lit(0.9) + writer.lit(0.1);
    let color = rgb.vec4_xyz_w(writer.lit(1.)).pack4x8unorm();
    let init_trails_color = SetAttributeModifier::new(Attribute::U32_0, color.expr());

    let drag = writer.lit(4.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let spawner = SpawnerSettings::rate((100., 300.).into());

    let mut size_gradient = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.7)); // gros au début
    size_gradient.add_key(0.2, Vec3::splat(0.15)); // shrink rapide au début
    size_gradient.add_key(1.0, Vec3::splat(0.05));

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, color_start);
    color_gradient.add_key(0.33, lerp(color_start, color_end, 0.33));
    color_gradient.add_key(0.66, lerp(color_start, color_end, 0.66));
    color_gradient.add_key(1.0, color_end);

    EffectAsset::new(3000, spawner, writer.finish())
        .with_name("rocket")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(init_trails_color)
        .with_simulation_space(SimulationSpace::Global)
        .update(update_drag)
        .render(OrientModifier {
            mode: OrientMode::FaceCameraPosition,
            ..Default::default()
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
            blend: ColorBlendMode::Overwrite,
            mask: ColorBlendMask::RGBA,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        })
}

pub fn enable_disable_rockets_particules(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keybinds: Res<crate::globals_structs::Keybinds>,
    mut query: ParamSet<(
        Query<(&mut EffectSpawner, &mut EffectProperties, &GlobalTransform), With<ParticleEffect>>,
        Single<&Velocity, With<crate::controller::structs::Player>>,
    )>,
) {
    let enabled = keybinds.forward.pressed(&keyboard, &mouse);
    let vel: Vec3;
    {
        vel = query.p1().0;
    }
    let speed: f32 = vel.length();

    if enabled {
        for (mut spawner, mut effect, transform) in &mut query.p0() {
            spawner.active = enabled;
            let forward: Vec3 = transform.forward().into();
            let dot = forward.dot(vel);
            let new_speed = if dot > 0.0_f32 {
                Vec3::ZERO
            } else {
                (-forward).normalize() * speed * 0.7
            };
            (*effect).set("direction", Value::Vector(VectorValue::new_vec3(new_speed)));
        }
    } else {
        for (mut spawner, _, _) in &mut query.p0() {
            spawner.active = enabled;
        }
    }
}

pub fn spawn_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    gameconfig: Res<crate::config::structs::GameConfig>,
    ship: Single<Entity, With<crate::controller::structs::Player>>,
) {
    let position1 = gameconfig.ship.thruster_left;
    let position2 = gameconfig.ship.thruster_right;

    let mut props: EffectProperties = EffectProperties::default();
    props.set("direction", Value::Vector(VectorValue::new_vec3(Vec3::Y)));
    props.set("speed", Value::Scalar(ScalarValue::Float(10.0)));

    let effect = effects.add(create_rocket_effect(
        gameconfig.ship.color_particules.0,
        gameconfig.ship.color_particules.1,
    ));
    let particules1 = commands
        .spawn((
            Name::new("rocket1"),
            Transform::from_translation(position1),
            ParticleEffect::new(effect.clone()),
            props.clone(),
        ))
        .id();

    let particules2 = commands
        .spawn((
            Name::new("rocket2"),
            Transform::from_translation(position2),
            ParticleEffect::new(effect.clone()),
            props.clone(),
        ))
        .id();

    let e = ship.into_inner();
    commands.entity(e).add_child(particules1);
    commands.entity(e).add_child(particules2);
}
