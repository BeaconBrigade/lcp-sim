#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::Mutex};

use chatelier::{AdjustError, Adjustment, System, SystemError};
use chem_eq::{error::EquationError, Equation};
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
    eq: &str,
    idx: usize,
) -> Result<(), AppError> {
    let eq = Equation::new(eq)?;
    let system = System::new(eq)?;

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
fn update_system(
    state: tauri::State<Mutex<QuestionSystems>>,
    idx: usize,
    adjust: Adjustment,
) -> Result<(), AppError> {
    state
        .lock()
        .unwrap()
        .0
        .get_mut(&idx)
        .map(|s| s.adjust(adjust))
        .ok_or(AppError::NotFound)??;

    Ok(())
}
