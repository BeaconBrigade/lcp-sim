mod eq_edit;
mod graph;
mod help;
mod menu;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use chem_eq::Equation;

use crate::error::Error;
pub use crate::ui::{eq_edit::eq_edit, graph::graph, help::help, menu::menu};

/// Store the apps curent state
#[derive(Debug, Clone, Resource, Inspectable)]
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
    #[inspectable(collapse)]
    pub eq_res: InsRes,
    /// k-expr
    #[inspectable(min = 0.0, max = 10.0)]
    pub eq_constant: f32,
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

/// Event to pause simulation
#[derive(Debug, Clone, Copy)]
pub struct PauseSimulation;

/// Event to stop simulation
#[derive(Debug, Clone, Copy)]
pub struct StopSimulation;

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_help: false,
            show_equation_edit: true,
            last_input: String::default(),
            input: "N2 + O2 <-> N2O2".to_string(),
            eq_res: Err(Error::WaitingForEquation).into(),
            eq_constant: 1.0,
        }
    }
}

impl UiState {
    /// Go back to default
    fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct InsRes(pub Result<Equation, Error>);

impl From<Result<Equation, Error>> for InsRes {
    fn from(r: Result<Equation, Error>) -> Self {
        Self(r)
    }
}

impl Default for InsRes {
    fn default() -> Self {
        Self(Err(Error::WaitingForEquation))
    }
}

impl Inspectable for InsRes {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        options: Self::Attributes,
        context: &mut bevy_inspector_egui::Context,
    ) -> bool {
        let Self(Ok(eq)) = self else {
            return false;
        };
        <Equation as Inspectable>::ui(eq, ui, options, context)
    }
}
