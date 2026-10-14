use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_light_2d::light::PointLight2d;

pub fn beacon_scene() -> impl Scene {
    bsn! { PointLight2d { radius: 150.0, color: WHITE } }
}
