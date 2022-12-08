use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, RichText},
    EguiContext,
};

use crate::{
    ui::{PauseSimulation, StartSimulation, UiState},
    AppState,
};

pub fn menu(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut exit: EventWriter<AppExit>,
    mut start_sim: EventWriter<StartSimulation>,
    mut pause_sim: EventWriter<PauseSimulation>,
    app_state: Res<AppState>,
) {
    egui::TopBottomPanel::top("header").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("lcp-sim", |ui| {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            });
            ui.menu_button("Simulation", |ui| {
                if ui.button("Restore defaults").clicked() {
                    ui_state.reset();
                    *ui_state = UiState::default();
                    ui.close_menu();
                }
                if ui.button("Edit equation").clicked() {
                    ui_state.show_equation_edit = true;
                    ui.close_menu();
                }
            });
            if ui.button("Help").clicked() {
                ui_state.show_help = !ui_state.show_help;
                ui.close_menu();
            }

            if app_state.is_running {
                if ui.button("Pause").clicked() {
                    pause_sim.send(PauseSimulation);
                }
            } else if ui.button("Run").clicked() && ui_state.eq_res.is_ok() {
                start_sim.send(StartSimulation);
            }

            ui.centered_and_justified(|ui| {
                ui.heading(RichText::new("Le Chateliers Principle Simulation").strong())
            });
        });
        ui.add_space(10.0);
    });
}
