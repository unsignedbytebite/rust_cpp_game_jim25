use async_compat::Compat;
use bevy::{
    diagnostic::DiagnosticsPlugin,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    log::{Level, LogPlugin},
    prelude::*,
    state::app::StatesPlugin,
    tasks::IoTaskPool,
};
use lightyear::prelude::server::*;
use lightyear::prelude::{
    server::{ServerPlugins, Start},
    *,
};
use lightyear::{netcode::NetcodeServer, prelude::server::NetcodeConfig};

use serde::{Deserialize, Serialize};
use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WebTransportCertificateSettings {
    /// Load certificate pem files from disk
    FromFile {
        /// Path to cert .pem file
        cert: String,
        /// Path to private key .pem file
        key: String,
    },
}

impl From<&WebTransportCertificateSettings> for Identity {
    fn from(wt: &WebTransportCertificateSettings) -> Identity {
        match wt {
            WebTransportCertificateSettings::FromFile {
                cert: cert_pem_path,
                key: private_key_pem_path,
            } => {
                println!(
                    "Reading certificate PEM files:\n * cert: {cert_pem_path}\n * key: {private_key_pem_path}",
                );
                // this is async because we need to load the certificate from io
                // we need async_compat because wtransport expects a tokio reactor
                let identity = IoTaskPool::get()
                    .scope(|s| {
                        s.spawn(Compat::new(async {
                            Identity::load_pemfiles(cert_pem_path, private_key_pem_path)
                                .await
                                .unwrap()
                        }));
                    })
                    .pop()
                    .unwrap();
                let digest = identity.certificate_chain().as_slice()[0].hash();
                println!("🔐 Certificate digest: {digest}");
                identity
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ServerTransports {
    WebTransport {
        local_port: u16,
        certificate: WebTransportCertificateSettings,
    },
}

#[derive(Component, Debug)]
#[component(on_add = ExampleServer::on_add)]
pub struct ExampleServer {
    /// Which transport to use
    pub transport: ServerTransports,
    pub shared: SharedSettings,
}

impl ExampleServer {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        world.commands().queue(move |world: &mut World| -> Result {
            let mut entity_mut = world.entity_mut(entity);
            let settings = entity_mut.take::<ExampleServer>().unwrap();
            entity_mut.insert((Name::from("Server"),));

            match settings.transport {
                ServerTransports::WebTransport {
                    local_port,
                    certificate,
                } => {
                    let private_key = settings.shared.private_key;

                    entity_mut.insert(NetcodeServer::new(NetcodeConfig {
                        protocol_id: settings.shared.protocol_id,
                        private_key,
                        ..Default::default()
                    }));

                    let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), local_port);
                    entity_mut.insert((
                        LocalAddr(server_addr),
                        WebTransportServerIo {
                            certificate: (&certificate).into(),
                        },
                    ));
                }
            }

            Ok(())
        });
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SharedSettings {
    /// An id to identify the protocol version
    pub protocol_id: u64,

    /// a 32-byte array to authenticate via the Netcode.io protocol
    pub private_key: [u8; 32],
}

pub(crate) fn start(mut commands: Commands, server: Single<Entity, With<Server>>) {
    commands.trigger(Start {
        entity: server.into_inner(),
    });
}

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

    app.world_mut().spawn(ExampleServer {
        transport: ServerTransports::WebTransport {
            local_port: 5888,
            certificate: WebTransportCertificateSettings::FromFile {
                cert: "./certificates/cert.pem".to_string(),
                key: "./certificates/key.pem".to_string(),
            },
        },
        shared: SharedSettings {
            protocol_id: 0,
            private_key: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
        },
    });
    app.add_systems(Startup, start);

    app.run();
}
