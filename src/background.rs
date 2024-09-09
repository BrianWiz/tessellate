use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils,
};

use crate::{color, error::Result};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(color::BACKGROUND))
        .add_systems(Startup, checkered_background.map(utils::warn))
        .add_systems(Update, on_window_resize.map(utils::warn));
}

#[derive(Component)]
pub struct BackgroundImage;

fn checkered_background(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) -> Result<()> {
    let window = windows.get_single()?;
    let size: UVec2 = window.physical_size();

    let image = Image::new_fill(
        Extent3d {
            width: 100,
            height: 100,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[25, 89, 135, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );

    let image_handle = images.add(image);

    commands.spawn((
        Name::new("BackgroundImage"),
        SpriteBundle {
            texture: image_handle,
            transform: Transform::from_scale(Vec3::new(size.x as f32, size.y as f32, f32::MIN)),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.5, // The image will tile every 128px
        },
        BackgroundImage,
    ));

    Ok(())
}

fn on_window_resize(
    windows: Query<&Window>,
    mut background_q: Query<&mut Transform, With<BackgroundImage>>,
) -> Result<()> {
    let window = windows.get_single()?;
    let size: UVec2 = window.physical_size();
    let mut background_transform = background_q.get_single_mut()?;
    *background_transform = Transform::from_scale(Vec3::new(size.x as f32, size.y as f32, 1.0))
        .with_translation(Vec3::new(0.0, 0.0, -100.0));

    Ok(())
}