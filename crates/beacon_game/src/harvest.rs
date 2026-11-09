use beacon_core::{
    harvest::{Harvestable, Harvested},
    item::{Drops, GroundItem, ItemDef},
};
use beacon_engine::{hitbox::Hit, manifest::Registry};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_transform::prelude::*;

pub struct HarvestPlugin;

impl Plugin for HarvestPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_hit);
    }
}

fn on_hit(
    event: On<Hit>,
    items: Res<Registry<ItemDef>>,
    mut commands: Commands,
    mut query: Query<(&mut Harvestable, Option<&Drops>, &GlobalTransform)>,
) {
    let Ok((mut harvestable, drops, transform)) = query.get_mut(event.target) else {
        return;
    };

    harvestable.0 = harvestable.0.saturating_sub(1);

    if harvestable.0 != 0 {
        return;
    }

    commands
        .entity(event.target)
        .remove::<Harvestable>()
        .insert(Harvested);

    for item_spec in drops.map(|drops| drops.0.as_slice()).unwrap_or_default() {
        let Some(item_def) = items.get(item_spec.id.get()) else {
            warn!("{0} is not in the items manifest", item_spec.id.get());
            continue;
        };

        let stack = item_def.stack(item_spec.count);

        commands.spawn((
            GroundItem(stack),
            Transform::from_translation(transform.translation()),
        ));
    }
}
