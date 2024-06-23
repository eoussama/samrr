use super::super::enums::error_enum::Error;
use super::steam_helper;

pub fn init() -> Result<bool, Error> {
    println!("Initializing Samrr...");

    if !steam_helper::is_installed()? {
        return Err(Error::SteamNotInstalled);
    }

    // TODO: check if steam is running.
    // TODO: check if user is connected.

    // TODO: load library.
    // TODO: wrap apis.
    let client = steam_helper::load_client();
    println!("client = {:?}", client);

    Ok(true)
}
