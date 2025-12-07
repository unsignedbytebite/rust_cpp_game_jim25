use super::*;
use bevy::prelude::*;
use lightyear::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::native::InputPlugin::<components::Inputs>::default());
        app.register_component::<components::PlayerId>();
        app.register_component::<components::PlayerPosition>();
    }
}
