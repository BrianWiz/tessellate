use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (
            trigger_on_press,
            trigger_on_release,
            apply_interaction_palette,
        ),
    );
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl InteractionPalette {
    pub fn default(color: Color) -> Self {
        InteractionPalette {
            none: color,
            hovered: color.lighter(0.05),
            pressed: color.darker(0.03),
        }
    }
}

#[derive(Component)]
pub struct PreviousInteraction(Interaction);

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

fn trigger_on_press(
    interaction_query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (entity, interaction) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            commands.trigger_targets(OnPress, entity);
            commands
                .entity(entity)
                .insert(PreviousInteraction(*interaction));
        }
    }
}

#[derive(Event)]
pub struct OnRelease;

fn trigger_on_release(
    interaction_query: Query<
        (Entity, &Interaction, Option<&PreviousInteraction>),
        Changed<Interaction>,
    >,
    mut commands: Commands,
) {
    for (entity, interaction, previous_interaction) in &interaction_query {
        if matches!(
            previous_interaction,
            Some(PreviousInteraction(Interaction::Pressed))
        ) && matches!(interaction, Interaction::Hovered | Interaction::None)
        {
            commands.trigger_targets(OnRelease, entity);
        }
    }
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}
