use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{ui::UiState, AppState};

pub fn eq_edit(
    mut ui_state: ResMut<UiState>,
    mut egui_context: ResMut<EguiContext>,
    app_state: Res<AppState>,
) {
    // always show window when the equation is invalid
    if ui_state.eq_res.is_err() {
        ui_state.show_equation_edit = true;
    }

    // equation editor
    let mut open = ui_state.show_equation_edit && !app_state.running.is_running();
    egui::Window::new("Choose an Equation")
        .collapsible(false)
        .resizable(true)
        .open(&mut open)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Equation");
            ui.add_space(10.0);

            ui.label("Input a chemical equation:");
            ui.text_edit_singleline(&mut ui_state.input);

            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(match ui_state.eq_res.0 {
                    Ok(_) => egui::Color32::GREEN,
                    Err(_) => egui::Color32::RED,
                });
                ui.label(ui_state.to_string());
            });

            ui.horizontal(|ui| {
                if ui.button("Balance Equation").clicked() && ui_state.eq_res.is_ok() {
                    let res = ui_state.eq_res.as_ref().unwrap().to_balancer().balance();
                    if let Ok(eq) = res.as_ref() {
                        ui_state.input = eq.equation().to_string();
                    }
                    ui_state.eq_res = res.map_err(Into::into).into();
                }

                // if the ok button is clicked, or enter is pressed, but only if the Equation is valid
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui_state.show_equation_edit = !((ui.button("Ok").clicked()
                        || ui.ctx().input().key_pressed(egui::Key::Enter))
                        && ui_state.eq_res.is_ok());
                    ui.add_space(10.0);
                });
            });
        });
    if !open {
        ui_state.show_equation_edit = open;
    }
}
