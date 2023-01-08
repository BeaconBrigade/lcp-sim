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

#[derive(Debug, Clone, Default)]
pub struct QuestionSystems(HashMap<usize, System>);

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
enum AppError {
    #[error("{0}")]
    Equation(#[from] EquationError),
    #[error("{0}")]
    System(#[from] SystemError),
    #[error("{0}")]
    Adjust(#[from] AdjustError),
    #[error("{0}")]
    Set(#[from] ConcentrationError),
    #[error("system not found")]
    NotFound,
}

fn main() {
    tauri::Builder::default()
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
fn quit(app_handle: tauri::AppHandle, code: i32) {
    app_handle.exit(code)
}

#[tauri::command]
fn close_splashscreen(window: tauri::Window) {
    // close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn add_system(
    state: tauri::State<Mutex<QuestionSystems>>,
    eq_str: &str,
    idx: usize,
    concentrations: Vec<f32>,
) -> Result<(), AppError> {
    let mut eq = Equation::new(eq_str)?;
    eq.set_concentrations(concentrations.as_slice())?;
    let system = System::new(eq)?;

    println!("Starting system {} with {}", idx, eq_str);
    state.lock().unwrap().0.insert(idx, system);

    Ok(())
}

#[tauri::command]
fn get_sys_concentration(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
) -> Option<Vec<f32>> {
    state
        .lock()
        .unwrap()
        .0
        .get(&idx)
        .map(|s| s.equation().get_concentrations())
}

#[tauri::command]
fn set_sys_concentration(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
    concentrations: Vec<f32>,
) -> Result<(), AppError> {
    println!("Setting concentrations for {}: {:?}", idx, concentrations);

    state
        .lock()
        .unwrap()
        .0
        .get_mut(&idx)
        .ok_or(AppError::NotFound)?
        .equation_mut()
        .set_concentrations(concentrations.as_slice())?;

    Ok(())
}

#[tauri::command]
fn update_system(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
    adjust: Adjustment,
) -> Result<(), AppError> {
    println!("Updating system {} with {:#?}", idx, adjust);

    state
        .lock()
        .unwrap()
        .0
        .get_mut(&idx)
        .ok_or(AppError::NotFound)?
        .adjust(adjust)?;

    Ok(())
}
