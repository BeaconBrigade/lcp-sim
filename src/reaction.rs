use bevy::{ecs::schedule::ShouldRun, prelude::*};
// use chem_eq::Equation;

use crate::{components::Particle, ui::UiState, AppState};

/// Find reactions while equilibrium is stable
pub fn reaction_continuation(ui_state: Res<UiState>, mut query: Query<&Particle>) {
    let Ok(eq) = ui_state.eq_res.as_ref() else {
        return;
    };
    for particle in query.iter_mut() {
        debug!(particle = ?eq.nth_compound(particle.compound_index));
    }
}

/// Get equilibrium to stable from start
pub fn start_reaction() {}

/// Calculate changes to equilibrium
pub fn adjust_equilibrium() {}

/// If the system is running or adjusting, it has finished setting up
pub fn system_is_running(app_state: Res<AppState>) -> ShouldRun {
    if app_state.running.is_running() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

/// If the system hasn't reached the initial equilibrium yet
pub fn system_is_setting_up(app_state: Res<AppState>) -> ShouldRun {
    if app_state.running.is_setting_up() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}