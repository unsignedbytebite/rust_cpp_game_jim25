use crate::client;
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    log::{Level, LogPlugin},
    prelude::*,
};
use lightyear::{
    netcode::NetcodeClient,
    prelude::{
        client::{NetcodeConfig, WebTransportClientIo},
        *,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    time::Duration,
};
use {bevy::window::PresentMode, bevy::winit::WinitSettings};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ClientTransports {
    WebTransport,
}

#[derive(Copy, Clone, Debug)]
pub struct SharedSettings {
    /// An id to identify the protocol version
    pub protocol_id: u64,

    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],
}

#[derive(Component, Clone, Debug)]
#[component(on_add = ExampleClient::on_add)]
pub struct ExampleClient {
    pub client_id: u64,
    /// The client port to listen on
    pub client_port: u16,
    /// The socket address of the server
    pub server_addr: SocketAddr,
    /// Which transport to use
    pub transport: ClientTransports,
    pub shared: SharedSettings,
}

impl ExampleClient {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<ExampleClient>().unwrap();
            let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), settings.client_port);
            entity_mut.insert((
                Client::default(),
                Link::new(None),
                LocalAddr(client_addr),
                PeerAddr(settings.server_addr),
                ReplicationReceiver::default(),
                PredictionManager::default(),
                Name::from("Client"),
            ));

            let add_netcode = |entity_mut: &mut EntityWorldMut| -> Result {
                // use dummy zeroed key explicitly here.
                let auth = Authentication::Manual {
                    server_addr: settings.server_addr,
                    client_id: settings.client_id,
                    private_key: settings.shared.private_key,
                    protocol_id: settings.shared.protocol_id,
                };
                let netcode_config = NetcodeConfig {
                    // Make sure that the server times out clients when their connection is closed
                    client_timeout_secs: 3,
                    token_expire_secs: -1,
                    ..default()
                };
                entity_mut.insert(NetcodeClient::new(auth, netcode_config)?);
                Ok(())
            };

            match settings.transport {
                ClientTransports::WebTransport => {
                    add_netcode(&mut entity_mut)?;
                    let certificate_digest =
                        { include_str!("../certificates/digest.txt").to_string() };
                    entity_mut.insert(WebTransportClientIo { certificate_digest });
                }
            };
            Ok(())
        });
    }
}

pub(crate) fn connect(mut commands: Commands, client: Single<Entity, With<Client>>) {
    commands.trigger(Connect {
        entity: client.into_inner(),
    });
}

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

    app.add_plugins(lightyear::prelude::client::ClientPlugins {
        tick_duration: Duration::from_secs_f64(1.0 / 60.0),
    });
    app.add_plugins(client::plugin::ClientPlugin);

    // we want the same frequency of updates for both focused and unfocused
    // Otherwise when testing the movement can look choppy for unfocused windows
    app.insert_resource(WinitSettings::continuous());

    let port = 5888;

    app.world_mut().spawn(ExampleClient {
        client_id: 0,
        client_port: port,
        server_addr: SocketAddr::new(IpAddr::from_str("18.133.225.101").unwrap(), port),
        transport: ClientTransports::WebTransport,
        shared: SharedSettings {
            protocol_id: 0,
            private_key: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        },
    });

    app.add_systems(Startup, connect);

    app.run();
}
