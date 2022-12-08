mod eq_edit;
mod graph;
mod help;
mod menu;

use bevy::prelude::*;
use chem_eq::Equation;

use crate::error::Error;
pub use crate::ui::{help::help, menu::menu, eq_edit::eq_edit, graph::graph};

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

/// Event to start simulation
#[derive(Debug, Clone, Copy)]
pub struct StartSimulation;

/// Event to start simulation
#[derive(Debug, Clone, Copy)]
pub struct PauseSimulation;

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
