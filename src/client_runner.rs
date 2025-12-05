use crate::client;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use {bevy::window::PresentMode, bevy::winit::WinitSettings};

pub fn init() {
    println!("init the client");

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .build()
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                // https://github.com/bevyengine/bevy/issues/10157
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(LogPlugin {
                level: Level::INFO,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,bevy_time=warn,naga=warn,bevy_enhanced_input::action::fns=error".to_string(),
                ..default()
            })
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("{}", env!("CARGO_PKG_NAME")),
                        resolution: (1920/2, 1080/2).into(),
                        present_mode: PresentMode::AutoVsync,
                        // set to true if we want to capture tab etc in wasm
                        prevent_default_event_handling: true,
                        ..Default::default()
                    }),
                    ..default()
                })
    );
    app.add_plugins(client::plugin::ClientPlugin);

    // we want the same frequency of updates for both focused and unfocused
    // Otherwise when testing the movement can look choppy for unfocused windows
    app.insert_resource(WinitSettings::continuous());

    app.run();
}
