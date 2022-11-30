use bevy::{app::AppExit, prelude::EventWriter};
use bevy_egui::egui::{self, RichText};

use crate::{ui::UiState, AppState};

pub fn menu(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    app_state: &mut AppState,
    mut exit: EventWriter<AppExit>,
) {
    ui.horizontal(|ui| {
        ui.menu_button("lcp-sim", |ui| {
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
        });
        ui.menu_button("Simulation", |ui| {
            if ui.button("Restore defaults").clicked() {
                ui_state.reset();
                *app_state = AppState::default();
                ui.close_menu();
            }
            if ui.button("Edit equation").clicked() {
                ui_state.show_equation_edit = true;
                ui.close_menu();
            }
        });
        ui.menu_button("Help", |ui| {
            if ui.button("Help").clicked() {
                ui_state.show_help = true;
                ui.close_menu();
            }
        });
        ui.centered_and_justified(|ui| {
            ui.heading(RichText::new("Le Chateliers Principle Simulation").strong())
        });
    });
    ui.add_space(10.0);
}
