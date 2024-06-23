#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod utils;

#[tauri::command]
fn test(value: &str) -> String {
    format!("Received value '{}' by Rust!", value)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            match utils::helpers::steam_helper::init() {
                Ok(true) => window.emit("success", "Steam is installed.").unwrap(),
                Ok(false) => window
                    .emit("error", utils::enums::error_enum::Error::SteamNotInstalled.to_string())
                    .unwrap(),
                Err(e) => window.emit("error", e.to_string()).unwrap(),
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
