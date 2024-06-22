#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use libloading::Library;
use std::io;
use std::u32;
use winreg::enums::*;
use winreg::RegKey;

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[tauri::command]
fn test(value: &str) -> String {
    format!("Received value '{}' by Rust!", value)
}

fn get_steam_path() -> Result<String, io::Error> {
    let hkey = RegKey::predef(HKEY_CURRENT_USER);

    let steam_key = hkey.open_subkey("Software\\Valve\\Steam").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to open Steam registry key"),
        )
    })?;

    let steam_path = steam_key.get_value("SteamPath").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read Steam installation path"),
        )
    });

    steam_path
}

fn load_library(dir_path: &str, dll_name: &str) -> Result<Library, libloading::Error> {
    // let mut path = PathBuf::from(dir_path);
    // path.push(dll_name);
    // let path_str: &OsStr = path.as_os_str();

    // unsafe {
    //     Library::new(path_str)
    // }

    let bin_path = Path::new(dir_path).join("bin");

    if let Some(mut current_path) = env::var_os("PATH") {
        current_path.push(";");
        current_path.push(r"D:\games\Steam");
		current_path.push(";");
		current_path.push(r"D:\games\Steam\bin");
        env::set_var("PATH", current_path);

        match env::var_os("PATH") {
            Some(value) => println!("current_path = {:?}", value),
            None => println!("current_path err"),
        }
    }

    let path_str = String::from(dir_path) + "/" + dll_name;

    println!("path_str = {:?}", path_str);
    unsafe { Library::new(r"D:\games\Steam\steamclient.dll") }
}

fn is_steam_installed() -> Result<bool, io::Error> {
    let path = get_steam_path()?;
    let dll_name = "steamclient.dll";

    // unsafe {

    let lib = load_library(&path, dll_name);
    println!("lib {:?}", lib);
    // let func: libloading::Symbol<unsafe extern fn(&str, u32) -> u32> = lib.get(b"CreateInterface").unwrap();

    // let result = func("SteamClient019", 0);

    // println!("result = {:?}", result);

    // }

    Ok(true)
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

    match is_steam_installed() {
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
