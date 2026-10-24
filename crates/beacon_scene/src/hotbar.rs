use beacon_assets::ItemRegistry;
use beacon_core::{
    inventory::{Inventory, SelectedSlot},
    player::Player,
};
use bevy::{color::palettes::css::YELLOW, image::TRANSPARENT_IMAGE_HANDLE, prelude::*};

pub struct HotbarPlugin;

impl Plugin for HotbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_hotbar, highlight_selected));
    }
}

#[derive(Component, Default, Clone)]
struct HotbarSlot(usize);

#[derive(Component, Default, Clone)]
struct HotbarCount(usize);

pub fn hotbar_scene() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(4.0),
        }
        Children [{ (0..9).map(slot_scene).collect::<Vec<_>>() }]
    }
}

fn slot_scene(i: usize) -> impl Scene {
    bsn! {
        HotbarSlot(i)
        Node {
            width: Val::Px(48.0),
            height: Val::Px(48.0),
            border: UiRect::all(Val::Px(2.0)),
        }
        BorderColor::all(Color::WHITE)
        BackgroundColor({ Color::BLACK.with_alpha(0.5) })
        ImageNode
        Children [
            HotbarCount(i)
            Text
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(2.0),
                bottom: Val::Px(0.0),
            }
        ]
    }
}

fn update_hotbar(
    inventory: Single<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut slots: Query<(&HotbarSlot, &mut ImageNode)>,
    mut counts: Query<(&HotbarCount, &mut Text)>,
    items: Res<ItemRegistry>,
) {
    for (slot, mut image) in &mut slots {
        image.image = inventory
            .get(slot.0)
            .and_then(|stack| items.image(&stack.def.id))
            .unwrap_or(TRANSPARENT_IMAGE_HANDLE);
    }
    for (count, mut text) in &mut counts {
        text.0 = inventory
            .get(count.0)
            .map(|s| s.count.to_string())
            .unwrap_or_default();
    }
}

fn highlight_selected(
    selected: Single<&SelectedSlot, (With<Player>, Changed<SelectedSlot>)>,
    mut slots: Query<(&HotbarSlot, &mut BorderColor)>,
) {
    for (slot, mut border) in &mut slots {
        *border = BorderColor::all(if slot.0 == selected.0 {
            YELLOW.into()
        } else {
            Color::WHITE
        });
    }
}
