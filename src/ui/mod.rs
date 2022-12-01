mod eq_edit;
mod graph;
mod help;
mod menu;

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};
use chem_eq::Equation;

use crate::error::Error;

/// Store the apps curent state
#[derive(Debug, Clone, Resource)]
pub struct UiState {
    /// Show help window
    pub show_help: bool,
    /// Show editing window
    pub show_equation_edit: bool,
    /// Input from last tick
    pub last_input: String,
    /// User input string
    pub input: String,
    /// Result of trying to balance equation
    pub eq_res: Result<Equation, Error>,
}

impl ToString for UiState {
    fn to_string(&self) -> String {
        let res = self
            .eq_res
            .as_ref()
            .map(Equation::equation)
            .map_err(ToString::to_string);
        match res {
            Ok(s) => s.to_string(),
            Err(s) => s,
        }
    }
}

pub fn app_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    exit: EventWriter<AppExit>,
) {
    // help
    egui::Window::new("Help")
        .open(&mut ui_state.show_help)
        .show(egui_context.ctx_mut(), |ui| {
            help::help(ui);
        });

    // header
    egui::TopBottomPanel::top("header").show(egui_context.ctx_mut(), |ui| {
        menu::menu(ui, &mut ui_state, exit);
    });

    // always show window when the equation is invalid
    if ui_state.eq_res.is_err() {
        ui_state.show_equation_edit = true;
    }

    // equation editor
    let mut open = ui_state.show_equation_edit;
    egui::Window::new("Choose an Equation")
        .collapsible(false)
        .resizable(true)
        .open(&mut open)
        .show(egui_context.ctx_mut(), |ui| {
            eq_edit::eq_edit(ui, &mut ui_state);
        });
    if !open {
        ui_state.show_equation_edit = open;
    }

    // show concentrations of each compound and graph it
    egui::SidePanel::left("equation graphs").show(egui_context.ctx_mut(), |ui| {
        graph::graph(ui, &mut ui_state);
    });
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_help: false,
            show_equation_edit: true,
            last_input: String::default(),
            input: "N2 + O2 <-> N2O2".to_string(),
            eq_res: Err(Error::WaitingForEquation),
        }
    }
}

impl UiState {
    /// Go back to default
    fn reset(&mut self) {
        *self = Self::default();
    }
}
