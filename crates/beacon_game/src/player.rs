use avian2d::prelude::*;
use beacon_core::{
    interaction::{Interactable, Interaction, InteractionZone},
    inventory::{Inventory, SelectedSlot},
    item::{ItemKind, ItemUsed},
    player::{Action, Facing, Player},
};
use beacon_engine::hitbox::Hitbox;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;
use bevy_math::prelude::*;
use bevy_time::prelude::*;
use bevy_transform::prelude::*;

const SPEED: f32 = 100.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, interact, use_item))
            .add_observer(spawn_tool_hitbox);
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

fn spawn_tool_hitbox(event: On<ItemUsed>, mut commands: Commands, facing: Query<&Facing>) {
    if matches!(event.item.id.def().kind, ItemKind::Tool) {
        let Ok(facing) = facing.get(event.player) else {
            return;
        };

        let x = match facing {
            Facing::Left => -16.0,
            Facing::Right => 16.0,
        };

        commands.entity(event.player).with_child((
            Hitbox {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
            Collider::rectangle(16.0, 16.0),
            Transform::from_xyz(x, 0.0, 0.0),
        ));
    }
}

fn use_item(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<Entity, With<Player>>,
    selected: Single<&SelectedSlot, With<Player>>,
    inventory: Single<&Inventory, With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let Some(selected_item) = inventory.get(selected.0) else {
            return;
        };

        commands.trigger(ItemUsed {
            player: player.entity(),
            item: *selected_item,
        });
    }
}

fn interact(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player: Single<(Entity, &CollidingEntities, &Inventory, &SelectedSlot), With<Player>>,
    interaction_zones: Query<&ChildOf, With<InteractionZone>>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }

    let (player, colliding, inventory, selected) = player.into_inner();

    let Some(interactable) = colliding.iter().copied().find_map(|colliding_entity| {
        interaction_zones
            .get(colliding_entity)
            .ok()
            .map(ChildOf::parent)
    }) else {
        return;
    };

    commands.trigger(Interaction {
        interactable,
        player,
        item: inventory.get(selected.0).copied(),
    });
}
