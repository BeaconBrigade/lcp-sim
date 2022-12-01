use bevy_egui::egui;
use chem_eq::balance::EquationBalancer;

use crate::ui::UiState;

pub fn eq_edit(ui: &mut egui::Ui, ui_state: &mut UiState) {
    ui.heading("Equation");
    ui.add_space(10.0);

    ui.label("Input a chemical equation:");
    ui.text_edit_singleline(&mut ui_state.input);

    ui.scope(|ui| {
        ui.visuals_mut().override_text_color = Some(match ui_state.eq_res {
            Ok(_) => egui::Color32::GREEN,
            Err(_) => egui::Color32::RED,
        });
        ui.label(ui_state.to_string());
    });

    ui.horizontal(|ui| {
        if ui.button("Balance Equation").clicked() && ui_state.eq_res.is_ok() {
            let res = EquationBalancer::new(ui_state.eq_res.as_ref().unwrap()).balance();
            if let Ok(eq) = res.as_ref() {
                ui_state.input = eq.equation().to_string();
            }
            ui_state.eq_res = res.map_err(Into::into);
        }

        // if the ok button is clicked, or enter is pressed, but only if the Equation is valid
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            ui_state.show_equation_edit = !((ui.button("Ok").clicked()
                || ui.ctx().input().key_pressed(egui::Key::Enter))
                && ui_state.eq_res.is_ok());
            ui.add_space(10.0);
        });
    });
}
