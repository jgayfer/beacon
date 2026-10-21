use beacon_core::{dialogue::TalkedTo, progression::Progression};
use bevy_asset::Assets;
use bevy_ecs::prelude::*;

use crate::{NpcRoutes, asset::RoutesHandle, conversation::Conversation, npc::LastRoute};

pub fn on_talked_to<N: Component, R: NpcRoutes>(
    event: On<TalkedTo>,
    mut npcs: Query<&mut LastRoute, With<N>>,
    progression: Res<Progression>,
    handle: Res<RoutesHandle<R>>,
    routes: Res<Assets<R>>,
    mut conversation: Conversation,
) {
    let Ok(mut last) = npcs.get_mut(event.npc) else {
        return;
    };
    if conversation.is_active() {
        return;
    }
    let Some(routes) = routes.get(&handle.0) else {
        return;
    };

    if let Some(route) = routes.pick(&progression, last.0.as_ref()) {
        conversation.start(route);
        last.0 = Some(route.clone());
    }
}
