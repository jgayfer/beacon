use beacon_core::{health::Health, item::Drops, state::GameState};
use beacon_engine::hitbox::Hit;
use beacon_scene::ground_item::ground_item_scene;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_scene::prelude::*;
use bevy_state::prelude::*;
use bevy_transform::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_dead.run_if(in_state(GameState::Playing)))
            .add_observer(on_hit);
    }
}

fn on_hit(event: On<Hit>, mut healths: Query<&mut Health>) {
    if let Ok(mut health) = healths.get_mut(event.target) {
        health.0 = health.0.saturating_sub(1);
    }
}

fn despawn_dead(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Drops>, &GlobalTransform), Changed<Health>>,
) {
    for (entity, health, drops, transform) in query {
        if health.0 != 0 {
            continue;
        }
        commands.entity(entity).despawn();
        for stack in drops.map(|drops| drops.0.as_slice()).unwrap_or_default() {
            let stack = stack.clone();
            commands.spawn_scene(bsn! {
                @ground_item_scene(stack)
                Transform { translation: {transform.translation()} }
            });
        }
    }
}
