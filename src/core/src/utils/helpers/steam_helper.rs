use std::path::Path;

use libloading::Library;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use super::super::enums::error_enum::Error;

fn get_path() -> Result<String, Error> {
    let hkey = RegKey::predef(HKEY_CURRENT_USER);

    let steam_key = hkey
        .open_subkey(r"Software\Valve\Steam")
        .map_err(|_| -> Error { Error::Other(format!("could not read Steam registry key")) })
        .unwrap();

    let steam_path = steam_key
        .get_value("SteamPath")
        .map_err(|_| -> Error { Error::Other(format!("could not read Steam path")) });

    steam_path
}

pub fn is_installed() -> Result<bool, Error> {
    let lib_path = get_path()?;
    let exists = Path::new(&lib_path).exists();

    Ok(exists)
}

pub fn load_client() -> Result<Library, Error> {
    let lib_path = get_path()?;
    let lib_name = "steamclient64.dll";

    let lib = super::library_helper::load(lib_path, lib_name);

    lib

    // unsafe {
    // let func: libloading::Symbol<unsafe extern fn(&str, u32) -> u32> = lib.get(b"CreateInterface").unwrap();
    // let result = func("SteamClient019", 0);
    // println!("result = {:?}", result);
    // }
}
