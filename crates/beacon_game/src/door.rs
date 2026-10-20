use avian2d::{collision::collision_events::CollisionStart, physics_transform::Position};
use beacon_core::{
    door::{Door, DoorEntered, DoorExit},
    player::Player,
};
use bevy_app::{App, Plugin};
use bevy_ecs::{
    observer::On,
    query::{With, Without},
    system::{Commands, Query},
};
use bevy_transform::components::GlobalTransform;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(detect_door_contact)
            .add_observer(on_door_entered);
    }
}

fn on_door_entered(
    event: On<DoorEntered>,
    doors: Query<&Door>,
    exits: Query<&GlobalTransform, With<DoorExit>>,
    mut target_positions: Query<&mut Position, Without<Door>>,
) {
    let Some(exit) = doors.get(event.door).ok().and_then(|door| door.exit) else {
        return;
    };

    let (Ok(exit_transform), Ok(mut position)) =
        (exits.get(exit), target_positions.get_mut(event.target))
    else {
        return;
    };

    position.0 = exit_transform.translation().truncate();
}

fn detect_door_contact(
    event: On<CollisionStart>,
    doors: Query<(), With<Door>>,
    players: Query<(), With<Player>>,
    mut commands: Commands,
) {
    let (door, other) = (event.collider1, event.collider2);
    if doors.contains(door) && players.contains(other) {
        commands.trigger(DoorEntered {
            door,
            target: other,
        });
    }
}
