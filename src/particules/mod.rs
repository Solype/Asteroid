use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_particles);
        // app.add_systems(Update, update_rocket_velocity_system);
    }
}

fn create_rocket_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    // Position initiale dans un cercle (zone de sortie du réacteur)
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(), // on placera le transform plus tard
        axis: writer.lit(Vec3::X).expr(),
        radius: writer.lit(0.5).expr(),
        dimension: ShapeDimension::Volume,
    };

    let vel = writer.rand(VectorType::VEC3F);
    let vel = vel * writer.lit(2.) - writer.lit(1.); // remap [0:1] to [-1:1]
    let vel = vel.normalized();
    let speed = writer.lit(1.); //.uniform(writer.lit(4.));
    let vel = (vel * speed).expr();
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, vel);


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
    size_gradient.add_key(0.0, Vec3::splat(0.7));   // gros au début
    size_gradient.add_key(0.2, Vec3::splat(0.15));  // shrink rapide au début
    size_gradient.add_key(1.0, Vec3::splat(0.05));

    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(0.2, 0.4, 1.0, 1.0)); // bleu
    color_gradient.add_key(0.4, Vec4::new(0.6, 0.8, 1.0, 1.0)); // bleu clair
    color_gradient.add_key(0.8, Vec4::new(1.0, 1.0, 1.0, 1.0)); // blanc chaud
    color_gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0)); // blanc fade

    EffectAsset::new(30000, spawner, writer.finish())
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

/// Spawn le réacteur avec les particules attachées
pub fn spawn_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    ship: Single<Entity, With<crate::controller::Player>>
) {
    let position1 = Vec3::new(-0.7, 1.0, 5.0);
    let position2 = Vec3::new(0.7, 1.0, 5.0);

    let effect = effects.add(create_rocket_effect());
    let particules1 = commands.spawn((
        Name::new("rocket1"),
        Transform::from_translation(position1),
        ParticleEffect::new(effect.clone())
    )).id();

    let particules2 = commands.spawn((
        Name::new("rocket2"),
        Transform::from_translation(position2),
        ParticleEffect::new(effect.clone())
    )).id();

    let e = ship.into_inner();
    commands.entity(e).add_child(particules1);
    commands.entity(e).add_child(particules2);
}
