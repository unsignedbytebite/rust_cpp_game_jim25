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
    mut predicted: Query<&Predicted>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("predicted spawn");
    let entity = trigger.entity;
    if let Ok(pre) = predicted.get_mut(entity) {
        warn!("Add InputMarker to Predicted entity: {:?}", entity);
        commands.entity(entity).insert((
            InputMarker::<Inputs>::default(),
            // Transform::from_xyz(20.0, 20.0, 0.0).with_scale(Vec3::splat(1.0)),
            RenderLayers::layer(0),
            AseAnimation {
                animation: Animation::tag("loop"),
                aseprite: asset_server.load("elf.aseprite"),
            },
            Sprite::default(),
            components::LocalPlayer,
        ));
    }
}

/// When the predicted copy of the client-owned entity is spawned, do stuff
/// - assign it a different saturation
/// - keep track of it in the Global resource
pub(crate) fn handle_interpolated_spawn(
    trigger: On<Add, Interpolated>,
    // mut interpolated: Query<Interpolated>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    warn!("interpolated spawn");
    commands.spawn((
        Transform::from_xyz(20.0, 20.0, 0.0).with_scale(Vec3::splat(1.0)),
        RenderLayers::layer(0),
        AseAnimation {
            animation: Animation::tag("loop"),
            aseprite: asset_server.load("elf.aseprite"),
        },
        Sprite::default(),
        components::LocalPlayer,
    ));
}
