use avian2d::collision::collider::Collider;
use beacon_core::{
    item::{GroundItem, Item},
    player::Player,
};
use beacon_engine::hitbox::{Hit, Hitbox};
use bevy::prelude::*;

use crate::Tree;

pub struct WoodcuttingPlugin;

impl Plugin for WoodcuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (chop, check_tree_health))
            .add_observer(on_tree_hit);
    }
}

fn on_tree_hit(event: On<Hit>, mut trees: Query<&mut Tree>) {
    if let Ok(mut tree) = trees.get_mut(event.target) {
        info!("Tree hit");
        tree.health = tree.health.saturating_sub(1);
    }
}

fn check_tree_health(
    mut commands: Commands,
    query: Query<(Entity, &Tree, &GlobalTransform), Changed<Tree>>,
) {
    for (entity, tree, tree_transform) in query {
        if tree.health == 0 {
            commands.entity(entity).despawn();
            commands.spawn_scene(bsn! {
                GroundItem(Item::Wood)
                Transform { translation: {tree_transform.translation()} }
                Sprite {
                    image: "log.png",
                }
            });
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
                Collider::rectangle(16.0, 16.0),
                Transform::from_xyz(x, 0.0, 0.0),
            ));
        }
    }
}
