#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
mod utils;

#[tauri::command]
fn ping(value: &str) -> String {
    format!("Received value '{}' by Rust!", value)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            match utils::helpers::client_helper::init() {
                Ok(true) => {
                    println!("Steam is installed.");
                    window.emit("success", "Steam is installed.").unwrap()
                }

                Ok(false) => {
                    println!("Steam is not installed.");
                    window
                        .emit(
                            "error",
                            utils::enums::error_enum::Error::SteamNotInstalled.to_string(),
                        )
                        .unwrap()
                }

                Err(e) => {
                    println!("[Error] {:?}", e);
                    window.emit("error", e.to_string()).unwrap()
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
