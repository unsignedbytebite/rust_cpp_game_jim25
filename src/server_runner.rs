use bevy::window::PresentMode;
use bevy::{
    diagnostic::DiagnosticsPlugin,
    log::{Level, LogPlugin},
    prelude::*,
    state::app::StatesPlugin,
};
use lightyear::prelude::server::ServerPlugins;
use std::time::Duration;

pub fn init() {
    println!("init the server");

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,bevy_time=warn,naga=warn,bevy_enhanced_input::action::fns=error".to_string(),
            ..default()
        },
        StatesPlugin,
        ServerPlugins {
              tick_duration: Duration::from_secs_f64(1.0 / 64.0),
          },      DiagnosticsPlugin,
    ));

    app.run();
}
