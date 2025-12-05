use super::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AsepriteUltraPlugin);

        app.add_systems(Startup, startups::setup_camera);
        app.add_systems(Startup, startups::load);
    }
}
