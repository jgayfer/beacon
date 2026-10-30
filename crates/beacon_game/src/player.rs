use avian2d::prelude::*;
use beacon_core::{
    interaction::{Interaction, InteractionZone},
    inventory::{Inventory, SelectedSlot},
    item::{ItemKind, ItemStack},
    player::Player,
};
use beacon_engine::hitbox::Hitbox;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_replique::prelude::*;

const SPEED: f32 = 100.0;
const SOLDIER_IDLE: &str = "Idle";
const SOLDIER_WALK: &str = "Walk";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components::<Player, Facing>()
            .register_required_components::<Player, Action>()
            .register_ldtk_entity_scene("player_spawn", PlayerSpawn)
            .add_systems(
                Update,
                (
                    spawn_player,
                    animate_player,
                    (move_player, interact, use_item)
                        .run_if(not(any_with_component::<DialogueRunner>)),
                    halt_player.run_if(any_with_component::<DialogueRunner>),
                ),
            )
            .add_observer(spawn_tool_hitbox);
    }
}

#[derive(Component, Clone, Default, Copy, PartialEq, Reflect)]
#[reflect(Component)]
enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Component, Clone, Default, Copy, PartialEq, Reflect)]
#[reflect(Component)]
enum Action {
    #[default]
    Idle,
    Walking,
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
struct PlayerSpawn;

#[derive(EntityEvent)]
struct ItemUsed {
    #[event_target]
    player: Entity,
    item: ItemStack,
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

fn halt_player(mut q: Query<(&mut LinearVelocity, &mut Action), With<Player>>) {
    for (mut velocity, mut action) in &mut q {
        velocity.set_if_neq(LinearVelocity::ZERO);
        action.set_if_neq(Action::Idle);
    }
}

fn spawn_tool_hitbox(event: On<ItemUsed>, mut commands: Commands, facing: Query<&Facing>) {
    if matches!(event.item.def.kind, ItemKind::Tool) {
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
            item: selected_item.clone(),
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
        item: inventory.get(selected.0).cloned(),
    });
}

fn spawn_player(
    mut commands: Commands,
    player: Query<&Player>,
    player_spawns: Query<&Transform, With<PlayerSpawn>>,
) {
    if player.is_empty()
        && let Some(transform) = player_spawns.iter().next()
    {
        let player_transform = *transform;
        commands.spawn_scene(bsn! {
            @player_scene()
            player_transform
        });
    }
}

fn player_scene() -> impl Scene {
    bsn! {
        #Player
        Player
        Sprite
        AseAnimation {
            aseprite: "player/sprite.aseprite",
            animation: Animation::tag(SOLDIER_IDLE),
        }
    }
}

fn animate_player(
    mut q: Query<
        (&Facing, &Action, &mut AseAnimation, &mut Sprite),
        (With<Player>, Or<(Changed<Facing>, Changed<Action>)>),
    >,
) {
    for (facing, action, mut animation, mut sprite) in &mut q {
        let animation_tag = match action {
            Action::Idle => SOLDIER_IDLE,
            Action::Walking => SOLDIER_WALK,
        };

        if animation.animation.tag.as_deref() != Some(animation_tag) {
            animation.animation.play_loop(animation_tag);
        }

        sprite.flip_x = matches!(facing, Facing::Left);
    }
}
