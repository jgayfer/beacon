use avian2d::prelude::*;
use bevy::prelude::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_hitboxes)
            .add_observer(detect_hitbox_collision);
    }
}

#[derive(Component, Default, Clone)]
#[require(Sensor, CollisionEventsEnabled)]
pub struct Hitbox {
    pub timer: Timer,
}

/// A marker to detect when an entity is hit.
///
/// Must be attached as a child of the entity being hit.
#[derive(Component, Default, Clone)]
#[require(Sensor, Collider)]
pub struct Hurtbox;

#[derive(EntityEvent)]
pub struct Hit {
    #[event_target]
    pub target: Entity,
    pub hitbox: Entity,
}

fn detect_hitbox_collision(
    event: On<CollisionStart>,
    hitboxes: Query<(), With<Hitbox>>,
    hurtboxes: Query<&ChildOf, With<Hurtbox>>,
    mut commands: Commands,
) {
    let (hitbox, target) = (event.collider1, event.collider2);
    if hitboxes.contains(hitbox)
        && let Ok(target) = hurtboxes.get(target).map(ChildOf::parent)
    {
        commands.trigger(Hit { target, hitbox });
    }
}

fn tick_hitboxes(mut commands: Commands, time: Res<Time>, hitboxes: Query<(Entity, &mut Hitbox)>) {
    for (entity, mut hitbox) in hitboxes {
        hitbox.timer.tick(time.delta());
        if hitbox.timer.just_finished()
            && let Ok(mut hitbox_entity) = commands.get_spawned_entity(entity)
        {
            hitbox_entity.despawn();
        }
    }
}
