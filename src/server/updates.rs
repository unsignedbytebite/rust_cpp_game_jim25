use super::*;
use bevy::{ecs::error::info, prelude::*};
use lightyear::prelude::server::ClientOf;
use lightyear::prelude::*;
use std::time::Duration;

/// When a new client tries to connect to a server, an entity is created for it with the `LinkOf` component.
/// This entity represents the link between the server and that client.
///
/// You can add additional components to update the link. In this case we will add a `ReplicationSender` that
/// will enable us to replicate local entities to that client.
pub fn handle_new_client(trigger: On<Add, LinkOf>, mut commands: Commands) {
    info!("Handle new client");
    commands.entity(trigger.entity).insert((
        ReplicationSender::new(
            Duration::from_millis(100),
            SendUpdatesMode::SinceLastAck,
            false,
        ),
        Name::from("Client"),
    ));
}

/// If the new client connects to the server, we want to spawn a new player entity for it.
///
/// We have to react specifically on `Connected` because there is no guarantee that the connection request we
/// received was valid. The server could reject the connection attempt for many reasons (server is full, packet is invalid,
/// DDoS attempt, etc.). We want to start the replication only when the client is confirmed as connected.
pub fn handle_connected(
    trigger: On<Add, Connected>,
    query: Query<&RemoteId, With<ClientOf>>,
    mut commands: Commands,
) {
    let Ok(client_id) = query.get(trigger.entity) else {
        return;
    };
    let client_id = client_id.0;
    // let entity = commands
    //     .spawn((
    //         PlayerBundle::new(client_id, Vec2::ZERO),
    //         // we replicate the Player entity to all clients that are connected to this server
    //         Replicate::to_clients(NetworkTarget::All),
    //         PredictionTarget::to_clients(NetworkTarget::Single(client_id)),
    //         InterpolationTarget::to_clients(NetworkTarget::AllExceptSingle(client_id)),
    //         ControlledBy {
    //             owner: trigger.entity,
    //             lifetime: Default::default(),
    //         },
    //     ))
    //     .id();

    info!("Create player entity for client {:?}", client_id);
}
