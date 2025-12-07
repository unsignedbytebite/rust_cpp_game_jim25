use super::*;
use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(crate::protocol::plugin::ProtocolPlugin);

        app.add_observer(updates::handle_new_client);
        app.add_observer(updates::handle_connected);

        app.add_systems(FixedUpdate, updates::movement);
    }
}
