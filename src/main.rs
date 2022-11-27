mod ui;
mod error;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use chem_eq::{balance::EquationBalancer, Direction, Equation};

use crate::{ui::UiState, error::Error};

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(update_equation)
        .add_system(ui::app_ui)
        .run();
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

