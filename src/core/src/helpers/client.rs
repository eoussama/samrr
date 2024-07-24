use libloading::Library;

use crate::helpers::library;
use crate::helpers::steam;
use crate::utils::error;
use crate::wrappers::i_steam_client_018;

fn load_steam_client() -> Result<Library, error::Error> {
    let lib_path = steam::get_path()?;
    let lib_name = "steamclient.dll";

    let lib = library::load(lib_path, lib_name);

    lib
}

pub fn load() -> Result<bool, error::Error> {
    let lib = load_steam_client()?;
    dbg!(&lib);

    let steam_client = i_steam_client_018::init(&lib);
    dbg!(&steam_client);

    Ok(true)
}
