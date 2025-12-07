use crate::distancemetric::structs::*;
use crate::globals_structs::UIRessources;
use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

static SCREEN_WIDTH: u32 = 512 * 2;
static SCREEN_HEIGHT: u32 = 256 * 2;

pub fn setup_metric_screen(
    mut commands: Commands,
    menu_texture: Res<MetricCameraTarget>,
    menu_ressources: Res<UIRessources>,
) {
    let handle = menu_texture.image.clone();
    let font: Handle<Font> = menu_ressources.font.clone();

    commands.insert_resource(DistanceTimer(Timer::from_seconds(
        0.2,
        TimerMode::Repeating,
    )));

    let cam_entity = commands
        .spawn((
            Camera2d::default(),
            Camera {
                target: RenderTarget::Image(handle.clone().into()),
                ..default()
            },
            MetricCamComponent,
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            UiTargetCamera(cam_entity),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("--m"),
                TextFont {
                    font: font.clone(),
                    font_size: 150.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                MetricText,
                Node {
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("00v"),
                TextFont {
                    font: font.clone(),
                    font_size: 150.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 1.0)),
                SpeedText,
                Node {
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                },
            ));
        });
}

pub fn apply_texture_to_quad(
    mut commands: Commands,
    screens: Query<(&MetricPlane, Entity)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    menu_texture: Res<MetricCameraTarget>,
) {
    let mat_handler = materials.add(StandardMaterial {
        base_color_texture: Some(menu_texture.image.clone()),
        reflectance: 0.02,
        unlit: true,
        ..default()
    });

    for (_, entity) in screens.iter() {
        commands.entity(entity).insert(MeshMaterial3d(mat_handler));
        return;
    }
}

pub fn setup_texture_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("menu_camera_target"),
            size: Extent3d {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(Extent3d {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        depth_or_array_layers: 1,
    });

    commands.insert_resource(MetricCameraTarget {
        image: images.add(image),
    });
}
