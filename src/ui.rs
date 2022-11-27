use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};
use chem_eq::{Equation, balance::EquationBalancer};

use crate::error::Error;

/// Store the apps curent state
#[derive(Debug, Clone, Resource)]
pub struct UiState {
    /// Show editing window
    pub show_equation_edit: bool,
    /// Input from last tick
    pub last_input: String,
    /// User input string
    pub input: String,
    /// Result of trying to balance user input
    pub equation_res: Result<Equation, Error>,
}

pub fn app_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut exit: EventWriter<AppExit>,
) {
    // header
    egui::TopBottomPanel::top("header").show(egui_context.ctx_mut(), |ui| {
        ui.menu_button("Simulation", |ui| {
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
            if ui.button("Reset simulation").clicked() {
                ui_state.reset();
                ui_state.input.clear();
                ui.close_menu();
            }
            if ui.button("Edit equation").clicked() {
                ui_state.show_equation_edit = true;
                ui.close_menu();
            }
        });
        ui.centered_and_justified(|ui| ui.heading("Le Chateliers Principle Simulation"));
    });

    // always show window when the equation is invalid
    if ui_state.equation_res.is_err() {
        ui_state.show_equation_edit = true;
    }

    let mut open = ui_state.show_equation_edit;
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
                ui.visuals_mut().override_text_color = Some(match ui_state.equation_res {
                    Ok(_) => egui::Color32::GREEN,
                    Err(_) => egui::Color32::RED,
                });
                ui.label(format!("\t{}", ui_state.to_string()));
            });

            ui.horizontal(|ui| {
                if ui.button("Balance Equation").clicked() && ui_state.equation_res.is_ok() {
                    let eq =
                        EquationBalancer::new(ui_state.equation_res.as_ref().unwrap()).balance();
                    ui_state.input = eq.equation().to_string();
                    ui_state.equation_res = Ok(eq);
                }

                // if the ok button is clicked, or enter is pressed, but only if the Equation is valid
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui_state.show_equation_edit = !((ui.button("Ok").clicked()
                        || ui.ctx().input().key_pressed(egui::Key::Enter))
                        && ui_state.equation_res.is_ok());
                    ui.add_space(10.0);
                });
            });
        });
    if !open {
        ui_state.show_equation_edit = open;
    }
}


impl Default for UiState {
    fn default() -> Self {
        Self {
            show_equation_edit: true,
            last_input: String::default(),
            input: "N2 + O2 <-> N2O2".to_string(),
            equation_res: Err(Error::WaitingForEquation),
        }
    }
}

impl ToString for UiState {
    fn to_string(&self) -> String {
        let res = self
            .equation_res
            .as_ref()
            .map(Equation::equation)
            .map_err(ToString::to_string);
        match res {
            Ok(s) => s.to_string(),
            Err(s) => s,
        }
    }
}

impl UiState {
    /// Go back to default
    fn reset(&mut self) {
        *self = Self::default();
    }
}
