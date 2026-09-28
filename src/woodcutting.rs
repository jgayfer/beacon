use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::{CollisionEventsEnabled, CollisionStart},
};
use bevy::prelude::*;

use crate::{Player, Tree};

pub struct WoodcuttingPlugin;

impl Plugin for WoodcuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_hitboxes, chop))
            .add_observer(on_hitbox_hit);
    }
}

#[derive(Component, Default, Clone)]
#[require(Collider::rectangle(16.0, 16.0), Sensor, CollisionEventsEnabled)]
pub struct Hitbox {
    pub timer: Timer,
}

fn on_hitbox_hit(
    event: On<CollisionStart>,
    hitbox_query: Query<(), With<Hitbox>>,
    tree_query: Query<(), With<Tree>>,
    mut commands: Commands,
) {
    let (hitbox, other) = (event.collider1, event.collider2);
    if hitbox_query.contains(hitbox) && tree_query.contains(other) {
        commands.entity(other).despawn();
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
