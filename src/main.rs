use std::fmt;

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use chem_eq::{balance::EquationBalancer, Direction, Equation};

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(update_equation)
        .add_system(app_ui)
        .run();
}

fn app_ui(
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

            let mut eq_is_good = false;
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(match ui_state.equation_res {
                    Ok(_) => {
                        eq_is_good = true;
                        egui::Color32::GREEN
                    }
                    Err(_) => {
                        eq_is_good = true;
                        egui::Color32::RED
                    }
                });
                ui.label(format!("balanced: {}", ui_state.to_string()));
            });

            ui.horizontal(|ui| {
                if ui.button("Balance Equation").clicked() && eq_is_good {
                    let eq =
                        EquationBalancer::new(ui_state.equation_res.as_ref().unwrap()).balance();
                    ui_state.input = eq.equation().to_string();
                    ui_state.equation_res = Ok(eq);
                }

                // if the ok button is clicked, or enter is pressed, but only if the Equation is valid
                ui_state.show_equation_edit = !((ui.button("Ok").clicked()
                    || ui.ctx().input().key_pressed(egui::Key::Enter))
                    && ui_state.equation_res.is_ok());
            });
        });
    if !open {
        ui_state.show_equation_edit = open;
    }
}

/// If input has changed, try to balance the equation and update the output
fn update_equation(mut ui_state: ResMut<UiState>) {
    // don't rerun if it hasn't changed
    if ui_state.input == ui_state.last_input {
        return;
    }
    ui_state.last_input = ui_state.input.clone();
    info!("Updating equation...");
    if ui_state.input.is_empty() {
        ui_state.equation_res = Err(Error::WaitingForEquation);
        return;
    }

    let res = Equation::new(ui_state.input.as_str());
    let Ok(eq) = res else {
        ui_state.equation_res = res.map_err(Into::into);
        return;
    };
    let balancer = EquationBalancer::new(&eq);
    let eq = balancer.balance();

    // must be reversible to be an equilibrium
    ui_state.equation_res = if *eq.direction() == Direction::Reversible {
        Ok(eq)
    } else {
        Err(Error::NotEquilibrium)
    }
}

/// Store the apps curent state
#[derive(Debug, Clone, Resource)]
struct UiState {
    /// Show editing window
    show_equation_edit: bool,
    /// Input from last tick
    last_input: String,
    /// User input string
    input: String,
    /// Result of trying to balance user input
    equation_res: Result<Equation, Error>,
}

#[derive(Debug, Clone)]
enum Error {
    ChemEq(chem_eq::Error),
    NotEquilibrium,
    WaitingForEquation,
}

impl From<chem_eq::Error> for Error {
    fn from(e: chem_eq::Error) -> Self {
        Self::ChemEq(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ChemEq(e) => e.to_string(),
                Self::NotEquilibrium => "Not an equilibrium".to_string(),
                Self::WaitingForEquation => "Waiting for equation...".to_string(),
            }
        )
    }
}

impl std::error::Error for Error {}

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
