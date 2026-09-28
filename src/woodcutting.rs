use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::{CollisionEventsEnabled, CollisionStart},
};
use bevy::prelude::*;

use crate::{Player, Tree};

pub struct WoodcuttingPlugin;

impl Plugin for WoodcuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_hitboxes, chop, check_tree_health))
            .add_observer(on_hitbox_hit)
            .add_observer(on_tree_hit);
    }
}

#[derive(Component, Default, Clone)]
#[require(Collider::rectangle(16.0, 16.0), Sensor, CollisionEventsEnabled)]
pub struct Hitbox {
    pub timer: Timer,
}

#[derive(EntityEvent)]
pub struct TreeHit(Entity);

fn on_hitbox_hit(
    event: On<CollisionStart>,
    hitbox_query: Query<(), With<Hitbox>>,
    tree_query: Query<(), With<Tree>>,
    mut commands: Commands,
) {
    let (hitbox, other) = (event.collider1, event.collider2);
    if hitbox_query.contains(hitbox) && tree_query.contains(other) {
        commands.trigger(TreeHit(other));
    }
}

fn on_tree_hit(event: On<TreeHit>, query: Query<(Entity, &mut Tree)>) {
    info!("Tree hit");
    for (entity, mut tree) in query {
        if entity == event.0 {
            tree.health -= 1;
        }
    }
}

fn check_tree_health(mut commands: Commands, query: Query<(Entity, &Tree), Changed<Tree>>) {
    for (entity, tree) in query {
        if tree.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn tick_hitboxes(mut commands: Commands, time: Res<Time>, hitboxes: Query<(Entity, &mut Hitbox)>) {
    for (entity, mut hitbox) in hitboxes {
        hitbox.timer.tick(time.delta());
        if hitbox.timer.just_finished() {
            if let Ok(mut hitbox_entity) = commands.get_spawned_entity(entity) {
                hitbox_entity.despawn();
            }
        }
    }
}

fn chop(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Sprite), With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, sprite) in player_query {
            let x = if sprite.flip_x { -16.0 } else { 16.0 };
            commands.entity(entity).with_child((
                Hitbox {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
                Transform::from_xyz(x, 0.0, 0.0),
            ));
        }
    }
}
