use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText},
    EguiContext,
};

use crate::ui::UiState;

pub fn help(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    egui::Window::new("Help")
        .open(&mut ui_state.show_help)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Simulation Controls");
            ui.add_space(5.0);
            ui.label("Look to the left side to see the controls for the simulation");
            ui.label("Use the scroll bars to adjust concentrations for each compound");
            ui.label("The graphs show each compounds concentration changing over time");
            ui.add_space(10.0);

            ui.heading("Equation controls");
            ui.add_space(5.0);
            ui.label("To access the equation editing controls through the menu:");
            ui.label(RichText::new("  Simulation -> Edit Equation").italics());
            ui.label("To restore the default equation in the menu:");
            ui.label(RichText::new("  Simulation -> Restore Defaults").italics());
            ui.add_space(10.0);

            ui.heading("lcp-sim");
            ui.add_space(5.0);
            ui.label("To leave, through the menu:");
            ui.label(RichText::new("  lcp-sim -> Quit").italics());
        });
}
