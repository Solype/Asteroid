use bevy::prelude::*;
use crate::score_display::structs::*;

pub fn toggle_screenshot_camera(
    time: Res<Time>,
    mut query: Query<(&mut Camera, &mut ScoreCamTimer)>
) {
    for (mut camera, mut sc) in &mut query {
        sc.timer.tick(time.delta());

        if sc.timer.just_finished() {
            camera.is_active = true;
        } else {
            camera.is_active = false;
        }
    }
}
