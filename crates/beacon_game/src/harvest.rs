use beacon_core::{
    harvest::{Harvestable, Harvested},
    item::Drops,
};
use beacon_engine::hitbox::Hit;
use beacon_scene::ground_item::ground_item_scene;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_scene::prelude::*;
use bevy_transform::prelude::*;

pub struct HarvestPlugin;

impl Plugin for HarvestPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_hit);
    }
}

fn on_hit(
    event: On<Hit>,
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

    for stack in drops.map(|drops| drops.0.as_slice()).unwrap_or_default() {
        let stack = stack.clone();

        commands.spawn_scene(bsn! {
            @ground_item_scene(stack)
            Transform { translation: {transform.translation()} }
        });
    }
}
