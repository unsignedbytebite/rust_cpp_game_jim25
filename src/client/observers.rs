use super::*;
use crate::protocol::components::*;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use lightyear::prelude::{input::native::InputMarker, *};

/// When the predicted copy of the client-owned entity is spawned, do stuff
/// - assign it a different saturation
/// - keep track of it in the Global resource
///
/// Note that this will be triggered multiple times: for the locally-controlled entity,
/// but also for the remote-controlled entities that are spawned with [`Interpolated`].
/// The `With<Predicted>` filter ensures we only add the `InputMarker` once.
pub(crate) fn handle_predicted_spawn(
    trigger: On<Add, PlayerId>,
    mut predicted: Query<(&PlayerPosition, &PlayerId), With<Predicted>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("predicted spawn");
    let entity = trigger.entity;
    if let Ok((pos, id)) = predicted.get_mut(entity) {
        info!("Add InputMarker to Predicted entity: {:?}", entity);
        commands.entity(entity).insert((
            InputMarker::<Inputs>::default(),
            Transform::from_xyz(pos.x, pos.y, 0.0),
            RenderLayers::layer(0),
            AseAnimation {
                animation: Animation::tag("loop"),
                aseprite: asset_server.load("elf.aseprite"),
            },
            Sprite::default(),
            id.to_owned(),
        ));
    }
}
