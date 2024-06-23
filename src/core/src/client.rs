use super::error;
use super::steam;
use super::library;

pub fn init() -> Result<bool, error::Error> {
    println!("Initializing Samrr...");

    if !steam::is_installed()? {
        return Err(error::Error::SteamNotInstalled);
    }

    let lib = steam::load_client()?;
    println!("lib = {:?}", lib);

    let func = library::export(lib, "CreateInterface");
    println!("func = {:?}", func);

    // TODO: check if steam is running.
    // TODO: check if user is connected.

    Ok(true)
}
