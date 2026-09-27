use avian2d::collision::collider::{Collider, Sensor};
use bevy::prelude::*;

use crate::Player;

pub struct WoodcuttingPlugin;

impl Plugin for WoodcuttingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_hitboxes, chop));
    }
}

#[derive(Component, Default, Clone)]
#[require(Collider::rectangle(16.0, 16.0), Sensor)]
pub struct Hitbox {
    pub timer: Timer,
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

fn chop_hitbox(x: f32) -> impl Scene {
    bsn! {
        Hitbox { timer: Timer::from_seconds(0.5, TimerMode::Once) }
        Transform { translation: Vec3 { x } }
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
            commands.spawn_scene(chop_hitbox(x)).insert(ChildOf(entity));
        }
    }
}
