#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod client;
mod env;
mod error;
mod library;
mod path;
mod steam;

#[tauri::command]
fn ping(value: &str) -> String {
    format!("Received value '{}' by Rust!", value)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ping])
        .setup(|app| {
            match client::init() {
                Ok(true) => {
                    println!("Steam is installed.");
                    app.emit_all("success", "Steam is installed.").unwrap()
                }

                Ok(false) => {
                    println!("Steam is not installed.");
                    app.emit_all("error", error::Error::SteamNotInstalled.to_string())
                        .unwrap()
                }

                Err(e) => {
                    println!("[Error] {:?}", e);
                    app.emit_all("error", e.to_string()).unwrap()
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
