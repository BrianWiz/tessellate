mod bind_groups;
pub mod brush;
mod compute;
pub mod mouse;
mod node;
mod pipeline;
pub mod sprite;

use bevy::{prelude::*, utils::warn};
use brush::{BrushColor, BrushSize};
use compute::CanvasComputePlugin;

pub const SIZE: (u32, u32) = (1920 * 3, 1920 * 3);
const SHADER_ASSET_PATH: &str = "shaders/canvas.wgsl";
const WORKGROUP_SIZE: u32 = 8;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(CanvasComputePlugin)
        .insert_resource(BrushSize(8.0))
        .insert_resource(BrushColor(Color::hsla(0.5, 0.5, 0.5, 1.0)))
        .add_systems(PreStartup, (sprite::setup, mouse::setup))
        .add_systems(Update, mouse::update_position.map(warn));
}
