use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};
use chem_eq::balance::EquationBalancer;

use crate::AppState;

/// Store the apps curent state
#[derive(Debug, Clone, Resource)]
pub struct UiState {
    /// Show editing window
    pub show_equation_edit: bool,
    /// Input from last tick
    pub last_input: String,
    /// User input string
    pub input: String,
}

pub fn app_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
    mut exit: EventWriter<AppExit>,
) {
    // header
    egui::TopBottomPanel::top("header").show(egui_context.ctx_mut(), |ui| {
        ui.menu_button("Simulation", |ui| {
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
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
        ui.centered_and_justified(|ui| ui.heading("Le Chateliers Principle Simulation"));
    });

    // always show window when the equation is invalid
    if app_state.eq_res.is_err() {
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
                ui.visuals_mut().override_text_color = Some(match app_state.eq_res {
                    Ok(_) => egui::Color32::GREEN,
                    Err(_) => egui::Color32::RED,
                });
                ui.label(app_state.to_string());
            });

            ui.horizontal(|ui| {
                if ui.button("Balance Equation").clicked() && app_state.eq_res.is_ok() {
                    let eq = EquationBalancer::new(app_state.eq_res.as_ref().unwrap()).balance();
                    ui_state.input = eq.equation().to_string();
                    app_state.eq_res = Ok(eq);
                }

                // if the ok button is clicked, or enter is pressed, but only if the Equation is valid
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui_state.show_equation_edit = !((ui.button("Ok").clicked()
                        || ui.ctx().input().key_pressed(egui::Key::Enter))
                        && app_state.eq_res.is_ok());
                    ui.add_space(10.0);
                });
            });
        });
    if !open {
        ui_state.show_equation_edit = open;
    }

    // show concentrations of each compound
    egui::SidePanel::left("equation graphs").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Concentrations");
        ui.add_space(10.0);
        let Ok(eq) = &mut app_state.eq_res else {
            return;
        };

        for (name, cmp) in eq.name_and_concentration_mut() {
            use egui::plot::{Plot, Line, PlotPoints};

            ui.label(&name);
            let series: PlotPoints = (0..1000).map(|i| {
                let x = i as f64 * 0.01;
                [x, x.sin()]
            }).collect();
            let line = Line::new(series);
            Plot::new(name).view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));

            ui.add(egui::Slider::new(cmp, 0.0..=20.0));
            ui.add_space(20.0);
        }
    });
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_equation_edit: true,
            last_input: "N2 + O2 <-> N2O2".to_string(),
            input: "N2 + O2 <-> N2O2".to_string(),
        }
    }
}

impl UiState {
    /// Go back to default
    fn reset(&mut self) {
        *self = Self::default();
    }
}
