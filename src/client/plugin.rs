use super::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use lightyear::prelude::client::input::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AsepriteUltraPlugin);
        app.add_plugins(crate::protocol::plugin::ProtocolPlugin);

        app.add_systems(Startup, startups::setup_camera);

        // app.add_systems(Update, updates::move_elf);
        app.add_systems(Update, updates::sync_transform);

        app.add_systems(
            FixedPreUpdate,
            // Inputs have to be buffered in the WriteClientInputs set
            updates::buffer_input.in_set(InputSystems::WriteClientInputs),
        );
        app.add_systems(FixedUpdate, updates::player_movement);
        app.add_observer(observers::handle_predicted_spawn);
    }
}
