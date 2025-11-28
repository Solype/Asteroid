use bevy::prelude::*;

// const SPRITE_COLUMNS: usize = 4;
// const SPRITE_ROWS: usize = 4;
// const TOTAL_FRAMES: usize = SPRITE_COLUMNS * SPRITE_ROWS;
// const FRAME_TIME: f32 = 0.1;

// #[derive(Component)]
// struct SpriteAnimation {
//     timer: Timer,
//     frame: usize,
// }

// #[derive(Component)]
// struct AnimatedSprite;
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationDuration {
    pub frame_left: u16 // 0 for infinite
}

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut AnimationDuration,
        &mut Sprite,
    )>,
) {
    for (entity, mut timer, mut duration, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let atlas = sprite.texture_atlas.as_mut().unwrap();
            if let Some(layouts) = atlases.get(&atlas.layout) {
                atlas.index = (atlas.index + 1) % layouts.textures.len();
            }
            if duration.frame_left > 1 {
                duration.frame_left -= 1
            } else if duration.frame_left == 1 {
                commands.entity(entity).despawn();
            }
        }
    }
}
