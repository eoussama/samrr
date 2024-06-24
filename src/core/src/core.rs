use super::client;
use super::error;
use super::steam;

pub fn init() -> Result<bool, error::Error> {
    println!("Initializing Samrr...");

    if !steam::is_installed()? {
        return Err(error::Error::SteamNotInstalled);
    }

    let client = client::load()?;
    println!("[client] {:?}", client);

    if !client {
        return Err(error::Error::ClientNotInitialized);
    }

    // TODO: check if steam is running.
    // TODO: check if user is connected.

    Ok(true)
}
