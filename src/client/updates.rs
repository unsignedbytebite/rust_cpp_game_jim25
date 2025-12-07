use super::*;
use crate::protocol::components::*;
use crate::protocol::messages::*;
use bevy::prelude::*;
use lightyear::prelude::client::input::*;
use lightyear::prelude::input::native::*;
use lightyear::prelude::{MessageReceiver, Predicted};

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

pub(crate) fn set_transform(players: Query<(&PlayerPosition, &mut Transform)>) {
    for position in &players {
        info!(">>>");
    }
}

// System that reads from peripherals and adds inputs to the buffer
/// This system must be run in the `InputSystemSet::BufferInputs` set in the `FixedPreUpdate` schedule
/// to work correctly.
///
/// I would also advise to use the `leafwing` feature to use the `LeafwingInputPlugin` instead of the
/// `InputPlugin`, which contains more features.
pub fn buffer_input(
    mut query: Query<&mut ActionState<Inputs>, With<InputMarker<Inputs>>>,
    keypress: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut action_state) = query.single_mut() {
        let mut direction = Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        };
        if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
            direction.up = true;
        }
        if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
            direction.down = true;
        }
        if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
            direction.left = true;
        }
        if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
            direction.right = true;
        }
        // we always set the value. Setting it to None means that the input was missing, it's not the same
        // as saying that the input was 'no keys pressed'
        action_state.0 = Inputs::Direction(direction);
    }
}

/// The client input only gets applied to predicted entities that we own
/// This works because we only predict the user's controlled entity.
/// If we were predicting more entities, we would have to only apply movement to the player owned one.
pub fn player_movement(
    // timeline: Single<&LocalTimeline>,
    mut position_query: Query<(&mut PlayerPosition, &ActionState<Inputs>), With<Predicted>>,
) {
    // let tick = timeline.tick();
    for (position, input) in position_query.iter_mut() {
        // trace!(?tick, ?position, ?input, "client");
        // NOTE: be careful to directly pass Mut<PlayerPosition>
        // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
        shared_movement_behaviour(position, input);
    }
}

/// System to receive messages on the client
pub fn receive_message1(mut receiver: Single<&mut MessageReceiver<Message1>>) {
    for message in receiver.receive() {
        info!("Received message: {:?}", message);
    }
}
