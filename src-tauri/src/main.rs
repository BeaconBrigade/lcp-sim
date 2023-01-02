#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, quit, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}", name)
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
