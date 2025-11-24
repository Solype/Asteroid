use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_particles);
    }
}


fn create_rocket_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::new(0.0f32, 0.0f32, -5.0f32)).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        radius: writer.lit(1.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let zero = writer.lit(0.);
    let y = writer.lit(35.).uniform(writer.lit(40.));
    let v = zero.clone().vec3(y, zero);
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, v.expr());

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);


    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).uniform(writer.lit(1.2)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Store a random color per particle, which will be inherited by the spark ones
    // on explosion. We don't store it in Attribute::COLOR otherwise it's going to
    // affect the color of the rocket particle itself.
    let rgb = writer.rand(VectorType::VEC3F) * writer.lit(0.9) + writer.lit(0.1);
    let color = rgb.vec4_xyz_w(writer.lit(1.)).pack4x8unorm();
    let init_trails_color = SetAttributeModifier::new(Attribute::U32_0, color.expr());

    // Add drag to make particles slow down as they ascend
    let drag = writer.lit(4.).expr();
    let update_drag = LinearDragModifier::new(drag);


    let spawner = SpawnerSettings::rate((100., 300.).into());

    let mut size_gradient = bevy_hanabi::Gradient::new();
    size_gradient.add_key(0.0, Vec3::splat(0.7)); // au début, grande taille
    size_gradient.add_key(1.0, Vec3::splat(0.05)); // à la fin, petite taille


    let mut color_gradient = bevy_hanabi::Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(0.8, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(4.0, 4.0, 4.0, 0.0));

    EffectAsset::new(30000, spawner, writer.finish())
        .with_name("rocket")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(init_trails_color)
        .init(init_trails_color)
        .update(update_drag)
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


pub fn spawn_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut position: Vec3 = Vec3::new(0.0f32, 0.0f32, -5.0f32);

    let effect = effects.add(create_rocket_effect());
    commands.spawn((
            Name::new("rocket"),
            Transform::from_translation(position),
            ParticleEffect::new(effect))
        );


    let debug_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    let mesh = meshes.add(Sphere::default().mesh().ico(5).unwrap());
    position.z -= 5.0f32;
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_translation(position),
    ));
}
