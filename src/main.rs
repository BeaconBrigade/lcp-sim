#![cfg_attr(not(debug_assertions), windows_subsytem = "windows")]

mod concentration;
mod error;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use chem_eq::{balance::EquationBalancer, Direction, Equation};

use crate::{error::Error, ui::UiState};

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(update_equation)
        .add_system(ui::app_ui)
        .run();
}

/// try to balance the equation and update the output
fn update_equation(mut ui_state: ResMut<UiState>) {
    // don't rerun if input hasn't changed
    if ui_state.input == ui_state.last_input {
        return;
    }
    ui_state.last_input = ui_state.input.clone();
    if ui_state.input.is_empty() {
        ui_state.eq_res = Err(Error::WaitingForEquation);
        return;
    }

    let res = Equation::new(ui_state.input.as_str());
    let Ok(eq) = res else {
        ui_state.eq_res = res.map_err(Into::into);
        return;
    };
    let res = eq.to_balancer().balance();
    let Ok(eq) = res else {
        ui_state.eq_res = res.map_err(Into::into);
        return;
    };

    // must be reversible to be an equilibrium
    ui_state.eq_res = if *eq.direction() == Direction::Reversible {
        Ok(eq)
    } else {
        Err(Error::NotEquilibrium)
    }
}
