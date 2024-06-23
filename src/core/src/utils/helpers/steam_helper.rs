use std::io::{Error, ErrorKind};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

pub fn get_steam_path() -> Result<String, Error> {
    let hkey = RegKey::predef(HKEY_CURRENT_USER);

    let steam_key = hkey.open_subkey(r"Software\Valve\Steam").map_err(|_| {
        Error::new(
            ErrorKind::NotFound,
            format!("Failed to open Steam registry key"),
        )
    })?;

    let steam_path = steam_key.get_value("SteamPath").map_err(|_| {
        Error::new(
            ErrorKind::NotFound,
            format!("Failed to read Steam installation path"),
        )
    });

    steam_path
}

pub fn is_steam_installed() -> Result<bool, Error> {
    let path = get_steam_path()?;
    let dll_name = "steamclient.dll";

    // unsafe {

    let lib = super::library_helper::load(path, dll_name);
    println!("lib {:?}", lib);
    // let func: libloading::Symbol<unsafe extern fn(&str, u32) -> u32> = lib.get(b"CreateInterface").unwrap();

    // let result = func("SteamClient019", 0);

    // println!("result = {:?}", result);

    // }

    Ok(true)
}
