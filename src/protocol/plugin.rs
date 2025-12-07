use super::*;
use bevy::prelude::*;
use lightyear::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.register_message::<messages::Message1>()
            .add_direction(NetworkDirection::ServerToClient);

        app.add_plugins(input::native::InputPlugin::<components::Inputs>::default());

        app.register_component::<components::PlayerId>();

        app.register_component::<components::PlayerPosition>()
            .add_prediction()
            .add_linear_interpolation();

        app.add_channel::<messages::Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        })
        .add_direction(NetworkDirection::ServerToClient);
    }
}
