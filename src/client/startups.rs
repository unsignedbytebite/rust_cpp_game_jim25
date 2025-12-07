use super::*;
use bevy::{
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
};
use bevy_aseprite_ultra::prelude::*;

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 1920 / 4,
        height: 1080 / 4,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut target_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    target_image.resize(size);

    let target_handle = images.add(target_image);

    commands.spawn((
        Camera {
            target: RenderTarget::Image(target_handle.clone().into()),
            order: -1,
            clear_color: ClearColorConfig::Custom(bevy::color::palettes::tailwind::CYAN_600.into()),
            ..Default::default()
        },
        Msaa::Off,
        Camera2d,
        Transform::default(),
        RenderLayers::layer(0),
    ));

    commands.spawn((
        Sprite::from_image(target_handle),
        RenderLayers::layer(1),
        Transform::default().with_scale(Vec3::splat(2.0)),
    ));
    commands.spawn((Camera2d, Msaa::Off, RenderLayers::layer(1)));
}
