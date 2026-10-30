mod balefire;
mod camera;
mod dialogue;
mod door;
mod flame_sprite;
mod ground_item;
mod harvest;
mod inventory;
mod player;
mod tree;
mod world;

use balefire::BalefirePlugin;
use beacon_core::{item::ItemDef, progression::Progression, state::GameState};
use beacon_engine::manifest::RegisterManifest;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use camera::CameraPlugin;
use dialogue::DialoguePlugin;
use door::DoorPlugin;
use flame_sprite::FlameSpritePlugin;
use ground_item::GroundItemPlugin;
use harvest::HarvestPlugin;
use inventory::InventoryPlugin;
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
            .register_manifest::<ItemDef>(GameState::Loading)
            .init_resource::<Progression>()
            .add_plugins((
                CameraPlugin,
                WorldPlugin,
                PlayerPlugin,
                HarvestPlugin,
                GroundItemPlugin,
                InventoryPlugin,
                BalefirePlugin,
                DoorPlugin,
                TreePlugin,
                FlameSpritePlugin,
                DialoguePlugin,
            ));
    }
}
