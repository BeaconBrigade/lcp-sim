mod eq_edit;
mod graph;
mod help;
mod menu;

use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContext};

use crate::AppState;

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
}

pub fn app_ui(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut app_state: ResMut<AppState>,
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
        menu::menu(ui, &mut ui_state, &mut app_state, exit);
    });

    // always show window when the equation is invalid
    if app_state.eq_res.is_err() {
        ui_state.show_equation_edit = true;
    }

    // equation editor
    let mut open = ui_state.show_equation_edit;
    egui::Window::new("Choose an Equation")
        .collapsible(false)
        .resizable(true)
        .open(&mut open)
        .show(egui_context.ctx_mut(), |ui| {
            eq_edit::eq_edit(ui, &mut ui_state, &mut app_state);
        });
    if !open {
        ui_state.show_equation_edit = open;
    }

    // show concentrations of each compound and graph it
    egui::SidePanel::left("equation graphs").show(egui_context.ctx_mut(), |ui| {
        graph::graph(ui, &mut app_state)
    });
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_help: false,
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
