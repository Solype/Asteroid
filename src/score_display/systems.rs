use bevy::{prelude::*};
use crate::{globals_structs::Score, score_display::structs::*};

pub fn toggle_screenshot_camera(
    time: Res<Time>,
    score: Res<Score>,
    mut query: Query<(&mut Camera, &mut ScoreCamTimer)>,
    mut query_score: Query<(&mut Text, &ScoreText)>
) {
    for (mut camera, mut sc) in &mut query {
        sc.timer.tick(time.delta());

        if sc.timer.just_finished() {
            camera.is_active = true;
            for (mut text, _) in &mut query_score {
                *text = Text::new(format!("{} $", (*score).value));
            }
        } else {
            camera.is_active = false;
        }
    }
}
