use std::time::Duration;

use avian2d::prelude::*;
use beacon_core::{
    TILE,
    interaction::{Interactable, Interaction, attach_interaction_zone},
    inventory::{Inventory, SelectedSlot},
};
use bevy::{
    color::palettes::css::{GREEN, ORANGE, RED, YELLOW},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    time::common_conditions::on_timer,
};
use bevy_ecs_ldtk::prelude::*;

const BALEFIRE_RADIUS: f32 = TILE;
const BALEFIRE_DROP_ZONE_RADIUS: f32 = BALEFIRE_RADIUS * 1.5;
const MAX_INTENSITY: u8 = 100;

pub struct BalefirePlugin;

impl Plugin for BalefirePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_scene("balefire", balefire_scene)
            .add_systems(
                Update,
                (
                    decay.run_if(on_timer(Duration::from_secs(1))),
                    update_color,
                    update_label,
                ),
            )
            .add_observer(on_interaction);
    }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(RigidBody::Static, Collider::circle(BALEFIRE_RADIUS), Interactable)]
#[component(on_add = on_balefire_add)]
struct Balefire {
    intensity: u8,
}

fn on_balefire_add(mut world: DeferredWorld, ctx: HookContext) {
    attach_interaction_zone(
        &mut world,
        ctx.entity,
        Collider::circle(BALEFIRE_DROP_ZONE_RADIUS),
        Vec2::ZERO,
    );
}

#[derive(Component, Default, Clone)]
struct BalefireLabel;

fn balefire_scene() -> impl Scene {
    bsn! {
        Balefire
        Mesh2d(asset_value(Circle::new(BALEFIRE_RADIUS)))
        MeshMaterial2d::<ColorMaterial>(asset_value(Color::WHITE))
        Children [
            BalefireLabel
            Text2d
            TextFont { font_size: FontSize::Px(12.0) }
            Transform::from_xyz(0.0, 0.0, 1.0)
        ]
    }
}

fn decay(mut balefires: Query<&mut Balefire>) {
    for mut balefire in &mut balefires {
        balefire.intensity = balefire.intensity.saturating_sub(1);
    }
}

fn on_interaction(
    event: On<Interaction>,
    mut balefires: Query<&mut Balefire>,
    mut players: Query<(&mut Inventory, &SelectedSlot)>,
) {
    if event
        .item
        .as_ref()
        .is_none_or(|stack| stack.def.id != *"wood")
    {
        return;
    }
    let Ok(mut balefire) = balefires.get_mut(event.interactable) else {
        return;
    };
    let Ok((mut inventory, selected)) = players.get_mut(event.player) else {
        return;
    };
    let Some(stack) = inventory.take(selected.0) else {
        return;
    };

    balefire.intensity = (balefire.intensity as u16 + stack.count).min(MAX_INTENSITY as u16) as u8;
}

fn intensity_color(intensity: u8) -> Color {
    match intensity {
        0..25 => GREEN,
        25..50 => YELLOW,
        50..75 => ORANGE,
        _ => RED,
    }
    .into()
}

fn update_color(
    balefire: Single<(&Balefire, &MeshMaterial2d<ColorMaterial>), Changed<Balefire>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (balefire, material) = *balefire;
    if let Some(mut material) = materials.get_mut(&material.0) {
        material.color = intensity_color(balefire.intensity);
    }
}

fn update_label(
    balefire: Single<&Balefire, Changed<Balefire>>,
    mut label: Single<&mut Text2d, With<BalefireLabel>>,
) {
    label.0 = balefire.intensity.to_string();
}
