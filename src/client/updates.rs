use super::*;
use bevy::prelude::*;

pub fn move_elf(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut local_player_trans: Single<&mut Transform, With<components::LocalPlayer>>,
) {
    let delta = time.delta_secs();
    let player_vel = 16.0 * delta;

    if keys.pressed(KeyCode::KeyW) {
        local_player_trans.translation.y += player_vel;
    } else if keys.pressed(KeyCode::KeyS) {
        local_player_trans.translation.y -= player_vel;
    } else if keys.pressed(KeyCode::KeyA) {
        local_player_trans.translation.x -= player_vel;
    } else if keys.pressed(KeyCode::KeyD) {
        local_player_trans.translation.x += player_vel;
    }
}
