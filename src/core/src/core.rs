use super::client;
use super::error;
use super::steam;

pub fn init() -> Result<bool, error::Error> {
    println!("Initializing Samrr...");

    let client = client::load();
    println!("client = {:?}", client);

    if !steam::is_installed()? {
        return Err(error::Error::SteamNotInstalled);
    }

    // let func = library::export(lib, "CreateInterface");
    // println!("func = {:?}", func);

    // TODO: check if steam is running.
    // TODO: check if user is connected.

    Ok(true)
}
