use beacon_core::{player::Player, progression::Progression};
use bevy::prelude::*;
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiContext, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui},
    bevy_inspector::{self, Filter},
};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin::default(), DefaultInspectorConfigPlugin))
            .add_systems(EguiPrimaryContextPass, inspector_ui);
    }
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Inspector")
        .default_size((320.0, 160.0))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                egui::CollapsingHeader::new("Progression")
                    .default_open(false)
                    .show(ui, |ui| {
                        bevy_inspector::ui_for_resource::<Progression>(world, ui);
                    });
                egui::CollapsingHeader::new("Player")
                    .default_open(false)
                    .show(ui, |ui| {
                        bevy_inspector::ui_for_entities_filtered(
                            world,
                            ui,
                            true,
                            &Filter::<With<Player>>::all(),
                        );
                    });
                egui::CollapsingHeader::new("World").show(ui, |ui| {
                    egui::CollapsingHeader::new("Entities").show(ui, |ui| {
                        bevy_inspector::ui_for_entities(world, ui);
                    });
                    egui::CollapsingHeader::new("Resources").show(ui, |ui| {
                        bevy_inspector::ui_for_resources(world, ui);
                    });
                    egui::CollapsingHeader::new("Assets").show(ui, |ui| {
                        bevy_inspector::ui_for_all_assets(world, ui);
                    });
                });
                ui.allocate_space(ui.available_size());
            });
        });
}
