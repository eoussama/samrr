use std::path::Path;

use libloading::Library;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use super::error;
use super::library;

fn get_path() -> Result<String, error::Error> {
    let hkey = RegKey::predef(HKEY_CURRENT_USER);

    let steam_key = hkey
        .open_subkey(r"Software\Valve\Steam")
        .map_err(|_| error::Error::Other(format!("could not read Steam registry key")))
        .unwrap();

    let steam_path = steam_key
        .get_value("SteamPath")
        .map_err(|_| error::Error::Other(format!("could not read Steam path")));

    steam_path
}

pub fn is_installed() -> Result<bool, error::Error> {
    let lib_path = get_path()?;
    let exists = Path::new(&lib_path).exists();

    Ok(exists)
}

pub fn load_client() -> Result<Library, error::Error> {
    let lib_path = get_path()?;
    let lib_name = "steamclient64.dll";

    let lib = library::load(lib_path, lib_name);

    lib
}
