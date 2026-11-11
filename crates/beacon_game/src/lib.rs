mod balefire;
mod camera;
mod dialogue;
mod door;
mod flame_sprite;
mod ground_item;
mod harvest;
mod hotbar;
mod item;
mod player;
mod tree;
mod world;

use balefire::BalefirePlugin;
use beacon_core::{progression::Progression, state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use camera::CameraPlugin;
use dialogue::DialoguePlugin;
use door::DoorPlugin;
use flame_sprite::FlameSpritePlugin;
use ground_item::GroundItemPlugin;
use harvest::HarvestPlugin;
use hotbar::HotbarPlugin;
use item::ItemPlugin;
use player::PlayerPlugin;
use tree::TreePlugin;
use world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
            )
            .init_resource::<Progression>()
            .add_plugins((
                ItemPlugin,
                CameraPlugin,
                WorldPlugin,
                PlayerPlugin,
                HarvestPlugin,
                GroundItemPlugin,
                HotbarPlugin,
                BalefirePlugin,
                DoorPlugin,
                TreePlugin,
                FlameSpritePlugin,
                DialoguePlugin,
            ));
    }
}
