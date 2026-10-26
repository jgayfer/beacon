use beacon_core::door::{Door, DoorExit};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const DOOR: &str = "door";
const DOOR_EXIT: &str = "door_exit";
const EXIT_FIELD: &str = "exit";

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resolve_door_exits)
            .register_ldtk_entity_scene(DOOR, unresolved_door)
            .register_ldtk_entity_scene(DOOR_EXIT, DoorExit);
    }
}

#[derive(Component, Clone, Default)]
struct DoorDestinationIid(String);

fn unresolved_door(ctx: &EntityInstanceContext) -> impl Scene + use<> {
    let door_destination_iid = ctx
        .entity_instance
        .field_instances
        .iter()
        .find(|f| f.identifier == EXIT_FIELD)
        .and_then(|f| match &f.value {
            FieldValue::EntityRef(Some(r)) => Some(r.entity_iid.clone()),
            _ => None,
        })
        .map(|iid| bsn! { DoorDestinationIid(iid) });

    bsn! {
        Door
        @door_destination_iid
    }
}

fn resolve_door_exits(
    mut commands: Commands,
    mut pending: Query<(Entity, &mut Door, &DoorDestinationIid)>,
    iids: Query<(Entity, &EntityIid)>,
) {
    for (entity, mut door, dest) in &mut pending {
        if let Some(target) = iids
            .iter()
            .find_map(|(exit, iid)| (iid.as_str() == dest.0).then_some(exit))
        {
            door.exit = Some(target);
            commands.entity(entity).remove::<DoorDestinationIid>();
        }
    }
}
