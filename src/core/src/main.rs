#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::u32;

use std::env;

mod utils;

#[tauri::command]
fn test(value: &str) -> String {
    format!("Received value '{}' by Rust!", value)
}

fn is_steam_running() -> bool {
    println!("TODO: Checking an instance of Steam is running...");
    false
}

fn is_steam_user_connected() -> bool {
    println!("TODO: Checking a user is connected to Steam...");
    false
}

fn load_user() -> () {
    println!("TODO: Loading connected Steam user...")
}

fn load_games() -> () {
    println!("TODO: Loading games...");

    load_user();
    load_game_ids();
    get_user_games();
}

fn load_game_ids() -> () {
    println!("TODO: Loading all game IDs...");
}

fn get_user_games() -> () {
    println!("TODO: Filtering out games that are not owned by the user...");

    load_game(10);
    load_game(480);
}

fn load_game(id: u32) -> () {
    println!("TODO: Loading info for game ID {}...", id);
}

fn main() {
    println!("Starting Samrr...");

    match utils::helpers::steam_helper::is_steam_installed() {
        Ok(installed) => {
            if !installed {
                io::Error::new(
                    io::ErrorKind::Other,
                    "Could not detect an installed version of Steam",
                );
            }
        }
        Err(e) => {
            println!("[Error] {}", e);
            return;
        }
    };

    if !is_steam_running() {
        println!("Error: Could not detect a running instance of Steam.");
        return;
    }

    if !is_steam_user_connected() {
        println!("Error: Could a connected user on Steam.");
        return;
    }

    load_games();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
