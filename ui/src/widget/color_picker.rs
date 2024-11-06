use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    ui::Val::*,
};

use crate::theme;

use super::Spawn;

pub trait ColorPickerWidget {
    fn color_picker(&mut self, ui_materials: ResMut<Assets<HueGradientMaterial>>)
        -> EntityCommands;
}

impl<T: Spawn> ColorPickerWidget for T {
    fn color_picker(
        &mut self,
        mut ui_materials: ResMut<Assets<HueGradientMaterial>>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("ColorPicker Parent"),
            NodeBundle {
                style: Style {
                    display: Display::Block,
                    width: Px(200.0),
                    height: Px(200.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(theme::PANEL_OUTLINE),
                ..default()
            },
        ));

        entity.with_children(|parent| {
            parent.spawn(MaterialNodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(250.0),
                    height: Val::Px(250.0),
                    ..default()
                },
                material: ui_materials.add(HueGradientMaterial {
                    color: LinearRgba::WHITE.to_f32_array().into(),
                }),
                ..default()
            });
        });

        entity
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct HueGradientMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for HueGradientMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_shader.wgsl".into()
    }
}
