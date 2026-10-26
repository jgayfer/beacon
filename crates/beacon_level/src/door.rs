use avian2d::physics_transform::Position;
use beacon_core::door::{Door, DoorEntered, DoorExit};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const DOOR: &str = "door";
const DOOR_EXIT: &str = "door_exit";
const EXIT_FIELD: &str = "exit";

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_door_entered)
            .add_systems(
                Update,
                move_to_pending_exit.run_if(resource_exists::<PendingExit>),
            )
            .register_ldtk_entity_scene(DOOR, door)
            .register_ldtk_entity_scene(DOOR_EXIT, DoorExit);
    }
}

#[derive(Component, Clone, Default)]
struct DoorTarget {
    level: LevelIid,
    exit: EntityIid,
}

#[derive(Resource)]
struct PendingExit {
    target: Entity,
    exit: EntityIid,
}

fn door(ctx: &EntityInstanceContext) -> impl Scene + use<> {
    let door_target = ctx
        .entity_instance
        .get_entity_ref_field(EXIT_FIELD)
        .ok()
        .map(|exit| {
            let level = LevelIid::new(exit.level_iid.clone());
            let exit = EntityIid::new(exit.entity_iid.clone());
            bsn! { DoorTarget { level, exit } }
        });

    bsn! {
        Door
        @door_target
    }
}

fn on_door_entered(
    event: On<DoorEntered>,
    targets: Query<&DoorTarget>,
    mut level_selection: ResMut<LevelSelection>,
    mut commands: Commands,
) {
    let Ok(target) = targets.get(event.door) else {
        return;
    };

    *level_selection = LevelSelection::Iid(target.level.clone());

    commands.insert_resource(PendingExit {
        target: event.target,
        exit: target.exit.clone(),
    });
}

fn move_to_pending_exit(
    mut commands: Commands,
    mut level_events: MessageReader<LevelEvent>,
    pending: Res<PendingExit>,
    exits: Query<(&EntityIid, &GlobalTransform), With<DoorExit>>,
    mut positions: Query<&mut Position>,
) {
    if !level_events
        .read()
        .any(|event| matches!(event, LevelEvent::Transformed(_)))
    {
        return;
    }

    let Some(exit_transform) = exits
        .iter()
        .find_map(|(iid, transform)| (*iid == pending.exit).then_some(transform))
    else {
        return;
    };

    if let Ok(mut position) = positions.get_mut(pending.target) {
        position.0 = exit_transform.translation().truncate();
    }

    commands.remove_resource::<PendingExit>();
}
