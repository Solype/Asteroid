use bevy::{
    asset::LoadState,
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::TextureViewDimension,
    },
    image::{ImageSampler, ImageSamplerDescriptor},
};


#[derive(Resource)]
pub struct SkyCubeMap {
    pub image: Handle<Image>,
    pub loaded: bool,
}


#[derive(Resource)]
pub struct CameraHolder(pub Entity);


pub fn plugin(app: &mut App)
{
    app
        .add_systems(Startup, setup)
        .add_systems(Update, reinterpret_cubemap);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    let sky_image = asset_server.load("skybox.png");

    commands.insert_resource(SkyCubeMap {
        image: sky_image.clone(),
        loaded: false,
    });
}


fn reinterpret_cubemap(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<SkyCubeMap>,
    mut commands: Commands,
    camera_holder: Res<CameraHolder>,
) {
    if !cubemap.loaded
        && matches!(
            asset_server.get_load_state(&cubemap.image),
            Some(LoadState::Loaded)
        )
    {
        cubemap.loaded = true;

        let image = images.get_mut(&cubemap.image).unwrap();

        let layers = image.height() / image.width();
        if layers == 6 {
            image.reinterpret_stacked_2d_as_array(layers);
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
            image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..Default::default()
            });

            commands.entity(camera_holder.0).insert(Skybox {
                image: cubemap.image.clone(),
                brightness: 1000.0,
                rotation: Quat::IDENTITY,
            });
        } else {
            warn!(
                "Skybox image must be 6xN pixels. Got {} layers.",
                layers
            );
        }
    }
}



