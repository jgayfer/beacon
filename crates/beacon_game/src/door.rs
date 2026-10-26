use avian2d::collision::collision_events::CollisionStart;
use beacon_core::{
    door::{Door, DoorEntered},
    player::Player,
};
use bevy_app::{App, Plugin};
use bevy_ecs::{
    observer::On,
    query::With,
    system::{Commands, Query},
};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(detect_door_contact);
    }
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
