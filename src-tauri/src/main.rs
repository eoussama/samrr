#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn is_steam_installed() -> bool {
    false
}

fn is_steam_running() -> bool {
    false
}

fn is_steam_user_connected() -> bool {
    false
}

fn main() {
    if !is_steam_installed() {
        println!("Error: Could not detected an installed version of Steam.");
        return;
    }

    if !is_steam_running() {
        println!("Error: Could not detect a running instance of Steam.");
        return;
    }

    if !is_steam_user_connected() {
        println!("Error: Could a connected user on Steam.");
        return;
    }

    println!("Starting Samrr...");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
