use std::io::Error;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn get_steam_path() -> Result<String, Error> {
    let hkey = RegKey::predef(HKEY_CURRENT_USER);

    let steam_key = hkey.open_subkey(r"Software\Valve\Steam")?;
    let steam_path = steam_key.get_value("SteamPath");

    steam_path
}

pub fn is_steam_installed() -> Result<bool, Error> {
    let lib_path = get_steam_path()?;
    let lib_name = "steamclient64.dll";

    let lib = super::library_helper::load(lib_path, lib_name);
    println!("lib {:?}", lib);

    // unsafe {
    // let func: libloading::Symbol<unsafe extern fn(&str, u32) -> u32> = lib.get(b"CreateInterface").unwrap();
    // let result = func("SteamClient019", 0);
    // println!("result = {:?}", result);
    // }

    Ok(true)
}
