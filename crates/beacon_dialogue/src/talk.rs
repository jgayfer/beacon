use beacon_core::{dialogue::TalkedTo, progression::Progression};
use bevy_asset::Assets;
use bevy_ecs::prelude::*;

use crate::{NpcRoutes, asset::RoutesHandle, conversation::Conversation};

pub fn on_talked_to<N: Component, R: NpcRoutes>(
    event: On<TalkedTo>,
    npcs: Query<(), With<N>>,
    progression: Res<Progression>,
    handle: Res<RoutesHandle<R>>,
    routes: Res<Assets<R>>,
    mut conversation: Conversation,
) {
    if !npcs.contains(event.npc) || conversation.is_active() {
        return;
    }
    let Some(routes) = routes.get(&handle.0) else {
        return;
    };

    if let Some(route) = routes.pick(&progression) {
        conversation.start(route);
    }
}
