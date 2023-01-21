#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::Mutex};

use chatelier::{AdjustError, Adjustment, System, SystemError};
use chem_eq::{
    error::{ConcentrationError, EquationError},
    Equation,
};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use thiserror::Error;
use tracing::{info, instrument};

#[derive(Debug, Clone, Default)]
pub struct QuestionSystems(HashMap<usize, System>);

impl std::ops::Deref for QuestionSystems {
    type Target = HashMap<usize, System>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for QuestionSystems {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
enum AppError {
    #[error("{0}")]
    Equation(#[from] EquationError),
    #[error("{0}")]
    System(#[from] SystemError),
    #[error("{0}")]
    Adjust(#[from] AdjustError),
    #[error("{0}")]
    ConcentrationSet(#[from] ConcentrationError),
    #[error("system not found")]
    SystemNotFound,
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            tracing_subscriber::fmt().init();
            Ok(())
        })
        .manage(Mutex::new(QuestionSystems::default()))
        .invoke_handler(tauri::generate_handler![
            quit,
            close_splashscreen,
            add_system,
            get_sys_concentration,
            set_sys_concentration,
            update_system,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[instrument(skip(app_handle))]
fn quit(app_handle: tauri::AppHandle, code: i32) {
    app_handle.exit(code)
}

#[tauri::command]
#[instrument(skip(window))]
fn close_splashscreen(window: tauri::Window) {
    // close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
#[instrument(skip(state))]
fn add_system(
    state: tauri::State<Mutex<QuestionSystems>>,
    eq_str: &str,
    idx: usize,
    concentrations: Vec<f32>,
    reset: bool,
) -> Result<(), AppError> {
    let exists = state.lock().unwrap().get(&idx).is_some();
    if exists && !reset {
        info!("System {} already exists, doing nothing", idx);
        return Ok(());
    } else if exists {
        info!("Reseting system {}", idx);
    }
    let mut eq = Equation::new(eq_str)?;
    eq.set_concentrations(concentrations.as_slice())?;
    let system = System::new(eq)?;

    info!("Starting system {}", idx);
    state.lock().unwrap().insert(idx, system);

    Ok(())
}

#[tauri::command]
#[instrument(skip(state))]
fn get_sys_concentration(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
) -> Option<Vec<f32>> {
    state
        .lock()
        .unwrap()
        .get(&idx)
        .map(|s| s.equation().get_concentrations())
}

#[tauri::command]
#[instrument(skip(state))]
fn set_sys_concentration(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
    concentrations: Vec<f32>,
) -> Result<(), AppError> {
    info!("Setting concentrations for {}: {:?}", idx, concentrations);

    state
        .lock()
        .unwrap()
        .get_mut(&idx)
        .ok_or(AppError::SystemNotFound)?
        .equation_mut()
        .set_concentrations(concentrations.as_slice())?;

    Ok(())
}

#[tauri::command]
#[instrument(skip(state))]
fn update_system(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
    adjust: Adjustment,
) -> Result<(), AppError> {
    info!("Updating system {} with {:?}...", idx, adjust);

    state
        .lock()
        .unwrap()
        .get_mut(&idx)
        .ok_or(AppError::SystemNotFound)?
        .adjust(adjust)?;

    info!("Finished adjusting system");

    Ok(())
}
