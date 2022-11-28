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
        .init_resource::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(update_equation)
        .add_system(ui::app_ui)
        .run();
}

#[derive(Debug, Resource)]
pub struct AppState {
    pub eq_res: Result<Equation, Error>,
}

impl ToString for AppState {
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

impl Default for AppState {
    fn default() -> Self {
        let mut eq = Equation::new("N2 + O2 <-> N2O2").unwrap();
        for cnc in eq.concentrations_mut().take(2) {
            *cnc = 1.0;
        }

        Self { eq_res: Ok(eq) }
    }
}

/// If input has changed, try to balance the equation and update the output
fn update_equation(mut ui_state: ResMut<UiState>, mut app_state: ResMut<AppState>) {
    // don't rerun if it hasn't changed
    if ui_state.input == ui_state.last_input {
        return;
    }
    ui_state.last_input = ui_state.input.clone();
    if ui_state.input.is_empty() {
        app_state.eq_res = Err(Error::WaitingForEquation);
        return;
    }

    let res = Equation::new(ui_state.input.as_str());
    let Ok(eq) = res else {
        app_state.eq_res = res.map_err(Into::into);
        return;
    };
    let balancer = EquationBalancer::new(&eq);
    let eq = balancer.balance();

    // must be reversible to be an equilibrium
    app_state.eq_res = if *eq.direction() == Direction::Reversible {
        Ok(eq)
    } else {
        Err(Error::NotEquilibrium)
    }
}
