use std::path::Path;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::utils::error;

pub fn get_path() -> Result<String, error::Error> {
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
