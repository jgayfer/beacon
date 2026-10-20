use beacon_core::balefire::{BALEFIRE_RADIUS, Balefire};
use bevy::{
    color::palettes::css::{GREEN, ORANGE, RED, YELLOW},
    prelude::*,
};

#[derive(Component, Default, Clone)]
struct BalefireLabel;

pub fn balefire_scene() -> impl Scene {
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

pub struct BalefireScenePlugin;

impl Plugin for BalefireScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_color, update_label));
    }
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
