use avian2d::prelude::*;
use beacon_core::player::{Action, Facing, Player};
use beacon_engine::hitbox::Hitbox;
use bevy::prelude::*;

const SPEED: f32 = 100.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, start_swing));
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut LinearVelocity, &mut Facing, &mut Action), With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    for (mut velocity, mut facing, mut action) in &mut q {
        velocity.0 = dir.normalize_or_zero() * SPEED;

        action.set_if_neq(if dir == Vec2::ZERO {
            Action::Idle
        } else {
            Action::Walking
        });

        if dir.x < 0.0 {
            facing.set_if_neq(Facing::Left);
        } else if dir.x > 0.0 {
            facing.set_if_neq(Facing::Right);
        }
    }
}

fn start_swing(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &Facing), With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, facing) in player_query {
            let x = match facing {
                Facing::Left => -16.0,
                Facing::Right => 16.0,
            };
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
